use std::fs;
use std::io::Error as IOError; 
use serde_json::Value as JsonValue;
use toml;
use tauri; 
use crate::config::get_flows_dir;

// Function to find all flows having a "nodeType" of "receiveChatNode"
// #[tauri::command]
// pub fn get_chat_flows() -> Result<Vec<String>, IOError> {
//     let mut flows_with_receive_chat_node = Vec::new();
    
//     // Assuming get_flows_dir() gets the directory where the flow folders are stored
//     let flows_dir = get_flows_dir()?;
    
//     // Read the directory
//     let entries = fs::read_dir(flows_dir)?;
    
//     for entry in entries {
//         if let Ok(entry) = entry {
//             let path = entry.path();
//             if path.is_dir() {
//                 let flow_name = path.file_name().unwrap().to_str().unwrap();
//                 let toml_file_path = path.join("flow.toml");

//                 // Read and parse the TOML file
//                 if let Ok(toml_content) = fs::read_to_string(&toml_file_path) {
//                     let parsed_toml: JsonValue = toml::from_str(&toml_content).expect("Failed to parse TOML");
                    
//                     if let Some(nodes) = parsed_toml.get("nodes") {
//                         for node in nodes.as_array().unwrap() {
//                             if let Some(node_type) = node.get("type") {
//                                 if node_type.as_str().unwrap_or("") == "receiveChatNode" {
//                                     flows_with_receive_chat_node.push(flow_name.to_string());
//                                     break;
//                                 }
//                             }
//                         }
//                     }
//                 }
//             }
//         }
//     }
    
//     Ok(flows_with_receive_chat_node)
// }


// use std::fs;
// use std::path::PathBuf;
// use tauri::command;
// use serde_json::Value as JsonValue;
// use std::io::{Result, Error as IOError, ErrorKind};
// use toml;

// Assuming you have these helper functions
// ...

#[tauri::command]
pub fn get_chat_flows() -> Result<Vec<String>, String> {
    let mut flows_with_receive_chat_node = Vec::new();

    let flows_dir = get_flows_dir().map_err(|e| e.to_string())?;

    let entries = fs::read_dir(flows_dir).map_err(|e| e.to_string())?;
    
    for entry in entries {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        if path.is_dir() {
            let flow_name = path.file_name().unwrap().to_str().unwrap();
            let toml_file_path = path.join("flow.toml");

            let toml_content = fs::read_to_string(&toml_file_path).map_err(|e| e.to_string())?;
            let parsed_toml: JsonValue = toml::from_str(&toml_content).map_err(|e| e.to_string())?;
            
            if let Some(nodes) = parsed_toml.get("nodes") {
                for node in nodes.as_array().unwrap() {
                    if let Some(node_type) = node.get("type") {
                        if node_type.as_str().unwrap_or("") == "receiveChatNode" {
                            flows_with_receive_chat_node.push(flow_name.to_string());
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

