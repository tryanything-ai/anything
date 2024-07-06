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
    description: String,
}


#[derive(Debug, Deserialize, Serialize)]
pub struct AnythingCreateSecretInput {
    secret_id: String, 
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
        description: payload.secret_description.clone(),
    }; 

    println!("insert_secret rpc Input?: {:?}", input);
  
    //Get Special Priveledges by passing service_role in auth()
    dotenv().ok();
    let supabase_service_role_api_key = env::var("SUPABASE_SERVICE_ROLE_API_KEY").expect("SUPABASE_SERVICE_ROLE_API_KEY must be set");

    // Create Secret in Vault
    let response = match client
        .rpc("insert_secret", serde_json::to_string(&input).unwrap())
        .auth(supabase_service_role_api_key.clone()) //Need to put service role key here I guess for it to show up current_setting in sql function
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


    println!("Response from vault insert: {:?}", body);

    let secret_vault_id = body.trim_matches('"');

    let anythingSecretInput = AnythingCreateSecretInput {
        secret_id: secret_vault_id.to_string(), //use the same id in vault and public secrets table for dx
        secret_name: payload.secret_name.clone(),
        vault_secret_id: secret_vault_id.to_string(), //vault_secret_id.clone(),
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


#[derive(Debug, Deserialize, Serialize)]
pub struct GetDecryptedSecretsInput {
    user_account_id: String,
}

// Secrets
pub async fn get_decrypted_secrets(
    State(client): State<Arc<Postgrest>>, 
    Extension(user): Extension<User>,
    headers: HeaderMap,
) -> impl IntoResponse {
    println!("Handling a get_actions");

    dotenv().ok();
    let supabase_service_role_api_key = env::var("SUPABASE_SERVICE_ROLE_API_KEY").expect("SUPABASE_SERVICE_ROLE_API_KEY must be set");

    let input = GetDecryptedSecretsInput {
        user_account_id: user.account_id.clone()
    }; 

    println!("get_decrypted_secrets rpc Input?: {:?}", input);

    let response = match client
        .rpc("get_decrypted_secrets", serde_json::to_string(&input).unwrap())
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

    let items: Value = match serde_json::from_str(&body) {
        Ok(items) => items,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to parse JSON").into_response(),
    };

    Json(items).into_response()
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateSecretPayload {
    secret_id: String,
    secret_value: String,
    secret_description: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateVaultSecretInput {
    id: String,
    name: String, 
    secret: String,
    description: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateAnythingSecretInput {
    secret_value: String,
    secret_description: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ReadVaultSecretInput {
    secret_id: String
}

pub async fn update_secret(
    State(client): State<Arc<Postgrest>>,
    Extension(user): Extension<User>,
    headers: HeaderMap,
    Json(payload): Json<UpdateSecretPayload>,
) -> impl IntoResponse {
    
    let read_secret_input = ReadVaultSecretInput {
        secret_id: payload.secret_id.clone()
    };

    //Get Special Priveledges by passing service_role in auth()
    dotenv().ok();
    let supabase_service_role_api_key = env::var("SUPABASE_SERVICE_ROLE_API_KEY").expect("SUPABASE_SERVICE_ROLE_API_KEY must be set");

    // Read Secret in Vault
    let response = match client
    .rpc("read_secret", serde_json::to_string(&read_secret_input).unwrap())
    .auth(supabase_service_role_api_key.clone()) //Need to put service role key here I guess for it to show up current_setting in sql function
    .execute()
    .await
    {
    Ok(response) => response,
    Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to execute request").into_response(),
    };

    let vault_secret_body = match response.text().await {
    Ok(body) => body,
    Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to read response body").into_response(),
    };

    println!("Vault Secret Body: {:?}", vault_secret_body);

    let vault_secret_json: serde_json::Value = serde_json::from_str(&vault_secret_body).unwrap();
    let secret_name = vault_secret_json["name"].as_str().unwrap_or_default();

    println!("Secret Name: {:?}", secret_name);

    println!("update_secret Input?: {:?}", payload);

    let input = UpdateVaultSecretInput {
        id: payload.secret_id.clone(),
        name: secret_name.to_string(),
        secret: payload.secret_value.clone(),
        description: payload.secret_description.clone(),
    }; 

    println!("update_secret rpc Input?: {:?}", input);
    

    // Create Secret in Vault
    let response = match client
        .rpc("update_secret", serde_json::to_string(&input).unwrap())
        .auth(supabase_service_role_api_key.clone()) //Need to put service role key here I guess for it to show up current_setting in sql function
        .execute()
        .await
    {
        Ok(response) => response,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to execute request").into_response(),
    };


    let anythingSecretInput = UpdateAnythingSecretInput {
        secret_value: payload.secret_value.clone(),
        secret_description: payload.secret_description.clone()
    };
    
    //Update Secret
    let db_secret_response = match client
        .from("secrets")
        .auth(user.jwt.clone())
        .eq("secret_id", &payload.secret_id.clone())
        .update(serde_json::to_string(&anythingSecretInput).unwrap())
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

    println!("Update DB Secret Body: {:?}", db_secret_body);

    Json(db_secret_body).into_response()
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DeleteVaultSecretInput {
    secret_id: String
}

pub async fn delete_secret(
    Path(secret_id): Path<String>,
    State(client): State<Arc<Postgrest>>,
    Extension(user): Extension<User>,
    headers: HeaderMap,
) -> impl IntoResponse {    

    println!("Delete Secret: {:?}", secret_id);

    // Delete in DB
    let response = match client
        .from("secrets")
        .auth(user.jwt)
        .eq("secret_id", &secret_id)
        .delete()
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

    println!("Delete DB Secret Body: {:?}", body);

    //Delete in Vault
    dotenv().ok();
    let supabase_service_role_api_key = env::var("SUPABASE_SERVICE_ROLE_API_KEY").expect("SUPABASE_SERVICE_ROLE_API_KEY must be set");

    //If the user is allowed to delete the secret from the anything.secrets table the RLS policy means they are allowed to delete from vault.
    // It should fail if the user is not allowed to delete from the anything.secrets table
    // So this should be safe ( but i wish it was safer )
    //TODO: protect this more. right now its a little open. 
    let input = DeleteVaultSecretInput {
        secret_id: secret_id.clone(), 
    };

    println!("delete secret rpc Input?: {:?}", input);

    let rpc_response = match client
        .rpc("delete_secret", serde_json::to_string(&input).unwrap())
        .auth(supabase_service_role_api_key.clone())
        .execute()
        .await
    {
        Ok(response) => response,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to execute request").into_response(),
    };

    let rpc_body = match rpc_response.text().await {
        Ok(body) => body,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to read response body").into_response(),
    };

    println!("Delete Vault Secret Body: {:?}", rpc_body);

    Json(body).into_response()
}



// Secrets
// pub async fn get_secrets(
//     State(client): State<Arc<Postgrest>>, 
//     Extension(user): Extension<User>,
//     headers: HeaderMap,
// ) -> impl IntoResponse {
//     println!("Handling a get_actions");

//     let response = match client
//         .from("secrets")
//         .auth(user.jwt)
//         .eq("archived", "false")
//         .select("*")
//         .execute()
//         .await
//     {
//         Ok(response) => response,
//         Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to execute request").into_response(),
//     };

//     let body = match response.text().await {
//         Ok(body) => body,
//         Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to read response body").into_response(),
//     };

//     let items: Value = match serde_json::from_str(&body) {
//         Ok(items) => items,
//         Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to parse JSON").into_response(),
//     };

//     Json(items).into_response()
// }
