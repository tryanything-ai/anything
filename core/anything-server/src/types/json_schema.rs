use serde::{Deserialize, Serialize};

use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ValidationFieldType {
    String,
    Number,
    Object,
    Boolean,
    Array,
    Null,
    #[serde(other)]
    Unknown,
}

impl ValidationFieldType {
    pub fn to_string(&self) -> String {
        match self {
            ValidationFieldType::String => "string".to_string(),
            ValidationFieldType::Number => "number".to_string(),
            ValidationFieldType::Object => "object".to_string(),
            ValidationFieldType::Boolean => "boolean".to_string(),
            ValidationFieldType::Array => "array".to_string(),
            ValidationFieldType::Null => "null".to_string(),
            ValidationFieldType::Unknown => "unknown".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum InputFieldType {
    SimpleText,
    NumberOrVariable,
    BooleanOrVariable,
    ObjectOrVariable,
    HtmlOrVariable,
    JavascriptOrVariable,
    XmlOrVariable,
    SelectOrVariable,
    Text,
    Account,
    Error,
    #[serde(other)]
    Unknown,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ValidationField {
    pub r#type: ValidationFieldType,
    #[serde(default = "default_strict")]
    pub strict: bool,
}

// Add this function to provide the default value for strict
fn default_strict() -> bool {
    true
}

impl Default for ValidationField {
    fn default() -> Self {
        ValidationField {
            r#type: ValidationFieldType::Unknown,
            strict: true,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct PresentationField {
    #[serde(rename = "inputType")]
    pub input_type: InputFieldType,
}

impl Default for PresentationField {
    fn default() -> Self {
        PresentationField {
            input_type: InputFieldType::Unknown,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JsonSchemaProperty {
    #[serde(rename = "x-any-validation")]
    pub x_any_validation: Option<ValidationField>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub r#type: Option<String>,
    pub default: Option<String>,
    #[serde(rename = "oneOf")]
    //Used for select fields
    pub one_of: Option<Vec<serde_json::Value>>,
    #[serde(rename = "x-jsf-presentation")]
    pub x_jsf_presentation: Option<PresentationField>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JsonSchema {
    pub r#type: Option<String>,
    pub properties: Option<HashMap<String, JsonSchemaProperty>>,
    pub required: Option<Vec<String>>,
    #[serde(rename = "allOf")]
    pub all_of: Option<Vec<serde_json::Value>>,
    #[serde(rename = "x-jsf-order")]
    pub x_jsf_order: Option<Vec<String>>,
    #[serde(rename = "additionalProperties")]
    pub additional_properties: Option<bool>,
}
