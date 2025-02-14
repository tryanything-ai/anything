use serde_json::Value;

// This is meant to be used for function calls if we do agents and voice call type thing
// And to be how we do reusable flows or subflows
pub fn process_output_task(
    bundled_context: &Value,
) -> Result<Option<Value>, Box<dyn std::error::Error + Send + Sync>> {
    println!("[OUTPUT] Starting output task processing");
    println!("[OUTPUT] Bundled context: {:?}", bundled_context);

    // Deep parse JSON to handle common escape issues helpful for all the dirty json we have
    fn deep_parse_json(input: &str) -> Result<Value, serde_json::Error> {
        // Try multiple parsing strategies in order
        let attempts = [
            // 1. Try parsing directly first
            input.to_string(),
            // 2. If wrapped in quotes and contains escaped quotes, unescape everything
            if input.contains("\\\"") {
                input
                    .replace("\\\"", "\"")
                    .replace("\\n", "\n")
                    .replace("\\/", "/")
                    .replace("\\\\", "\\")
            } else {
                input.to_string()
            },
            // 3. If wrapped in quotes, remove them and unescape
            if input.starts_with('"') && input.ends_with('"') {
                let inner = &input[1..input.len() - 1];
                inner
                    .replace("\\\"", "\"")
                    .replace("\\n", "\n")
                    .replace("\\/", "/")
                    .replace("\\\\", "\\")
            } else {
                input.to_string()
            },
        ];

        // Try each cleaning strategy
        for (i, attempt) in attempts.iter().enumerate() {
            match serde_json::from_str(attempt) {
                Ok(mut parsed) => {
                    println!(
                        "[DEEP PARSE JSON IN OUTPUT] Successfully parsed JSON using strategy {}",
                        i + 1
                    );

                    // Recursively clean any string values that might be JSON
                    fn clean_recursive(value: &mut Value) {
                        match value {
                            Value::Object(map) => {
                                for (_, v) in map.iter_mut() {
                                    clean_recursive(v);
                                }
                            }
                            Value::Array(arr) => {
                                for v in arr.iter_mut() {
                                    clean_recursive(v);
                                }
                            }
                            Value::String(s) => {
                                // Only try to parse if it looks like JSON
                                if (s.starts_with('{') && s.ends_with('}'))
                                    || (s.starts_with('[') && s.ends_with(']'))
                                {
                                    if let Ok(parsed) = serde_json::from_str(s) {
                                        *value = parsed;
                                    }
                                }
                            }
                            _ => {}
                        }
                    }

                    clean_recursive(&mut parsed);
                    return Ok(parsed);
                }
                Err(_) => continue,
            }
        }

        // If all parsing attempts fail, return the original input as a string Value
        Ok(Value::String(input.to_string()))
    }

    // Get the body from the bundled context
    let body = bundled_context
        .get("json")
        .map(|v| v.to_string())
        .unwrap_or_else(|| "{}".to_string());

    println!("[OUTPUT] Body: {}", body);

    // Parse and return the body
    if !body.is_empty() {
        match deep_parse_json(&body) {
            Ok(parsed_body) => Ok(Some(parsed_body)),
            Err(e) => {
                println!("[OUTPUT] Failed to parse JSON body: {}", e);
                Ok(Some(Value::String(body)))
            }
        }
    } else {
        Ok(Some(Value::Object(serde_json::Map::new())))
    }
}
