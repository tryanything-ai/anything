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

use crate::templater::Templater;
use uuid::Uuid;

// Fake account data for testing purposes
pub async fn get_fake_account_auth_provider_account(
) -> Result<Vec<AccountAuthProviderAccount>, Box<dyn std::error::Error + Send + Sync>> {
    let fake_account = AccountAuthProviderAccount {
        account_auth_provider_account_id: Uuid::new_v4(),
        account_id: Uuid::new_v4(),
        access_token_vault_id: "airtable_access_token".to_string(),
        refresh_token_vault_id: "airtable_refresh_token".to_string(),
        auth_provider: Some(serde_json::json!({
            "auth_provider_id": "airtable",
            "provider_name": "airtable",
            "provider_label": "airtable",
            "provider_icon": "<svg>...</svg>",
            "provider_description": "Connect with your airtable account",
            "provider_readme": "Internal notes for managing airtable connection",
            "auth_type": "oauth2",
            "auth_url": "https://accounts.airtable.com/o/oauth2/auth",
            "token_url": "https://oauth2.airtableapis.com/token",
            "provider_data": {},
            "access_token_lifetime_seconds": "3600",
            "refresh_token_lifetime_seconds": "2592000",
            "redirect_url": "https://example.com/auth/callback",
            "client_id": "your_client_id",
            "client_secret": "your_client_secret",
            "scopes": "email profile",
            "public": false
        })),
        auth_provider_id: "airtable".to_string(),
        account_auth_provider_account_label: "My airtable Account".to_string(),
        account_auth_provider_account_slug: "airtable".to_string(),
        account_data: Some(serde_json::json!({
            "email": "user@example.com",
            "name": "Test User"
        })),
        access_token: "fake_access_token".to_string(),
        access_token_expires_at: Some(chrono::Utc::now() + chrono::Duration::hours(1)),
        refresh_token: Some("fake_refresh_token".to_string()),
        refresh_token_expires_at: Some(chrono::Utc::now() + chrono::Duration::days(30)),
        updated_at: Some(chrono::Utc::now()),
        created_at: Some(chrono::Utc::now()),
        updated_by: Some(Uuid::new_v4()),
        created_by: Some(Uuid::new_v4()),
    };

    Ok(vec![fake_account])
}
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DecryptedSecret {
    pub secret_id: Uuid,
    pub secret_name: String,
    pub secret_value: String,
    pub secret_description: String,
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

    let mut render_variables_context: HashMap<String, Value> = HashMap::new();

    println!("[BUNDLER] Initial context: {:?}", render_variables_context);

    let mut accounts: HashMap<String, Value> = HashMap::new();
    for account in get_refreshed_auth_accounts(client, &task.account_id.to_string()).await? {
        // for account in get_fake_account_auth_provider_account().await? {
        let slug = account.account_auth_provider_account_slug.clone();
        println!(
            "[BUNDLER] Inserting account with slug: {} at accounts.{}",
            slug, slug
        );

        accounts.insert(slug, serde_json::to_value(account)?);
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

    let mut render_input_context: HashMap<String, Value> = HashMap::new();

    render_input_context.insert("variables".to_string(), rendered_variables_definition);

    // println!("[BUNDLER] Final context: {:?}", render_variables_context);
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
