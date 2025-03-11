use axum::{
    extract::{Extension, Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use serde_json::Value;
use std::sync::Arc;

use crate::supabase_jwt_middleware::User;
use crate::AppState;

use crate::system_plugins::registry;

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
            // println!("Successfully parsed JSON: {:?}", items);
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

    // Load schema templates from the registry
    let json_items = match registry::load_schema_templates() {
        Ok(templates) => {
            println!("Successfully loaded schema templates");
            Value::Array(templates)
        }
        Err(err) => {
            eprintln!("Failed to load schema templates: {:?}", err);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to load schema templates",
            )
                .into_response();
        }
    };

    // Filter JSON items to only include "action" types
    let json_items: Value = json_items
        .as_array()
        .map(|arr| {
            arr.iter()
                .filter(|item| {
                    item.get("type")
                        .and_then(|t| t.as_str())
                        .map(|t| t == "action")
                        .unwrap_or(false)
                })
                .cloned()
                .collect::<Vec<_>>()
        })
        .map(|filtered| Value::Array(filtered))
        .unwrap_or(Value::Array(vec![]));

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

// Actions
pub async fn get_triggers(
    Path(account_id): Path<String>,
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
) -> impl IntoResponse {
    println!("Handling a get_actions");

    // Load schema templates from the registry
    let json_items = match registry::load_schema_templates() {
        Ok(templates) => {
            println!("Successfully loaded schema templates");
            Value::Array(templates)
        }
        Err(err) => {
            eprintln!("Failed to load schema templates: {:?}", err);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to load schema templates",
            )
                .into_response();
        }
    };

    // Filter JSON items to only include "trigger" types
    let filtered_items = json_items
        .as_array()
        .map(|arr| {
            arr.iter()
                .filter(|item| {
                    item.get("type")
                        .and_then(|t| t.as_str())
                        .map(|t| t == "trigger")
                        .unwrap_or(false)
                })
                .cloned()
                .collect::<Vec<_>>()
        })
        .map(|filtered| Value::Array(filtered))
        .unwrap_or(Value::Array(vec![]));

    Json(filtered_items).into_response()
}

pub async fn get_other_actions(
    Path(account_id): Path<String>,
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
) -> impl IntoResponse {
    println!("Handling get_other_actions");
    // Load schema templates from the registry
    let json_items = match registry::load_schema_templates() {
        Ok(templates) => {
            println!("Successfully loaded schema templates");
            Value::Array(templates)
        }
        Err(err) => {
            eprintln!("Failed to load schema templates: {:?}", err);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to load schema templates",
            )
                .into_response();
        }
    };

    // Filter JSON items to exclude "action" and "trigger" types
    let filtered_items = json_items
        .as_array()
        .map(|arr| {
            arr.iter()
                .filter(|item| {
                    item.get("type")
                        .and_then(|t| t.as_str())
                        .map(|t| t != "action" && t != "trigger" && t != "response")
                        .unwrap_or(true)
                })
                .cloned()
                .collect::<Vec<_>>()
        })
        .map(|filtered| Value::Array(filtered))
        .unwrap_or(Value::Array(vec![]));

    Json(filtered_items).into_response()
}

pub async fn get_responses(
    Path(account_id): Path<String>,
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
) -> impl IntoResponse {
    println!("Handling get_other_actions");
    // Load schema templates from the registry
    let json_items = match registry::load_schema_templates() {
        Ok(templates) => {
            println!("Successfully loaded schema templates");
            Value::Array(templates)
        }
        Err(err) => {
            eprintln!("Failed to load schema templates: {:?}", err);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to load schema templates",
            )
                .into_response();
        }
    };

    // Filter JSON items to exclude "action" and "trigger" types
    let filtered_items = json_items
        .as_array()
        .map(|arr| {
            arr.iter()
                .filter(|item| {
                    item.get("type")
                        .and_then(|t| t.as_str())
                        .map(|t| t == "response")
                        .unwrap_or(false)
                })
                .cloned()
                .collect::<Vec<_>>()
        })
        .map(|filtered| Value::Array(filtered))
        .unwrap_or(Value::Array(vec![]));

    Json(filtered_items).into_response()
}
