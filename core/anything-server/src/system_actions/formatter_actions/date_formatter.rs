use chrono::{DateTime, Duration, NaiveDateTime, Offset, Utc};
use chrono_tz::Tz;
use html2md::parse_html;
use pulldown_cmark::{html, Options, Parser};
use rand::Rng;
use regex::Regex;
use serde_json::{json, Value};

pub async fn process_date_task(
    bundled_context: &Value,
) -> Result<Value, Box<dyn std::error::Error + Send + Sync>> {
    let input = bundled_context
        .get("input")
        .and_then(|v| v.as_str())
        .unwrap_or("");

    let operation = bundled_context
        .get("operation")
        .and_then(|v| v.as_str())
        .unwrap_or("");

    let amount = bundled_context
        .get("amount")
        .and_then(|v| v.as_i64())
        .unwrap_or(0);

    let source_timezone = bundled_context
        .get("source_timezone")
        .and_then(|v| v.as_str())
        .unwrap_or("UTC");

    let target_timezone = bundled_context
        .get("target_timezone")
        .and_then(|v| v.as_str())
        .unwrap_or("UTC");

    let format = bundled_context
        .get("format")
        .and_then(|v| v.as_str())
        .unwrap_or("%Y-%m-%d %H:%M:%S");

    // Parse the input date string
    let parsed_date = if let Ok(dt) = DateTime::parse_from_rfc3339(input) {
        dt
    } else if let Ok(dt) = NaiveDateTime::parse_from_str(input, "%Y-%m-%d %H:%M:%S") {
        DateTime::from_naive_utc_and_offset(dt, Utc.fix())
    } else {
        return Ok(json!({
            "result": "Invalid date format. Please use RFC3339 or YYYY-MM-DD HH:MM:SS"
        }));
    };

    let result = match operation {
        "add_days" => parsed_date
            .checked_add_signed(Duration::days(amount))
            .map_or("Invalid date calculation".to_string(), |d| d.to_rfc3339()),
        "subtract_days" => parsed_date
            .checked_sub_signed(Duration::days(amount))
            .map_or("Invalid date calculation".to_string(), |d| d.to_rfc3339()),
        "add_hours" => parsed_date
            .checked_add_signed(Duration::hours(amount))
            .map_or("Invalid date calculation".to_string(), |d| d.to_rfc3339()),
        "subtract_hours" => parsed_date
            .checked_sub_signed(Duration::hours(amount))
            .map_or("Invalid date calculation".to_string(), |d| d.to_rfc3339()),
        "add_minutes" => parsed_date
            .checked_add_signed(Duration::minutes(amount))
            .map_or("Invalid date calculation".to_string(), |d| d.to_rfc3339()),
        "subtract_minutes" => parsed_date
            .checked_sub_signed(Duration::minutes(amount))
            .map_or("Invalid date calculation".to_string(), |d| d.to_rfc3339()),
        "format" => parsed_date.format(format).to_string(),
        "to_timezone" => {
            if let Ok(tz) = target_timezone.parse::<Tz>() {
                parsed_date.with_timezone(&tz).format(format).to_string()
            } else {
                "Invalid timezone".to_string()
            }
        }
        "convert_timezone" => {
            if let (Ok(source_tz), Ok(target_tz)) =
                (source_timezone.parse::<Tz>(), target_timezone.parse::<Tz>())
            {
                parsed_date
                    .with_timezone(&source_tz)
                    .with_timezone(&target_tz)
                    .format(format)
                    .to_string()
            } else {
                "Invalid timezone(s)".to_string()
            }
        }
        "to_unix" => parsed_date.timestamp().to_string(),
        "from_unix" => {
            if let Ok(ts) = input.parse::<i64>() {
                DateTime::<Utc>::from_timestamp(ts, 0)
                    .map_or("Invalid unix timestamp".to_string(), |dt| dt.to_rfc3339())
            } else {
                "Invalid unix timestamp".to_string()
            }
        }
        "is_future" => (parsed_date > Utc::now()).to_string(),
        "is_past" => (parsed_date < Utc::now()).to_string(),
        "difference_days" => {
            if let Ok(end_date) = input.parse::<DateTime<Utc>>() {
                let start_date = parsed_date;
                (end_date.signed_duration_since(start_date).num_days()).to_string()
            } else {
                "Invalid end date".to_string()
            }
        }
        "difference_hours" => {
            if let Ok(end_date) = input.parse::<DateTime<Utc>>() {
                let start_date = parsed_date;
                (end_date.signed_duration_since(start_date).num_hours()).to_string()
            } else {
                "Invalid end date".to_string()
            }
        }
        "difference_minutes" => {
            if let Ok(end_date) = input.parse::<DateTime<Utc>>() {
                let start_date = parsed_date;
                (end_date.signed_duration_since(start_date).num_minutes()).to_string()
            } else {
                "Invalid end date".to_string()
            }
        }
        "difference_seconds" => {
            if let Ok(end_date) = input.parse::<DateTime<Utc>>() {
                let start_date = parsed_date;
                (end_date.signed_duration_since(start_date).num_seconds()).to_string()
            } else {
                "Invalid end date".to_string()
            }
        }
        _ => input.to_string(),
    };

    Ok(json!({ "result": result }))
}
