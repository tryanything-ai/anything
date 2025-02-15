use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct TwilioPhoneNumber {
    pub phone_number: String,
    pub sid: String,
}

pub async fn provision_twilio_number(target_area_code: &str) -> Result<TwilioPhoneNumber> {
    let account_sid = std::env::var("TWILIO_ACCOUNT_SID")?;
    let auth_token = std::env::var("TWILIO_AUTH_TOKEN")?;
    let client = Client::new();

    // Search for available numbers in target area code
    let mut available_numbers = client
        .get(&format!(
            "https://api.twilio.com/2010-04-01/Accounts/{}/AvailablePhoneNumbers/US/Local.json",
            account_sid
        ))
        .query(&[("AreaCode", target_area_code)])
        .basic_auth(&account_sid, Some(&auth_token))
        .send()
        .await?
        .json::<Value>()
        .await?;

    // If no numbers found in target area code, search nearby area codes
    if available_numbers["available_phone_numbers"].as_array().map_or(true, |arr| arr.is_empty()) {
        available_numbers = client
            .get(&format!(
                "https://api.twilio.com/2010-04-01/Accounts/{}/AvailablePhoneNumbers/US/Local.json",
                account_sid
            ))
            .query(&[("NearNumber", format!("+1{}", target_area_code))])
            .basic_auth(&account_sid, Some(&auth_token))
            .send()
            .await?
            .json::<Value>()
            .await?;
    }

    let phone_number = available_numbers["available_phone_numbers"][0]["phone_number"]
        .as_str()
        .ok_or_else(|| anyhow::anyhow!("No available phone numbers found"))?;

    // Purchase the first available number
    let purchased_number = client
        .post(&format!(
            "https://api.twilio.com/2010-04-01/Accounts/{}/IncomingPhoneNumbers.json",
            account_sid
        ))
        .basic_auth(&account_sid, Some(&auth_token))
        .form(&[("PhoneNumber", phone_number)])
        .send()
        .await?
        .json::<Value>()
        .await?;

    Ok(TwilioPhoneNumber {
        phone_number: purchased_number["phone_number"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Failed to get phone number from response"))?
            .to_string(),
        sid: purchased_number["sid"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Failed to get SID from response"))?
            .to_string(),
    })
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
