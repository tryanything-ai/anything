#[tauri::command]
pub fn get_flows() -> String {
    "Stub for Flows".to_string()
}

#[tauri::command]
pub fn get_chat_flows() -> String {
    "Stub for Flows with chats".to_string()
}

#[tauri::command]
pub fn get_flow(flow_id: String) -> String {
    format!("Stub for Flow of id == {}", flow_id)
}

#[tauri::command]
pub fn get_flow_node(flow_id: String, node_id: String) -> String {
    format!("Stub for Flow of id == {} to get node id == {}", flow_id, node_id)
}

#[tauri::command]
pub fn get_nodes() -> String {
    "Stub for getting all nodes ( templates )".to_string()
}

