use crate::supabase_jwt_middleware::User;
use crate::AppState;
use anyhow::Result;
use axum::extract::Extension;
use axum::http::StatusCode;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::Arc;

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

// pub async fn provision_twilio_number(target_area_code: &str) -> Result<TwilioPhoneNumber> {
//     let account_sid = std::env::var("TWILIO_ACCOUNT_SID")?;
//     let auth_token = std::env::var("TWILIO_AUTH_TOKEN")?;
//     let client = Client::new();

//     // Search for available numbers in target area code
//     let mut available_numbers = client
//         .get(&format!(
//             "https://api.twilio.com/2010-04-01/Accounts/{}/AvailablePhoneNumbers/US/Local.json",
//             account_sid
//         ))
//         .query(&[("AreaCode", target_area_code)])
//         .basic_auth(&account_sid, Some(&auth_token))
//         .send()
//         .await?
//         .json::<Value>()
//         .await?;

//     // If no numbers found in target area code, search nearby area codes
//     if available_numbers["available_phone_numbers"]
//         .as_array()
//         .map_or(true, |arr| arr.is_empty())
//     {
//         available_numbers = client
//             .get(&format!(
//                 "https://api.twilio.com/2010-04-01/Accounts/{}/AvailablePhoneNumbers/US/Local.json",
//                 account_sid
//             ))
//             .query(&[("NearNumber", format!("+1{}", target_area_code))])
//             .basic_auth(&account_sid, Some(&auth_token))
//             .send()
//             .await?
//             .json::<Value>()
//             .await?;
//     }

//     let phone_number = available_numbers["available_phone_numbers"][0]["phone_number"]
//         .as_str()
//         .ok_or_else(|| anyhow::anyhow!("No available phone numbers found"))?;

//     // Purchase the first available number
//     let purchased_number = client
//         .post(&format!(
//             "https://api.twilio.com/2010-04-01/Accounts/{}/IncomingPhoneNumbers.json",
//             account_sid
//         ))
//         .basic_auth(&account_sid, Some(&auth_token))
//         .form(&[("PhoneNumber", phone_number)])
//         .send()
//         .await?
//         .json::<Value>()
//         .await?;

// Ok(TwilioPhoneNumber {
//     phone_number: purchased_number["phone_number"]
//         .as_str()
//         .ok_or_else(|| anyhow::anyhow!("Failed to get phone number from response"))?
//         .to_string(),
//     sid: purchased_number["sid"]
//         .as_str()
//         .ok_or_else(|| anyhow::anyhow!("Failed to get SID from response"))?
//         .to_string(),
// })
// }

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

use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};

//https://www.twilio.com/docs/phone-numbers/api/availablephonenumberlocal-resource
pub async fn search_phone_numbers(
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
    // params.push(("PageSize", "20".to_string()));

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
