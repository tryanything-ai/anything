use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;

use crate::types::json_schema::ValidationFieldType;

#[derive(Debug)]
pub struct TemplateError {
    pub message: String,
    pub variable: String,
}

impl std::fmt::Display for TemplateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Template error for variable '{}': {}",
            self.variable, self.message
        )
    }
}

impl Error for TemplateError {}

pub struct Templater {
    templates: HashMap<String, Value>,
}

impl Templater {
    pub fn new() -> Self {
        Templater {
            templates: HashMap::new(),
        }
    }

    pub fn add_template(&mut self, name: &str, template: Value) {
        self.templates.insert(name.to_string(), template);
    }

    pub fn get_template_variables(
        &self,
        template_name: &str,
    ) -> Result<Vec<String>, TemplateError> {
        let template = self
            .templates
            .get(template_name)
            .ok_or_else(|| TemplateError {
                message: "Template not found".to_string(),
                variable: template_name.to_string(),
            })?;

        self.extract_variables(template)
    }

    fn extract_variables(&self, value: &Value) -> Result<Vec<String>, TemplateError> {
        let mut variables = Vec::new();
        match value {
            Value::Object(map) => {
                for (_, v) in map {
                    variables.extend(self.extract_variables(v)?);
                }
            }
            Value::Array(arr) => {
                for v in arr {
                    variables.extend(self.extract_variables(v)?);
                }
            }
            Value::String(s) => {
                let mut start = 0;
                while let Some(open_idx) = s[start..].find("{{") {
                    let open_idx = start + open_idx;
                    let close_idx = s[open_idx..].find("}}").ok_or_else(|| TemplateError {
                        message: "Unclosed template variable".to_string(),
                        variable: s.to_string(),
                    })?;
                    let close_idx = open_idx + close_idx;
                    let variable = s[open_idx + 2..close_idx].trim().to_string();
                    variables.push(variable);
                    start = close_idx + 2;
                }
            }
            _ => {}
        }
        Ok(variables)
    }

    fn get_value_from_path(
        context: &Value,
        path: &str,
        expected_type: &ValidationFieldType,
    ) -> Option<Value> {
        let mut current = context;
        let parts: Vec<&str> = path.split('.').collect();

        for (i, part) in parts.iter().enumerate() {
            if let Some(index_start) = part.find('[') {
                let key = &part[..index_start];
                let index_end = part.find(']').unwrap_or(part.len());
                let index: usize = part[index_start + 1..index_end].parse().ok()?;

                current = current.get(key)?;
                if current.is_array() {
                    current = current.get(index)?;
                } else {
                    return None; // Not an array when we expected one
                }
            } else {
                current = current.get(part)?;
            }

            if let Value::String(s) = current {
                // Only parse JSON if the expected type is not String
                if *expected_type != ValidationFieldType::String {
                    if let Ok(parsed) = serde_json::from_str(s) {
                        if i < parts.len() - 1 {
                            // If not the last part, continue traversing
                            return Self::get_value_from_path(
                                &parsed,
                                &parts[i + 1..].join("."),
                                expected_type,
                            );
                        } else {
                            // If it's the last part, return the parsed value
                            return Some(parsed);
                        }
                    }
                }
            }
        }
        Some(current.clone())
    }

    pub fn render(
        &self,
        template_name: &str,
        context: &Value,
        validations: HashMap<String, ValidationFieldType>,
    ) -> Result<Value, TemplateError> {
        let template = self
            .templates
            .get(template_name)
            .ok_or_else(|| TemplateError {
                message: "Template not found".to_string(),
                variable: template_name.to_string(),
            })?;

        println!("[TEMPLATER] Template: {:?}", template);
        println!("[TEMPLATER] Context: {:?}", context);
        println!("[TEMPLATER] Validations: {:?}", validations);

        self.render_value(template, context, &validations, &[])
    }

