use axum::{extract::Path, response::IntoResponse, Extension, Json};
use chrono::Utc;
use serde_json::Value;
use std::collections::HashMap;

use crate::supabase_jwt_middleware::User;

pub fn get_system_variables() -> HashMap<String, Value> {
    let mut system_vars = HashMap::new();

    let now = Utc::now();

    // Current UTC timestamp and date/time components
    system_vars.insert(
        "utc_timestamp".to_string(),
        Value::Number(now.timestamp().into()),
    ); // Unix timestamp
    system_vars.insert(
        "utc_date".to_string(),
        Value::String(now.format("%Y-%m-%d").to_string()),
    ); // YYYY-MM-DD
    system_vars.insert(
        "utc_time".to_string(),
        Value::String(now.format("%H:%M:%S").to_string()),
    ); // HH:MM:SS
    system_vars.insert(
        "utc_year".to_string(),
        Value::String(now.format("%Y").to_string()),
    ); // YYYY
    system_vars.insert(
        "utc_month".to_string(),
        Value::String(now.format("%m").to_string()),
    ); // MM
    system_vars.insert(
        "utc_day".to_string(),
        Value::String(now.format("%d").to_string()),
    ); // DD
    system_vars.insert(
        "utc_hour".to_string(),
        Value::String(now.format("%H").to_string()),
    ); // HH
    system_vars.insert(
        "utc_minute".to_string(),
        Value::String(now.format("%M").to_string()),
    ); // MM
    system_vars.insert(
        "utc_second".to_string(),
        Value::String(now.format("%S").to_string()),
    ); // SS

    // ISO 8601 formats
    system_vars.insert("utc_iso_8601".to_string(), Value::String(now.to_rfc3339())); // Full ISO 8601/RFC 3339 format
    system_vars.insert(
        "utc_iso_date".to_string(),
        Value::String(now.format("%Y-%m-%d").to_string()),
    ); // ISO date only
    system_vars.insert(
        "utc_iso_time".to_string(),
        Value::String(now.format("%H:%M:%S%:z").to_string()),
    ); // ISO time with timezone

    // RFC formats
    system_vars.insert("utc_rfc_2822".to_string(), Value::String(now.to_rfc2822())); // RFC 2822 format
    system_vars.insert("utc_rfc_3339".to_string(), Value::String(now.to_rfc3339())); // RFC 3339 format

    // Additional useful formats
    system_vars.insert(
        "utc_timestamp_millis".to_string(),
        Value::Number(now.timestamp_millis().into()),
    ); // Unix timestamp with milliseconds
    system_vars.insert(
        "utc_week".to_string(),
        Value::String(now.format("%V").to_string()),
    ); // ISO week number
    system_vars.insert(
        "utc_weekday".to_string(),
        Value::String(now.format("%A").to_string()),
    ); // Full weekday name
    system_vars.insert(
        "utc_month_name".to_string(),
        Value::String(now.format("%B").to_string()),
    ); // Full month name

    system_vars
}

pub async fn get_system_variables_handler(
    Path(account_id): Path<String>,
    Extension(_user): Extension<User>,
) -> impl IntoResponse {
    println!(
        "[SYSTEM VARIABLES] Getting system variables for account: {}",
        account_id
    );

    let system_vars = get_system_variables();
    let result = serde_json::json!(system_vars);

    println!("[SYSTEM VARIABLES] Returning response");
    Json(result).into_response()
}
