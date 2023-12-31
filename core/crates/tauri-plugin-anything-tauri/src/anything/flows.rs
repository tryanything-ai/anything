use crate::{error::FlowResult, AnythingState, Error};
use anything_graph::Flow;
use serde::Serialize;

#[derive(Serialize)]
pub struct GetFlowsResponse {
    flows: Option<Vec<Flow>>,
}

#[tauri::command]
pub async fn get_flows(state: tauri::State<'_, AnythingState>) -> FlowResult<GetFlowsResponse> {
    match state.inner.try_lock() {
        Err(_e) => Err(Error::CoordinatorNotInitialized),
        Ok(ref inner) => match inner.get_flows().await {
            Ok(flows) => Ok(GetFlowsResponse { flows: Some(flows) }),
            Err(e) => {
                eprintln!("Error getting flows: {:?}", e);
                Ok(GetFlowsResponse { flows: None })
            }
        },
    }
}

#[derive(Serialize)]
pub struct CreateFlowResponse {
    flow: Option<Flow>,
}

#[tauri::command]
pub async fn create_flow(
    state: tauri::State<'_, AnythingState>,
    flow_name: String,
    flow_id: String,
) -> FlowResult<CreateFlowResponse> {
    match state.inner.try_lock() {
        Err(_e) => Err(Error::CoordinatorNotInitialized),
        Ok(ref inner) => match inner.create_flow(flow_name, flow_id).await {
            Ok(flow) => Ok(CreateFlowResponse { flow: Some(flow) }),
            Err(e) => {
                eprintln!("Error getting flows: {:?}", e);
                Ok(CreateFlowResponse { flow: None })
            }
        },
    }
}

#[derive(Serialize)]
pub struct DeleteFlowResponse {
    flow: Option<Flow>,
}

#[tauri::command]
pub async fn delete_flow(
    state: tauri::State<'_, AnythingState>,
    flow_name: String,
) -> FlowResult<DeleteFlowResponse> {
    match state.inner.try_lock() {
        Err(_e) => Err(Error::CoordinatorNotInitialized),
        Ok(ref inner) => match inner.delete_flow(flow_name).await {
            Ok(flow) => Ok(DeleteFlowResponse { flow: Some(flow) }),
            Err(e) => {
                eprintln!("Error getting flows: {:?}", e);
                Ok(DeleteFlowResponse { flow: None })
            }
        },
    }
}

#[derive(Serialize)]
pub struct UpdateFlowResponse {
    flow: Option<Flow>,
}

#[tauri::command]
pub async fn update_flow(
    state: tauri::State<'_, AnythingState>,
    flow_name: String,
) -> FlowResult<DeleteFlowResponse> {
    match state.inner.try_lock() {
        Err(_e) => Err(Error::CoordinatorNotInitialized),
        Ok(ref inner) => match inner.update_flow(flow_name).await {
            Ok(flow) => Ok(DeleteFlowResponse { flow: Some(flow) }),
            Err(e) => {
                eprintln!("Error getting flows: {:?}", e);
                Ok(DeleteFlowResponse { flow: None })
            }
        },
    }
}
