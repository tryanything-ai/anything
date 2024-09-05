use crate::auth;
use crate::auth::init::AccountAuthProviderAccount;
use crate::workflow_types::Task;
use dotenv::dotenv;
use postgrest::Postgrest;
use serde_json::Value;
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fmt;
use std::hash::Hash;
// use tera::{Context, Tera};
use crate::templater::Templater;
use uuid::Uuid;

// Secrets for building context with API KEYS
pub async fn get_decrypted_secrets(
    client: &Postgrest,
    account_id: Uuid,
) -> Result<Value, Box<dyn std::error::Error + Send + Sync>> {
    dotenv().ok();
    let supabase_service_role_api_key = env::var("SUPABASE_SERVICE_ROLE_API_KEY")?;

    println!(
        "[BUNDLER] Attempting to get decrypted secrets for account_id: {}",
        account_id
    );

    let input = serde_json::json!({
        "user_account_id": account_id.to_string()
    })
    .to_string();

    let response = client
        .rpc("get_decrypted_secrets", &input)
        .auth(supabase_service_role_api_key.clone())
        .execute()
        .await?;

    let body = response.text().await?;
    let items: Value = serde_json::from_str(&body)?;

    println!("[BUNDLER] Successfully retrieved decrypted secrets");

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
        .execute()
        .await?;

    let body = response.text().await?;
    let tasks: Vec<Task> = serde_json::from_str(&body)?;

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

#[derive(Debug)]
struct CustomError(String);

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for CustomError {}

pub async fn bundle_context(
    client: &Postgrest,
    task: &Task,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    println!("[BUNDLER] Starting to bundle context for task: {:?}", task);

    let mut context: HashMap<String, Value> = HashMap::new();

    println!("[BUNDLER] Initial context: {:?}", context);

    let mut accounts: HashMap<String, Value> = HashMap::new();
    for account in get_refreshed_auth_accounts(client, &task.account_id.to_string()).await? {
        let slug = account.account_auth_provider_account_slug.clone();
        println!(
            "[BUNDLER] Inserting account with slug: {} at accounts.{}",
            slug, slug
        );

        accounts.insert(slug, serde_json::to_value(account)?);
    }

    context.insert("accounts".to_string(), serde_json::to_value(accounts)?);

    println!("[BUNDLER] Context after adding accounts: {:?}", context);

    // Create a new Templater instance
    let mut templater = Templater::new();

    // Add the task definition as a template
    if let Some(variables) = task.config.get("variables") {
        let variables_str = variables.to_string();
        println!(
            "[BUNDLER] Task variables definition: {}",
            variables_str
        );
        templater.add_template("task_variables_definition", &variables_str);
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

    // Render the task definition with the context
    let rendered_definition = templater.render("task_variables_definition", &context)?;
    println!(
        "[BUNDLER] Rendered variables definition: {}",
        rendered_definition
    );

    // Add the rendered definition to the context
    context.insert(
        "rendered_definition".to_string(),
        Value::String(rendered_definition),
    );

    // Add the original variables to the context
    context.insert("variables".to_string(), serde_json::to_value(variables)?);

    println!("[BUNDLER] Final context: {:?}", context);

    Ok(serde_json::to_value(context)?)
}