    fn render_value(
        &self,
        value: &Value,
        context: &Value,
        validations: &HashMap<String, ValidationFieldType>,
        path: &[String],
    ) -> Result<Value, TemplateError> {
        match value {
            Value::Object(map) => {
                let mut result = serde_json::Map::new();
                for (k, v) in map {
                    let mut current_path = path.to_vec();
                    current_path.push(k.clone());
                    if path.is_empty() {
                        let expected_validation_type = Self::get_validation_type(validations, &k)?;
                        let rendered = self.render_value(v, context, validations, &current_path)?;
                        let validated =
                            self.validate_and_convert_value(rendered, expected_validation_type, k)?;
                        result.insert(k.clone(), validated);
                    } else {
                        result.insert(
                            k.clone(),
                            self.render_value(v, context, validations, &current_path)?,
                        );
                    }
                }
                Ok(Value::Object(result))
            }
            Value::Array(arr) => {
                let mut result = Vec::new();
                for v in arr.iter() {
                    result.push(self.render_value(v, context, validations, path)?);
                }
                Ok(Value::Array(result))
            }
            Value::String(s) => {
                let trimmed = s.trim();
                if trimmed.starts_with("{{") && trimmed.ends_with("}}") {
                    let variable = trimmed[2..trimmed.len() - 2].trim();
                    // Only validate if this is a top-level path
                    if path.is_empty() {
                        let validation_key = variable.to_string();
                        let expected_type =
                            Self::get_validation_type(validations, &validation_key)?;
                        let value = Self::get_value_from_path(context, variable, expected_type)
                            .ok_or_else(|| TemplateError {
                                message: format!("Variable not found in context: {}", variable),
                                variable: variable.to_string(),
                            })?;
                        let value =
                            self.validate_and_convert_value(value, expected_type, &validation_key)?;
                        return Ok(value);
                    } else {
                        // For nested variables, just get the value without validation
                        let value = Self::get_value_from_path(
                            context,
                            variable,
                            &ValidationFieldType::Unknown,
                        )
                        .ok_or_else(|| TemplateError {
                            message: format!("Variable not found in context: {}", variable),
                            variable: variable.to_string(),
                        })?;
                        return Ok(value);
                    }
                }

                // Regular string interpolation logic
                let mut result = s.clone();
                let mut start = 0;

                while let Some(open_idx) = result[start..].find("{{") {
                    let open_idx = start + open_idx;
                    let close_idx = result[open_idx..].find("}}").ok_or_else(|| TemplateError {
                        message: "Unclosed template variable".to_string(),
                        variable: result.clone(),
                    })?;
                    let close_idx = open_idx + close_idx;
                    let variable = result[open_idx + 2..close_idx].trim();

                    // Only validate if this is a top-level path
                    let value = if path.is_empty() {
                        let validation_key = variable.to_string();
                        let expected_type =
                            Self::get_validation_type(validations, &validation_key)?;
                        let value = Self::get_value_from_path(context, variable, expected_type)
                            .ok_or_else(|| TemplateError {
                                message: "Variable not found in context".to_string(),
                                variable: variable.to_string(),
                            })?;
                        self.validate_and_convert_value(value, expected_type, &validation_key)?
                    } else {
                        // For nested variables, just get the value without validation
                        Self::get_value_from_path(context, variable, &ValidationFieldType::Unknown)
                            .ok_or_else(|| TemplateError {
                                message: "Variable not found in context".to_string(),
                                variable: variable.to_string(),
                            })?
                    };

                    let replacement = match value {
                        Value::String(s) => {
                            if s.contains('<') && s.contains('>') || s.contains('\n') {
                                // Escape quotes and newlines manually, without adding extra quotes
                                s.replace('\\', "\\\\")
                                    .replace('"', "\\\"")
                                    .replace('\n', "\\n")
                            } else {
                                s.clone()
                            }
                        }
                        _ => value.to_string(),
                    };
                    result.replace_range(open_idx..close_idx + 2, &replacement);
                    start = open_idx + replacement.len();
                }

                Ok(Value::String(result))
            }
            _ => Ok(value.clone()),
        }
    }

