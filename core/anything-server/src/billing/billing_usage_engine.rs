use postgrest::Postgrest;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::sync::Arc;
use tokio::time::{sleep, Duration};

use crate::AppState;

#[derive(Debug, Serialize, Deserialize)]
struct AccountUsage {
    account_id: String,
    total_execution_time_ms: i64,
    task_count: i64,
}

pub async fn billing_processing_loop(state: Arc<AppState>) {
    println!("[BILLING_USAGE_ENGINE] Starting billing processing engine");
    let client = state.anything_client.clone();
    let interval = Duration::from_secs(300); // 1 minute

    loop {
        match process_billing_usage(&client).await {
            Ok(_) => println!("[BILLING_USAGE_ENGINE] Billing usage processed successfully"),
            Err(e) => eprintln!(
                "[BILLING_USAGE_ENGINE] Error processing billing usage: {}",
                e
            ),
        }
        sleep(interval).await;
    }
}

async fn process_billing_usage(client: &Postgrest) -> Result<(), Box<dyn Error + Send + Sync>> {
    let supabase_service_role_api_key = env::var("SUPABASE_SERVICE_ROLE_API_KEY")?;

    println!("[BILLING_USAGE_ENGINE] Processing billing usage");
    // Aggregate usage data
    let response = client
        .from("tasks_billing")
        .auth(&supabase_service_role_api_key)
        .select("task_id, account_id, execution_time_ms, task_status")
        .eq("task_status", "completed")
        .eq("usage_reported_to_billing_provider", "false")
        .execute()
        .await?;

    let tasks: Vec<HashMap<String, serde_json::Value>> = response.json().await?;

    println!("[BILLING_USAGE_ENGINE] Retrieved {} tasks", tasks.len());

    // Collect task_ids first
    let task_ids: Vec<String> = tasks
        .iter()
        .filter_map(|task| {
            task.get("task_id")
                .and_then(|id| id.as_str().map(String::from))
        })
        .collect();

    println!(
        "[BILLING_USAGE_ENGINE] Collected {} task IDs",
        task_ids.len()
    );

    let mut account_usage: HashMap<String, AccountUsage> = HashMap::new();

    for task in &tasks {
        println!("[BILLING_USAGE_ENGINE] Processing task: {:?}", task);

        let account_id = task["account_id"].as_str().unwrap_or("unknown").to_string();
        let execution_time_ms = task["execution_time_ms"].as_i64().unwrap_or(0);

        println!(
            "[BILLING_USAGE_ENGINE] Task details - Account ID: {}, Execution Time: {}ms",
            account_id, execution_time_ms
        );

        account_usage
            .entry(account_id.clone())
            .and_modify(|usage| {
                usage.total_execution_time_ms += execution_time_ms;
                usage.task_count += 1;
            })
            .or_insert(AccountUsage {
                account_id,
                total_execution_time_ms: execution_time_ms,
                task_count: 1,
            });
    }

    println!(
        "[BILLING_USAGE_ENGINE] Aggregated usage for {} accounts",
        account_usage.len()
    );

    // Update accounts_billing table and send usage to Stripe
    for (account_id, usage) in account_usage {
        println!("[BILLING_USAGE_ENGINE] Processing account: {}", account_id);
        let billing_info = client
            .from("accounts_billing")
            .auth(&supabase_service_role_api_key)
            .select("*")
            .eq("account_id", &account_id)
            .single()
            .execute()
            .await?;

        let billing_data: HashMap<String, serde_json::Value> = billing_info.json().await?;

        println!(
            "[BILLING_USAGE_ENGINE] Billing data for account {}: {:?}",
            account_id, billing_data
        );

        let free_trial_task_limit = billing_data["free_trial_task_limit"]
            .as_i64()
            .unwrap_or(1000);
        let free_trial_task_usage = billing_data["free_trial_task_usage"].as_i64().unwrap_or(0);
        let trial_ended = billing_data["trial_ended"].as_bool().unwrap_or(false);
        let new_free_trial_task_usage = free_trial_task_usage + usage.task_count;
        let new_total_task_usage =
            billing_data["total_task_usage"].as_i64().unwrap_or(0) + usage.task_count;
        let new_total_execution_time_ms = billing_data["total_execution_time_ms"]
            .as_i64()
            .unwrap_or(0)
            + usage.total_execution_time_ms;

        let mut update_data = json!({
            "free_trial_task_usage": new_free_trial_task_usage,
            "total_task_usage": new_total_task_usage,
            "total_execution_time_ms": new_total_execution_time_ms
        });

        // Check if free trial time is over
        let current_time = chrono::Utc::now();
        let free_trial_ends_at = billing_data["free_trial_ends_at"]
            .as_str()
            .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&chrono::Utc));

        // Check if free trial tasks are over
        if !trial_ended
            && (new_free_trial_task_usage > free_trial_task_limit
                || free_trial_ends_at.map_or(false, |end| current_time > end))
        {
            update_data["trial_ended"] = json!(true);
            println!(
                "[BILLING_USAGE_ENGINE] Trial ended for account {}",
                account_id
            );
        }

        // Send usage to Stripe
        if let Ok(()) = send_usage_to_stripe(client, &account_id, usage).await {
            // Only update accounts_billing if send_usage_to_stripe is successful
            println!(
                "[BILLING_USAGE_ENGINE] Updating accounts_billing for account {}",
                account_id
            );
            client
                .from("accounts_billing")
                .auth(&supabase_service_role_api_key)
                .eq("account_id", &account_id)
                .update(json!(update_data).to_string())
                .execute()
                .await?;
        } else {
            eprintln!(
                "[BILLING_USAGE_ENGINE] Failed to send usage to Stripe for account {}",
                account_id
            );
        }
    }

    if !task_ids.is_empty() {
        println!(
            "[BILLING_USAGE_ENGINE] Updating usage_reported_to_billing_provider for {} tasks",
            task_ids.len()
        );

        let update_data = json!({
            "usage_reported_to_billing_provider": true
        });

        let update_response = client
            .from("tasks_billing")
            .auth(&supabase_service_role_api_key)
            .in_("task_id", task_ids)
            .update(update_data.to_string())
            .execute()
            .await?;

        match update_response.text().await {
            Ok(response_text) => {
                println!(
                    "[BILLING_USAGE_ENGINE] Update response text: {}",
                    response_text
                );
                match serde_json::from_str::<serde_json::Value>(&response_text) {
                    Ok(parsed_response) => {
                        println!(
                            "[BILLING_USAGE_ENGINE] Parsed update response: {:?}",
                            parsed_response
                        );
                    }
                    Err(e) => {
                        println!(
                            "[BILLING_USAGE_ENGINE] Failed to parse update response: {}",
                            e
                        );
                    }
                }
            }
            Err(e) => {
                println!(
                    "[BILLING_USAGE_ENGINE] Failed to read update response text: {}",
                    e
                );
            }
        }

        // println!("[BILLING_USAGE_ENGINE] Update response: {:?}", update_response);
        // println!("[BILLING_USAGE_ENGINE] Updating usage_reported_to_billing_provider for {} tasks", task_ids.len());
        // let upsert_data: Vec<serde_json::Value> = task_ids
        //     .iter()
        //     .map(|task_id| {
        //         json!({
        //             "task_id": task_id,
        //             "usage_reported_to_billing_provider": true
        //         })
        //     })
        //     .collect();

        // println!("[BILLING_USAGE_ENGINE] Upsert data: {:?}", upsert_data);

        // let upsert_response = client
        //     .from("tasks_billing")
        //     .auth(&supabase_service_role_api_key)
        //     .upsert(serde_json::to_string(&upsert_data)?)
        //     .execute()
        //     .await?;

        // match upsert_response.text().await {
        //     Ok(response_text) => {
        //         println!("[BILLING_USAGE_ENGINE] Upsert response text: {}", response_text);
        //         match serde_json::from_str::<serde_json::Value>(&response_text) {
        //             Ok(parsed_response) => {
        //                 println!("[BILLING_USAGE_ENGINE] Parsed upsert response: {:?}", parsed_response);
        //             },
        //             Err(e) => {
        //                 println!("[BILLING_USAGE_ENGINE] Failed to parse upsert response: {}", e);
        //             }
        //         }
        //     },
        //     Err(e) => {
        //         println!("[BILLING_USAGE_ENGINE] Failed to read upsert response text: {}", e);
        //     }
        // }

        // println!("[BILLING_USAGE_ENGINE] Upsert response: {:?}", upsert_response);
    } else {
        println!("[BILLING_USAGE_ENGINE] No billing_tasks to update");
    }

    Ok(())
}

