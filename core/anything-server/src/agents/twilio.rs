use crate::supabase_jwt_middleware::User;
use crate::AppState;
use anyhow::Result;
use axum::extract::Extension;
use axum::http::StatusCode;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::Arc;

use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct TwilioPhoneNumber {
    #[serde(rename = "friendly_name")]
    pub friendly_name: Option<String>,
    pub phone_number: Option<String>,
    pub lata: Option<String>,
    pub locality: Option<String>,
    #[serde(rename = "rate_center")]
    pub rate_center: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub region: Option<String>,
    #[serde(rename = "postal_code")]
    pub postal_code: Option<String>,
    #[serde(rename = "iso_country")]
    pub iso_country: Option<String>,
    #[serde(rename = "address_requirements")]
    pub address_requirements: Option<String>,
    pub beta: Option<bool>,
    pub capabilities: Option<PhoneNumberCapabilities>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PhoneNumberCapabilities {
    pub voice: bool,
    pub sms: bool,
    pub mms: bool,
}

#[derive(Debug, Deserialize)]
pub struct PurchasePhoneNumberInput {
    phone_number: String,
}

pub async fn purchase_phone_number(
    Path(account_id): Path<String>,
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
    Json(payload): Json<PurchasePhoneNumberInput>,
) -> impl IntoResponse {
    println!(
        "[TWILIO] Attempting to purchase phone number: {}",
        payload.phone_number
    );

    // Get Twilio credentials
    let account_sid = match std::env::var("TWILIO_ACCOUNT_SID") {
        Ok(sid) => sid,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Missing Twilio account SID",
            )
                .into_response()
        }
    };

    let auth_token = match std::env::var("TWILIO_AUTH_TOKEN") {
        Ok(token) => token,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Missing Twilio auth token",
            )
                .into_response()
        }
    };

    let client = Client::new();

    // Purchase the phone number
    let response = match client
        .post(&format!(
            "https://api.twilio.com/2010-04-01/Accounts/{}/IncomingPhoneNumbers.json",
            account_sid
        ))
        .basic_auth(&account_sid, Some(&auth_token))
        .form(&[("PhoneNumber", &payload.phone_number)])
        .send()
        .await
    {
        Ok(response) => response,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to make Twilio API request",
            )
                .into_response()
        }
    };

    let body = match response.text().await {
        Ok(body) => body,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to read Twilio API response",
            )
                .into_response()
        }
    };

    let phone_number: Value = match serde_json::from_str(&body) {
        Ok(number) => number,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to parse Twilio response",
            )
                .into_response()
        }
    };

    println!("[TWILIO] Phone number: {:?}", phone_number);

    // Insert the phone number into our database
    let phone_number_input = serde_json::json!({
        "account_id": account_id,
        "phone_number": phone_number["phone_number"].as_str().unwrap_or(""),
        "twilio_sid": phone_number["sid"].as_str().unwrap_or(""),
        "twilio_friendly_name": phone_number["friendly_name"].as_str().unwrap_or(""),
        "voice_url": phone_number["voice_url"].as_str().unwrap_or(""),
        "status": "active",
        "twilio_properties": phone_number,
        "capabilities": phone_number["capabilities"],
        "active": true
    });

    let db_client = &state.anything_client;

    let db_response = match db_client
        .from("phone_numbers")
        .auth(user.jwt)
        .insert(phone_number_input.to_string())
        .execute()
        .await
    {
        Ok(response) => response,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to insert phone number into database",
            )
                .into_response()
        }
    };

    let _db_body = match db_response.text().await {
        Ok(body) => body,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to read database response",
            )
                .into_response()
        }
    };

    //TODO: add this number to an agent

    Json(phone_number).into_response()
}

pub async fn delete_twilio_number(phone_number_sid: &str) -> Result<()> {
    let account_sid = std::env::var("TWILIO_ACCOUNT_SID")?;
    let auth_token = std::env::var("TWILIO_AUTH_TOKEN")?;
    let client = reqwest::Client::new();

    // Delete the phone number using its SID
    client
        .delete(&format!(
            "https://api.twilio.com/2010-04-01/Accounts/{}/IncomingPhoneNumbers/{}.json",
            account_sid, phone_number_sid
        ))
        .basic_auth(&account_sid, Some(&auth_token))
        .send()
        .await?;

    Ok(())
}

//https://www.twilio.com/docs/phone-numbers/api/availablephonenumberlocal-resource
pub async fn search_available_phone_numbers_on_twilio(
    Path((account_id, country, area_code)): Path<(String, String, String)>,
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
) -> Result<Json<Value>, (StatusCode, String)> {
    println!(
        "[TWILIO] Searching for phone numbers in country: {}, area code: {}",
        country, area_code
    );

    println!("[TWILIO] Getting Twilio credentials from environment");
    let account_sid = std::env::var("TWILIO_ACCOUNT_SID")
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    let auth_token = std::env::var("TWILIO_AUTH_TOKEN")
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    let client = reqwest::Client::new();

    let mut params = vec![];
    params.push(("AreaCode", &area_code));

    println!("[TWILIO] Making API request to search for available numbers");
    let available_numbers = client
        .get(&format!(
            "https://api.twilio.com/2010-04-01/Accounts/{}/AvailablePhoneNumbers/{}/Local.json",
            account_sid, country
        ))
        .basic_auth(&account_sid, Some(&auth_token))
        .query(&params)
        .send()
        .await
        .map_err(|e| {
            println!("[TWILIO] Error making API request: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        })?
        .json::<serde_json::Value>()
        .await
        .map_err(|e| {
            println!("[TWILIO] Error parsing API response: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        })?;

    println!("[TWILIO] Processing available phone numbers from response");
    println!("[TWILIO] Available numbers: {:?}", available_numbers);

    Ok(Json(available_numbers["available_phone_numbers"].clone()))
}

pub async fn get_account_phone_numbers(
    Path(account_id): Path<String>,
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
) -> impl IntoResponse {
    println!("Handling a get_phone_numbers");

    let client = &state.anything_client;

    //Orde_with_options docs
    //https://github.com/supabase-community/postgrest-rs/blob/d740c1e739547d6c36482af61fc8673e23232fdd/src/builder.rs#L196
    let response = match client
        .from("phone_numbers")
        .auth(&user.jwt) // Pass a reference to the JWT
        .select("*, agent_communication_channels(*)")
        .eq("archived", "false")
        .eq("account_id", &account_id)
        .execute()
        .await
    {
        Ok(response) => response,
        Err(err) => {
            println!("Failed to execute request: {:?}", err);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to execute request",
            )
                .into_response();
        }
    };

    if response.status() == 204 {
        return (StatusCode::NO_CONTENT, "No content").into_response();
    }

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

    let items: Value = match serde_json::from_str(&body) {
        Ok(items) => items,
        Err(err) => {
            println!("Failed to parse JSON: {:?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to parse JSON").into_response();
        }
    };

    Json(items).into_response()
}
