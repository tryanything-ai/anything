use crate::auth;
use crate::auth::init::AccountAuthProviderAccount;
use crate::workflow_types::Task;
use dotenv::dotenv;
use postgrest::Postgrest;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fmt;
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

#[derive(Debug)]
struct CustomError(String);

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for CustomError {}

pub async fn bundle_variables(
    client: &Postgrest,
    task: &Task,
    refresh_auth: bool,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    println!("[BUNDLER] Starting to bundle variables");

    let mut render_variables_context: HashMap<String, Value> = HashMap::new();

    println!(
        "[BUNDLER] Initial variables context: {:?}",
        render_variables_context
    );

    let mut accounts: HashMap<String, Value> = HashMap::new();

    if refresh_auth {
        for account in get_refreshed_auth_accounts(client, &task.account_id.to_string()).await? {
            // for account in get_fake_account_auth_provider_account().await? {
            let slug = account.account_auth_provider_account_slug.clone();
            println!(
                "[BUNDLER] Inserting account with slug: {} at accounts.{}",
                slug, slug
            );

            accounts.insert(slug, serde_json::to_value(account)?);
        }
    } else {
        println!("[BUNDLER] Skipping refresh of auth accounts. Just Bundling.");
        for account in get_auth_accounts(client, &task.account_id.to_string()).await? {
            let slug = account.account_auth_provider_account_slug.clone();
            println!(
                "[BUNDLER] Inserting account with slug: {} at accounts.{}",
                slug, slug
            );

            accounts.insert(slug, serde_json::to_value(account)?);
        }
    }

    render_variables_context.insert("accounts".to_string(), serde_json::to_value(accounts)?);

    println!(
        "[BUNDLER] Context after adding accounts: {:?}",
        render_variables_context
    );

    // Add secrets to the render_variables_context
    let mut secrets: HashMap<String, Value> = HashMap::new();
    for secret in get_decrypted_secrets(client, &task.account_id.to_string()).await? {
        let secret_name = secret.secret_name.clone();
        let secret_value = secret.secret_value.clone();
        println!("[BUNDLER] Inserting secret with name: {}", secret_name);

        secrets.insert(secret_name, serde_json::to_value(secret_value)?);
    }

    render_variables_context.insert("secrets".to_string(), serde_json::to_value(secrets)?);

    println!(
        "[BUNDLER] Context after adding secrets: {:?}",
        render_variables_context
    );

    //Add task responses to the render_variables_context
    // Add secrets to the render_variables_context
    let mut completed_tasks: HashMap<String, Value> = HashMap::new();
    for completed_task in
        get_completed_tasks_for_session(client, &&task.flow_session_id.to_string()).await?
    {
        completed_tasks.insert(
            completed_task.action_id.to_string(),
            serde_json::to_value(completed_task)?,
        );
    }

    render_variables_context.insert(
        "actions".to_string(),
        serde_json::to_value(completed_tasks)?,
    );

    println!(
        "[BUNDLER] Context after adding completed tasks: {:?}",
        render_variables_context
    );

    // Create a new Templater instance
    let mut templater = Templater::new();

    // Add the task definition as a template
    if let Some(variables) = task.config.get("variables") {
        // let variables_str = variables.to_string();
        println!("[BUNDLER] Task variables definition: {}", variables.clone());
        templater.add_template("task_variables_definition", variables.clone());
    } else {
        println!("[BUNDLER] No variables found in task config");
    }

    // Get the variables from the task definition
    let variables = templater.get_template_variables("task_variables_definition")?;

    // Print the variables
    println!("[BUNDLER] Variables from task variables definition:");
    for (index, variable) in variables.iter().enumerate() {
        println!("  {}. {}", index + 1, variable);
    }

    // Convert context HashMap to Value
    let context_value = serde_json::to_value(render_variables_context.clone())?;

    // Render the task definition with the context
    let rendered_variables_definition =
        templater.render("task_variables_definition", &context_value)?;
    println!(
        "[BUNDLER] Rendered variables output: {}",
        rendered_variables_definition
    );

    Ok(rendered_variables_definition)
}

pub async fn bundle_context(
    client: &Postgrest,
    task: &Task,
    refresh_auth: bool,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    println!("[BUNDLER] Starting to bundle context for task: {:?}", task);

    let rendered_variables_definition = bundle_variables(client, task, refresh_auth).await?;

    let mut render_input_context: HashMap<String, Value> = HashMap::new();

    render_input_context.insert("variables".to_string(), rendered_variables_definition);

    // Create a new Templater instance for rendering inputs
    let mut templater = Templater::new();

    // Convert context HashMap to Value
    let iputs_context_value = serde_json::to_value(render_input_context.clone())?;

    // Add the task definition as a template
    if let Some(inputs) = task.config.get("inputs") {
        // let inputs_str = inputs.to_string();
        println!("[BUNDLER] Task inputs definition: {}", inputs.clone());
        templater.add_template("task_inputs_definition", inputs.clone());
    } else {
        println!("[BUNDLER] No variables found in task config");
    }

    // Render the task definition with the context
    let rendered_inputs_definition =
        templater.render("task_inputs_definition", &iputs_context_value)?;
    println!(
        "[BUNDLER] Rendered inputs ouput: {}",
        rendered_inputs_definition
    );

    Ok(rendered_inputs_definition)
}
