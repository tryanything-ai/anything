#![allow(unused)]
// Temporary
use std::sync::Arc;

use chrono::Utc;
use serde_json::json;
use sqlx::{Pool, SqlitePool};
use tonic::{Request, Response, Status};

use crate::{
    context::Context,
    errors::EventsError,
    events::{events_server::Events, TriggerEventRequest, TriggerEventResponse},
    models::event::CreateEvent,
    repositories::event_repo::EventRepo,
};

// --------------------------------------------------
// Errors
// --------------------------------------------------
const NO_EVENT: &str = "No event provided";
const NO_SOURCE_IDENTIFIER: &str = "No source identifier provided";
const NO_EVENT_DETAILS: &str = "No event details provided";
const NO_EVENT_NAME_PROVIDED: &str = "No event name provided";
const NO_EVENT_DATA_PROVIDED: &str = "No event data provided";
const UNABLE_TO_SAVE_EVENT: &str = "Unable to save event";

// --------------------------------------------------
// Event Server impl
// --------------------------------------------------

#[derive(Debug)]
pub struct EventManager {
    context: Arc<Context>,
}

impl EventManager {
    pub fn new(context: Context) -> Self {
        Self {
            context: Arc::new(context),
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

        Ok(Response::new(TriggerEventResponse {
            status: "success".into(),
            event_id,
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn event_save() -> anyhow::Result<()> {
        Ok(())
    }
}
