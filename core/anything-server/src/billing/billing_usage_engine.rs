use postgrest::Postgrest;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use std::env;
use std::sync::Arc;
use tokio::time::{sleep, Duration};
use stripe::{UsageRecord};

use crate::AppState;

#[derive(Debug, Serialize, Deserialize)]
struct AccountUsage {
    account_id: String,
    total_execution_time_ms: i64,
    task_count: i64,
}

pub async fn billing_processing_loop(state: Arc<AppState>) {
    let client = state.anything_client.clone();
    let interval = Duration::from_secs(300); // 5 minutes

    loop {
        if let Err(e) = process_billing_usage(&client).await {
            eprintln!(
                "[BILLING_USAGE_ENGINE] Error processing billing usage: {:?}",
                e
            );
        }
        sleep(interval).await;
    }
}

async fn process_billing_usage(client: &Postgrest) -> Result<(), Box<dyn std::error::Error>> {
    let supabase_service_role_api_key = env::var("SUPABASE_SERVICE_ROLE_API_KEY")?;

    // Aggregate usage data
    let response = client
        .from("tasks_billing")
        .auth(&supabase_service_role_api_key)
        .select("account_id, execution_time_ms, task_status")
        .eq("task_status", "Completed")
        .eq("usage_reported_to_billing_provider", "false")
        .execute()
        .await?;

    let tasks: Vec<HashMap<String, serde_json::Value>> = response.json().await?;

    let mut account_usage: HashMap<String, AccountUsage> = HashMap::new();

    for task in tasks {
        let account_id = task["account_id"].as_str().unwrap().to_string();
        let execution_time_ms = task["execution_time_ms"].as_i64().unwrap_or(0);

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

    // Update accounts_billing table and check for free trial overages
    for (account_id, usage) in &account_usage {
        let billing_info = client
            .from("accounts_billing")
            .auth(&supabase_service_role_api_key)
            .select("*")
            .eq("account_id", account_id)
            .single()
            .execute()
            .await?;

        let billing_data: HashMap<String, serde_json::Value> = billing_info.json().await?;

        let free_trial_task_limit = billing_data["free_trial_task_limit"]
            .as_i64()
            .unwrap_or(1000);
        let free_trial_task_usage = billing_data["free_trial_task_usage"].as_i64().unwrap_or(0);
        let trial_ended = billing_data["trial_ended"].as_bool().unwrap_or(false);
        let new_free_trial_task_usage = free_trial_task_usage + usage.task_count;
        let new_total_tasks = billing_data["total_tasks"].as_i64().unwrap_or(0) + usage.task_count;

        let mut update_data = json!({
            "free_trial_task_usage": new_free_trial_task_usage,
            "total_tasks": new_total_tasks
        });

        //Check if free trial time is over
        let current_time = chrono::Utc::now();
        let free_trial_ends_at = billing_data["free_trial_ends_at"]
            .as_str()
            .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&chrono::Utc));

        //check if free trial tasks are over
        if !trial_ended
            && (new_free_trial_task_usage > free_trial_task_limit
                || free_trial_ends_at.map_or(false, |end| current_time > end))
        {
            update_data["trial_ended"] = json!(true);
        }

        client
            .from("accounts_billing")
            .auth(&supabase_service_role_api_key)
            .eq("account_id", account_id)
            .update(json!(update_data).to_string())
            .execute()
            .await?;
    }

    // Send updates to Stripe (you'll need to implement this part)
    for (account_id, usage) in account_usage {
        send_usage_to_stripe(&account_id, usage).await?;
    }

    // Mark tasks as billed
    client
        .from("tasks_billing")
        .auth(&supabase_service_role_api_key)
        .eq("task_status", "Completed")
        .eq("usage_reported_to_billing_provider", "false")
        .update(json!({ "usage_reported_to_billing_provider": true }).to_string())
        .execute()
        .await?;

    Ok(())
}

async fn send_usage_to_stripe(
    account_id: &str,
    usage: AccountUsage,
) -> Result<(), Box<dyn std::error::Error>> {
    // Implement Stripe API call here
    println!(
        "[BILLING_USAGE_ENGINE] Sending usage to Stripe for account {}: {:?}",
        account_id, usage
    );

    // let usage = UsageRecord::create(
    //     "sub_123",
    //     "item_123",
    //     &UsageRecord::create_params(usage.task_count as u64, "tasks"),
    // )


    Ok(())
}
