use anything_common::AnythingConfig;
use arraydeque::ArrayDeque;
use ractor::{async_trait, Actor, ActorRef, RpcReplyPort};

use crate::{processing::processor::ProcessorMessage, CoordinatorActorResult};

#[derive(Debug)]
pub enum UpdateActorMessage {
    FlowLifecycle(ProcessorMessage),
    GetLatestProcessorMessages(RpcReplyPort<Vec<ProcessorMessage>>),
}

pub struct UpdateActorState {
    pub config: AnythingConfig,
    pub latest_messages: ArrayDeque<ProcessorMessage, 32>,
}
pub struct UpdateActor;

#[async_trait]
impl Actor for UpdateActor {
    type Msg = UpdateActorMessage;
    type State = UpdateActorState;
    type Arguments = UpdateActorState;

    async fn pre_start(
        &self,
        _myself: ActorRef<Self::Msg>,
        args: Self::Arguments,
    ) -> CoordinatorActorResult<Self::State> {
        Ok(args)
    }

    async fn handle(
        &self,
        _myself: ActorRef<Self::Msg>,
        message: Self::Msg,
        state: &mut Self::State,
    ) -> CoordinatorActorResult<()> {
        match message {
            UpdateActorMessage::GetLatestProcessorMessages(reply) => {
                // self.handle_get_latest_messages(state).await?;
                let latest_messages = state
                    .latest_messages
                    .iter()
                    .cloned()
                    .collect::<Vec<ProcessorMessage>>();
                let _ = reply.send(latest_messages);
            }
            UpdateActorMessage::FlowLifecycle(msg) => {
                let latest_messages = &mut state.latest_messages;
                let _ = latest_messages.push_back(msg);
            }
        }
        Ok(())
    }
}
