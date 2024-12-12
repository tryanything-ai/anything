use serde_json::{json, Value};
use std::collections::HashMap;

use crate::types::action_types::{Action, JsonSchema, JsonSchemaProperty};

pub fn upgrade_action(plugin_id: &str, action_definition_json: &Value) -> Result<Action, String> {
    // Find existing action template with matching plugin_id
    let current_dir =
        std::env::current_dir().map_err(|e| format!("Failed to get current directory: {}", e))?;

    let json_file_path = current_dir.join("template_db/action_templates.json");
    let file = std::fs::File::open(&json_file_path)
        .map_err(|e| format!("Failed to open action templates file: {}", e))?;

    let reader = std::io::BufReader::new(file);
    let templates: Value = serde_json::from_reader(reader)
        .map_err(|e| format!("Failed to parse action templates: {}", e))?;

    // Find template with matching plugin_id
    let template = templates
        .as_array()
        .ok_or("Templates is not an array")?
        .iter()
        .find(|t| {
            t.get("action_template_definition")
                .and_then(|d| d.get("plugin_id"))
                .and_then(|id| id.as_str())
                == Some(plugin_id)
        })
        .ok_or(format!("No template found for plugin_id: {}", plugin_id))?;

    // Get the template definition
    let mut template_def: Action = serde_json::from_value(
        template
            .get("action_template_definition")
            .ok_or("Template missing action_template_definition")?
            .clone(),
    )
    .map_err(|e| format!("Failed to parse action template: {}", e))?;

    // Extract variables from provided action definition
    if let Some(vars) = action_definition_json.get("variables") {
        template_def.variables = Some(vars.clone());
    }

    // Extract and rebuild variables_schema from provided action definition
    if let Some(vars_schema) = action_definition_json.get("variables_schema") {
        let new_schema = JsonSchema {
            r#type: vars_schema
                .get("type")
                .and_then(|t| t.as_str())
                .map(String::from),
            properties: vars_schema
                .get("properties")
                .and_then(|props| props.as_object())
                .map(|props| {
                    props
                        .iter()
                        .map(|(key, value)| {
                            let prop = JsonSchemaProperty {
                                x_any_validation: value
                                    .get("x-any-validation")
                                    .map(|v| serde_json::from_value(v.clone()).unwrap_or_default()),
                                title: value
                                    .get("title")
                                    .and_then(|t| t.as_str())
                                    .map(String::from),
                                description: value
                                    .get("description")
                                    .and_then(|d| d.as_str())
                                    .map(String::from),
                                r#type: value
                                    .get("type")
                                    .and_then(|t| t.as_str())
                                    .map(String::from),
                                one_of: value
                                    .get("oneOf")
                                    .map(|o| o.as_array().unwrap_or(&vec![]).clone()),
                                x_jsf_presentation: value
                                    .get("x-jsf-presentation")
                                    .map(|p| serde_json::from_value(p.clone()).unwrap_or_default()),
                            };
                            (key.clone(), prop)
                        })
                        .collect::<HashMap<_, _>>()
                }),
            required: vars_schema
                .get("required")
                .and_then(|r| r.as_array())
                .map(|arr| {
                    arr.iter()
                        .filter_map(|v| v.as_str())
                        .map(String::from)
                        .collect()
                }),
            all_of: vars_schema
                .get("allOf")
                .and_then(|a| a.as_array())
                .map(|arr| arr.clone()),
            x_jsf_order: vars_schema
                .get("x-jsf-order")
                .and_then(|o| o.as_array())
                .map(|arr| {
                    arr.iter()
                        .filter_map(|v| v.as_str())
                        .map(String::from)
                        .collect()
                }),
            additional_properties: vars_schema
                .get("additionalProperties")
                .and_then(|a| a.as_bool()),
        };
        template_def.variables_schema = Some(new_schema);
    }

    Ok(template_def)
}

///Used to validate an action written by ai
pub fn parse_action_json(json_str: &str) -> Result<crate::types::action_types::Action, String> {
    match serde_json::from_str::<crate::types::action_types::Action>(json_str) {
        Ok(action) => Ok(action),
        Err(e) => Err(format!("Failed to parse action JSON: {}", e))
    }
}

