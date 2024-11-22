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
        "math" => {
            // Parse and evaluate mathematical expression
            let expr = input.replace(" ", ""); // Remove spaces
            let mut stack_nums: Vec<f64> = Vec::new();
            let mut stack_ops: Vec<char> = Vec::new();
            let mut num = String::new();

            // Helper function to apply operator
            let apply_op = |a: f64, b: f64, op: char| -> f64 {
                match op {
                    '+' => a + b,
                    '-' => a - b,
                    '*' => a * b,
                    '/' => a / b,
                    _ => 0.0,
                }
            };

            // Helper function to get operator precedence
            let precedence = |op: char| -> i32 {
                match op {
                    '+' | '-' => 1,
                    '*' | '/' => 2,
                    _ => 0,
                }
            };

            for c in expr.chars() {
                if c.is_digit(10) || c == '.' {
                    num.push(c);
                } else {
                    if !num.is_empty() {
                        if let Ok(n) = num.parse::<f64>() {
                            stack_nums.push(n);
                        }
                        num.clear();
                    }

                    if c == '(' {
                        stack_ops.push(c);
                    } else if c == ')' {
                        while let Some(&op) = stack_ops.last() {
                            if op == '(' {
                                stack_ops.pop();
                                break;
                            }
                            if stack_nums.len() >= 2 {
                                let b = stack_nums.pop().unwrap();
                                let a = stack_nums.pop().unwrap();
                                stack_nums.push(apply_op(a, b, stack_ops.pop().unwrap()));
                            }
                        }
                    } else if ['+', '-', '*', '/'].contains(&c) {
                        while !stack_ops.is_empty()
                            && stack_ops.last().unwrap() != &'('
                            && precedence(*stack_ops.last().unwrap()) >= precedence(c)
                        {
                            if stack_nums.len() >= 2 {
                                let b = stack_nums.pop().unwrap();
                                let a = stack_nums.pop().unwrap();
                                stack_nums.push(apply_op(a, b, stack_ops.pop().unwrap()));
                            }
                        }
                        stack_ops.push(c);
                    }
                }
            }

            if !num.is_empty() {
                if let Ok(n) = num.parse::<f64>() {
                    stack_nums.push(n);
                }
            }

            while !stack_ops.is_empty() && stack_nums.len() >= 2 {
                let b = stack_nums.pop().unwrap();
                let a = stack_nums.pop().unwrap();
                stack_nums.push(apply_op(a, b, stack_ops.pop().unwrap()));
            }

            stack_nums.first().unwrap_or(&0.0).to_string()
        }
        "random_number" => {
            let mut rng = rand::thread_rng();
            rng.gen_range(min..=max).to_string()
        }
        _ => input.to_string(),
    };

    Ok(json!({ "result": result }))
}

pub async fn process_text_task(
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

    let pattern = bundled_context
        .get("pattern")
        .and_then(|v| v.as_str())
        .unwrap_or("");

    let replacement = bundled_context
        .get("replacement")
        .and_then(|v| v.as_str())
        .unwrap_or("");

    let length = bundled_context
        .get("length")
        .and_then(|v| v.as_u64())
        .unwrap_or(0) as usize;

    let result = match operation {
        "capitalize" => input.chars().next().map_or(String::new(), |c| {
            let mut s = c.to_uppercase().collect::<String>();
            s.push_str(&input[1..]);
            s
        }),
        "lowercase" => input.to_lowercase(),
        "uppercase" => input.to_uppercase(),
        "trim" => input.trim().to_string(),
        "length" => input.len().to_string(),
        "word_count" => input.split_whitespace().count().to_string(),
        "extract_emails" => {
            let re = Regex::new(r"\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Z|a-z]{2,}\b").unwrap();
            re.find_iter(input)
                .map(|m| m.as_str().to_string())
                .collect::<Vec<String>>()
                .join(", ")
        }
        "extract_urls" => {
            let re = Regex::new(r"https?://[^\s]+").unwrap();
            re.find_iter(input)
                .map(|m| m.as_str().to_string())
                .collect::<Vec<String>>()
                .join(", ")
        }
        "url_encode" => urlencoding::encode(input).to_string(),
        "url_decode" => urlencoding::decode(input).unwrap_or_default().to_string(),
        "html_to_markdown" => parse_html(input),
        "markdown_to_html" => {
            let mut options = Options::empty();
            options.insert(Options::ENABLE_TABLES);
            let parser = Parser::new_ext(input, options);
            let mut html_output = String::new();
            html::push_html(&mut html_output, parser);
            html_output
        }
        "replace" => input.replace(pattern, replacement),
        "truncate" => {
            if input.len() <= length {
                input.to_string()
            } else {
                format!("{}...", &input[..length])
            }
        }
        "extract_pattern" => {
            if let Ok(re) = Regex::new(pattern) {
                re.find_iter(input)
                    .map(|m| m.as_str().to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            } else {
                "Invalid regex pattern".to_string()
            }
        }
        _ => input.to_string(),
    };

    Ok(json!({ "result": result }))
}
