use axum::{
    extract::{Extension, Path, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};
use hyper::header::AUTHORIZATION;
use postgrest::Postgrest;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::Arc;
use std::env;

use crate::auth::User;

use slugify::slugify;

use dotenv::dotenv;



#[derive(Debug, Deserialize, Serialize)]
pub struct CreateSecretPayload {
    secret_name: String,
    secret_value: String,
    secret_description: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateSecretInput {
    name: String,
    secret: String,
    // description: String,
}


#[derive(Debug, Deserialize, Serialize)]
pub struct AnythingCreateSecretInput {
    secret_name: String,
    vault_secret_id: String,
    secret_description: String,
    account_id: String,
}

pub async fn create_secret(
    State(client): State<Arc<Postgrest>>,
    Extension(user): Extension<User>,
    headers: HeaderMap,
    Json(payload): Json<CreateSecretPayload>,
) -> impl IntoResponse {

 

    println!("create_secret Input?: {:?}", payload);

    let vault_secret_name = slugify!(format!("{}_{}", user.account_id.clone(), payload.secret_name.clone()).as_str(), separator = "_");

    println!("New Name: {}", vault_secret_name);

    let input = CreateSecretInput {
        name: vault_secret_name,
        secret: payload.secret_value.clone(),
        // description: payload.secret_description.clone(),
    }; 

    println!("insert_secret rpc Input?: {:?}", input);
  
    //Create Service Role Client   
    //We need a service role client here so that we only ever trigger
    // vault functions with a service role priveledges
    // Otherwise users might find ways into a very naughty place
    dotenv().ok();
    let supabase_url = env::var("SUPABASE_URL").expect("SUPABASE_URL must be set");
    let supabase_service_role_api_key = env::var("SUPABASE_SERVICE_ROLE_API_KEY").expect("SUPABASE_SERVICE_ROLE_API_KEY must be set");
    
    println!("Supabase URL: {:?}", supabase_url);
    println!("Supabase SERVICE_ROLE API Key: {:?}", supabase_service_role_api_key);

    // let service_role_client = 
    //     Postgrest::new(supabase_url.clone())
    //     // .insert_header("apikey", supabase_service_role_api_key.clone())
    //     .schema("anything");
    

    // let jwt = user.jwt.clone();

    // Create Secret in Vault
    let response = match client
        .rpc("insert_secret", serde_json::to_string(&input).unwrap())
        .auth(supabase_service_role_api_key.clone())
        .execute()
        .await
    {
        Ok(response) => response,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to execute request").into_response(),
    };

    let body = match response.text().await {
        Ok(body) => body,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to read response body").into_response(),
    };

       // Extract the id from the response
    // Extract the id from the response
    // let vault_secret_id = match response.get("id").and_then(|id| id.as_str()) {
    //     Some(id) => id.to_string(),
    //     None => return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to get id from response").into_response(),
    // };

    // println!("Response from vault insert: {:?}", vault_secret_id);

    println!("Response from vault insert: {:?}", body);

    let cleaned_response = body.trim_matches('"');

    let anythingSecretInput = AnythingCreateSecretInput {
        secret_name: payload.secret_name.clone(),
        vault_secret_id: cleaned_response.to_string(), //vault_secret_id.clone(),
        secret_description: payload.secret_description.clone(),
        account_id: user.account_id.clone()
    };
    
    //Create Flow Version
    let db_secret_response = match client
        .from("secrets")
        .auth(user.jwt.clone())
        .insert(serde_json::to_string(&anythingSecretInput).unwrap())
        .execute()
        .await
    {
        Ok(response) => response,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to execute request").into_response(),
    };

    let db_secret_body = match db_secret_response.text().await {
        Ok(body) => body,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to read response body").into_response(),
    };

    println!("DB Secret Body: {:?}", db_secret_body);

    Json(db_secret_body).into_response()
}
