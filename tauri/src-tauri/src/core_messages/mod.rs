use anything_events::clients::{
    flows_service_client::FlowsServiceClient, GetFlowRequest, GetFlowResponse, GetFlowsRequest,
    GetFlowsResponse,
};
use anything_events::models::Flow as FlowModel;
use tonic::Request;

#[tauri::command]
pub async fn get_flows() -> Result<Vec<FlowModel>, ()> {
    let mut client = FlowsServiceClient::connect("http://localhost:50234")
        .await
        .unwrap();
    let request = Request::new(GetFlowsRequest {});
    let response = client
        .get_flows(request)
        .await
        .expect("error making request");

    let flows = response.into_inner().flows;
    let flows: Vec<FlowModel> = flows
        .into_iter()
        .map(|flow| FlowModel::from(flow))
        .collect();
    Ok(flows)
}

#[tauri::command]
pub async fn get_chat_flows() -> Result<Vec<FlowModel>, ()> {
    //TODO: actually only send over flows with chats
    let mut client = FlowsServiceClient::connect("http://localhost:50234")
        .await
        .unwrap();
    let request = Request::new(GetFlowsRequest {});
    let response = client
        .get_flows(request)
        .await
        .expect("error making request");

    let flows = response.into_inner().flows;
    let flows: Vec<FlowModel> = flows
        .into_iter()
        .map(|flow| FlowModel::from(flow))
        .collect();
    Ok(flows)
}

#[tauri::command]
pub async fn get_flow(flow_id: String) -> Result<FlowModel, ()> {
    let mut client = FlowsServiceClient::connect("http://localhost:50234")
        .await
        .unwrap();
    let request = Request::new(GetFlowRequest { flow_id });
    let response = client
        .get_flow(request)
        .await
        .expect("error making request");

    let flow = FlowModel::from(response.into_inner().flow.unwrap()); 

    Ok(flow)
}

#[tauri::command]
pub fn get_flow_node(flow_id: String, node_id: String) -> String {
    format!(
        "Stub for Flow of id == {} to get node id == {}",
        flow_id, node_id
    )
}

#[tauri::command]
pub fn get_nodes() -> String {
    "Stub for getting all nodes ( templates )".to_string()
}

#[tauri::command]
pub fn create_flow() -> String {
    "Stub for creating a flow".to_string()
}

#[tauri::command]
pub fn create_event(event: String) -> String {
    "Stub for creating an event".to_string()
}
