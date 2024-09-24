use regex::Regex;
use serde_json::Value;
use std::collections::HashSet;

// Define the regex patterns for API keys
fn api_key_patterns() -> Vec<Regex> {
    vec![
        Regex::new(r"[A-Za-z0-9_\-]{32}").unwrap(), // Generic 32-character API key
        Regex::new(r"[A-Za-z0-9_\-]{40}").unwrap(), // Generic 40-character API key
        Regex::new(r"[A-Za-z0-9_\-]{64}").unwrap(), // Generic 64-character API key
        Regex::new(r"AIza[0-9A-Za-z-_]{35}").unwrap(), // Google API key pattern
        Regex::new(r"[A-Za-z0-9]{20}\.[A-Za-z0-9]{20}\.[A-Za-z0-9]{30}").unwrap(), // JWT token
    ]
}

// Sensitive field keywords to look for
fn sensitive_field_keywords() -> HashSet<&'static str> {
    let keywords: HashSet<&str> = vec![
        "api_key",
        "apikey",
        "access_token",
        "auth_token",
        "token",
        "secret",
        "client_id",
        "client_secret",
    ]
    .into_iter()
    .collect();
    keywords
}

// Function to scan the JSON document for naughty keys
fn scan_json_for_naughty_keys(json_value: &Value, path: String, errors: &mut Vec<String>) {
    match json_value {
        Value::Object(map) => {
            for (key, value) in map {
                let new_path = if path.is_empty() {
                    key.clone()
                } else {
                    format!("{}.{}", path, key)
                };

                // Check if the key matches any sensitive keywords
                if sensitive_field_keywords().contains(key.as_str()) {
                    // If the value is a string, check if it matches any API key patterns
                    if let Value::String(s) = value {
                        for pattern in api_key_patterns() {
                            if pattern.is_match(s) {
                                errors.push(format!(
                                    "Naughty key found: '{}' with value '{}'",
                                    new_path, s
                                ));
                            }
                        }
                    }
                }

                // Recurse into nested objects
                scan_json_for_naughty_keys(value, new_path, errors);
            }
        }
        Value::Array(array) => {
            for (index, item) in array.iter().enumerate() {
                let new_path = format!("{}[{}]", path, index);
                scan_json_for_naughty_keys(item, new_path, errors);
            }
        }
        _ => {} // Ignore other types (numbers, booleans, null)
    }
}

// Main function to call for scanning
pub fn is_workflow_data_safe_to_share(json_value: &Value) -> (bool, Option<Vec<String>>) {
    let mut errors = Vec::new();
    scan_json_for_naughty_keys(json_value, "".to_string(), &mut errors);

    if errors.is_empty() {
        (true, None)
    } else {
        (false, Some(errors))
    }
}
