use std::fs;
use serde_json::Value as JsonValue;
use toml;
use tauri; 
use crate::config::get_flows_dir;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct FlowInfo {
    flow_name: String,
    flow_value: JsonValue,
}

#[tauri::command]
pub fn get_chat_flows() -> Result<Vec<FlowInfo>, String> {
    let mut flows_with_receive_chat_node: Vec<FlowInfo> = Vec::new();

    let flows_dir = get_flows_dir().map_err(|e| e.to_string())?;

    let entries = fs::read_dir(flows_dir).map_err(|e| e.to_string())?;
    
    for entry in entries {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        if path.is_dir() {
            // let flow_name = path.file_name().unwrap().to_str().unwrap();
            let toml_file_path = path.join("flow.toml");

            let toml_content = fs::read_to_string(&toml_file_path).map_err(|e| e.to_string())?;
            let parsed_toml: JsonValue = toml::from_str(&toml_content).map_err(|e| e.to_string())?;
            //BUG: broken when we changed how we generate Nodes
            if let Some(nodes) = parsed_toml.get("nodes") {
                for node in nodes.as_array().unwrap() {
                    if let Some(node_type) = node.get("type") {
                        if node_type.as_str().unwrap_or("") == "receiveChatNode" {
                            // flows_with_receive_chat_node.push(flow_name.to_string());
                            if let Some(flow_value) = parsed_toml.get("flow") {
                                flows_with_receive_chat_node.push(FlowInfo {
                                    flow_name: path
                                        .file_name()
                                        .and_then(|os_str| os_str.to_str())
                                        .unwrap_or("Unknown")
                                        .to_string(),
                                    flow_value: flow_value.clone(),
                                });
                            }
                            break;
                        }
                    }
                }
            }
        }
    }
    println!("{:?}", flows_with_receive_chat_node);
    Ok(flows_with_receive_chat_node)
}
