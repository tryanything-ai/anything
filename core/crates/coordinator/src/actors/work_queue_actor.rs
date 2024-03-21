use anything_persistence::EventRepoImpl;
use ractor::{async_trait, Actor, ActorProcessingErr, ActorRef};

// Messages for Work Queue Actor
#[derive(Debug, Clone)]
pub enum WorkQueueActorMessage {
    StartWorkQueue,
    WorkCompleted(String),
}

pub struct WorkQueueActorState {
    pub event_repo: EventRepoImpl,
}

pub struct WorkQueueActor;

#[async_trait]
impl Actor for WorkQueueActor {
    type Msg = WorkQueueActorMessage;
    type State = WorkQueueActorState;
    type Arguments = WorkQueueActorState;

    async fn pre_start(
        &self,
        _myself: ActorRef<Self::Msg>,
        args: Self::Arguments,
    ) -> Result<Self::State, ActorProcessingErr> {
        Ok(args)
    }

    async fn handle(
        &self,
        _myself: ActorRef<Self::Msg>,
        message: Self::Msg,
        _state: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        match message {
            WorkQueueActorMessage::StartWorkQueue => {
                // Implementation for starting the work queue goes here
                tracing::debug!("Hinting To Start Work Queue");
                println!("println: Hinting To Start Work Queue");
            }
            WorkQueueActorMessage::WorkCompleted(work_id) => {
                // Implementation for handling work completion goes here
                tracing::debug!("Work Complete? {} ", work_id);
                println!("println: Work Complete? {}", work_id);
            }
        }
        Ok(())
    }
}
