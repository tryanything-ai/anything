use anything_persistence::{EventRepo, EventRepoImpl};
use anything_runtime::PluginManager;
// use anything_runtime::Runner;

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
    // pub runner: Runner,
    pub plugin_manager: PluginManager,
    // pub current_event_id: Option<String>,
    // pub current_session_id: Option<String>,
    // pub current_trigger_session_id: Option<String>,
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
        myself: ActorRef<Self::Msg>,
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
                    self.process_next(state, myself).await?;
                } else {
                    tracing::debug!("Already processing work");
                    println!("println: Already processing work");
                }
            }
            WorkQueueActorMessage::WorkCompleted(event_id) => {
                // Implementation for handling work completion goes here
                tracing::debug!("Work Complete? {} ", event_id);
                println!("println: Work Complete? {}", event_id);
                // state.processing = false; // Reset the processing flag after work completion
                self.event_processed(event_id, state, myself).await?;
            }
        }
        Ok(())
    }
}

impl WorkQueueActor {
    pub async fn process_next(
        &self,
        state: &mut <WorkQueueActor as Actor>::State,
        myself: ActorRef<WorkQueueActorMessage>, // Add the myself parameter
    ) -> Result<(), ActorProcessingErr> {
        println!("Processing Next Event");

        //Query DB for an event that is pending and old ( or whatver we think should be done next)
        let event = state.event_repo.get_oldest_waiting_event().await?;

        println!("Event found to PROCESS yes? {:?}", event);
        if let Some(event) = event {
            //// Update Database Processing State
            state
                .event_repo
                .mark_event_as_processing(event.event_id.clone())
                .await?;

            println!("Event found to PROCESS {} ", event.event_id);

            //TODO: Bundle Context fro Transaction
            //TODO: SEND event to Engine for Processing
            //TODO: use runner to run the event
            // state.runner.execute(stage_name, execution_config)
            //engine will send event that its done to work queue actor we will mock that here for now
            let _ =
                myself.send_message(WorkQueueActorMessage::WorkCompleted(event.event_id.clone()));
        //update state for curernt_event_id etc
        //
        } else {
            //we beleive we are done processing all events
            state.processing = false;
            // Handle the case when event is None
            println!("println: No event found to mark as PROCESSING");
        }

        Ok(())
    }

    pub async fn event_processed(
        &self,
        event_id: String,
        state: &mut <WorkQueueActor as Actor>::State,
        myself: ActorRef<WorkQueueActorMessage>, // Add the myself parameter
    ) -> Result<(), ActorProcessingErr> {
        println!("Event Processed");
        //Update db on event completion //TODO: need to write result here and debug result and anything else like that
        let _event = state.event_repo.mark_event_as_complete(event_id).await?;
        //Let work queue know to start next event
        // let _ = myself.send_message(WorkQueueActorMessage::StartWorkQueue);
        self.process_next(state, myself).await?;

        Ok(())
    }

    // async fn bundle_context_for_transaction(
    //     &self,
    //     event_id: String,
    //     state: &mut <WorkQueueActor as Actor>::State,
    // ) -> Result<(), ActorProcessingErr> {
    //     Ok(())
    // }
}
