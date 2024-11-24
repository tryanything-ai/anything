use crate::auth;
use crate::auth::init::AccountAuthProviderAccount;
use crate::system_variables::get_system_variables;
use crate::workflow_types::Task;
use dotenv::dotenv;
use postgrest::Postgrest;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::env;
use std::error::Error;
use tracing::debug;
use uuid::Uuid;

use crate::templater::Templater;

#[derive(Debug, Serialize, Deserialize)]
pub struct DecryptedSecret {
    pub secret_id: Uuid,
    pub secret_name: String,
    pub secret_value: String,
    pub secret_description: Option<String>,
}

// Secrets for building context with API KEYS
pub async fn get_decrypted_secrets(
    client: &Postgrest,
    account_id: &str,
) -> Result<Vec<DecryptedSecret>, Box<dyn std::error::Error + Send + Sync>> {
    dotenv().ok();
    let supabase_service_role_api_key = env::var("SUPABASE_SERVICE_ROLE_API_KEY")?;

    println!(
        "[BUNDLER] Attempting to get decrypted secrets for account_id: {}",
        account_id
    );

    let input = serde_json::json!({
        "team_account_id": account_id.to_string()
    })
    .to_string();

    let response = client
        .rpc("get_decrypted_secrets", &input)
        .auth(supabase_service_role_api_key.clone())
        .execute()
        .await?;

    println!(
        "[BUNDLER] Response for get_decryped_secrets: {:?}",
        response
    );

    let body = response.text().await?;
    let items: Vec<DecryptedSecret> = match serde_json::from_str(&body) {
        Ok(parsed) => parsed,
        Err(e) => {
            println!("[BUNDLER] Error parsing decrypted secrets: {}", e);
            println!("[BUNDLER] Response body: {}", body);
            return Err(Box::new(e));
        }
    };

    println!(
        "[BUNDLER] Successfully retrieved {} decrypted secrets",
        items.len()
    );

    Ok(items)
}

pub async fn get_completed_tasks_for_session(
    client: &Postgrest,
    session_id: &str,
) -> Result<Vec<Task>, Box<dyn std::error::Error + Send + Sync>> {
    dotenv().ok();
    let supabase_service_role_api_key = env::var("SUPABASE_SERVICE_ROLE_API_KEY")?;

    println!(
        "[BUNDLER] Fetching completed tasks for session_id: {}",
        session_id
    );

    let response = client
        .from("tasks")
        .auth(supabase_service_role_api_key.clone())
        .select("*")
        .eq("flow_session_id", session_id)
        .eq("task_status", "completed")
        .execute()
        .await?;

    let body = response.text().await?;
    let tasks: Vec<Task> = serde_json::from_str(&body)?;

    // Print tasks for debugging
    println!("[BUNDLER] Completed tasks:");
    for task in &tasks {
        println!(
            "[BUNDLER] [COMPLETED_TASK] Task ID: {}, Action ID: {}, Status: {:?}, Result: {:?}",
            task.task_id, task.action_id, task.task_status, task.result
        );
    }

    println!("[BUNDLER] Retrieved {} completed tasks", tasks.len());

    Ok(tasks)
}

pub async fn get_refreshed_auth_accounts(
    client: &Postgrest,
    account_id: &str,
) -> Result<Vec<AccountAuthProviderAccount>, Box<dyn std::error::Error + Send + Sync>> {
    println!(
        "[BUNDLER] Refreshing auth accounts for account_id: {}",
        account_id
    );

    let accounts = auth::refresh::refresh_accounts(client, account_id).await?;

    println!(
        "[BUNDLER] Successfully refreshed {} auth accounts",
        accounts.len()
    );

    Ok(accounts)
}

pub async fn get_auth_accounts(
    client: &Postgrest,
    account_id: &str,
) -> Result<Vec<AccountAuthProviderAccount>, Box<dyn std::error::Error + Send + Sync>> {
    dotenv().ok();
    let supabase_service_role_api_key = env::var("SUPABASE_SERVICE_ROLE_API_KEY")?;

    println!(
        "[BUNDLER] Fetching auth accounts for account_id: {}",
        account_id
    );

    let response = client
        .rpc(
            "get_account_auth_provider_accounts",
            json!({"p_account_id": account_id}).to_string(),
        )
        .auth(supabase_service_role_api_key.clone())
        .execute()
        .await?;

    let body = response.text().await?;
    let accounts: Vec<AccountAuthProviderAccount> = serde_json::from_str(&body)?;

    println!("[BUNDLER] Retrieved {} auth accounts", accounts.len());

    Ok(accounts)
}

