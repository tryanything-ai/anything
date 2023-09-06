use std::pin::Pin;

use serde_json::Value as JsonValue;
use sqlx::AnyPool;
use tokio_stream::{wrappers::ReceiverStream, Stream, StreamExt};
use tonic::{Request, Response, Status, Streaming};

use crate::{
    event::event::Event,
    pb::{
        event_action_server::EventAction, AppendToStreamRequest, AppendToStreamResponse,
        ReadStreamRequest, ReadStreamResponse,
    },
};

pub struct EventService {
    pool: AnyPool,
}

impl EventService {
    pub fn new(pool: AnyPool) -> Self {
        Self { pool }
    }
}

type ServiceResponse<T> = Result<Response<T>, Status>;
type ResponseStream = Pin<Box<dyn Stream<Item = Result<ReadStreamResponse, Status>> + Send>>;

#[tonic::async_trait]
impl EventAction for EventService {
    type ReadStreamStream = ResponseStream;

    async fn append_to_stream(
        &self,
        req: Request<AppendToStreamRequest>,
    ) -> ServiceResponse<AppendToStreamResponse> {
        let req = req.into_inner();

        let mut events: Vec<Event> = vec![];
        let stream_name = req.stream_name;
        for event in req.events {
            let id = event.id;
            let payload = JsonValue::from(event.payload);
            let tags = event.tags;

            let mut new_event = Event::new(stream_name.clone(), payload, tags);
            if let Some(id) = id {
                if let Ok(i64_id) = id.parse::<i64>() {
                    new_event = new_event.with_id(i64_id);
                }
            }
            events.push(new_event.with_name(stream_name.clone()));
        }
        // let stream = Event::new();
        Err(Status::new(tonic::Code::Internal, "Invalid for now"))
    }

    async fn read_stream(
        &self,
        _req: Request<Streaming<ReadStreamRequest>>,
    ) -> ServiceResponse<Self::ReadStreamStream> {
        Err(Status::new(tonic::Code::Internal, "Invalid for now"))
    }
}
