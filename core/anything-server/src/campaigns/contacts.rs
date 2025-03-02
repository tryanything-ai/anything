use axum::{
    extract::{Extension, Path, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;

use crate::supabase_jwt_middleware::User;
use crate::AppState;

#[derive(Debug, Deserialize, Serialize)]
pub struct UploadContactsInput {
    pub contacts: Vec<ContactInput>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ContactInput {
    pub contact_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NewContactInput {
    pub name: String,
    pub phone_number: String,
    pub email: Option<String>,
    pub additional_data: Option<Value>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateAndAddContactsInput {
    pub contacts: Vec<NewContactInput>,
}

pub async fn get_campaign_contacts(
    Path((account_id, campaign_id)): Path<(String, String)>,
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
) -> impl IntoResponse {
    println!(
        "[CONTACTS] Handling get_campaign_contacts for campaign {} in account {}",
        campaign_id, account_id
    );

    let client = &state.anything_client;

    // First, get the campaign contacts
    let response = match client
        .from("campaign_contacts")
        .auth(&user.jwt)
        .select("*, contacts(*)")
        .eq("campaign_id", &campaign_id)
        .eq("account_id", &account_id)
        .eq("archived", "false")
        .order("created_at.desc")
        .execute()
        .await
    {
        Ok(response) => {
            println!("[CONTACTS] Successfully queried campaign contacts");
            response
        }
        Err(err) => {
            println!("[CONTACTS] Failed to execute request: {:?}", err);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to execute request",
            )
                .into_response();
        }
    };

    if response.status() == 204 {
        println!("[CONTACTS] No contacts found for campaign {}", campaign_id);
        return (StatusCode::NO_CONTENT, "No content").into_response();
    }

    let body = match response.text().await {
        Ok(body) => {
            println!("[CONTACTS] Successfully read response body");
            body
        }
        Err(err) => {
            println!("[CONTACTS] Failed to read response body: {:?}", err);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to read response body",
            )
                .into_response();
        }
    };

    let items: Value = match serde_json::from_str(&body) {
        Ok(items) => {
            println!("[CONTACTS] Successfully parsed JSON response");
            items
        }
        Err(err) => {
            println!("[CONTACTS] Failed to parse JSON: {:?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to parse JSON").into_response();
        }
    };

    Json(items).into_response()
}

pub async fn add_contacts_to_campaign(
    Path((account_id, campaign_id)): Path<(String, String)>,
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
    _headers: HeaderMap,
    Json(payload): Json<UploadContactsInput>,
) -> impl IntoResponse {
    println!(
        "[CONTACTS] Handling add_contacts_to_campaign for campaign {} with {} contacts",
        campaign_id,
        payload.contacts.len()
    );

    if payload.contacts.is_empty() {
        println!("[CONTACTS] No contacts provided in payload");
        return (StatusCode::BAD_REQUEST, "No contacts provided").into_response();
    }

    let client = &state.anything_client;

    // Create campaign_contacts entries for each contact
    let mut campaign_contacts = Vec::new();
    for contact in payload.contacts {
        println!(
            "[CONTACTS] Adding contact {} to campaign {}",
            contact.contact_id, campaign_id
        );
        campaign_contacts.push(serde_json::json!({
            "account_id": account_id,
            "campaign_id": campaign_id,
            "contact_id": contact.contact_id,
            "status": "active",
            "active": true,
            "archived": false
        }));
    }

    // Insert all contacts at once
    let response = match client
        .from("campaign_contacts")
        .auth(&user.jwt)
        .insert(serde_json::to_string(&campaign_contacts).unwrap())
        .execute()
        .await
    {
        Ok(response) => {
            println!(
                "[CONTACTS] Successfully inserted {} contacts into campaign",
                campaign_contacts.len()
            );
            response
        }
        Err(err) => {
            println!("[CONTACTS] Failed to execute request: {:?}", err);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to add contacts to campaign",
            )
                .into_response();
        }
    };

    let body = match response.text().await {
        Ok(body) => {
            println!("[CONTACTS] Successfully read response body");
            body
        }
        Err(err) => {
            println!("[CONTACTS] Failed to read response body: {:?}", err);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to read response body",
            )
                .into_response();
        }
    };

    let result: Value = match serde_json::from_str(&body) {
        Ok(result) => {
            println!("[CONTACTS] Successfully parsed JSON response");
            result
        }
        Err(err) => {
            println!("[CONTACTS] Failed to parse JSON: {:?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to parse JSON").into_response();
        }
    };

    (StatusCode::CREATED, Json(result)).into_response()
}

pub async fn remove_contact_from_campaign(
    Path((account_id, campaign_id, contact_id)): Path<(String, String, String)>,
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
) -> impl IntoResponse {
    println!(
        "[CONTACTS] Handling remove_contact_from_campaign for contact {} in campaign {}",
        contact_id, campaign_id
    );

    let client = &state.anything_client;

    // We don't actually delete the contact, just mark it as archived and inactive
    let update_data = serde_json::json!({
        "archived": true,
        "active": false
    });

    let response = match client
        .from("campaign_contacts")
        .auth(&user.jwt)
        .eq("campaign_id", &campaign_id)
        .eq("contact_id", &contact_id)
        .eq("account_id", &account_id)
        .update(update_data.to_string())
        .execute()
        .await
    {
        Ok(response) => {
            println!(
                "[CONTACTS] Successfully marked contact {} as archived",
                contact_id
            );
            response
        }
        Err(err) => {
            println!("[CONTACTS] Failed to execute request: {:?}", err);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to remove contact from campaign",
            )
                .into_response();
        }
    };

    let body = match response.text().await {
        Ok(body) => {
            println!("[CONTACTS] Successfully read response body");
            body
        }
        Err(err) => {
            println!("[CONTACTS] Failed to read response body: {:?}", err);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to read response body",
            )
                .into_response();
        }
    };

    let result: Value = match serde_json::from_str(&body) {
        Ok(result) => {
            println!("[CONTACTS] Successfully parsed JSON response");
            result
        }
        Err(err) => {
            println!("[CONTACTS] Failed to parse JSON: {:?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to parse JSON").into_response();
        }
    };

    Json(result).into_response()
}

