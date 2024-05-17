use anything_pdk::{Action, Event, Handle, Log};
use extism_pdk::*;
use serde::{Deserialize, Serialize};

#[host_fn]
extern "ExtismHost" {
    fn host_log(log: String) -> Json<Log>;
    fn create_event(event: String) -> Json<Event>;
}

#[plugin_fn]
pub fn execute(message: String) -> FnResult<String> {
    let _res = unsafe { host_log(message.clone())? };
    Ok(message)
}

#[plugin_fn]
pub fn register() -> FnResult<Action> {
    let handle = Handle {
        id: "a".to_string(),
        position: "top".to_string(),
        r#type: "target".to_string(), // Using `r#type` to escape the `type` keyword
    };

    let config_json = serde_json::json!({
        "method": "GET",
        "url": "http://example.com",
        "headers": {
            "Authorization": "Bearer token"
        },
        "body": ""
    });

    let action = Action {
        trigger: false,
        node_name: "test_action".to_string(),
        node_label: "Test Action".to_string(),
        icon: "<svg width=\"16\" height=\"16\" viewBox=\"0 0 16 16\" xmlns=\"http://www.w3.org/2000/svg\" fill=\"currentColor\"><path fill-rule=\"evenodd\" clip-rule=\"evenodd\" d=\"M2.998 5.58a5.55 5.55 0 0 1 1.62-3.88l-.71-.7a6.45 6.45 0 0 0 0 9.16l.71-.7a5.55 5.55 0 0 1-1.62-3.88zm1.06 0a4.42 4.42 0 0 0 1.32 3.17l.71-.71a3.27 3.27 0 0 1-.76-1.12 3.45 3.45 0 0 1 0-2.67 3.22 3.22 0 0 1 .76-1.13l-.71-.71a4.46 4.46 0 0 0-1.32 3.17zm7.65 3.21l-.71-.71c.33-.32.59-.704.76-1.13a3.449 3.449 0 0 0 0-2.67 3.22 3.22 0 0 0-.76-1.13l.71-.7a4.468 4.468 0 0 1 0 6.34zM13.068 1l-.71.71a5.43 5.43 0 0 1 0 7.74l.71.71a6.45 6.45 0 0 0 0-9.16zM9.993 5.43a1.5 1.5 0 0 1-.245.98 2 2 0 0 1-.27.23l3.44 7.73-.92.4-.77-1.73h-5.54l-.77 1.73-.92-.4 3.44-7.73a1.52 1.52 0 0 1-.33-1.63 1.55 1.55 0 0 1 .56-.68 1.5 1.5 0 0 1 2.325 1.1zm-1.595-.34a.52.52 0 0 0-.25.14.52.52 0 0 0-.11.22.48.48 0 0 0 0 .29c.04.09.102.17.18.23a.54.54 0 0 0 .28.08.51.51 0 0 0 .\"></path></svg>".to_string(),
        description: "Testing Actions".to_string(),
        handles: vec![handle],
        variables: vec![],  // Adjust as needed
        config: config_json,
        extension_id: "test".to_string(),
    };

    Ok(action)
}
