use crate::{error::FlowResult, AnythingState};
use anything_common::tracing::{self};

use serde_json::Value;

use anything_persistence::{
    CreateFlowVersion, FlowVersion, StoreEvent, UpdateFlowArgs, UpdateFlowVersion,
};
use anything_persistence::{EventList, StoredFlow};
use serde::Serialize;

#[derive(Serialize)]
pub struct GetFlowsResponse {
    flows: Option<Vec<StoredFlow>>,
}

#[tauri::command]
pub async fn get_flows(state: tauri::State<'_, AnythingState>) -> FlowResult<GetFlowsResponse> {
    // Acquire the lock asynchronously and await until it becomes available.
    let inner = state.inner.lock().await;

    // Proceed with getting flows once the lock is acquired.
    match inner.get_flows().await {
        Ok(flows) => Ok(GetFlowsResponse { flows: Some(flows) }),
        Err(e) => {
            tracing::error!("Error getting flows: {:?}", e);
            // Consider if returning an empty vector is the best approach in case of an error.
            // It might be better to return an appropriate error response instead.
            Ok(GetFlowsResponse {
                flows: Some(vec![]),
            })
        }
    }
}

#[derive(Serialize)]
pub struct GetFlowResponse {
    flow: Option<StoredFlow>,
}

#[tauri::command]
pub async fn get_flow_by_name(
    state: tauri::State<'_, AnythingState>,
    flow_name: String,
) -> FlowResult<GetFlowResponse> {
    // Acquire the lock asynchronously. This will await until the lock becomes available.
    let manager = state.inner.lock().await;

    // Once the lock is acquired, proceed with fetching the flow by name.
    match manager.get_flow(flow_name).await {
        Ok(flows) => Ok(GetFlowResponse { flow: Some(flows) }),
        Err(e) => {
            tracing::error!("Error getting flows: {:?}", e);
            // Consider the appropriateness of returning None in case of an error.
            // It may be more suitable to return a different error response.
            Ok(GetFlowResponse { flow: None })
        }
    }
}

#[derive(Serialize)]
pub struct CreateFlowResponse {
    flow: Option<StoredFlow>,
}

#[tauri::command]
pub async fn create_flow(
    state: tauri::State<'_, AnythingState>,
    flow_name: String,
) -> FlowResult<CreateFlowResponse> {
    tracing::debug!("Creating flow inside tauri plugin with name: {}", flow_name);

    // Acquire the lock asynchronously. This will await until the lock becomes available.
    let mut inner = state.inner.lock().await;

    // Once the lock is acquired, proceed with creating the flow.
    match inner.create_flow(flow_name).await {
        Ok(flow) => {
            tracing::debug!("Created flow inside tauri plugin successfully: {:#?}", flow);
            Ok(CreateFlowResponse { flow: Some(flow) })
        }
        Err(e) => {
            eprintln!("Error getting flows after creating flow: {:?}", e);
            // Consider if returning None is appropriate in case of an error.
            // You might want to handle errors more explicitly, depending on your application's requirements.
            Ok(CreateFlowResponse { flow: None })
        }
    }
}

#[derive(Serialize)]
pub struct DeleteFlowResponse {
    flow: Option<String>,
}

#[tauri::command]
pub async fn delete_flow(
    state: tauri::State<'_, AnythingState>,
    flow_id: String,
) -> FlowResult<DeleteFlowResponse> {
    // Acquire the lock asynchronously.
    let inner = state.inner.lock().await;

    // Proceed with deleting the flow.
    match inner.delete_flow(flow_id).await {
        Ok(flow) => Ok(DeleteFlowResponse { flow: Some(flow) }),
        Err(e) => {
            eprintln!("Error deleting flow: {:?}", e);
            Ok(DeleteFlowResponse { flow: None })
        }
    }
}

#[derive(Serialize)]
pub struct UpdateFlowResponse {
    flow: Option<StoredFlow>,
}

#[tauri::command]
pub async fn update_flow(
    state: tauri::State<'_, AnythingState>,
    flow_id: String,
    args: UpdateFlowArgs,
) -> FlowResult<UpdateFlowResponse> {
    // Acquire the lock asynchronously.
    let mut inner = state.inner.lock().await;

    // Proceed with updating the flow.
    match inner.update_flow(flow_id, args).await {
        Ok(flow) => {
            tracing::debug!("Updated flow inside tauri plugin");
            Ok(UpdateFlowResponse { flow: Some(flow) })
        }
        Err(e) => {
            eprintln!("Error updating flow: {:?}", e);
            Ok(UpdateFlowResponse { flow: None })
        }
    }
}

#[derive(Serialize)]
pub struct CreateFlowVersionResponse {
    flow_version: Option<FlowVersion>,
}

