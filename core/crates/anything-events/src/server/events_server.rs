#![allow(unused)]
// Temporary
use std::sync::Arc;

use chrono::Utc;
use crossbeam::channel::Sender;
use serde_json::json;
use sqlx::{Pool, Row, SqlitePool};
use tonic::{Request, Response, Status};

use crate::{
    context::Context,
    errors::EventsError,
    events::{
        events_server::Events, EventIdentifier, GetEventRequest, GetEventResponse,
        TriggerEventRequest, TriggerEventResponse,
    },
    models::event::{CreateEvent, Event},
    repositories::event_repo::EventRepo,
};

use crate::events::Event as ProtoEvent;

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
impl Events for EventManager {
    async fn trigger_event(
        &self,
        request: Request<TriggerEventRequest>,
    ) -> Result<Response<TriggerEventResponse>, Status> {
        let req = request.into_inner();

        let event = match req.event {
            Some(e) => e,
            None => return Err(Status::invalid_argument(NO_EVENT)),
        };

        // Handle source
        let source_id = match event.identifier.as_ref() {
            Some(source) => source.source_id.to_owned(),
            None => return Err(Status::invalid_argument(NO_SOURCE_IDENTIFIER)),
        };

        let event_details = match event.details.as_ref() {
            None => return Err(Status::invalid_argument(NO_EVENT_DETAILS)),
            Some(details) => details.to_owned(),
        };

        let event_name = event_details.name.to_owned();
        let payload = event_details.payload.to_owned();
        let metadata = match event_details.metadata {
            None => String::default(),
            Some(v) => v.to_owned(),
        };
        let event_tags = event_details.tags.to_owned();

        let event_repo = self.context.repositories.event_repo.clone();

        let event_id = match event_repo
            .save_event(CreateEvent {
                event_name,
                source_id,
                payload: json!(payload),
                metadata: json!(metadata),
            })
            .await
        {
            Ok(v) => v,
            Err(_e) => return Err(Status::internal(UNABLE_TO_SAVE_EVENT)),
        };

        let event = match self
            .context
            .repositories
            .event_repo
            .find_by_id(event_id)
            .await
        {
            Ok(r) => {
                self.update_tx.send(r);
            }
            Err(_) => {
                // Something should be done here... maybe?
                return Err(Status::internal("unable to save event"));
            }
        };

        Ok(Response::new(TriggerEventResponse {
            status: "success".into(),
            event_id,
        }))
    }

    async fn get_event(
        &self,
        request: Request<GetEventRequest>,
    ) -> Result<Response<GetEventResponse>, Status> {
        let req = request.into_inner();

        let event_id = match req.id {
            Some(e) => e.id,
            None => return Err(Status::invalid_argument(NO_EVENT)),
        };

        let event_repo = self.context.repositories.event_repo.clone();

        let found = match event_repo.find_by_id(event_id).await {
            Ok(v) => v,
            Err(e) => {
                println!("ERROR => {:?}", e);
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

    use crate::events::{Event as ProtoEvent, EventDetails, SourceIdentifier};
    use crate::internal::test_helper::{
        get_test_context, get_test_context_from_pool, get_test_pool, insert_dummy_data,
        TestEventRepo,
    };
    use crate::models::event::Event;
    use crate::{internal::test_helper::get_test_config, utils::bootstrap};

    use crate::events::{events_client::EventsClient, events_server::EventsServer};
    use serde::{Deserialize, Serialize};

    use super::*;

    #[derive(Serialize, Deserialize, Debug)]
    struct TestPayload {
        file: String,
    }

    #[tokio::test]
    async fn event_save() -> anyhow::Result<()> {
        let pool = get_test_pool().await.unwrap();
        let context = get_test_context_from_pool(&pool).await;
        let test = TestEventRepo::new_with_pool(&context.pool);

        let event_manager = EventManager::new(&context, test.with_sender().await);
        let event = test.dummy_create_event();
        let create_event_request = TriggerEventRequest {
            event: Some(event.clone().into()),
        };

        let request = Request::new(create_event_request);
        let response = event_manager.trigger_event(request).await;

        assert!(response.is_ok());
        let response = response.unwrap().into_inner();
        assert_eq!(response.status, "success".to_string());

        let found = context
            .repositories
            .event_repo
            .find_by_id(response.event_id.clone())
            .await;
        assert!(found.is_ok());
        let found = found.unwrap();
        assert_eq!(found.event_name, event.event_name);

        Ok(())
    }

    #[tokio::test]
    async fn test_event_sends_update() -> anyhow::Result<()> {
        let context = get_test_context().await;
        let test = TestEventRepo::new().await;
        let test_tx = test.with_sender().await;
        let test_rx = test.with_receiver().await;
        let event_manager = EventManager::new(&context, test_tx);

        let event = test.dummy_create_event();
        let create_event_request = TriggerEventRequest {
            event: Some(event.clone().into()),
        };

        let request = Request::new(create_event_request);
        let _response = event_manager.trigger_event(request).await;

        let msg = test_rx.recv();
        assert!(msg.is_ok());
        let msg = msg.unwrap();
        assert_eq!(msg.event_name, event.event_name);

        Ok(())
    }

    #[tokio::test]
    async fn event_get() -> anyhow::Result<()> {
        let context = get_test_context().await;
        let test = TestEventRepo::new().await;

        let event_manager = EventManager::new(&context, test.with_sender().await);
        let p = &event_manager.context.repositories.event_repo.pool;
        let r = insert_dummy_data(&p).await.unwrap();
        test.insert_dummy_event().await;
        test.insert_dummy_event().await.unwrap();

        let request = Request::new(GetEventRequest {
            id: Some(EventIdentifier { id: 1 }),
        });
        let response = event_manager.get_event(request).await;
        assert!(response.is_ok());
        let response = response.unwrap().into_inner();
        // assert_eq!(response.event)

        Ok(())
    }
}

/*
let name = "test-event".to_string();
        let payload = serde_json::json!(TestPayload {
            file: "/tmp/hello".to_string(),
        })
        .to_string();

        let event = Event {
            identifier: Some(SourceIdentifier { source_id: 1 }),
            details: Some(EventDetails {
                payload,
                name: name.clone(),
                tags: Vec::default(),
                metadata: None,
            }),
        };
         */
