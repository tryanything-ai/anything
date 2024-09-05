use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;

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

    fn get_value_from_path(context: &Value, path: &str) -> Option<Value> {
        let mut current = context;
        for key in path.split('.') {
            match current {
                Value::Object(map) => {
                    current = map.get(key)?;
                }
                _ => return None,
            }
        }
        Some(current.clone())
    }

    pub fn render(&self, template_name: &str, context: &Value) -> Result<Value, TemplateError> {
        let template = self
            .templates
            .get(template_name)
            .ok_or_else(|| TemplateError {
                message: "Template not found".to_string(),
                variable: template_name.to_string(),
            })?;

        self.render_value(template, context)
    }

    fn render_value(&self, value: &Value, context: &Value) -> Result<Value, TemplateError> {
        match value {
            Value::Object(map) => {
                let mut result = serde_json::Map::new();
                for (k, v) in map {
                    result.insert(k.clone(), self.render_value(v, context)?);
                }
                Ok(Value::Object(result))
            }
            Value::Array(arr) => {
                let mut result = Vec::new();
                for v in arr {
                    result.push(self.render_value(v, context)?);
                }
                Ok(Value::Array(result))
            }
            Value::String(s) => {
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

                    let value = Self::get_value_from_path(context, variable).ok_or_else(|| {
                        TemplateError {
                            message: "Variable not found in context".to_string(),
                            variable: variable.to_string(),
                        }
                    })?;

                    let replacement = match value {
                        Value::String(s) => s.clone(),
                        _ => value.to_string(),
                    };
                    result.replace_range(open_idx..close_idx + 2, &replacement);
                    start = open_idx + replacement.len();
                }

                // Try to parse the result as JSON
                match serde_json::from_str(&result) {
                    Ok(json_value) => Ok(json_value),
                    Err(_) => Ok(Value::String(result)),
                }
            }
            _ => Ok(value.clone()),
        }
    }

}