#[tauri::command]
pub async fn create_flow_version(
    state: tauri::State<'_, AnythingState>,
    flow_name: String,
    create_flow: CreateFlowVersion,
) -> FlowResult<CreateFlowVersionResponse> {
    // Acquire the lock asynchronously.
    let mut inner = state.inner.lock().await;

    // Proceed with creating the flow version.
    match inner.create_flow_version(flow_name, create_flow).await {
        Ok(flow) => {
            tracing::debug!("Created flow version inside tauri plugin");
            Ok(CreateFlowVersionResponse {
                flow_version: Some(flow),
            })
        }
        Err(e) => {
            eprintln!("Error creating flow version: {:?}", e);
            Ok(CreateFlowVersionResponse { flow_version: None })
        }
    }
}

#[derive(Serialize)]
pub struct UpdateFlowVersionResponse {
    flow_version: Option<FlowVersion>,
}

#[tauri::command]
pub async fn update_flow_version(
    state: tauri::State<'_, AnythingState>,
    flow_id: String,
    flow_version_id: String,
    update_flow: UpdateFlowVersion,
) -> FlowResult<UpdateFlowVersionResponse> {
    // Acquire the lock asynchronously.
    let mut inner = state.inner.lock().await;

    // Proceed with updating the flow version.
    match inner
        .update_flow_version(flow_id, flow_version_id, update_flow)
        .await
    {
        Ok(flow) => {
            tracing::debug!("Updated flow version inside tauri plugin");
            Ok(UpdateFlowVersionResponse {
                flow_version: Some(flow),
            })
        }
        Err(e) => {
            eprintln!("Error updating flow version: {:?}", e);
            Ok(UpdateFlowVersionResponse { flow_version: None })
        }
    }
}

#[derive(Serialize)]
pub struct ExecuteFlowResponse {
    flow_session_id: Option<String>,
}

#[tauri::command]
pub async fn execute_flow(
    state: tauri::State<'_, AnythingState>,
    flow_id: String,
    flow_version_id: String,
    session_id: Option<String>,
    stage: Option<String>,
) -> FlowResult<ExecuteFlowResponse> {
    // Acquire the lock asynchronously. This will await until the lock becomes available.
    let inner = state.inner.lock().await;

    // Now that we have the lock, proceed with executing the flow.
    match inner
        .execute_flow(flow_id, flow_version_id, session_id, stage)
        .await
    {
        Ok(flow_session_id) => {
            tracing::debug!("Executed flow flow inside tauri plugin");
            Ok(ExecuteFlowResponse {
                flow_session_id: Some(flow_session_id),
            })
        }
        Err(e) => {
            eprintln!("Error getting flows after executing flow: {:?}", e);
            Ok(ExecuteFlowResponse {
                flow_session_id: None,
            })
        }
    }
}

#[derive(Serialize)]
pub struct GetEventsResponse {
    events: Option<EventList>,
}

#[tauri::command]
pub async fn fetch_session_events(
    state: tauri::State<'_, AnythingState>,
    session_id: String,
) -> FlowResult<GetEventsResponse> {
    // Acquire the lock asynchronously and wait until it becomes available.
    let manager = state.inner.lock().await;

    // Proceed with fetching session events.
    match manager.fetch_session_events(session_id).await {
        Ok(events) => Ok(GetEventsResponse {
            events: Some(events),
        }),
        Err(e) => {
            tracing::error!("Error getting events: {:?}", e);
            // Consider whether returning None is appropriate for your application's error handling strategy.
            Ok(GetEventsResponse { events: None })
        }
    }
}

#[derive(Serialize)]
pub struct GetEventResponse {
    event: Option<StoreEvent>,
}

#[tauri::command]
pub async fn get_event(
    state: tauri::State<'_, AnythingState>,
    event_id: String,
) -> FlowResult<GetEventResponse> {
    // Acquire the lock asynchronously.
    let manager = state.inner.lock().await;

    // Once the lock is acquired, proceed with fetching the specific event.
    match manager.get_event(event_id).await {
        Ok(event) => Ok(GetEventResponse { event: Some(event) }),
        Err(e) => {
            tracing::error!("Error getting event: {:?}", e);
            // Similar consideration here regarding error handling and what should be returned.
            Ok(GetEventResponse { event: None })
        }
    }
}

#[derive(Serialize)]
pub struct GetActionsResponse {
    actions: Option<Vec<Value>>,
}

#[tauri::command]
pub async fn get_actions(state: tauri::State<'_, AnythingState>) -> FlowResult<GetActionsResponse> {
    // Acquire the lock asynchronously.
    let manager = state.inner.lock().await;

    // Proceed with fetching actions.
    match manager.get_actions().await {
        Ok(actions) => Ok(GetActionsResponse {
            actions: Some(actions),
        }),
        Err(e) => {
            tracing::error!("Error getting actions: {:?}", e);
            Ok(GetActionsResponse { actions: None })
        }
    }
}
