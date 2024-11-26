use html2md::parse_html;
use pulldown_cmark::{html, Options, Parser};
use serde_json::{json, Value};

pub fn process_text_task(
    bundled_context: &Value,
) -> Result<Option<Value>, Box<dyn std::error::Error + Send + Sync>> {
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

    let start_index = bundled_context
        .get("start_index")
        .and_then(|v| match v {
            Value::String(s) => s.parse::<i64>().ok(),
            Value::Number(n) => n.as_i64(),
            _ => None,
        })
        .unwrap_or(0) as i32;

    let end_index = bundled_context.get("end_index").and_then(|v| match v {
        Value::String(s) => s.parse::<i64>().ok(),
        Value::Number(n) => n.as_i64(),
        _ => None,
    });

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
        //TODO: make into a dif action also when we do loops i would say
        // "extract_emails" => {
        //     let re = Regex::new(r"\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Z|a-z]{2,}\b").unwrap();
        //     re.find_iter(input)
        //         .map(|m| m.as_str().to_string())
        //         .collect::<Vec<String>>()
        //         .join(", ")
        // }
        // "extract_urls" => {
        //     let re = Regex::new(r"https?://[^\s]+").unwrap();
        //     re.find_iter(input)
        //         .map(|m| m.as_str().to_string())
        //         .collect::<Vec<String>>()
        //         .join(", ")
        // }
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
        "substring" => {
            let len = input.chars().count() as i32;
            // Convert negative indices to positive (Python-like behavior)
            let normalized_start = if start_index < 0 {
                (len + start_index).max(0)
            } else {
                start_index.min(len)
            } as usize;

            let normalized_end = match end_index {
                Some(end) => {
                    let end = end as i32;
                    if end < 0 {
                        (len + end).max(0)
                    } else {
                        end.min(len)
                    }
                }
                None => len,
            } as usize;

            if normalized_start >= normalized_end {
                String::new()
            } else {
                input
                    .chars()
                    .skip(normalized_start)
                    .take(normalized_end - normalized_start)
                    .collect()
            }
        }

        // "extract_pattern" => {
        //     if let Ok(re) = Regex::new(pattern) {
        //         re.find_iter(input)
        //             .map(|m| m.as_str().to_string())
        //             .collect::<Vec<String>>()
        //             .join(", ")
        //     } else {
        //         "Invalid regex pattern".to_string()
        //     }
        // }
        _ => input.to_string(),
    };

    Ok(Some(json!({ "formatted_text": result })))
}
