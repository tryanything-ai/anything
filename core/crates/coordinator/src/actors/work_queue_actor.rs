use anything_persistence::EventRepoImpl;
use ractor::{async_trait, Actor, ActorProcessingErr, ActorRef};

// Messages for Work Queue Actor
#[derive(Debug, Clone)]
pub enum WorkQueueActorMessage {
    StartWorkQueue,
    WorkCompleted(String),
}

pub struct WorkQueueActorState {
    pub processing: bool,
    pub event_repo: EventRepoImpl,
    pub current_event_id: Option<String>,
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
        state: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        match message {
            WorkQueueActorMessage::StartWorkQueue => {
                if !state.processing {
                    state.processing = true;
                    // Implementation for starting the work queue goes here
                    tracing::debug!("Hinting To Start Work Queue");
                    println!("println: Hinting To Start Work Queue");
                } else {
                    tracing::debug!("Already processing work");
                    println!("println: Already processing work");
                }
            }
            WorkQueueActorMessage::WorkCompleted(work_id) => {
                // Implementation for handling work completion goes here
                tracing::debug!("Work Complete? {} ", work_id);
                println!("println: Work Complete? {}", work_id);
                state.processing = false; // Reset the processing flag after work completion
            }
        }
        Ok(())
    }
}

impl WorkQueueActor {
    async fn get_started(
        &self,
        flow: anything_graph::Flow,
        state: &<WorkQueueActor as Actor>::State,
    ) -> Result<(), ActorProcessingErr> {
        Ok(())
    }
}