pub async fn bundle_variables(
    client: &Postgrest,
    account_id: &str,
    flow_session_id: &str,
    variables_config: Option<&Value>,
    refresh_auth: bool,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    debug!("[BUNDLER] Starting to bundle variables");

    // Pre-allocate with known capacity
    let mut render_variables_context = HashMap::with_capacity(4);

    // Parallel fetch of secrets, accounts, and tasks
    let (secrets_result, accounts_result, tasks_result) = tokio::join!(
        get_decrypted_secrets(client, account_id),
        async {
            if refresh_auth {
                get_refreshed_auth_accounts(client, account_id).await
            } else {
                get_auth_accounts(client, account_id).await
            }
        },
        get_completed_tasks_for_session(client, flow_session_id)
    );

    // Process accounts
    let mut accounts = HashMap::new();
    for account in accounts_result? {
        let slug = account.account_auth_provider_account_slug.clone();
        debug!("[BUNDLER] Inserting account with slug: {}", slug);
        accounts.insert(slug, serde_json::to_value(account)?);
    }
    render_variables_context.insert("accounts".to_string(), serde_json::to_value(accounts)?);

    // Process secrets
    let mut secrets = HashMap::new();
    for secret in secrets_result? {
        let secret_name = secret.secret_name.clone();
        debug!("[BUNDLER] Inserting secret with name: {}", secret_name);
        secrets.insert(secret_name, serde_json::to_value(secret.secret_value)?);
    }
    render_variables_context.insert("secrets".to_string(), serde_json::to_value(secrets)?);

    // Fetch and process tasks
    let tasks_result = tasks_result?;
    let mut tasks_map = HashMap::with_capacity(tasks_result.len());
    for task in tasks_result {
        tasks_map.insert(task.action_id.to_string(), serde_json::to_value(task)?);
    }
    render_variables_context.insert("actions".to_string(), serde_json::to_value(tasks_map)?);

    // Add system variables
    render_variables_context.insert(
        "system".to_string(),
        serde_json::to_value(get_system_variables())?,
    );

    // Process variables config if present
    if let Some(variables) = variables_config {
        let mut templater = Templater::new();
        templater.add_template("task_variables_definition", variables.clone());

        let context_value = serde_json::to_value(&render_variables_context)?;
        let rendered = templater.render("task_variables_definition", &context_value)?;

        debug!("[BUNDLER] Rendered variables output: {}", rendered);
        Ok(rendered)
    } else {
        debug!("[BUNDLER] No variables found in task config");
        Ok(json!({}))
    }
}

fn bundle_inputs(
    rendered_variables: Value,
    inputs: Option<&Value>,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let mut render_input_context: HashMap<String, Value> = HashMap::new();
    render_input_context.insert("variables".to_string(), rendered_variables);

    // Create a new Templater instance for rendering inputs
    let mut templater = Templater::new();

    // Convert context HashMap to Value
    let inputs_context_value = serde_json::to_value(render_input_context.clone())?;

    // Add the task definition as a template and render if it exists
    if let Some(inputs) = inputs {
        println!("[BUNDLER] Task inputs definition: {}", inputs.clone());
        templater.add_template("task_inputs_definition", inputs.clone());

        // Render the task definition with the context
        let rendered_inputs_definition =
            templater.render("task_inputs_definition", &inputs_context_value)?;
        println!(
            "[BUNDLER] Rendered inputs output: {}",
            rendered_inputs_definition
        );
        Ok(rendered_inputs_definition)
    } else {
        println!("[BUNDLER] No inputs found in task config, returning empty object");
        Ok(json!({}))
    }
}

pub async fn bundle_context(
    client: &Postgrest,
    account_id: &str,
    flow_session_id: &str,
    variables_config: Option<&Value>,
    inputs_config: Option<&Value>,
    refresh_auth: bool,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    println!("[BUNDLER] Starting to bundle context from parts");

    let rendered_variables_definition = bundle_variables(
        client,
        account_id,
        flow_session_id,
        variables_config,
        refresh_auth,
    )
    .await?;

    bundle_inputs(rendered_variables_definition, inputs_config)
}

// Helper function to bundle context for a task
pub async fn bundle_task_context(
    client: &Postgrest,
    task: &Task,
    refresh_auth: bool,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    bundle_context(
        client,
        task.account_id.to_string().as_str(),
        task.flow_session_id.to_string().as_str(),
        task.config.get("variables"),
        task.config.get("input"),
        refresh_auth,
    )
    .await
}