async fn send_usage_to_stripe(
    client: &Postgrest,
    account_id: &str,
    usage: AccountUsage,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let supabase_service_role_api_key = env::var("SUPABASE_SERVICE_ROLE_API_KEY")?;
    // Fetch the Stripe customer ID from the accounts_billing table
    let response = match client
        .from("accounts_billing")
        .auth(&supabase_service_role_api_key)
        .select("stripe_customer_id")
        .eq("account_id", account_id)
        .single()
        .execute()
        .await
    {
        Ok(response) => response,
        Err(_) => return Err("Failed to execute request".into()),
    };

    let body = match response.text().await {
        Ok(body) => body,
        Err(_) => return Err("Failed to read response body".into()),
    };

    let item: Value = match serde_json::from_str(&body) {
        Ok(item) => item,
        Err(_) => return Err("Failed to parse JSON".into()),
    };

    let stripe_customer_id = item["stripe_customer_id"]
        .as_str()
        .ok_or("Stripe customer ID not found")?
        .to_string();

    if stripe_customer_id.is_empty() {
        return Err("Stripe customer ID is empty".into());
    }

    println!(
        "[BILLING_USAGE_ENGINE] Fetched Stripe customer ID for account {}: {}",
        account_id, stripe_customer_id
    );
    // Implement Stripe API call here
    println!(
        "[BILLING_USAGE_ENGINE] Sending usage to Stripe for account {}: {:?}",
        account_id, usage
    );

    // Handle the new account creation
    let stripe_secret_key = std::env::var("STRIPE_SECRET_KEY")
        .map_err(|_| "Stripe secret key not found".to_string())?;

    let client = reqwest::Client::new();

    let response = client
        .post("https://api.stripe.com/v1/billing/meter_events")
        .header("Authorization", format!("Bearer {}", stripe_secret_key))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .form(&[
            ("event_name", "anything_tasks"),
            ("payload[value]", &usage.task_count.to_string()),
            ("payload[stripe_customer_id]", &stripe_customer_id),
        ])
        .send()
        .await?;

    if !response.status().is_success() {
        let error_text = response.text().await?;
        println!(
            "[BILLING_USAGE_ENGINE] Failed to create meter event: {}",
            error_text
        );
        return Err(format!("Failed to create meter event: {}", error_text).into());
    }

    let meter_event: serde_json::Value = response.json().await?;
    println!(
        "[BILLING_USAGE_ENGINE] Created meter event: {:?}",
        meter_event
    );

    Ok(())
}
