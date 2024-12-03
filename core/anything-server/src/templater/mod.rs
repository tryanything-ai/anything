use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;

use crate::types::action_types::ValidationFieldType;

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
                    result.insert(
                        k.clone(),
                        self.render_value(v, context, validations, &current_path)?,
                    );
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
                    // Retrieve the full path as the validation key
                    let validation_key = if !path.is_empty() {
                        path.join(".")
                    } else {
                        variable.to_string()
                    };

                    let expected_type =
                        validations
                            .get(&validation_key)
                            .ok_or_else(|| TemplateError {
                                message: format!(
                                    "Validation not found for key '{}'",
                                    validation_key.clone()
                                ),
                                variable: validation_key.clone(),
                            })?;

                    let value = Self::get_value_from_path(context, variable, expected_type)
                        .ok_or_else(|| TemplateError {
                            message: format!("Variable not found in context: {}", variable),
                            variable: variable.to_string(),
                        })?;

                    let value =
                        self.validate_and_convert_value(value, expected_type, &validation_key)?;

                    return Ok(value);
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
                    let validation_key = if !path.is_empty() {
                        path.join(".")
                    } else {
                        variable.to_string()
                    };

                    let expected_type =
                        validations
                            .get(&validation_key)
                            .ok_or_else(|| TemplateError {
                                message: format!(
                                    "Validation not found for key '{}'",
                                    validation_key
                                ),
                                variable: validation_key.clone(),
                            })?;

                    let value = Self::get_value_from_path(context, variable, expected_type)
                        .ok_or_else(|| TemplateError {
                            message: "Variable not found in context".to_string(),
                            variable: variable.to_string(),
                        })?;

                    let value =
                        self.validate_and_convert_value(value, expected_type, &validation_key)?;

                    let replacement = match value {
                        Value::String(s) => s.clone(),
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
                _ => Err(TemplateError {
                    message: format!("Expected object, got: {:?}", value),
                    variable: variable.to_string(),
                }),
            },
            ValidationFieldType::Array => match value {
                Value::Array(_) => Ok(value),
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
                "a_array_string": "[1, 2, 3]",
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
}
