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
                    return None;
                }
            } else {
                current = current.get(part)?;
            }

            // Try to parse string values as JSON for nested traversal
            if let Value::String(s) = current {
                if let Ok(parsed) = Self::try_parse_json(s) {
                    if i < parts.len() - 1 {
                        return Self::get_value_from_path(&parsed, &parts[i + 1..].join("."));
                    } else {
                        return Some(parsed);
                    }
                }
            }
        }
        Some(current.clone())
    }

    fn try_parse_json(s: &str) -> Result<Value, serde_json::Error> {
        // Try parsing with various cleaning steps
        let attempts = vec![
            s.to_string(),
            s.replace("\n", "").replace("\r", ""),
            s.replace("\n", "").replace("\r", "").replace(" ", ""),
        ];

        for attempt in attempts {
            if let Ok(parsed) = serde_json::from_str(&attempt) {
                return Ok(parsed);
            }
        }

        // If all attempts fail, return the original parsing error
        serde_json::from_str(s)
    }

    fn deep_clean_value(value: Value) -> Value {
        match value {
            Value::String(s) => {
                if let Ok(parsed) = Self::try_parse_json(&s) {
                    Self::deep_clean_value(parsed)
                } else {
                    Value::String(s)
                }
            }
            Value::Array(arr) => {
                Value::Array(arr.into_iter().map(Self::deep_clean_value).collect())
            }
            Value::Object(map) => Value::Object(
                map.into_iter()
                    .map(|(k, v)| (k, Self::deep_clean_value(v)))
                    .collect(),
            ),
            _ => value,
        }
    }

    pub fn render(&self, template_name: &str, context: &Value) -> Result<Value, TemplateError> {
        let template = self
            .templates
            .get(template_name)
            .ok_or_else(|| TemplateError {
                message: "Template not found".to_string(),
                variable: template_name.to_string(),
            })?;

        // Clean the context first
        let cleaned_context = Self::deep_clean_value(context.clone());
        self.render_value(template, &cleaned_context)
    }

    fn render_value(&self, value: &Value, context: &Value) -> Result<Value, TemplateError> {
        println!("[TEMPLATER] Rendering value: {:?}", value);
        match value {
            Value::Object(map) => {
                println!("[TEMPLATER] Rendering object");
                let mut result = serde_json::Map::new();
                for (k, v) in map {
                    println!("[TEMPLATER] Rendering object key: {}", k);
                    result.insert(k.clone(), self.render_value(v, context)?);
                }
                Ok(Value::Object(result))
            }
            Value::Array(arr) => {
                println!("[TEMPLATER] Rendering array");
                let mut result = Vec::new();
                for (i, v) in arr.iter().enumerate() {
                    println!("[TEMPLATER] Rendering array index: {}", i);
                    result.push(self.render_value(v, context)?);
                }
                Ok(Value::Array(result))
            }
            Value::String(s) => {
                println!("[TEMPLATER] Rendering string: {}", s);

                // Try to parse the string as JSON first
                if let Ok(parsed_value) = Self::try_parse_json(s) {
                    println!("[TEMPLATER] Successfully parsed string as JSON");
                    return self.render_value(&parsed_value, context);
                }

                // Special case: if the string is exactly "{{variables}}" (or any other full variable),
                // return the raw value instead of string conversion
                if s.trim().starts_with("{{") && s.trim().ends_with("}}") {
                    let variable = s.trim()[2..s.trim().len() - 2].trim();
                    if !variable.contains('.') {
                        if let Some(value) = Self::get_value_from_path(context, variable) {
                            return Ok(value);
                        }
                    }
                }

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

                    println!("[TEMPLATER] Found variable: {}", variable);

                    let value = Self::get_value_from_path(context, variable).ok_or_else(|| {
                        println!("[TEMPLATER] Variable not found in context: {}", variable);
                        TemplateError {
                            message: "Variable not found in context".to_string(),
                            variable: variable.to_string(),
                        }
                    })?;

                    println!("[TEMPLATER] Variable value: {:?}", value);

                    let replacement = match value {
                        Value::String(s) => s.clone(),
                        _ => value.to_string(),
                    };
                    result.replace_range(open_idx..close_idx + 2, &replacement);
                    start = open_idx + replacement.len();
                }

                println!("[TEMPLATER] Rendered string: {}", result);
                Ok(Value::String(result))
            }
            _ => {
                println!("[TEMPLATER] Returning value as-is: {:?}", value);
                Ok(value.clone())
            }
        }
    }
}