/// Creates new contacts and adds them to a campaign.
///
/// This function processes each contact individually (no batching):
/// 1. For each contact, it first checks if a contact with the same phone number already exists
/// 2. If the contact exists, it uses that contact's ID
/// 3. If the contact doesn't exist, it creates a new contact
/// 4. It then adds the contact to the campaign
///
/// This approach ensures that:
/// - Contacts are deduplicated by phone number
/// - Each contact is processed independently
/// - No batch operations are performed, reducing the risk of partial failures
pub async fn create_and_add_contacts_to_campaign(
    Path((account_id, campaign_id)): Path<(String, String)>,
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
    _headers: HeaderMap,
    Json(payload): Json<CreateAndAddContactsInput>,
) -> impl IntoResponse {
    println!(
        "[CONTACTS] Handling create_and_add_contacts_to_campaign for campaign {} with {} contacts",
        campaign_id,
        payload.contacts.len()
    );

    if payload.contacts.is_empty() {
        println!("[CONTACTS] No contacts provided in payload");
        return (StatusCode::BAD_REQUEST, "No contacts provided").into_response();
    }

    let client = &state.anything_client;

    // Track statistics for the operation
    let mut existing_contacts_count = 0;
    let mut created_contacts_count = 0;
    let mut added_to_campaign_count = 0;

    // Process each contact individually
    for contact in payload.contacts {
        println!(
            "[CONTACTS] Processing contact with phone number: {}",
            contact.phone_number
        );

        // Step 1: Check if contact with this phone number already exists
        let response = match client
            .from("contacts")
            .auth(&user.jwt)
            .select("contact_id, phone")
            .eq("account_id", &account_id)
            .eq("archived", "false")
            .eq("phone", &contact.phone_number)
            .execute()
            .await
        {
            Ok(response) => response,
            Err(err) => {
                println!("[CONTACTS] Failed to check for existing contact: {:?}", err);
                continue; // Skip this contact and move to the next one
            }
        };

        let contact_id = if response.status() != 204 {
            let body = match response.text().await {
                Ok(body) => body,
                Err(err) => {
                    println!("[CONTACTS] Failed to read response body: {:?}", err);
                    continue; // Skip this contact and move to the next one
                }
            };

            let existing_contact: Value = match serde_json::from_str(&body) {
                Ok(contact) => contact,
                Err(err) => {
                    println!("[CONTACTS] Failed to parse JSON: {:?}", err);
                    continue; // Skip this contact and move to the next one
                }
            };

            // Check if we found an existing contact
            if let Some(contacts_array) = existing_contact.as_array() {
                if !contacts_array.is_empty() {
                    if let Some(id) = contacts_array[0].get("contact_id").and_then(Value::as_str) {
                        println!("[CONTACTS] Found existing contact with ID: {}", id);
                        existing_contacts_count += 1;
                        id.to_string()
                    } else {
                        // Create new contact as we couldn't extract the ID
                        match create_new_contact(client, &user.jwt, &account_id, &contact).await {
                            Some(id) => {
                                created_contacts_count += 1;
                                id
                            }
                            None => {
                                println!("[CONTACTS] Failed to create new contact");
                                continue; // Skip this contact and move to the next one
                            }
                        }
                    }
                } else {
                    // No existing contact found, create a new one
                    match create_new_contact(client, &user.jwt, &account_id, &contact).await {
                        Some(id) => {
                            created_contacts_count += 1;
                            id
                        }
                        None => {
                            println!("[CONTACTS] Failed to create new contact");
                            continue; // Skip this contact and move to the next one
                        }
                    }
                }
            } else {
                // No array returned, create a new contact
                match create_new_contact(client, &user.jwt, &account_id, &contact).await {
                    Some(id) => {
                        created_contacts_count += 1;
                        id
                    }
                    None => {
                        println!("[CONTACTS] Failed to create new contact");
                        continue; // Skip this contact and move to the next one
                    }
                }
            }
        } else {
            // No existing contact found, create a new one
            match create_new_contact(client, &user.jwt, &account_id, &contact).await {
                Some(id) => {
                    created_contacts_count += 1;
                    id
                }
                None => {
                    println!("[CONTACTS] Failed to create new contact");
                    continue; // Skip this contact and move to the next one
                }
            }
        };

        // Step 3: Add contact to campaign
        let campaign_contact = serde_json::json!({
            "account_id": account_id,
            "campaign_id": campaign_id,
            "contact_id": contact_id,
            "status": "active",
            "active": true,
            "archived": false
        });

        println!(
            "[CONTACTS] Adding contact {} to campaign {}",
            contact_id, campaign_id
        );
        let response = match client
            .from("campaign_contacts")
            .auth(&user.jwt)
            .insert(campaign_contact.to_string())
            .execute()
            .await
        {
            Ok(response) => {
                println!("[CONTACTS] Successfully added contact to campaign");
                added_to_campaign_count += 1;
                response
            }
            Err(err) => {
                println!("[CONTACTS] Failed to add contact to campaign: {:?}", err);
                continue; // Skip this contact and move to the next one
            }
        };
    }

    // Return summary of entire operation
    let summary = serde_json::json!({
        "created_contacts": created_contacts_count,
        "existing_contacts": existing_contacts_count,
        "added_to_campaign": added_to_campaign_count
    });

    println!(
        "[CONTACTS] Operation complete - Created: {}, Existing: {}, Added to campaign: {}",
        created_contacts_count, existing_contacts_count, added_to_campaign_count
    );

    (StatusCode::CREATED, Json(summary)).into_response()
}

