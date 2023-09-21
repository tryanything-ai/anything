#![allow(unused)]
use std::sync::Arc;

use postage::dispatch::Sender;
use tonic::{Request, Response, Status};

use crate::{
    generated::{triggers_server::Triggers, CreateTriggerRequest, CreateTriggerResponse},
    repositories::trigger_repo::TriggerRepo,
    Context, CreateTrigger, Trigger,
};

#[derive(Debug)]
pub struct TriggersManager {
    context: Arc<Context>,
    update_tx: Sender<Trigger>,
}

impl TriggersManager {
    pub fn new(context: &Context, update_tx: Sender<Trigger>) -> Self {
        Self {
            context: Arc::new(context.clone()),
            update_tx,
        }
    }
}

#[tonic::async_trait]
impl Triggers for TriggersManager {
    async fn create_trigger(
        &self,
        request: Request<CreateTriggerRequest>,
    ) -> Result<Response<CreateTriggerResponse>, Status> {
        let resp = request.into_inner();

        let event_name = resp.event_name;
        let payload = serde_json::from_str(&resp.payload).map_err(|e| {
            tracing::error!("Failed to parse payload: {}", e.to_string());
            Status::invalid_argument(format!("Failed to parse payload: {}", e.to_string()))
        })?;
        let metadata = match resp.metadata {
            Some(m) => Some(serde_json::from_str(&m).map_err(|e| {
                tracing::error!("Failed to parse metadata: {}", e.to_string());
                Status::invalid_argument(format!("Failed to parse metadata: {}", e.to_string()))
            })?),
            None => None,
        };

        let create_trigger = CreateTrigger::new(event_name, payload, metadata);

        let trigger_id = match self
            .context
            .repositories
            .trigger_repo
            .create_trigger(create_trigger.into())
            .await
        {
            Err(e) => {
                tracing::error!("Failed to create trigger: {}", e.to_string());
                return Err(Status::internal(format!(
                    "Failed to create trigger: {}",
                    e.to_string()
                )));
            }
            Ok(id) => id,
        };

        Ok(Response::new(CreateTriggerResponse {
            trigger_id: trigger_id.to_string(),
        }))
    }
}

#[cfg(test)]
mod tests {
    use crate::internal::test_helper::{
        get_test_context_from_pool, get_test_pool, TestTriggerRepo,
    };

    use super::*;
    use anyhow::Result;

    #[tokio::test]
    async fn test_trigger_is_created() -> Result<()> {
        let pool = get_test_pool().await.unwrap();
        let context = get_test_context_from_pool(&pool).await;
        let test = TestTriggerRepo::new_with_pool(&context.pool);

        let dummy_trigger = test.dummy_create_trigger();

        let req = Request::new(CreateTriggerRequest {
            event_name: dummy_trigger.event_name,
            payload: dummy_trigger.payload.clone().to_string(),
            metadata: None,
        });
        let trigger_manager = TriggersManager::new(&context, test.with_sender().await);
        let resp = trigger_manager.create_trigger(req).await;
        assert!(resp.is_ok());
        let resp = resp.unwrap();
        let response = resp.into_inner();

        let trigger_id = response.trigger_id;

        let found_trigger = test.select_trigger_by_id(trigger_id.clone()).await?;

        assert_eq!(dummy_trigger.payload, found_trigger.payload);

        Ok(())
    }
}
