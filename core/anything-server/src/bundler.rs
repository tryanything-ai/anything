use crate::auth::{self, AccountAuthProviderAccount};
use serde_json::Value;
use std::collections::HashMap;
use std::io::{self, BufRead, BufReader};
use tera::{Context, Tera};
// use std::fs::File;
use crate::secrets::GetDecryptedSecretsInput;
use crate::workflow_types::Task;
use dotenv::dotenv;
use postgrest::Postgrest;
use serde_with::{serde_as, DisplayFromStr};
use std::env;
use std::error::Error;
use std::fmt;
use uuid::Uuid;

// Secrets for building context with API KEYS
pub async fn get_decrypted_secrets(
    client: &Postgrest,
    account_id: Uuid,
) -> Result<Value, Box<dyn std::error::Error + Send + Sync>> {
    dotenv().ok();
    let supabase_service_role_api_key = env::var("SUPABASE_SERVICE_ROLE_API_KEY")?;

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

    Ok(items)
}

pub async fn get_completed_tasks_for_session(
    client: &Postgrest,
    session_id: &str,
) -> Result<Vec<Task>, Box<dyn std::error::Error + Send + Sync>> {
    dotenv().ok();
    let supabase_service_role_api_key = env::var("SUPABASE_SERVICE_ROLE_API_KEY")?;

    let response = client
        .from("tasks")
        .auth(supabase_service_role_api_key.clone())
        .select("*")
        .eq("flow_session_id", session_id)
        .execute()
        .await?;

    let body = response.text().await?;
    let tasks: Vec<Task> = serde_json::from_str(&body)?;

    Ok(tasks)
}

// pub async fn get_auth_accounts(
//     client: &Postgrest,
//     account_id: &str,
// ) -> Result<Vec<AccountAuthProviderAccount>, Box<dyn std::error::Error + Send + Sync>> {
//     dotenv().ok();
//     let supabase_service_role_api_key = env::var("SUPABASE_SERVICE_ROLE_API_KEY")?;

//     let response = client
//         .from("account_auth_provider_accounts")
//         .auth(supabase_service_role_api_key.clone())
//         .select("*")
//         .eq("account_id", account_id)
//         .execute()
//         .await?;

//     let body = response.text().await?;
//     let accounts: Vec<AccountAuthProviderAccount> = serde_json::from_str(&body)?;

//     Ok(accounts)
// }

pub async fn get_refreshed_auth_accounts(
    client: &Postgrest,
    account_id: &str,
) -> Result<Vec<AccountAuthProviderAccount>, Box<dyn std::error::Error + Send + Sync>> {

    let accounts = auth::refresh_accounts(client, account_id).await?;

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
    let mut context = Context::new();

    // Fetch decrypted secrets for account_id
    //TODO: add secrets.SECRET_SLUG functionality?? maybe its already heare
    // let secrets = get_decrypted_secrets(client, task.account_id).await?;
    // context.insert("secrets", &secrets);
    let secrets = get_decrypted_secrets(client, task.account_id).await?;

    if let Some(secrets_map) = secrets.as_object() {
        for (key, value) in secrets_map {
            context.insert(&format!("secrets.{}", key), value);
        }
    }

    // Retrieve completed events for the session to use results
    let complete_tasks = get_completed_tasks_for_session(client, &task.flow_session_id).await?;

    // Add results to context by node_id
    for event in complete_tasks {
        if let Some(result) = event.result {
            context.insert(&event.node_id, &result);
        }
    }

    //TODO: add accounts.ACCOUNT_SLUG functionality
    let auth_provider_accounts =
        get_refreshed_auth_accounts(client, &task.account_id.to_string()).await?;

    for account in auth_provider_accounts {
        context.insert(
            &format!("accounts.{}", account.account_auth_provider_account_slug),
            &account,
        );
    }

    // Prepare the Tera template engine
    let mut tera = Tera::default();

    // Add variables to Tera template engine if present
    if let Some(variables) = task.config.get("variables") {
        let variables_str = variables.to_string();

        tera.add_raw_template("variables", &variables_str)
            .map_err(|e| {
                println!("Failed to add raw template for variables to Tera: {}", e);
                Box::new(CustomError(e.to_string()))
            })?;

        let rendered_variables = tera.render("variables", &context).map_err(|e| {
            println!("Failed to render variables with Tera: {}", e);
            Box::new(CustomError(e.to_string()))
        })?;

        // Add rendered variables to context
        println!("Rendered variables: {}", rendered_variables);
        context.insert("variables", &rendered_variables);
    }

    // Add inputs to Tera template engine if present
    if let Some(inputs) = task.config.get("inputs") {
        let inputs_str = inputs.to_string();
        tera.add_raw_template("inputs", &inputs_str).map_err(|e| {
            println!("Failed to add raw template for inputs to Tera: {}", e);
            Box::new(CustomError(e.to_string()))
        })?;

        // Convert rendered inputs to Value
        let rendered_context_str = tera.render("inputs", &context).map_err(|e| {
            println!("Failed to render config with Tera: {}", e);
            Box::new(CustomError(e.to_string()))
        })?;

        let rendered_context =
            serde_json::from_str::<Value>(&rendered_context_str).map_err(|e| {
                println!("Failed to convert rendered config to Value: {}", e);
                Box::new(CustomError(e.to_string()))
            })?;

        println!("Rendered context: {:?}", rendered_context);

        Ok(rendered_context)
    } else {
        println!("No inputs found in task config");
        return Err(
            Box::new(CustomError("No inputs found in task config".to_string()))
                as Box<dyn Error + Send + Sync>,
        );
    }
}
