use crate::{error::EventResult, AnythingState, Error};
use anything_persistence::models::*;
use anything_persistence::EventRepo;
use anything_persistence::TriggerRepo;
use chrono::DateTime;
use chrono::Utc;
use serde::Serialize;
use tauri::Runtime;

#[derive(Serialize)]
pub struct CreateEventResponse {
    event_id: Option<String>,
}

#[tauri::command]
pub async fn save_event(
    state: tauri::State<'_, AnythingState>,
    create_event: CreateEvent,
) -> EventResult<CreateEventResponse> {
    match state.inner.try_lock() {
        Err(_e) => Err(Error::RuntimeError),
        Ok(ref inner) => {
            let event_repo = inner.event_repo().unwrap();
            match event_repo.save_event(create_event).await {
                Ok(event) => Ok(CreateEventResponse {
                    event_id: Some(event),
                }),
                Err(e) => {
                    eprintln!("Error saving event: {:?}", e);
                    Ok(CreateEventResponse { event_id: None })
                }
            }
        }
    }
}

#[derive(Serialize)]
pub struct FindEventByIdResponse {
    event: Option<StoreEvent>,
}

#[tauri::command]
pub async fn find_event_by_id(
    state: tauri::State<'_, AnythingState>,
    event_id: String,
) -> EventResult<FindEventByIdResponse> {
    match state.inner.try_lock() {
        Err(_e) => Err(Error::RuntimeError),
        Ok(ref inner) => {
            let event_repo = inner.event_repo().unwrap();
            match event_repo.find_by_id(event_id).await {
                Ok(event) => Ok(FindEventByIdResponse { event: Some(event) }),
                Err(e) => {
                    eprintln!("Error finding event: {:?}", e);
                    Ok(FindEventByIdResponse { event: None })
                }
            }
        }
    }
}

#[derive(Serialize)]
pub struct FindEventsSinceResponse {
    events: Option<Vec<StoreEvent>>,
}

#[tauri::command]
pub async fn find_events_since(
    state: tauri::State<'_, AnythingState>,
    since: DateTime<Utc>,
) -> EventResult<FindEventsSinceResponse> {
    match state.inner.try_lock() {
        Err(_e) => Err(Error::RuntimeError),
        Ok(ref inner) => {
            let event_repo = inner.event_repo().unwrap();
            match event_repo.find_events_since(since).await {
                Ok(events) => Ok(FindEventsSinceResponse {
                    events: Some(events),
                }),
                Err(e) => {
                    eprintln!("Error finding event: {:?}", e);
                    Ok(FindEventsSinceResponse { events: None })
                }
            }
        }
    }
}

#[derive(Serialize)]
pub struct FindFlowEventsResponse {
    events: Option<Vec<StoreEvent>>,
}

#[tauri::command]
pub async fn find_flow_events(
    state: tauri::State<'_, AnythingState>,
    flow_name: String,
) -> EventResult<FindFlowEventsResponse> {
    match state.inner.try_lock() {
        Err(_e) => Err(Error::RuntimeError),
        Ok(ref inner) => {
            let event_repo = inner.event_repo().unwrap();
            match event_repo.find_flow_events(flow_name).await {
                Ok(events) => Ok(FindFlowEventsResponse {
                    events: Some(events),
                }),
                Err(e) => {
                    eprintln!("Error finding event: {:?}", e);
                    Ok(FindFlowEventsResponse { events: None })
                }
            }
        }
    }
}

#[derive(Serialize)]
pub struct CreateTriggerResponse {
    trigger_id: Option<String>,
}

#[tauri::command]
pub async fn save_trigger(
    state: tauri::State<'_, AnythingState>,
    create_trigger: CreateTrigger,
) -> EventResult<CreateTriggerResponse> {
    match state.inner.try_lock() {
        Err(_e) => Err(Error::RuntimeError),
        Ok(ref inner) => {
            let event_repo = inner.trigger_repo().unwrap();
            match event_repo.create_trigger(create_trigger).await {
                Ok(trigger) => Ok(CreateTriggerResponse {
                    trigger_id: Some(trigger),
                }),
                Err(e) => {
                    eprintln!("Error saving trigger: {:?}", e);
                    Ok(CreateTriggerResponse { trigger_id: None })
                }
            }
        }
    }
}

#[derive(Serialize)]
pub struct GetTriggerByIdResponse {
    trigger: Option<StoredTrigger>,
}

#[tauri::command]
pub async fn get_trigger_by_id(
    state: tauri::State<'_, AnythingState>,
    trigger_id: String,
) -> EventResult<GetTriggerByIdResponse> {
    match state.inner.try_lock() {
        Err(_e) => Err(Error::RuntimeError),
        Ok(ref inner) => {
            let event_repo = inner.trigger_repo().unwrap();
            match event_repo.get_trigger_by_id(trigger_id).await {
                Ok(trigger) => Ok(GetTriggerByIdResponse {
                    trigger: Some(trigger),
                }),
                Err(e) => {
                    eprintln!("Error getting trigger: {:?}", e);
                    Ok(GetTriggerByIdResponse { trigger: None })
                }
            }
        }
    }
}