    fn get_validation_type<'a>(
        validations: &'a HashMap<String, ValidationFieldType>,
        key: &str,
    ) -> Result<&'a ValidationFieldType, TemplateError> {
        validations.get(key).ok_or_else(|| TemplateError {
            message: format!("Validation not found for key '{}'", key),
            variable: key.to_string(),
        })
    }

    fn validate_and_convert_value(
        &self,
        value: Value,
        expected_type: &ValidationFieldType,
        variable: &str,
    ) -> Result<Value, TemplateError> {
        match expected_type {
            ValidationFieldType::String => match value {
                Value::String(_) => Ok(value),
                _ => Ok(Value::String(value.to_string())),
            },
            ValidationFieldType::Number => match value {
                Value::Number(_) => Ok(value),
                Value::String(s) => s.parse::<f64>().map_or_else(
                    |_| {
                        Err(TemplateError {
                            message: format!("Cannot convert value to number: {}", s),
                            variable: variable.to_string(),
                        })
                    },
                    |n| Ok(Value::Number(serde_json::Number::from_f64(n).unwrap())),
                ),
                _ => Err(TemplateError {
                    message: format!("Expected number, got: {:?}", value),
                    variable: variable.to_string(),
                }),
            },
            ValidationFieldType::Boolean => match value {
                Value::Bool(_) => Ok(value),
                Value::String(s) => s.parse::<bool>().map_or_else(
                    |_| {
                        Err(TemplateError {
                            message: format!("Cannot convert value to boolean: {}", s),
                            variable: variable.to_string(),
                        })
                    },
                    |b| Ok(Value::Bool(b)),
                ),
                _ => Err(TemplateError {
                    message: format!("Expected boolean, got: {:?}", value),
                    variable: variable.to_string(),
                }),
            },
            ValidationFieldType::Object => match value {
                Value::Object(_) => Ok(value),
                Value::String(s) => {
                    println!("Attempting to parse as object. String length: {}", s.len());
                    println!("First 100 chars: {}", &s[..s.len().min(100)]);
                    // Try to parse string as JSON object
                    match serde_json::from_str(&s) {
                        Ok(parsed) => match parsed {
                            Value::Object(_) => {
                                println!("Successfully parsed as object");
                                Ok(parsed)
                            }
                            _ => {
                                println!("Parsed but not an object");
                                Err(TemplateError {
                                    message: format!("String parsed but not an object: {}", s),
                                    variable: variable.to_string(),
                                })
                            }
                        },
                        Err(e) => {
                            println!("Parse error: {}", e);
                            Err(TemplateError {
                                message: format!("Cannot parse string as object: {}", s),
                                variable: variable.to_string(),
                            })
                        }
                    }
                }
                _ => Err(TemplateError {
                    message: format!("Expected object, got: {:?}", value),
                    variable: variable.to_string(),
                }),
            },
            ValidationFieldType::Array => match value {
                Value::Array(_) => Ok(value),
                Value::String(s) => {
                    // Try to parse string as JSON array
                    match serde_json::from_str(&s) {
                        Ok(parsed) => match parsed {
                            Value::Array(_) => Ok(parsed),
                            _ => Err(TemplateError {
                                message: format!("String parsed but not an array: {}", s),
                                variable: variable.to_string(),
                            }),
                        },
                        Err(_) => Err(TemplateError {
                            message: format!("Cannot parse string as array: {}", s),
                            variable: variable.to_string(),
                        }),
                    }
                }
                _ => Err(TemplateError {
                    message: format!("Expected array, got: {:?}", value),
                    variable: variable.to_string(),
                }),
            },
            ValidationFieldType::Null => Ok(Value::Null),
            ValidationFieldType::Unknown => Ok(value),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn string_variable_replacement() {
        let mut templater = Templater::new();

        let template = json!({
            "greeting": "Hello {{variables.name}}"
        });

        templater.add_template("test_template", template);

        let context = json!({
            "variables": {
                "name": "World"
            }
        });

        let mut validations = HashMap::new();
        validations.insert("greeting".to_string(), ValidationFieldType::String);

        let result = templater
            .render("test_template", &context, validations)
            .unwrap();

        assert_eq!(
            result,
            json!({
                "greeting": "Hello World"
            })
        );
    }

    #[test]
    fn string_variable_replacement_with_type_coercion_from_number() {
        let mut templater = Templater::new();

        let template = json!({
            "greeting": "{{variables.name}}"
        });

        templater.add_template("test_template", template);

        let context = json!({
            "variables": {
                "name": 42  // Providing a number instead of string
            }
        });

        let mut validations = HashMap::new();
        validations.insert("greeting".to_string(), ValidationFieldType::String);

        let result = templater
            .render("test_template", &context, validations)
            .unwrap();

        assert_eq!(
            result,
            json!({
                "greeting": "42"
            })
        );
    }
    #[test]
    fn number_variable_replacement_with_type_coercion_from_string() {
        let mut templater = Templater::new();

        let template = json!({
            "greeting": "{{variables.name}}"
        });

        templater.add_template("test_template", template);

        let context = json!({
            "variables": {
                "name": "42"
            }
        });

        let mut validations = HashMap::new();
        validations.insert("greeting".to_string(), ValidationFieldType::Number);

        let result = templater
            .render("test_template", &context, validations)
            .unwrap();

        assert_eq!(
            result,
            json!({
                "greeting": 42,
            })
        );
    }

    #[test]
    fn object_variable_replacement() {
        let mut templater = Templater::new();

        let template = json!({
            "an_object": "{{variables.the_object}}"
        });

        templater.add_template("test_template", template);

        let context = json!({
            "variables": {
                "the_object": {
                    "a_number": 42
                }
            }
        });

        let mut validations = HashMap::new();
        validations.insert("an_object".to_string(), ValidationFieldType::Object);

        let result = templater
            .render("test_template", &context, validations)
            .unwrap();

        assert_eq!(
            result,
            json!({
                "an_object": {
                    "a_number": 42,
                }
            })
        );
    }

    #[test]
    fn complicated_replacement() {
        let mut templater = Templater::new();

        let template = json!({
            "an_object": "{{variables.the_object}}",
            "a_number": "{{variables.a_number_var}}",
            "a_string": "{{variables.a_string_var}}",
            "a_boolean": "{{variables.a_boolean_var}}",
            "an_array": "{{variables.an_array_var}}",
            "a_null": "{{variables.a_null_var}}",
            "a_float": "{{variables.a_float_var}}",
            "a_number_string": "{{variables.a_number_string_var}}",
            "a_boolean_string": "{{variables.a_boolean_string_var}}",
            "a_array_string": "{{variables.a_array_string_var}}",
        });

        templater.add_template("test_template", template);

        let context = json!({
            "variables": {
                "the_object": {
                    "a_number": 42
                },
                "a_number_var": 43,
                "a_string_var": "hello",
                "a_boolean_var": true,
                "an_array_var": [1, 2, 3],
                "a_null_var": null,
                "a_float_var": 1.23,
                "a_number_string_var": "44",
                "a_boolean_string_var": "true",
                "a_array_string_var": "[1, 2, 3]",
            }
        });

        let mut template_key_validations = HashMap::new();
        template_key_validations.insert("an_object".to_string(), ValidationFieldType::Object);
        template_key_validations.insert("a_number".to_string(), ValidationFieldType::Number);
        template_key_validations.insert("a_string".to_string(), ValidationFieldType::String);
        template_key_validations.insert("a_boolean".to_string(), ValidationFieldType::Boolean);
        template_key_validations.insert("an_array".to_string(), ValidationFieldType::Array);
        template_key_validations.insert("a_null".to_string(), ValidationFieldType::Null);
        template_key_validations.insert("a_float".to_string(), ValidationFieldType::Number);
        template_key_validations.insert("a_number_string".to_string(), ValidationFieldType::String);
        template_key_validations
            .insert("a_boolean_string".to_string(), ValidationFieldType::String);
        template_key_validations.insert("a_array_string".to_string(), ValidationFieldType::String);

        let result = templater
            .render("test_template", &context, template_key_validations)
            .unwrap();

        assert_eq!(
            result,
            json!({
                "an_object": {
                    "a_number": 42,
                },
                "a_number": 43,
                "a_string": "hello",
                "a_boolean": true,
                "an_array": [1, 2, 3],
                "a_null": null,
                "a_float": 1.23,
                "a_number_string": "44",
                "a_boolean_string": "true",
                "a_array_string": "[1,2,3]", //DANGER ZONE. Be careful with this. It's a string. and serde makes the spaces go away
            })
        );
    }

    #[test]
    fn test_deep_array_path() {
        let mut templater = Templater::new();
        templater.add_template(
            "test_template",
            json!({
                "result": "{{variables.data.items[0].subitems[1].value}}",
                "obj_results": "{{variables.data.items[0]}}",
                "obj_as_string": "{{variables.data.items[0]}}"
            }),
        );

        let context = json!({
            "variables": {
                "data": {
                    "items": [
                    {
                        "subitems": [
                            {"value": "first"},
                            {"value": "second"},
                            {"value": 42}
                        ]
                    },
                    {
                        "subitems": [
                            {"value": "other"}
                        ]
                    }
                ]
            }
        }});

        let mut template_key_validations = HashMap::new();
        template_key_validations.insert("result".to_string(), ValidationFieldType::String);
        template_key_validations.insert("obj_results".to_string(), ValidationFieldType::Object);
        template_key_validations.insert("obj_as_string".to_string(), ValidationFieldType::String);
        let result = templater
            .render("test_template", &context, template_key_validations)
            .unwrap();

        assert_eq!(
            result,
            json!({
                "result": "second",
                "obj_results": {
                    "subitems": [
                        {"value": "first"},
                        {"value": "second"},
                        {"value": 42}
                    ]
                },
                "obj_as_string": "{\"subitems\":[{\"value\":\"first\"},{\"value\":\"second\"},{\"value\":42}]}"
            })
        );
    }

    #[test]
    fn test_string_interpolation() {
        let mut templater = Templater::new();
        templater.add_template(
            "test_template",
            json!({
                "message": "Hello {{variables.user.name}}, your score is {{variables.user.score}}!",
                "description": "User {{variables.user.id}} ({{variables.user.name}}) joined on {{variables.user.date}}"
            }),
        );

        let context = json!({
            "variables": {
                "user": {
                    "name": "Alice",
                    "score": 95,
                    "id": "usr_123",
                    "date": "2023-10-15"
                }
            }
        });

        let mut template_key_validations = HashMap::new();
        template_key_validations.insert("message".to_string(), ValidationFieldType::String);
        template_key_validations.insert("description".to_string(), ValidationFieldType::String);

        let result = templater
            .render("test_template", &context, template_key_validations)
            .unwrap();

        assert_eq!(
            result,
            json!({
                "message": "Hello Alice, your score is 95!",
                "description": "User usr_123 (Alice) joined on 2023-10-15"
            })
        );
    }

    #[test]
    fn test_object_key_interpolation() {
        let mut templater = Templater::new();
        templater.add_template(
            "test_template",
            json!({
                "user": {
                    "name": "{{variables.user.name}}",
                    "role": "{{variables.user.role}}",
                    "active": true,
                    "data": "{{variables.user}}"
                }
            }),
        );

        let context = json!({
            "variables": {
                "user": {
                    "id": "usr_456",
                    "name": "Bob",
                    "role": "admin"
                }
            }
        });

        let mut template_key_validations = HashMap::new();
        template_key_validations.insert("user".to_string(), ValidationFieldType::Object);

        let result = templater
            .render("test_template", &context, template_key_validations)
            .unwrap();

        assert_eq!(
            result,
            json!({
                "user": {
                    "name": "Bob",
                    "role": "admin",
                    "active": true,
                    "data": {
                        "id": "usr_456",
                        "name": "Bob",
                        "role": "admin"
                    }
                }
            })
        );
    }

    #[test]
    fn test_string_json_conversion() {
        let mut templater = Templater::new();
        templater.add_template(
            "test_template",
            json!({
                "object_field": "{{variables.string_object}}",
                "array_field": "{{variables.string_array}}"
            }),
        );

        let context = json!({
            "variables": {
                "string_object": "{}",
                "string_array": "[]"
            }
        });

        let mut validations = HashMap::new();
        validations.insert("object_field".to_string(), ValidationFieldType::Object);
        validations.insert("array_field".to_string(), ValidationFieldType::Array);

        let result = templater
            .render("test_template", &context, validations)
            .unwrap();

        assert_eq!(
            result,
            json!({
                "object_field": {},
                "array_field": []
            })
        );
    }

    #[test]
    fn test_email_with_html_body() {
        let mut templater = Templater::new();

        // This represents your template string with the email structure
        let template = json!({
            "body": {
                "drafts": {
                    "subject": "{{inputs.subject}}",
                    "body": "{{inputs.body}}",
                    "to_fields": [{
                        "address": "{{inputs.to_address}}"
                    }],
                    "from_field": {
                        "name": "{{inputs.from_name}}",
                        "address": "{{inputs.from_address}}"
                    },
                    "attachments": [{
                        "base64_data": "{{inputs.attachment_data}}",
                        "filename": "{{inputs.attachment_name}}"
                    }]

                }
            }
        });

        templater.add_template("email_template", template);

        // This represents your input variables
        let context = json!({
            "inputs": {
                "subject": "Test Email",
                "body": "<!DOCTYPE html><html><body><p>Hello!</p><p>This is a <strong>test</strong> email with <em>HTML</em> content.</p><p>Best regards,<br/>Test Sender</p></body></html>",
                "to_address": "recipient@example.com",
                "from_name": "Test Sender",
                "from_address": "sender@example.com",
                "attachment_data": "base64_encoded_string",
                "attachment_name": "test.pdf"
            }
        });

        let mut validations = HashMap::new();
        validations.insert("body".to_string(), ValidationFieldType::Object);

        let result = templater
            .render("email_template", &context, validations)
            .unwrap();

        // Verify the result matches what we expect
        assert_eq!(
            result,
            json!({
                    "body": {
                        "drafts": {
                            "subject": "Test Email",
                            "body": "<!DOCTYPE html><html><body><p>Hello!</p><p>This is a <strong>test</strong> email with <em>HTML</em> content.</p><p>Best regards,<br/>Test Sender</p></body></html>",
                            "to_fields": [{
                            "address": "recipient@example.com"
                        }],
                        "from_field": {
                            "name": "Test Sender",
                            "address": "sender@example.com"
                        },
                        "attachments": [{
                            "base64_data": "base64_encoded_string",
                            "filename": "test.pdf"
                        }]
                    }
                }
            })
        );
    }

    #[test]
    fn test_missive_draft_bundling() {
        let mut templater = Templater::new();

        // The plugin config that needs to be rendered
        let template = json!({
            "url": "https://public.missiveapp.com/v1/drafts",
            "method": "POST",
            "headers": "{\n  \"Authorization\": \"Bearer {{inputs.MISSIVE_API_KEY}}\",\n  \"Content-Type\": \"application/json\"\n}",
            "body": "{\n  \"drafts\": {\n    \"subject\": \"{{inputs.subject}}\",\n    \"body\": \"{{inputs.body}}\",\n    \"to_fields\": [\n      {\n        \"address\": \"{{inputs.to_address}}\"\n      }\n    ],\n    \"from_field\": {\n      \"name\": \"{{inputs.from_name}}\",\n      \"address\": \"{{inputs.from_address}}\"\n    },\n    \"attachments\": [\n      {\n        \"base64_data\": \"{{inputs.attachement_as_base64}}\",\n        \"filename\": \"FAKE New Bars Retail Dsplay.png\"\n      }\n    ]\n  }\n}"
        });

        templater.add_template("missive_draft", template);

        // The rendered inputs that would come from bundle_cached_inputs
        let context = json!({
            "inputs": {
                "subject": "[DO NOT SEND!] Test Draft from Carl @ Anything AI",
                "body": "<!DOCTYPE html>\\n<html>\\n<body>\\n    <p>Hi Test Person!,</p>\\n\\n    <p>I hope you're well!</p>\\n\\n    <p>\\n        I just wanted to confirm you received your Organic Protein Bars, and see if you have any initial feedback or questions.\\n    </p>\\n\\n    <p>\\n        <strong>To support sales & marketing, most of our clients like to use our retail displays (below)</strong>, \\n        as well as our collection of \\n        <a href=\\\"https://drive.google.com/drive/folders/13l0f67j1SsXaw59y3XHQkGm1-cx15IWb\\\" target=\\\"_blank\\\">shelf talkers</a> \\n        that fit well into \\n        <a href=\\\"https://www.amazon.com/dp/B0BWDJXY48?smid=A128CO7G80NO6D&ref_=chk_typ_imgToDp&th=1\\\" target=\\\"_blank\\\">these frames</a>.\\n    </p>\\n\\n    <p>\\n        Please let us know if you're interested, or we'd be happy to can design something custom for you.\\n    </p>\\n\\n    <p>Thanks for your support!</p>\\n\\n    <p>Warmly,<br>Person</p>\\n</body>\\n</html>\\n",
                "to_address": "carl@gmail.com",
                "from_address": "person@company.com",
                "from_name": "Person",
                "attachement_as_base64": "asfasdfasf",
                "MISSIVE_API_KEY": "test_api_key"
            }
        });

        // Set up validations for the template fields
        let mut validations = HashMap::new();
        validations.insert("url".to_string(), ValidationFieldType::String);
        validations.insert("method".to_string(), ValidationFieldType::String);
        validations.insert("headers".to_string(), ValidationFieldType::Object);
        validations.insert("body".to_string(), ValidationFieldType::Object);

        // Render the template
        let result = templater
            .render("missive_draft", &context, validations)
            .unwrap();

        // Expected results after rendering
        let expected_body = json!({
            "drafts": {
                "subject": "[DO NOT SEND!] Test Draft from Carl @ Anything AI",
                "body": "<!DOCTYPE html>\\n<html>\\n<body>\\n    <p>Hi Test Person!,</p>\\n\\n    <p>I hope you're well!</p>\\n\\n    <p>\\n        I just wanted to confirm you received your Organic Protein Bars, and see if you have any initial feedback or questions.\\n    </p>\\n\\n    <p>\\n        <strong>To support sales & marketing, most of our clients like to use our retail displays (below)</strong>, \\n        as well as our collection of \\n        <a href=\\\"https://drive.google.com/drive/folders/13l0f67j1SsXaw59y3XHQkGm1-cx15IWb\\\" target=\\\"_blank\\\">shelf talkers</a> \\n        that fit well into \\n        <a href=\\\"https://www.amazon.com/dp/B0BWDJXY48?smid=A128CO7G80NO6D&ref_=chk_typ_imgToDp&th=1\\\" target=\\\"_blank\\\">these frames</a>.\\n    </p>\\n\\n    <p>\\n        Please let us know if you're interested, or we'd be happy to can design something custom for you.\\n    </p>\\n\\n    <p>Thanks for your support!</p>\\n\\n    <p>Warmly,<br>Person</p>\\n</body>\\n</html>\\n",
                "to_fields": [
                    {
                        "address": "carl@gmail.com"
                    }
                ],
                "from_field": {
                    "name": "Person",
                    "address": "person@company.com"
                },
                "attachments": [
                    {
                        "base64_data": "asfasdfasf",
                        "filename": "FAKE New Bars Retail Dsplay.png"
                    }
                ]
            }
        });

        let expected_headers = json!({
            "Authorization": "Bearer test_api_key",
            "Content-Type": "application/json"
        });

        assert_eq!(result["body"], expected_body);
        assert_eq!(result["headers"], expected_headers);

        // Verify each part of the result
        assert_eq!(result["url"], "https://public.missiveapp.com/v1/drafts");
        assert_eq!(result["method"], "POST");
    }

    #[test]
    fn test_automation_data_types() {
        let mut templater = Templater::new();

        // Template covering various automation system data types
        let template = json!({
            "xml_content": "{{inputs.xml_data}}",
            "sql_query": "{{inputs.sql_query}}",
            "csv_data": "{{inputs.csv_data}}",
            "base64_file": "{{inputs.file_content}}",
            "markdown": "{{inputs.markdown}}",
            "yaml_config": "{{inputs.yaml_config}}",
            "shell_script": "{{inputs.shell_script}}",
            "api_response": "{{inputs.api_json}}"
        });

        templater.add_template("automation_test", template);

        // Context with matching keys
        let context = json!({
            "inputs": {
                "xml_data": "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<root>\n  <customer id=\"123\">\n    <name>John Doe</name>\n    <email>john@example.com</email>\n    <orders>\n      <order id=\"A1\">\n        <item>Product 1</item>\n        <quantity>2</quantity>\n      </order>\n    </orders>\n  </customer>\n</root>",

                "sql_query": "SELECT u.name, u.email, COUNT(o.id) as order_count\nFROM users u\nLEFT JOIN orders o ON u.id = o.user_id\nWHERE u.status = 'active'\nGROUP BY u.id\nHAVING COUNT(o.id) > 5;",

                "csv_data": "id,name,email,status\n1,John Doe,john@example.com,active\n2,Jane Smith,jane@example.com,pending\n3,Bob Wilson,bob@example.com,inactive",

                "file_content": "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAQAAAC1HAwCAAAAC0lEQVR42mNk+A8AAQUBAScY42YAAAAASUVORK5CYII=",

                "markdown": "# Title\n\n## Subtitle\n\nThis is a *markdown* document with:\n- Lists\n- **Bold** text\n- [Links](https://example.com)\n\n```code\nsome code block\n```",

                "yaml_config": "version: '3'\nservices:\n  web:\n    image: nginx:latest\n    ports:\n      - \"80:80\"\n    volumes:\n      - ./html:/usr/share/nginx/html",

                "shell_script": "#!/bin/bash\n\necho \"Starting backup process...\"\nBACKUP_DIR=\"/var/backups/$(date +%Y-%m-%d)\"\n\nif [ ! -d \"$BACKUP_DIR\" ]; then\n  mkdir -p \"$BACKUP_DIR\"\nfi\n\ntar -czf \"$BACKUP_DIR/backup.tar.gz\" /var/www/\necho \"Backup completed\"",

                "api_json": "{\n  \"status\": \"success\",\n  \"data\": {\n    \"users\": [\n      {\n        \"id\": 1,\n        \"name\": \"John Doe\"\n      }\n    ]\n  }\n}"
            }
        });

        // Set up validations - all strings since we want to preserve formatting
        let mut validations = HashMap::new();
        validations.insert("xml_content".to_string(), ValidationFieldType::String);
        validations.insert("sql_query".to_string(), ValidationFieldType::String);
        validations.insert("csv_data".to_string(), ValidationFieldType::String);
        validations.insert("base64_file".to_string(), ValidationFieldType::String);
        validations.insert("markdown".to_string(), ValidationFieldType::String);
        validations.insert("yaml_config".to_string(), ValidationFieldType::String);
        validations.insert("shell_script".to_string(), ValidationFieldType::String);
        validations.insert("api_response".to_string(), ValidationFieldType::Object);

        let result = templater
            .render("automation_test", &context, validations)
            .unwrap();

        // Verify each type of content is handled correctly
        assert!(result["xml_content"].as_str().unwrap().starts_with("<?xml"));
        assert!(result["sql_query"].as_str().unwrap().contains("SELECT"));
        assert!(result["csv_data"]
            .as_str()
            .unwrap()
            .contains("id,name,email"));
        assert!(result["base64_file"]
            .as_str()
            .unwrap()
            .contains("iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAQAAAC1"));
        assert!(result["markdown"].as_str().unwrap().contains("# Title"));
        assert!(result["yaml_config"].as_str().unwrap().contains("version:"));
        assert!(result["shell_script"]
            .as_str()
            .unwrap()
            .contains("#!/bin/bash"));

        // API response should be parsed as an object
        assert_eq!(
            result["api_response"],
            json!({
                "status": "success",
                "data": {
                    "users": [
                        {
                            "id": 1,
                            "name": "John Doe"
                        }
                    ]
                }
            })
        );
    }

    #[test]
    fn test_nested_automation_data_types() {
        let mut templater = Templater::new();

        // Template with nested structure
        let template = json!({
            "request": {
                "endpoint": "/api/v1/process",
                "payload": {
                    "document": {
                        "xml_section": "{{inputs.document.xml_content}}",
                        "metadata": {
                            "query": "{{inputs.document.sql_query}}",
                            "format": {
                                "csv_part": "{{inputs.document.csv_data}}"
                            }
                        }
                    },
                    "attachments": [
                        {
                            "name": "config.yaml",
                            "content": "{{inputs.configs.yaml_content}}",
                            "metadata": {
                                "script": "{{inputs.configs.deployment_script}}"
                            }
                        },
                        {
                            "name": "readme.md",
                            "content": "{{inputs.docs.markdown_content}}"
                        }
                    ],
                    "api_configs": {
                        "endpoints": "{{inputs.configs.api_endpoints}}",
                        "auth": {
                            "token": "{{inputs.configs.auth_token}}",
                            "signature": "{{inputs.configs.signature}}"
                        }
                    }
                }
            }
        });

        templater.add_template("nested_automation_test", template);

        // Context with nested data
        let context = json!({
            "inputs": {
                "document": {
                    "xml_content": "<?xml version=\"1.0\"?>\n<data>\n  <record id=\"1\">\n    <field>Nested XML Value</field>\n  </record>\n</data>",
                    "sql_query": "WITH recursive_data AS (\n  SELECT id, parent_id, name\n  FROM nested_table\n  WHERE parent_id IS NULL\n  UNION ALL\n  SELECT t.id, t.parent_id, t.name\n  FROM nested_table t\n  INNER JOIN recursive_data rd ON rd.id = t.parent_id\n)\nSELECT * FROM recursive_data;",
                    "csv_data": "parent_id,child_id,relationship\n1,2,\"direct\"\n1,3,\"indirect\"\n2,4,\"direct\""
                },
                "configs": {
                    "yaml_content": "global:\n  environment: production\nservices:\n  frontend:\n    replicas: 3\n    config:\n      api_version: v2\n      timeout: 30s",
                    "deployment_script": "#!/bin/bash\nset -e\n\nDEPLOY_ENV=\"prod\"\nfor service in $(cat services.txt); do\n  kubectl apply -f \"${service}.yaml\"\ndone",
                    "api_endpoints": {
                        "prod": "https://api.prod.example.com",
                        "staging": "https://api.staging.example.com",
                        "dev": "https://api.dev.example.com"
                    },
                    "auth_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
                    "signature": "sha256:d8e8fca2dc0f896fd7cb4cb0031ba249"
                },
                "docs": {
                    "markdown_content": "# Nested Documentation\n\n## System Architecture\n\n```mermaid\ngraph TD\n    A[Client] --> B[Load Balancer]\n    B --> C[Server 1]\n    B --> D[Server 2]\n```\n\n### Configuration\nRefer to `config.yaml` for detailed settings."
                }
            }
        });

        // Set up validations
        let mut validations = HashMap::new();
        validations.insert("request".to_string(), ValidationFieldType::Object);

        let result = templater
            .render("nested_automation_test", &context, validations)
            .unwrap();

        // Verify nested structure
        let payload = &result["request"]["payload"];

        // Check document section
        assert!(payload["document"]["xml_section"]
            .as_str()
            .unwrap()
            .contains("<?xml version=\"1.0\"?>"));

        assert!(payload["document"]["metadata"]["query"]
            .as_str()
            .unwrap()
            .contains("WITH recursive_data"));

        assert!(payload["document"]["metadata"]["format"]["csv_part"]
            .as_str()
            .unwrap()
            .contains("parent_id,child_id"));

        // Check attachments
        let attachments = payload["attachments"].as_array().unwrap();

        // Verify YAML attachment
        assert!(attachments[0]["content"]
            .as_str()
            .unwrap()
            .contains("global:"));

        assert!(attachments[0]["metadata"]["script"]
            .as_str()
            .unwrap()
            .contains("#!/bin/bash"));

        // Verify Markdown attachment
        assert!(attachments[1]["content"]
            .as_str()
            .unwrap()
            .contains("# Nested Documentation"));

        // Verify API configs
        let api_configs = &payload["api_configs"];
        assert_eq!(
            api_configs["auth"]["token"],
            "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
        );

        // Verify nested JSON object
        assert_eq!(
            api_configs["endpoints"],
            json!({
                "prod": "https://api.prod.example.com",
                "staging": "https://api.staging.example.com",
                "dev": "https://api.dev.example.com"
            })
        );
    }
}
