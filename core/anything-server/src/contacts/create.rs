use axum::{
    extract::{Extension, Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::Arc;
use uuid::Uuid;

use crate::supabase_jwt_middleware::User;
use crate::AppState;

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateContactInput {
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

pub async fn create_contact(
    Path(account_id): Path<String>,
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
    Json(payload): Json<CreateContactInput>,
) -> impl IntoResponse {
    println!("Handling create_contact");

    let client = &state.anything_client;

    // Check if phone number is provided
    if let Some(phone) = &payload.phone {
        println!("Checking for existing contact with phone: {}", phone);

        // Query for existing contact with the same phone number
        let response = match client
            .from("contacts")
            .auth(&user.jwt)
            .select("*")
            .eq("account_id", &account_id)
            .eq("phone", phone)
            .eq("archived", "false")
            .execute()
            .await
        {
            Ok(response) => response,
            Err(err) => {
                println!("Failed to check for existing contact: {:?}", err);
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to check for existing contact",
                )
                    .into_response();
            }
        };

        // If we got a response with content, parse it
        if response.status() != 204 {
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

            let existing_contacts: Value = match serde_json::from_str(&body) {
                Ok(result) => result,
                Err(err) => {
                    println!("Failed to parse JSON: {:?}", err);
                    return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to parse JSON")
                        .into_response();
                }
            };

            // If we found an existing contact, return it
            if let Some(contacts_array) = existing_contacts.as_array() {
                if !contacts_array.is_empty() {
                    println!("Found existing contact with phone: {}", phone);

                    // Return the first matching contact with a 200 status (not created)
                    return (StatusCode::OK, Json(contacts_array[0].clone())).into_response();
                }
            }
        }
    }

    // No existing contact found, proceed with creating a new one
    println!("No existing contact found, creating new contact");

    // Generate a UUID for the new contact
    let contact_id = Uuid::new_v4().to_string();

    // Create the contact data
    let contact_data = serde_json::json!({
        "contact_id": contact_id,
        "account_id": account_id,
        "first_name": payload.first_name,
        "last_name": payload.last_name,
        "email": payload.email,
        "phone": payload.phone,
        "company": payload.company,
        "title": payload.title,
        "address": payload.address,
        "city": payload.city,
        "state": payload.state,
        "postal_code": payload.postal_code,
        "country": payload.country,
        "status": payload.status,
        "source": payload.source,
        "notes": payload.notes,
        "tags": payload.tags,
        "custom_fields": payload.custom_fields,
        "archived": false
    });

    let response = match client
        .from("contacts")
        .auth(&user.jwt)
        .insert(contact_data.to_string())
        .execute()
        .await
    {
        Ok(response) => response,
        Err(err) => {
            println!("Failed to execute request: {:?}", err);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to create contact",
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

    (StatusCode::CREATED, Json(result)).into_response()
}
