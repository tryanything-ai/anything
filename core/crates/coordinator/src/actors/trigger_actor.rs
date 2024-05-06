use anything_persistence::{FlowRepo, FlowRepoImpl};
use chrono::{DateTime, Utc};
use cron::Schedule; //will need to parse cron syntax
use serde_json::Value;
use std::str::FromStr;
use std::time::Duration;

use crate::{CoordinatorActorResult, CoordinatorError, CoordinatorResult};
use ractor::{async_trait, Actor, ActorProcessingErr, ActorRef};
use tokio::time::sleep;

use std::sync::Arc;
use std::sync::Mutex;

use super::work_queue_actor::WorkQueueActorMessage;

#[derive(Debug, Clone)]
pub enum TriggerMessage {
    HydrateTriggers,
}

pub struct TriggerActor;
pub struct TriggerActorState {
    pub flow_repo: FlowRepoImpl,
    pub triggers: Arc<Mutex<Vec<Trigger>>>,
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
        println!("Hydrating Triggers");

        let trigger_data = state.flow_repo.get_flow_triggers().await.map_err(|e| {
            tracing::error!("Failed to retrieve triggers: {:#?}", e);
            CoordinatorError::PersistenceError(e)
        })?;

        // println!("triggger_data in trigger_actor: {:#?}", trigger_data);

        //creates scoep for existing trigger mutex
        {
            let mut existing_triggers = state.triggers.lock().unwrap();
            let mut updated_triggers = Vec::new();

            //Remove triggers that are no longer present in the trigger_data and update triggers that are present
            for existing_trigger in &*existing_triggers {
                if let Some(trigger_info) = trigger_data.iter().find(|&trigger_info| {
                    // let parsed_argument =
                    //     serde_json::from_str::<serde_json::Value>(&trigger_info.2).ok();
                    // parsed_argument.map_or(false, |parsed_argument| {
                    let flow_id = trigger_info.1.clone();
                    let flow_version_id = trigger_info.0.clone();
                    flow_id == existing_trigger.flow_id
                        && flow_version_id == existing_trigger.flow_version_id
                    // })
                }) {
                    let parsed_argument =
                        serde_json::from_str::<serde_json::Value>(&trigger_info.2)
                            .expect("Valid JSON expected");
                    let updated_trigger = Trigger {
                        trigger_id: existing_trigger.trigger_id.clone(), // retain the existing trigger_id
                        flow_id: existing_trigger.flow_id.clone(),
                        flow_version_id: existing_trigger.flow_version_id.clone(),
                        config: serde_json::Value::from(parsed_argument["config"].clone()), // update config
                        last_fired: existing_trigger.last_fired,
                        next_fire: existing_trigger.next_fire,
                    };
                    updated_triggers.push(updated_trigger);
                }
            }

            // Replace old triggers with the updated list
            *existing_triggers = updated_triggers;

            // Add new triggers
            // Now, handle adding new triggers that do not exist already
            for trigger_info in trigger_data {
                let parsed_argument = serde_json::from_str::<serde_json::Value>(&trigger_info.2)
                    .map_err(|e| {
                        tracing::error!("Failed to parse trigger argument: {:#?}", e);
                        CoordinatorError::ParsingError(e.to_string())
                    })?;

                let new_trigger = Trigger {
                    trigger_id: parsed_argument["node_name"]
                        .to_string()
                        .trim_matches('"')
                        .to_string(),
                    flow_id: trigger_info.1.clone(),
                    flow_version_id: trigger_info.0.clone(),
                    config: serde_json::Value::from(parsed_argument["config"].clone()),
                    last_fired: None,
                    next_fire: None,
                };

                let trigger_exists = existing_triggers.iter().any(|t| {
                    t.flow_id == new_trigger.flow_id
                        && t.flow_version_id == new_trigger.flow_version_id
                });
                if !trigger_exists {
                    existing_triggers.push(new_trigger);
                }
            }
        }
        //end of scope for existing trigger mutex
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
