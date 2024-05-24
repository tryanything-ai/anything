use anything_pdk::{AnythingPlugin, Event, Handle, Log};
use extism_pdk::*;
use serde::Deserialize;
use serde_json::{json, Value};

//Called when the plugin is loaded. This gives the host the needed information about the plugin.
//It also provides Information to generate a nice UI including icons and labels.
//Not all hosts will use all information ( likely )
#[plugin_fn]
pub fn register() -> FnResult<AnythingPlugin> {
    let plugin: AnythingPlugin = AnythingPlugin::builder()
        .trigger(false)
        .label("HTTP Plugin".to_string())
        .icon("<svg width=\"16\" height=\"16\" viewBox=\"0 0 16 16\" xmlns=\"http://www.w3.org/2000/svg\" fill=\"currentColor\"><path fill-rule=\"evenodd\" clip-rule=\"evenodd\" d=\"M2.998 5.58a5.55 5.55 0 0 1 1.62-3.88l-.71-.7a6.45 6.45 0 0 0 0 9.16l.71-.7a5.55 5.55 0 0 1-1.62-3.88zm1.06 0a4.42 4.42 0 0 0 1.32 3.17l.71-.71a3.27 3.27 0 0 1-.76-1.12 3.45 3.45 0 0 1 0-2.67 3.22 3.22 0 0 1 .76-1.13l-.71-.71a4.46 4.46 0 0 0-1.32 3.17zm7.65 3.21l-.71-.71c.33-.32.59-.704.76-1.13a3.449 3.449 0 0 0 0-2.67 3.22 3.22 0 0 0-.76-1.13l.71-.7a4.468 4.468 0 0 1 0 6.34zM13.068 1l-.71.71a5.43 5.43 0 0 1 0 7.74l.71.71a6.45 6.45 0 0 0 0-9.16zM9.993 5.43a1.5 1.5 0 0 1-.245.98 2 2 0 0 1-.27.23l3.44 7.73-.92.4-.77-1.73h-5.54l-.77 1.73-.92-.4 3.44-7.73a1.52 1.52 0 0 1-.33-1.63 1.55 1.55 0 0 1 .56-.68 1.5 1.5 0 0 1 2.325 1.1zm-1.595-.34a.52.52 0 0 0-.25.14.52.52 0 0 0-.11.22.48.48 0 0 0 0 .29c.04.09.102.17.18.23a.54.54 0 0 0 .28.08.51.51 0 0 0 .5-.5.54.54 0 0 0-.08-.28.58.58 0 0 0-.23-.18.48.48 0 0 0-.29 0zm.23 2.05h-.27l-.87 1.94h2l-.86-1.94zm2.2 4.94l-.89-2h-2.88l-.89 2h4.66z\"/></svg>".to_string())
        .description("Make an HTTP request".to_string())
        .variables(vec![])
        .input(serde_json::json!({
            "method": "GET",
            "url": "https://google.com",
            "headers": {},
            "body": ""
        })) 
        .input_schema(serde_json::json!({
            "type": "object",
            "properties": {
                "method": {
                    "type": "string",
                    "enum": ["GET", "POST", "PUT", "DELETE"]
                },
                "url": {
                    "type": "string"
                },
                "headers": {
                    "type": "object"
                },
                "body": {
                    "type": "string"
                }
            },
            "required": ["method", "url"],
            "additionalProperties": false
        }))
        .output_schema(serde_json::json!({
            "type": "object",
            "properties": {
                "status": {
                    "type": "string",
                    "enum": ["success", "error"]
                },
                "output": {
                    "type": "object"
                },
                "error": {
                    "type": "object"
                }
            },
            "required": ["status"],
            "additionalProperties": false
        }))
        .plugin_id("http".to_string())
        .build();

    Ok(plugin)
}

#[plugin_fn]
pub fn execute(config: Value) -> FnResult<Value> {
    let res = serde_json::json!({
        "status": "success",
        "output": {},
        "error": {}, 
    });
    Ok(res)
}

// #[plugin_fn]
// pub fn validate(config: Value) -> FnResult<Value> {
//     let mut errors = serde_json::Map::new();

//     if let Some(method) = config.get("method") {
//         let mut method_errors = Vec::new();
//         if !method.is_string() || method.as_str().unwrap().is_empty() {
//             method_errors.push("The 'method' field must be a non-empty string.".to_string());
//         }
//         if !method_errors.is_empty() {
//             errors.insert("method".to_string(), json!(method_errors));
//         }
//     } else {
//         errors.insert(
//             "method".to_string(),
//             json!(["The 'method' field is required.".to_string()]),
//         );
//     }

//     if let Some(url) = config.get("url") {
//         let mut url_errors = Vec::new();
//         if !url.is_string() || url.as_str().unwrap().is_empty() {
//             url_errors.push("The 'url' field must be a non-empty string.".to_string());
//         }
//         if !url_errors.is_empty() {
//             errors.insert("url".to_string(), json!(url_errors));
//         }
//     } else {
//         errors.insert(
//             "url".to_string(),
//             json!(["The 'url' field is required.".to_string()]),
//         );
//     }

//     if let Some(headers) = config.get("headers") {
//         let mut headers_errors = Vec::new();
//         if !headers.is_object() {
//             headers_errors.push("The 'headers' field must be an object.".to_string());
//         }
//         if !headers_errors.is_empty() {
//             errors.insert("headers".to_string(), json!(headers_errors));
//         }
//     } else {
//         errors.insert(
//             "headers".to_string(),
//             json!(["The 'headers' field is required.".to_string()]),
//         );
//     }

//     if let Some(body) = config.get("body") {
//         let mut body_errors = Vec::new();
//         if !body.is_string() {
//             body_errors.push("The 'body' field must be a string.".to_string());
//         }
//         if !body_errors.is_empty() {
//             errors.insert("body".to_string(), json!(body_errors));
//         }
//     } else {
//         errors.insert(
//             "body".to_string(),
//             json!(["The 'body' field is required.".to_string()]),
//         );
//     }

//     if errors.is_empty() {
//         Ok(json!({"valid": true}))
//     } else {
//         Ok(json!({"valid": false, "errors": errors}))
//     }
// }
