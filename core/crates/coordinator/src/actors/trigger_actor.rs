use anything_persistence::{FlowRepo, FlowRepoImpl};
use chrono::{DateTime, Utc};
use cron::Schedule;
use std::str::FromStr;
use std::time::Duration;
// use anything_store::{types::ChangeMessage, FileStore};
use crate::{CoordinatorActorResult, CoordinatorError, CoordinatorResult};
use ractor::{async_trait, Actor, ActorProcessingErr, ActorRef};
use tokio::time::sleep;

use std::sync::Arc;
use std::sync::Mutex;

#[derive(Debug, Clone)]
pub enum TriggerMessage {
    HydrateTriggers,
}

pub struct TriggerActor;
pub struct TriggerActorState {
    pub flow_repo: FlowRepoImpl,
    // pub triggers: Vec<Trigger>,
    pub triggers: Arc<Mutex<Vec<Trigger>>>,
}

// Define a Trigger structure if it doesn't exist yet
#[derive(Debug, Clone)]
pub struct Trigger {
    pub cron_expression: String, // Store the cron job expression
    pub task_identifier: String, // Some identifier for the task related to the trigger
    pub last_fired: Option<DateTime<Utc>>,
}

//Actor that watches flow files and updates state in system when they change
#[async_trait]
impl Actor for TriggerActor {
    type Msg = TriggerMessage;
    type State = TriggerActorState;
    type Arguments = TriggerActorState;

    async fn pre_start(
        &self,
        myself: ActorRef<Self::Msg>,
        args: Self::Arguments,
    ) -> Result<Self::State, ActorProcessingErr> {
        myself.send_message(TriggerMessage::HydrateTriggers);
        Ok(args)
    }

    async fn handle(
        &self,
        myself: ActorRef<Self::Msg>,
        message: Self::Msg,
        state: &mut Self::State,
    ) -> CoordinatorActorResult<()> {
        match message {
            TriggerMessage::HydrateTriggers => {
                tracing::debug!("Hydrating Triggers from FlowVersions Table");
                // Load and rehydrate triggers
                let myself_clone = myself.clone();
                // let myself_clone_2 = myself.clone();
                self.hydrate_triggers(myself_clone, message, state).await?;
                // self.hydrate_triggers().await;
                // Start cron checker after triggers are hydrated
                // myself.send_message(TriggerMessage::CheckCron).await;

                //this is angry
                // tokio::spawn(async move {
                //     self.start_cron_checker(myself_clone_2, state).await;
                // });
            } // Future handling for other trigger types could be added here
        }
        Ok(())
    }
}

impl TriggerActor {
    pub async fn hydrate_triggers(
        &self,
        _myself: ActorRef<<TriggerActor as Actor>::Msg>,
        _message: <TriggerActor as Actor>::Msg,
        state: &mut <TriggerActor as Actor>::State,
    ) -> CoordinatorResult<()> {
        let _triggers = state.flow_repo.get_flow_triggers().await.map_err(|e| {
            tracing::error!("Failed to retrieve triggers: {:#?}", e);
            CoordinatorError::PersistenceError(e)
        })?;

        //TODO: add triggers to vector
        

        self.start_cron_checker(state).await;
        Ok(())
    }

    // pub async fn start_cron_checker(&self, state: &mut <TriggerActor as Actor>::State) {
    //     let triggers = state.triggers.clone();
    //     // this gets mad find a better solution
    //     // let state_clone = (*state).clone();
    //     tokio::task::spawn_blocking(async move {
    //         loop {
    //             let now = Utc::now();
    //             let triggers = triggers.lock().unwrap();
    //             for trigger in triggers.iter() {
    //                 // Your updated trigger logic here
    //             }
    //             sleep(Duration::from_secs(60)).await; // Check every minute
    //         }
    //     });
    // }

    // pub async fn start_cron_checker(&self, state: &mut <TriggerActor as Actor>::State) {
    //     let triggers = state.triggers.clone();
    //     tokio::spawn(async move {
    //         loop {
    //             let now = Utc::now();
    //             let triggers = triggers.lock().unwrap();
    //             for trigger in triggers.iter() {
    //                 // Your updated trigger logic here
    //             }
    //             drop(triggers); // Release the lock before the sleep
    //             sleep(Duration::from_secs(60)).await; // Check every minute
    //         }
    //     });
    // }
    pub async fn start_cron_checker(&self, state: &mut <TriggerActor as Actor>::State) {
        let triggers = state.triggers.clone();
        tokio::task::spawn_blocking(move || {
            let runtime = tokio::runtime::Runtime::new().unwrap();
            runtime.block_on(async {
                loop {
                    let now = Utc::now();
                    let triggers = triggers.lock().unwrap();
                    println!("Cron every minute hit!");
                    for trigger in triggers.iter() {
                        // Your updated trigger logic here
                    }
                    drop(triggers); // Release the lock before the sleep
                    tokio::time::sleep(Duration::from_secs(60)).await; // Check every minute
                }
            });
        });
    }
}
