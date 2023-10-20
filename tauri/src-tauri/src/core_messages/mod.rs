use anything_events::clients::{
    flows_service_client::FlowsServiceClient, GetFlowByNameRequest, GetFlowRequest,
    GetFlowVersionsRequest, GetFlowsRequest,
};
use anything_events::models::Flow;
use anything_events::models::FlowVersion;
use tonic::Request;

use tracing::info;

static BACKEND_ENDPOINT: &str = "http://localhost:50234";

#[tauri::command]
pub async fn get_flows() -> Result<Vec<Flow>, ()> {
    
    let mut client = FlowsServiceClient::connect(BACKEND_ENDPOINT).await.unwrap();
    let request = Request::new(GetFlowsRequest {});
    let response = client
        .get_flows(request)
        .await
        .expect("error making request");

    let flows = response.into_inner().flows;
    let flows: Vec<Flow> = flows.into_iter().map(|flow| Flow::from(flow)).collect();
    Ok(flows)
}

// #[tauri::command]
// pub async fn get_flow_versions(flow_id: String) -> Result<Vec<FlowVersion>, ()> {
//     let mut client = FlowsServiceClient::connect(BACKEND_ENDPOINT).await.unwrap();
//     let request = Request::new(GetFlowVersionsRequest { flow_id });
//     let response = client
//         .get_flow_versions(request)
//         .await
//         .expect("error making request");

//     let flow_versions = response.into_inner().flow_versions;
//     let flow_versions: Vec<FlowVersion> = flow_versions
//         .into_iter()
//         .map(|flow_version| FlowVersion::from(flow_version))
//         .collect();
//     Ok(flow_versions)
// }

#[tauri::command]
pub async fn get_chat_flows() -> Result<Vec<Flow>, ()> {
    //TODO: actually only send over flows with chats
    let mut client = FlowsServiceClient::connect(BACKEND_ENDPOINT).await.unwrap();
    let request = Request::new(GetFlowsRequest {});
    let response = client
        .get_flows(request)
        .await
        .expect("error making request");

    let flows = response.into_inner().flows;
    let flows: Vec<Flow> = flows.into_iter().map(|flow| Flow::from(flow)).collect();
    Ok(flows)
}

#[tauri::command]
pub async fn get_flow(flow_id: String) -> Result<Flow, ()> {
    let mut client = FlowsServiceClient::connect(BACKEND_ENDPOINT).await.unwrap();
    let request = Request::new(GetFlowRequest { flow_id });
    let response = client
        .get_flow(request)
        .await
        .expect("error making request");

    let flow = Flow::from(response.into_inner().flow.unwrap());

    Ok(flow)
}

#[tauri::command]
pub async fn get_flow_by_name(flow_name: String) -> Result<Flow, ()> {
    let mut client = FlowsServiceClient::connect("http://localhost:50234")
        .await
        .unwrap();
    let request = Request::new(GetFlowByNameRequest { flow_name });
    let response = client
        .get_flow_by_name(request)
        .await
        .expect("error making request");

    info!("Get_flow_by_name respone {:?}", response);

    // Try to access the Flow from the response, returning an error if not found.
    let flow_option = response.into_inner().flow;
    let flow = flow_option.ok_or(())?; // If the flow is None, return an error.

    let flow_model = Flow::from(flow);

    Ok(flow_model)
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
