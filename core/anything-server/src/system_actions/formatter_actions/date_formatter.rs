use chrono::{DateTime, Duration, NaiveDateTime, Offset, Utc};
use chrono_tz::Tz;
use serde_json::{json, Value};

pub async fn process_date_task(
    bundled_context: &Value,
) -> Result<Value, Box<dyn std::error::Error + Send + Sync>> {
    println!("[DATE FORMATTER] Starting date task processing");
    println!("[DATE FORMATTER] Bundled context: {:?}", bundled_context);

    let input = bundled_context
        .get("input")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    println!("[DATE FORMATTER] Input value: {}", input);

    let operation = bundled_context
        .get("operation")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    println!("[DATE FORMATTER] Operation: {}", operation);

    let amount = bundled_context
        .get("amount")
        .and_then(|v| match v {
            Value::Number(n) => n.as_i64(),
            Value::String(s) => s.parse::<i64>().ok(),
            _ => None,
        })
        .unwrap_or(0);
    println!("[DATE FORMATTER] Amount: {}", amount);

    let source_timezone = bundled_context
        .get("source_timezone")
        .and_then(|v| v.as_str())
        .unwrap_or("UTC");
    println!("[DATE FORMATTER] Source timezone: {}", source_timezone);

    let target_timezone = bundled_context
        .get("target_timezone")
        .and_then(|v| v.as_str())
        .unwrap_or("UTC");
    println!("[DATE FORMATTER] Target timezone: {}", target_timezone);

    let format = bundled_context
        .get("format")
        .and_then(|v| v.as_str())
        .unwrap_or("%Y-%m-%d %H:%M:%S");
    println!("[DATE FORMATTER] Format: {}", format);

    // Parse the input date string
    println!("[DATE FORMATTER] Attempting to parse date");
    let parsed_date = if let Ok(dt) = DateTime::parse_from_rfc3339(input) {
        println!("[DATE FORMATTER] Successfully parsed RFC3339 date");
        dt
    } else if let Ok(dt) = NaiveDateTime::parse_from_str(input, "%Y-%m-%d %H:%M:%S") {
        println!("[DATE FORMATTER] Successfully parsed YYYY-MM-DD HH:MM:SS date");
        DateTime::from_naive_utc_and_offset(dt, Utc.fix())
    } else {
        println!("[DATE FORMATTER] Failed to parse date");
        return Ok(json!({
            "error": "Invalid date format. Please use RFC3339 or YYYY-MM-DD HH:MM:SS"
        }));
    };
    println!("[DATE FORMATTER] Parsed date: {}", parsed_date);

    println!("[DATE FORMATTER] Processing operation: {}", operation);
    let result = match operation {
        "add_days" => {
            println!("[DATE FORMATTER] Adding {} days", amount);
            parsed_date
                .checked_add_signed(Duration::days(amount))
                .map_or("Invalid date calculation".to_string(), |d| d.to_rfc3339())
        }
        "subtract_days" => {
            println!("[DATE FORMATTER] Subtracting {} days", amount);
            parsed_date
                .checked_sub_signed(Duration::days(amount))
                .map_or("Invalid date calculation".to_string(), |d| d.to_rfc3339())
        }
        "add_hours" => {
            println!("[DATE FORMATTER] Adding {} hours", amount);
            parsed_date
                .checked_add_signed(Duration::hours(amount))
                .map_or("Invalid date calculation".to_string(), |d| d.to_rfc3339())
        }
        "subtract_hours" => {
            println!("[DATE FORMATTER] Subtracting {} hours", amount);
            parsed_date
                .checked_sub_signed(Duration::hours(amount))
                .map_or("Invalid date calculation".to_string(), |d| d.to_rfc3339())
        }
        "add_minutes" => {
            println!("[DATE FORMATTER] Adding {} minutes", amount);
            parsed_date
                .checked_add_signed(Duration::minutes(amount))
                .map_or("Invalid date calculation".to_string(), |d| d.to_rfc3339())
        }
        "subtract_minutes" => {
            println!("[DATE FORMATTER] Subtracting {} minutes", amount);
            parsed_date
                .checked_sub_signed(Duration::minutes(amount))
                .map_or("Invalid date calculation".to_string(), |d| d.to_rfc3339())
        }
        "format" => {
            //https://docs.rs/chrono/latest/chrono/format/strftime/index.html
            println!("[DATE FORMATTER] Formatting date with format: {}", format);
            parsed_date.format(format).to_string()
        }
        "to_timezone" => {
            println!(
                "[DATE FORMATTER] Converting to timezone: {}",
                target_timezone
            );
            if let Ok(tz) = target_timezone.parse::<Tz>() {
                parsed_date.with_timezone(&tz).format(format).to_string()
            } else {
                println!("[DATE FORMATTER] Invalid target timezone");
                "Invalid timezone".to_string()
            }
        }
        "convert_timezone" => {
            println!(
                "[DATE FORMATTER] Converting from {} to {}",
                source_timezone, target_timezone
            );
            //https://en.wikipedia.org/wiki/List_of_tz_database_time_zones
            if let (Ok(source_tz), Ok(target_tz)) =
                (source_timezone.parse::<Tz>(), target_timezone.parse::<Tz>())
            {
                parsed_date
                    .with_timezone(&source_tz)
                    .with_timezone(&target_tz)
                    .format(format)
                    .to_string()
            } else {
                println!("[DATE FORMATTER] Invalid timezone(s)");
                "Invalid timezone(s)".to_string()
            }
        }
        "to_unix" => {
            println!("[DATE FORMATTER] Converting to Unix timestamp");
            parsed_date.timestamp().to_string()
        }
        "from_unix" => {
            println!("[DATE FORMATTER] Converting from Unix timestamp: {}", input);
            if let Ok(ts) = input.parse::<i64>() {
                DateTime::<Utc>::from_timestamp(ts, 0)
                    .map_or("Invalid unix timestamp".to_string(), |dt| dt.to_rfc3339())
            } else {
                println!("[DATE FORMATTER] Invalid Unix timestamp");
                "Invalid unix timestamp".to_string()
            }
        }
        _ => {
            println!("[DATE FORMATTER] Unknown operation, returning error");
            return Ok(json!({ "error": "Invalid operation" }));
        }
    };

    println!("[DATE FORMATTER] Operation result: {:?}", result);
    Ok(json!({ "formatted_date": result }))
}
