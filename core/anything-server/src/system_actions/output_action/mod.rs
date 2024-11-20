use serde_json::Value;

pub async fn process_output_task(
    bundled_context: &Value,
) -> Result<Value, Box<dyn std::error::Error + Send + Sync>> {
    println!("[PROCESS OUTPUT] Starting process_output_task");
    println!(
        "[PROCESS OUTPUT] Initial bundled context: {:?}",
        bundled_context
    );

    // Get the output value from the bundled context
    if let Some(output) = bundled_context.get("output") {
        println!("[PROCESS OUTPUT] Found output value: {:?}", output);

        // Convert output to string if needed
        let output_str = match output {
            Value::String(s) => s.to_string(),
            _ => output.to_string(),
        };

        println!("[PROCESS OUTPUT] Output string: {}", output_str);

        // Helper function to recursively clean and parse JSON strings
        fn deep_parse_json(input: &str) -> Result<Value, serde_json::Error> {
            // First try parsing directly
            match serde_json::from_str(input) {
                Ok(mut parsed) => {
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
                                // Try to parse string as JSON
                                if let Ok(parsed) = deep_parse_json(s) {
                                    *value = parsed;
                                }
                            }
                            _ => {}
                        }
                    }
                    clean_recursive(&mut parsed);
                    Ok(parsed)
                }
                Err(_) => {
                    // If direct parsing fails, try cleaning the string
                    let cleaned = input
                        .replace("\\n", "\n")
                        .replace("\\\"", "\"")
                        .replace("\\/", "/");
                    
                    // Try parsing cleaned string
                    match serde_json::from_str(&cleaned) {
                        Ok(mut parsed) => {
                            // Apply same recursive cleaning to cleaned parse result
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
                                        if let Ok(parsed) = deep_parse_json(s) {
                                            *value = parsed;
                                        }
                                    }
                                    _ => {}
                                }
                            }
                            clean_recursive(&mut parsed);
                            Ok(parsed)
                        }
                        Err(e) => Err(e)
                    }
                }
            }
        }

        // Try deep parsing the output string
        match deep_parse_json(&output_str) {
            Ok(parsed) => {
                println!("[PROCESS OUTPUT] Successfully parsed and cleaned: {:?}", parsed);
                Ok(parsed)
            }
            Err(e) => {
                println!("[PROCESS OUTPUT] Failed to parse: {}. Returning original output.", e);
                Ok(output.clone())
            }
        }
    } else {
        println!("[PROCESS OUTPUT] No output value found in bundled context");
        Err("Output task requires an 'output' field in the context".into())
    }
}
