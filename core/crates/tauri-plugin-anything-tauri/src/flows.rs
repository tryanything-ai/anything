use serde::Serialize;
use std::path::PathBuf;

use anything_graph::Flow;
use tauri::{AppHandle, Runtime};

use crate::{AnythingState, Error};

#[allow(unused)]
#[derive(Clone)]
pub struct Flows<R: Runtime> {
    app: AppHandle<R>,
    pub(crate) path: PathBuf,
}

pub type FlowResult<T> = Result<T, Error>;

#[derive(Serialize)]
pub struct GetFlowsResponse {
    flows: Option<Vec<Flow>>,
}

#[derive(Serialize)]
pub struct CreateFlowResponse {
    flow: Option<Flow>,
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

#[tauri::command]
pub async fn create_flow(
    state: tauri::State<'_, AnythingState>,
    flow_name: String,
    flow_id: String,
) -> FlowResult<CreateFlowResponse> {
    eprintln!("create_flow in flows.rs: {} {}", flow_name, flow_id);
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