/// Helper function to create a new contact in the database
///
/// This function creates a single contact and returns its ID if successful.
/// It handles all error cases internally and returns None if the contact creation fails.
///
/// Parameters:
/// - client: The Postgrest client to use for the database operation
/// - jwt: The JWT token for authentication
/// - account_id: The account ID to associate with the contact
/// - contact: The contact data to create
///
/// Returns:
/// - Some(contact_id) if the contact was created successfully
/// - None if there was an error creating the contact
async fn create_new_contact(
    client: &postgrest::Postgrest,
    jwt: &str,
    account_id: &str,
    contact: &NewContactInput,
) -> Option<String> {
    println!(
        "[CONTACTS] Creating new contact with phone number: {}",
        contact.phone_number
    );

    // Split the name into first_name and last_name if possible
    let (first_name, last_name) = if let Some(space_idx) = contact.name.find(' ') {
        let (first, last) = contact.name.split_at(space_idx);
        (first.trim().to_string(), last.trim().to_string())
    } else {
        (contact.name.clone(), String::new())
    };

    // Create contact with fields matching the SQL schema
    let new_contact = serde_json::json!({
        "account_id": account_id,
        "first_name": first_name,
        "last_name": last_name,
        "phone": contact.phone_number,
        "email": contact.email,
        // Store additional data in custom_fields
        "custom_fields": contact.additional_data,
        "archived": false
    });

    let response = match client
        .from("contacts")
        .auth(jwt)
        .insert(new_contact.to_string())
        .execute()
        .await
    {
        Ok(response) => response,
        Err(err) => {
            println!("[CONTACTS] Failed to create new contact: {:?}", err);
            return None;
        }
    };

    let body = match response.text().await {
        Ok(body) => body,
        Err(err) => {
            println!("[CONTACTS] Failed to read response body: {:?}", err);
            return None;
        }
    };

    let created_contact: Value = match serde_json::from_str(&body) {
        Ok(contact) => contact,
        Err(err) => {
            println!("[CONTACTS] Failed to parse JSON: {:?}", err);
            return None;
        }
    };

    if let Some(contacts_array) = created_contact.as_array() {
        if !contacts_array.is_empty() {
            if let Some(id) = contacts_array[0].get("contact_id").and_then(Value::as_str) {
                println!("[CONTACTS] Created new contact with ID: {}", id);
                return Some(id.to_string());
            }
        }
    }

    None
}
