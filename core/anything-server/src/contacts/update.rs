use axum::{
    extract::{Extension, Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::Arc;

use crate::supabase_jwt_middleware::User;
use crate::AppState;

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateContactInput {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub company: Option<String>,
    pub title: Option<String>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub postal_code: Option<String>,
    pub country: Option<String>,
    pub status: Option<String>,
    pub source: Option<String>,
    pub notes: Option<String>,
    pub tags: Option<Vec<String>>,
    pub custom_fields: Option<Value>,
}

pub async fn update_contact(
    Path((account_id, contact_id)): Path<(String, String)>,
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
    Json(payload): Json<UpdateContactInput>,
) -> impl IntoResponse {
    println!("Handling update_contact");

    let client = &state.anything_client;

    // Create a mutable JSON object to build our update data
    let mut update_data = serde_json::Map::new();

    // Only add fields that are explicitly provided (not None)
    if let Some(first_name) = payload.first_name {
        update_data.insert(
            "first_name".to_string(),
            serde_json::Value::String(first_name),
        );
    }
    if let Some(last_name) = payload.last_name {
        update_data.insert(
            "last_name".to_string(),
            serde_json::Value::String(last_name),
        );
    }
    if let Some(email) = payload.email {
        update_data.insert("email".to_string(), serde_json::Value::String(email));
    }
    if let Some(phone) = payload.phone {
        update_data.insert("phone".to_string(), serde_json::Value::String(phone));
    }
    if let Some(company) = payload.company {
        update_data.insert("company".to_string(), serde_json::Value::String(company));
    }
    if let Some(title) = payload.title {
        update_data.insert("title".to_string(), serde_json::Value::String(title));
    }
    if let Some(address) = payload.address {
        update_data.insert("address".to_string(), serde_json::Value::String(address));
    }
    if let Some(city) = payload.city {
        update_data.insert("city".to_string(), serde_json::Value::String(city));
    }
    if let Some(state_val) = payload.state {
        update_data.insert("state".to_string(), serde_json::Value::String(state_val));
    }
    if let Some(postal_code) = payload.postal_code {
        update_data.insert(
            "postal_code".to_string(),
            serde_json::Value::String(postal_code),
        );
    }
    if let Some(country) = payload.country {
        update_data.insert("country".to_string(), serde_json::Value::String(country));
    }
    if let Some(status) = payload.status {
        update_data.insert("status".to_string(), serde_json::Value::String(status));
    }
    if let Some(source) = payload.source {
        update_data.insert("source".to_string(), serde_json::Value::String(source));
    }
    if let Some(notes) = payload.notes {
        update_data.insert("notes".to_string(), serde_json::Value::String(notes));
    }
    if let Some(tags) = payload.tags {
        update_data.insert(
            "tags".to_string(),
            serde_json::Value::Array(
                tags.into_iter()
                    .map(|t| serde_json::Value::String(t))
                    .collect(),
            ),
        );
    }
    if let Some(custom_fields) = payload.custom_fields {
        update_data.insert("custom_fields".to_string(), custom_fields);
    }

    // If no fields were provided, return early
    if update_data.is_empty() {
        return (StatusCode::BAD_REQUEST, "No fields to update provided").into_response();
    }

    // Convert the map to a JSON value
    let update_json = serde_json::Value::Object(update_data);

    let response = match client
        .from("contacts")
        .auth(&user.jwt)
        .eq("contact_id", &contact_id)
        .eq("account_id", &account_id)
        .update(update_json.to_string())
        .execute()
        .await
    {
        Ok(response) => response,
        Err(err) => {
            println!("Failed to execute request: {:?}", err);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to update contact",
            )
                .into_response();
        }
    };

    let body = match response.text().await {
        Ok(body) => body,
        Err(err) => {
            println!("Failed to read response body: {:?}", err);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to read response body",
            )
                .into_response();
        }
    };

    let result: Value = match serde_json::from_str(&body) {
        Ok(result) => result,
        Err(err) => {
            println!("Failed to parse JSON: {:?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to parse JSON").into_response();
        }
    };

    Json(result).into_response()
}
