use axum::{
    extract::{Extension, Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use serde_json::Value;
use std::sync::Arc;

use crate::supabase_auth_middleware::User;
use crate::AppState;

use std::env;

use std::fs::File;
use std::io::BufReader;

// Actions
pub async fn get_actions(
    Path(account_id): Path<String>,
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
) -> impl IntoResponse {
    println!("Handling a get_actions");

    let client = &state.anything_client;
    let marketplace_client = &state.marketplace_client;

    // Fetch data from the database
    println!("Fetching data from the database");
    let response = match client
        .from("action_templates")
        .auth(user.jwt.clone())
        .eq("account_id", &account_id)
        .eq("archived", "false")
        .select("*")
        .execute()
        .await
    {
        Ok(response) => {
            println!(
                "Successfully fetched data from the database: {:?}",
                response
            );
            response
        }
        Err(err) => {
            eprintln!("Failed to execute request: {:?}", err);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to execute request",
            )
                .into_response();
        }
    };

    println!("Reading response body");
    let body = match response.text().await {
        Ok(body) => {
            println!("Successfully read response body");
            body
        }
        Err(err) => {
            eprintln!("Failed to read response body: {:?}", err);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to read response body",
            )
                .into_response();
        }
    };

    println!("Parsing response body as JSON");
    let mut db_items: Value = match serde_json::from_str(&body) {
        Ok(items) => {
            println!("Successfully parsed JSON: {:?}", items);
            items
        }
        Err(err) => {
            eprintln!("Failed to parse JSON: {:?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to parse JSON").into_response();
        }
    };

    // Fetch marketplace action templates
    println!("Fetching marketplace action templates");
    let marketplace_response = match marketplace_client
        .from("action_templates")
        .auth(user.jwt.clone())
        .select("*")
        .execute()
        .await
    {
        Ok(response) => {
            println!("Successfully fetched marketplace data: {:?}", response);
            response
        }
        Err(err) => {
            eprintln!("Failed to execute marketplace request: {:?}", err);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to execute marketplace request",
            )
                .into_response();
        }
    };

    let marketplace_body = match marketplace_response.text().await {
        Ok(body) => {
            println!("Successfully read marketplace response body");
            body
        }
        Err(err) => {
            eprintln!("Failed to read marketplace response body: {:?}", err);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to read marketplace response body",
            )
                .into_response();
        }
    };

    let marketplace_items: Value = match serde_json::from_str(&marketplace_body) {
        Ok(items) => {
            println!("Successfully parsed marketplace JSON: {:?}", items);
            items
        }
        Err(err) => {
            eprintln!("Failed to parse marketplace JSON: {:?}", err);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to parse marketplace JSON",
            )
                .into_response();
        }
    };

    // Log the current working directory
    println!("Logging the current working directory");
    let current_dir = match env::current_dir() {
        Ok(path) => {
            println!("Current directory: {}", path.display());
            path
        }
        Err(e) => {
            println!("Failed to get current directory: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to get current directory",
            )
                .into_response();
        }
    };

    // Load data from the JSON file
    let json_file_path = current_dir.join("action_db/action_templates.json");
    println!("Loading data from the JSON file at {:?}", json_file_path);
    let file = match File::open(&json_file_path) {
        Ok(file) => {
            println!("Successfully opened JSON file");
            file
        }
        Err(err) => {
            eprintln!("Failed to open JSON file: {:?}", err);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to open JSON file",
            )
                .into_response();
        }
    };

    println!("Reading JSON file");
    let reader = BufReader::new(file);
    let json_items: Value = match serde_json::from_reader(reader) {
        Ok(items) => {
            println!("Successfully parsed JSON file: {:?}", items);
            items
        }
        Err(err) => {
            eprintln!("Failed to parse JSON file: {:?}", err);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to parse JSON file",
            )
                .into_response();
        }
    };

    // Combine database, marketplace, and JSON file items into a single array
    println!("Combining database, marketplace, and JSON file items");
    if let Some(db_array) = db_items.as_array_mut() {
        if let Some(marketplace_array) = marketplace_items.as_array() {
            db_array.extend(marketplace_array.clone());
        }
        if let Some(json_array) = json_items.as_array() {
            db_array.extend(json_array.clone());
        }
    }

    Json(db_items).into_response()
}
