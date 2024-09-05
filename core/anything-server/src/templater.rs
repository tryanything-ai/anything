use std::collections::HashMap;
use std::error::Error;

#[derive(Debug)]
pub struct TemplateError {
    pub message: String,
    pub variable: String,
}

impl std::fmt::Display for TemplateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Template error for variable '{}': {}", self.variable, self.message)
    }
}

impl Error for TemplateError {}

pub struct Templater {
    templates: HashMap<String, String>,
}

impl Templater {
    pub fn new() -> Self {
        Templater {
            templates: HashMap::new(),
        }
    }

    pub fn add_template(&mut self, name: &str, template: &str) {
        self.templates.insert(name.to_string(), template.to_string());
    }

    pub fn get_template_variables(&self, template_name: &str) -> Result<Vec<String>, TemplateError> {
        let template = self.templates.get(template_name).ok_or_else(|| TemplateError {
            message: "Template not found".to_string(),
            variable: template_name.to_string(),
        })?;

        let mut variables = Vec::new();
        let mut start = 0;
        while let Some(open_idx) = template[start..].find("{{") {
            let open_idx = start + open_idx;
            let close_idx = template[open_idx..].find("}}").ok_or_else(|| TemplateError {
                message: "Unclosed template variable".to_string(),
                variable: template_name.to_string(),
            })?;
            let close_idx = open_idx + close_idx;
            let variable = template[open_idx + 2..close_idx].trim().to_string();
            variables.push(variable);
            start = close_idx + 2;
        }

        Ok(variables)
    }

    pub fn render(&self, template_name: &str, context: &HashMap<String, serde_json::Value>) -> Result<String, TemplateError> {
        let template = self.templates.get(template_name).ok_or_else(|| TemplateError {
            message: "Template not found".to_string(),
            variable: template_name.to_string(),
        })?;

        let mut result = String::new();
        let mut start = 0;
        while let Some(open_idx) = template[start..].find("{{") {
            let open_idx = start + open_idx;
            result.push_str(&template[start..open_idx]);
            let close_idx = template[open_idx..].find("}}").ok_or_else(|| TemplateError {
                message: "Unclosed template variable".to_string(),
                variable: template_name.to_string(),
            })?;
            let close_idx = open_idx + close_idx;
            let variable = template[open_idx + 2..close_idx].trim();
            let value = context.get(variable).ok_or_else(|| TemplateError {
                message: "Variable not found in context".to_string(),
                variable: variable.to_string(),
            })?;
            result.push_str(&serde_json::to_string(value).map_err(|e| TemplateError {
                message: e.to_string(),
                variable: variable.to_string(),
            })?);
            start = close_idx + 2;
        }
        result.push_str(&template[start..]);
        Ok(result)
    }
}
