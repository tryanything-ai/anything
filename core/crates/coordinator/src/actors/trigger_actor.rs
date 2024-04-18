use anything_persistence::{FlowRepo, FlowRepoImpl};
use chrono::{DateTime, Utc};
use cron::Schedule; //will need to parse cron syntax
use serde_json::Value;
use std::str::FromStr;
use std::time::Duration;
// use anything_store::{types::ChangeMessage, FileStore};
use crate::{CoordinatorActorResult, CoordinatorError, CoordinatorResult};
use ractor::{async_trait, Actor, ActorProcessingErr, ActorRef};
use tokio::time::sleep;

use std::sync::Arc;
use std::sync::Mutex;

use super::work_queue_actor::WorkQueueActor;
use super::work_queue_actor::WorkQueueActorMessage;

#[derive(Debug, Clone)]
pub enum TriggerMessage {
    HydrateTriggers,
}

pub struct TriggerActor;
pub struct TriggerActorState {
    pub flow_repo: FlowRepoImpl,
    pub triggers: Arc<Mutex<Vec<Trigger>>>,
    // pub config: AnythingConfig,
    pub work_queue_actor: ActorRef<WorkQueueActorMessage>,
}

#[derive(Debug, Clone)]
pub struct Trigger {
    pub trigger_id: String,                //type of trigger essentially
    pub flow_id: String,                   //flow id
    pub flow_version_id: String,           // Some identifier for the task related to the trigger
    pub config: Value,                     // Store the trigger configuration
    pub last_fired: Option<DateTime<Utc>>, //data so we know when it was last fired
    pub next_fire: Option<DateTime<Utc>>,  //data so we know when it will fire next
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
                self.hydrate_triggers(myself_clone, message, state).await?;
            }
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
        let trigger_data = state.flow_repo.get_flow_triggers().await.map_err(|e| {
            tracing::error!("Failed to retrieve triggers: {:#?}", e);
            CoordinatorError::PersistenceError(e)
        })?;

        println!("triggger_data in trigger_actor: {:#?}", trigger_data);

        //TODO: add triggers to vector
        for trigger_info in trigger_data {
            //destructure trigger_info into what is usefull for us.
            let parsed_argument: serde_json::Value = serde_json::from_str(&trigger_info.2)
                .map_err(|e| {
                    tracing::error!("Failed to parse trigger argument: {:#?}", e);
                    CoordinatorError::ParsingError(e.to_string())
                })?;

            // println!("Parsed Argument: {:?}", parsed_argument);

            let prospect_new_trigger = Trigger {
                trigger_id: parsed_argument["node_name"]
                    .to_string()
                    .trim_matches('"')
                    .to_string(),
                flow_id: trigger_info.1,
                flow_version_id: trigger_info.0,
                config: serde_json::Value::from(parsed_argument["config"].clone()),
                last_fired: None,
                next_fire: None,
            };

            // println!("Prospect Trigger: {:?}", prospect_new_trigger);

            if state
                .triggers
                .lock()
                .unwrap()
                .iter()
                .any(|trigger| trigger.flow_version_id == prospect_new_trigger.flow_version_id)
            {
                //trigger already exists. leave it in state or else it messes up our maintenance of "lastTriggered"
                continue;
            }

            //add new trigger to state
            state.triggers.lock().unwrap().push(prospect_new_trigger);
        }

        self.start_cron_checker(state).await;
        Ok(())
    }

    pub async fn start_cron_checker(&self, state: &mut <TriggerActor as Actor>::State) {
        let triggers = state.triggers.clone();
        let work_queue_actor = state.work_queue_actor.clone(); // Clone the actor reference

        tokio::task::spawn_blocking(move || {
            let runtime = tokio::runtime::Runtime::new().unwrap();
            runtime.block_on(async {
                loop {
                    let now = Utc::now();
                    let mut triggers = triggers.lock().unwrap();
                    println!("Cron every minute hit!");

                    for trigger in triggers.iter_mut() {
                        // Your updated trigger logic here
                        println!("Checking triggers every minute: {:?}", trigger);

                        // for trigger in triggers.iter_mut() {
                        if trigger.trigger_id == "cron_trigger" {
                            println!("Handling cron trigger {:?}", trigger);
                            match Schedule::from_str(&trigger.config["pattern"].as_str().unwrap()) {
                                Ok(schedule) => {
                                    let next = schedule.upcoming(Utc).next().unwrap();
                                    if let Some(next_fire) = trigger.next_fire {
                                        if now >= next_fire
                                            && (trigger.last_fired.is_none()
                                                || trigger.last_fired.unwrap() < next_fire)
                                        {
                                            println!(
                                                "Firing trigger for: {:?}",
                                                trigger.flow_version_id
                                            );

                                            // Asynchronously send a message to the WorkQueueActor
                                            let actor_ref = work_queue_actor.clone();
                                            let flow_id = trigger.flow_id.clone();
                                            let flow_version_id = trigger.flow_version_id.clone();

                                            tokio::spawn(async move {
                                                actor_ref
                                                    .send_message(
                                                        WorkQueueActorMessage::ExecuteFlow {
                                                            flow_id,
                                                            flow_version_id,
                                                            session_id: None,
                                                            stage: None,
                                                        },
                                                    )
                                                    .expect("Failed to send ExecuteFlow message");
                                            });

                                            // let work_queue_actor = state.work_queue_actor.clone();
                                            // let flow_id = trigger.flow_id.clone();
                                            // let flow_version_id = trigger.flow_version_id.clone();

                                            // Here we just call send_message without await
                                            // let send_result = work_queue_actor.send_message(
                                            //     WorkQueueActorMessage::ExecuteFlow {
                                            //         flow_id,
                                            //         flow_version_id,
                                            //         session_id: None,
                                            //         stage: None,
                                            //     },
                                            // );

                                            // match send_result {
                                            //     Ok(_) => println!("Message sent successfully"),
                                            //     Err(e) => {
                                            //         println!("Failed to send message: {:?}", e)
                                            //     }
                                            // }
                                            // let send_result = work_queue_actor.send_message(
                                            //     WorkQueueActorMessage::ExecuteFlow {
                                            //         flow_id: trigger.flow_id.clone(),
                                            //         flow_version_id: trigger
                                            //             .flow_version_id
                                            //             .clone(),
                                            //         session_id: None,
                                            //         stage: None,
                                            //     },
                                            // );

                                            // match send_result {
                                            //     Ok(_) => println!("Message sent successfully"),
                                            //     Err(e) => {
                                            //         println!("Failed to send message: {:?}", e)
                                            //     }
                                            // }
                                            // }

                                            // tokio::spawn(async move {
                                            //     work_queue_actor.send_message(WorkQueueActorMessage::ExecuteFlow {
                                            //         flow_id,
                                            //         flow_version_id,
                                            //         session_id: None,
                                            //         stage: None,
                                            //     }).await.expect("Failed to send ExecuteFlow message");
                                            // });
                                            //TODO: fire event
                                            // state
                                            //     .work_queue_actor
                                            //     .send_message(WorkQueueActorMessage::ExecuteFlow {
                                            //         flow_id: trigger.flow_id.clone(),
                                            //         flow_version_id: trigger.flow_version_id.clone(),
                                            //         session_id: None,
                                            //         stage: None,
                                            //     });
                                        }
                                    }
                                    trigger.next_fire = Some(next); // Update the next fire time
                                    println!("Scheduled next fire time: {:?}", next);
                                }
                                Err(e) => {
                                    println!(
                                        "Invalid cron expression for trigger {}: {:?}",
                                        trigger.flow_version_id, e
                                    );
                                    // Handle invalid cron expressions, e.g., log error, send alert, etc.
                                }
                            }
                        } else {
                            println!("Doing nothing. we only handle cron triggers for now.")
                        }
                    }
                    drop(triggers); // Release the lock before the sleep
                    sleep(Duration::from_secs(20)).await; // Check every minute
                }
            });
        });
    }
}