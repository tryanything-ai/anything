#![allow(unused)]
// Temporary
use std::sync::Arc;

use chrono::Utc;
use postage::dispatch::Sender;
use postage::prelude::*;
use serde_json::json;
use sqlx::{Pool, Row, SqlitePool};
use tonic::{metadata, Request, Response, Status};

use crate::{
    context::Context,
    errors::EventsError,
    generated::{
        events::{GetEventRequest, GetEventResponse, TriggerEventRequest, TriggerEventResponse},
        events_service_server::EventsService,
    },
    models::event::{CreateEvent, Event},
    repositories::{event_repo::EventRepo, AnythingRepo},
};

use crate::generated::events::Event as ProtoEvent;

// --------------------------------------------------
// Errors
// --------------------------------------------------
const NO_EVENT: &str = "No event provided";
const NO_SOURCE_IDENTIFIER: &str = "No source identifier provided";
const NO_EVENT_DETAILS: &str = "No event details provided";
const NO_EVENT_NAME_PROVIDED: &str = "No event name provided";
const NO_EVENT_DATA_PROVIDED: &str = "No event data provided";
const UNABLE_TO_SAVE_EVENT: &str = "Unable to save event";
const EVENT_NOT_FOUND: &str = "Event not found";

// --------------------------------------------------
// Event Server impl
// --------------------------------------------------

#[derive(Debug)]
pub struct EventManager {
    context: Arc<Context>,
    update_tx: Sender<Event>,
}

impl EventManager {
    pub fn new(context: &Context, update_tx: Sender<Event>) -> Self {
        Self {
            context: Arc::new(context.clone()),
            update_tx,
        }
    }
}

#[tonic::async_trait]
impl EventsService for EventManager {
    async fn trigger_event(
        &self,
        request: Request<TriggerEventRequest>,
    ) -> Result<Response<TriggerEventResponse>, Status> {
        let req = request.into_inner();

        let event = match req.event {
            Some(e) => e,
            None => return Err(Status::invalid_argument(NO_EVENT)),
        };

        let name = event.event_name.clone();
        let trigger_id = event.trigger_id.clone();
        let payload = event.payload.clone();
        let metadata = event.metadata.clone();

        if name.is_empty() {
            return Err(Status::invalid_argument(NO_EVENT_NAME_PROVIDED));
        }
        if trigger_id.is_empty() {
            return Err(Status::invalid_argument(NO_SOURCE_IDENTIFIER));
        }
        if payload.is_empty() {
            return Err(Status::invalid_argument(NO_EVENT_DATA_PROVIDED));
        }

        // // Handle source
        let event_repo = self.context.repositories.event_repo.clone();

        let event_id = match event_repo
            .save_event(CreateEvent {
                trigger_id: Some(trigger_id.clone()),
                flow_id: None,
                name,
                context: serde_json::from_str(&payload).unwrap(),
                started_at: None,
                ended_at: None, // tags,
            })
            .await
        {
            Ok(v) => v,
            Err(e) => {
                tracing::error!("Error saving a new event {:?}", e);
                return Err(Status::internal(UNABLE_TO_SAVE_EVENT));
            }
        };
        event_repo
            .and_confirm(&event_id, self.update_tx.clone())
            .await?;

        Ok(Response::new(TriggerEventResponse {
            status: "success".into(),
            trigger_id: event_id,
        }))
    }

    async fn get_event(
        &self,
        request: Request<GetEventRequest>,
    ) -> Result<Response<GetEventResponse>, Status> {
        let req = request.into_inner();

        let event_id = req.event_id;

        let event_repo = self.context.repositories.event_repo.clone();

        let found = match event_repo.find_by_id(event_id).await {
            Ok(v) => v,
            Err(e) => {
                return Err(Status::not_found(EVENT_NOT_FOUND));
            }
        };

        Ok(Response::new(GetEventResponse {
            event: Some(found.into()),
        }))
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Once;

    use serde_json::Value;
    use sqlx::Row;
    use tonic::transport::{Channel, Uri};
    use tracing::{debug, info};

    use crate::generated::events::Event as ProtoEvent;
    use crate::internal::test_helper::{
        get_test_context, get_test_context_from_pool, get_test_pool, insert_dummy_data,
        select_all_events, TestEventRepo, TestTriggerRepo,
    };
    use crate::models::event::Event;
    use crate::{internal::test_helper::get_test_config, utils::bootstrap};

    use serde::{Deserialize, Serialize};

    use super::*;

    #[derive(Serialize, Deserialize, Debug)]
    struct TestPayload {
        file: String,
    }

    #[tokio::test]
    async fn test_event_save() -> anyhow::Result<()> {
        let pool = get_test_pool().await.unwrap();
        let context = get_test_context_from_pool(&pool).await;
        let test = TestEventRepo::new_with_pool(&context.pool);

        let event_manager = EventManager::new(&context, test.with_sender().await);
        let dummy_event = test.dummy_create_event();
        let event = test.insert_dummy_event(dummy_event.clone()).await.unwrap();
        let create_event_request = TriggerEventRequest {
            event: Some(dummy_event.clone().into()),
        };

        let request = Request::new(create_event_request);
        let response = event_manager.trigger_event(request).await;

        assert!(response.is_ok());
        let response = response.unwrap().into_inner();
        assert_eq!(response.status, "success".to_string());

        let found = context
            .repositories
            .event_repo
            .find_by_id(response.trigger_id.clone())
            .await;
        assert!(found.is_ok());
        let found = found.unwrap();
        assert_eq!(found.name, event.name);

        Ok(())
    }

    #[tokio::test]
    async fn test_event_sends_update() -> anyhow::Result<()> {
        let context = get_test_context().await;
        let test = TestEventRepo::new().await;
        let mut test_tx = test.with_sender().await;
        let mut test_rx = test.with_receiver().await;
        let event_manager = EventManager::new(&context, test_tx.clone());

        let event = test.dummy_create_event();
        let create_event_request = TriggerEventRequest {
            event: Some(event.clone().into()),
        };

        let request = Request::new(create_event_request);
        let _response = event_manager.trigger_event(request).await;

        let msg = test_rx.recv().await;
        assert!(msg.is_some());
        let msg = msg.unwrap();
        assert_eq!(msg.name, event.name);

        Ok(())
    }

    #[tokio::test]
    async fn test_save_event_triggers_callback() -> anyhow::Result<()> {
        let pool = get_test_pool().await.unwrap();
        let context = get_test_context_from_pool(&pool).await;
        let test = TestEventRepo::new_with_pool(&context.pool);

        let event_manager = EventManager::new(&context, test.with_sender().await);

        let dummy_event = test.dummy_create_event();
        let r = test
            .insert_dummy_event(test.dummy_create_event())
            .await
            .unwrap();
        test.insert_dummy_event(dummy_event).await.unwrap();

        let request = Request::new(GetEventRequest { event_id: r.id });
        let response = event_manager.get_event(request).await;
        assert!(response.is_ok());
        let response = response.unwrap().into_inner();
        assert_eq!(response.event.unwrap().name, r.name);

        Ok(())
    }
}
