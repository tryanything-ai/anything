use serde_json::Value;
use std::collections::HashMap;
use tera::{Context, Tera};
use std::io::{self, BufRead, BufReader};
// use std::fs::File;
use uuid::Uuid;
use postgrest::Postgrest;
use dotenv::dotenv;
use std::env;

use crate::workflow_types::Task;
use crate::secrets::GetDecryptedSecretsInput;

// Secrets for building context with API KEYS
// pub async fn get_decrypted_secrets(client: &Postgrest, account_id: Uuid) -> Result<Value, Box<dyn std::error::Error>> {
//     dotenv().ok();
//     let supabase_service_role_api_key = env::var("SUPABASE_SERVICE_ROLE_API_KEY").expect("SUPABASE_SERVICE_ROLE_API_KEY must be set");

//     let input = GetDecryptedSecretsInput {
//         user_account_id: account_id.to_string(), 
//     };

//     let response = match client
//         .rpc("get_decrypted_secrets", serde_json::to_string(&input).unwrap())
//         .auth(supabase_service_role_api_key.clone())
//         .execute()
//         .await
//     {
//         Ok(response) => response,
//         Err(e) => {
//             println!("Error executing request: {:?}", e);
//             return Err(Box::new(e));
//         },
//     };

//     let body = match response.text().await {
//         Ok(body) => body,
//         Err(e) => {
//             println!("Error reading response body: {:?}", e);
//             return Err(Box::new(e));
//         },
//     };

//     let items: Value = match serde_json::from_str(&body) {
//         Ok(items) => items,
//         Err(e) => {
//             println!("Error parsing JSON: {:?}", e);
//             return Err(Box::new(e));
//         },
//     };

//     Ok(items)
// }
pub async fn get_decrypted_secrets(client: &Postgrest, account_id: Uuid) -> Result<Value, Box<dyn std::error::Error + Send + Sync>> {
    dotenv().ok();
    let supabase_service_role_api_key = env::var("SUPABASE_SERVICE_ROLE_API_KEY")?;
    
    let input = serde_json::json!({
        "user_account_id": account_id.to_string()
    }).to_string();

    let response = client
        .rpc("get_decrypted_secrets", &input)
        .auth(supabase_service_role_api_key.clone())
        .execute()
        .await?;

    let body = response.text().await?;
    let items: Value = serde_json::from_str(&body)?;

    Ok(items)
}
// pub async fn get_decrypted_secrets(client: &Postgrest, account_id: Uuid) -> Result<Value, Box<dyn std::error::Error + Send + Sync>> {
//     dotenv().ok();
//     let supabase_service_role_api_key = env::var("SUPABASE_SERVICE_ROLE_API_KEY")?;
    
//     let input = serde_json::json!({
//         "user_account_id": account_id
//     });

//     let response = client
//         .rpc("get_decrypted_secrets", &input)
//         .auth(supabase_service_role_api_key.clone())
//         .execute()
//         .await?;

//     let body = response.text().await?;
//     let items: Value = serde_json::from_str(&body)?;

//     Ok(items)
// }

pub async fn get_completed_tasks_for_session(client: &Postgrest, session_id: &str) -> Result<Vec<Task>, Box<dyn std::error::Error + Send + Sync>> {
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
// pub async fn get_completed_tasks_for_session(client: &Postgrest, session_id: &str) -> Result<Vec<Task>, Box<dyn std::error::Error>> {
//     dotenv().ok();
//     let supabase_service_role_api_key = env::var("SUPABASE_SERVICE_ROLE_API_KEY").expect("SUPABASE_SERVICE_ROLE_API_KEY must be set");

//     let response = match client
//         .from("tasks")
//         .auth(supabase_service_role_api_key.clone())
//         .select("*")
//         .eq("session_id", session_id)
//         .execute()
//         .await
//     {
//         Ok(response) => response,
//         Err(e) => {
//             println!("Error executing request: {:?}", e);
//             return Err(Box::new(e));
//         },
//     };

//     let body = match response.text().await {
//         Ok(body) => body,
//         Err(e) => {
//             println!("Error reading response body: {:?}", e);
//             return Err(Box::new(e));
//         },
//     };

//     let tasks: Vec<Task> = match serde_json::from_str(&body) {
//         Ok(tasks) => tasks,
//         Err(e) => {
//             println!("Error parsing JSON: {:?}", e);
//             return Err(Box::new(e));
//         },
//     };

//     Ok(tasks)
// }

pub async fn bundle_context(client: &Postgrest, task: &Task) -> Result<Value, Box<dyn std::error::Error + Send + Sync>> {
// pub async fn bundle_context(client: &Postgrest, task: &Task) -> Result<Value, Box<dyn std::error::Error>> {
    let mut context = Context::new();

    // Fetch decrypted secrets for account_id
    let secrets = get_decrypted_secrets(client, task.account_id).await?;
    context.insert("secrets", &secrets);
    
    context.insert("secrets", &secrets);

    // Retrieve completed events for the session to use results
    let complete_tasks = get_completed_tasks_for_session(client, &task.flow_session_id).await?;

    // Add results to context by node_id
    for event in complete_tasks {
        if let Some(result) = event.result {
            context.insert(&event.node_id, &result);
        }
    }

    // Prepare the Tera template engine
    let mut tera = Tera::default();

    // Add variables to Tera template engine if present
    if let Some(variables) = task.config.get("variables") {

        let variables_str = variables.to_string();

        tera.add_raw_template("variables", &variables_str).map_err(|e| {
            println!("Failed to add raw template for variables to Tera: {}", e);
            e
        })?;

        let rendered_variables = tera.render("variables", &context).map_err(|e| {
            println!("Failed to render variables with Tera: {}", e);
            e
        })?;

        println!("Rendered variables: {}", rendered_variables);
        context.insert("variables", &rendered_variables);
    }

      // Add inputs to Tera template engine if present
      if let Some(inputs) = task.config.get("inputs") {
        let inputs_str = inputs.to_string();
        tera.add_raw_template("inputs", &inputs_str).map_err(|e| {
            println!("Failed to add raw template for inputs to Tera: {}", e);
            e
        })?;

        let rendered_inputs = tera.render("inputs", &context).map_err(|e| {
            println!("Failed to render inputs with Tera: {}", e);
            e
        })?;
        println!("Rendered inputs: {}", rendered_inputs);
        context.insert("inputs", &rendered_inputs);
    }

    // Prepare and render the Tera template for config
    let config_str = task.config.to_string();
    tera.add_raw_template("config", &config_str).map_err(|e| {
        println!("Failed to add raw template for config to Tera: {}", e);
        e
    })?;

    let rendered_config = tera.render("config", &context).map_err(|e| {
        println!("Failed to render config with Tera: {}", e);
        e
    })?;
    println!("Rendered config: {}", rendered_config);

    // Convert rendered config to Value
    // serde_json::from_str::<Value>(&rendered_config).map_err(|e| {
    //     println!("Failed to convert rendered config to Value: {}", e);
    //     Box::new(e) as Box<dyn std::error::Error>
    // })
      // Convert rendered config to Value
      serde_json::from_str::<Value>(&rendered_config).map_err(|e| {
        println!("Failed to convert rendered config to Value: {}", e);
        Box::new(e) as Box<dyn std::error::Error + Send + Sync>
    })
}