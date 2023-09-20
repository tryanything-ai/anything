use std::sync::Arc;

use postage::dispatch::Sender;
use tonic::{Request, Response, Status};

use crate::{
    generated::{triggers_server::Triggers, CreateTriggerRequest, CreateTriggerResponse},
    Context, Trigger,
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
        Err(Status::unimplemented("not yet implemented"))
    }
}
