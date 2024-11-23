use chrono::{DateTime, Duration, NaiveDateTime, Offset, Utc};
use chrono_tz::Tz;
use html2md::parse_html;
use pulldown_cmark::{html, Options, Parser};
use rand::Rng;
use regex::Regex;
use serde_json::{json, Value};

pub async fn process_number_task(
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

    let min = bundled_context
        .get("min")
        .and_then(|v| v.as_f64())
        .unwrap_or(0.0);

    let max = bundled_context
        .get("max")
        .and_then(|v| v.as_f64())
        .unwrap_or(100.0);

    let result = match operation {
        "random_number" => {
            let mut rng = rand::thread_rng();
            rng.gen_range(min..=max).to_string()
        }
        _ => input.to_string(),
    };

    Ok(json!({ "result": result }))
}
