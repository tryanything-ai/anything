use anything_common::AnythingConfig;
use anything_persistence::{EventRepo, EventRepoImpl, StoreEvent};
use anything_runtime::{ExecuteConfigBuilder, PluginManager, Scope};
use anything_store::FileStore;
use serde_json::Value;
use std::collections::HashMap;

use std::fs::File;
use std::io::{self, BufRead, BufReader};
use tera::{Context, Tera};

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
    pub plugin_manager: PluginManager,
    pub file_store: FileStore,
    pub anything_config: AnythingConfig,
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

            let event_id = event.event_id.clone();
            let engine_id = event.engine_id.clone();

            //BUndle .env and results from previous actions in the context
            let bundled_context = self
                .create_bundled_context(event, &state.anything_config.clone(), state)
                .await;

            //store the context on the event object
            state
                .event_repo
                .store_event_context(event_id.clone(), bundled_context.clone())
                .await?;

            //we don't do this with triggers
            if engine_id == "trigger" {
                println!("Not running action. Event is a trigger.");
                let _ = myself.send_message(WorkQueueActorMessage::WorkCompleted(event_id));
            } else {
                let engine = state.plugin_manager.get_plugin(&engine_id).unwrap();

                let config = ExecuteConfigBuilder::default()
                    .plugin_name(engine_id)
                    .runtime("bash")
                    .context(bundled_context)
                    .build()
                    .unwrap();

                let result = engine.execute(&Scope::default(), &config);
                //TODO: store the result in the db or the error
                match result {
                    Ok(execution_result) => {
                        state
                            .event_repo
                            .store_execution_result(event_id.clone(), execution_result.result)
                            .await?;
                    }
                    Err(e) => {
                        println!("Error occurred while executing the engine: {:?}", e);
                    }
                }

                //TODO: a mountain of error handling and passing that into the db

                let _ = myself.send_message(WorkQueueActorMessage::WorkCompleted(event_id));
            }
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

    async fn create_bundled_context(
        &self,
        event: StoreEvent,
        config: &AnythingConfig,
        state: &mut <WorkQueueActor as Actor>::State,
    ) -> Value {
        let mut context = Context::new();
        //fetch .env data
        let env_vars = self.read_env_file(config).expect("Failed to read env file");
        //place in tera context

        // Create your context and insert the `env` map.
        // let mut context = Context::new();
        context.insert("env", &env_vars);

        // for (key, value) in env_vars {
        //     context.insert(&format!("env.{}", key), &value);
        // }
        // Add environment variables to the context
        let events = state
            .event_repo
            .get_completed_events_for_session(event.flow_session_id.unwrap_or_default())
            .await;

        println!("Completed Session Events query result: {:?}", events);

        //add results to context by node_id
        if let Ok(events) = events {
            for event in events {
                if let Some(result) = event.result {
                    context.insert(event.node_id, &result);
                }
            }
        }

        let mut tera = Tera::default();

        if let Some(config) = &event.config {
            let config_str = config.to_string();
            println!("Config string before tera rendering: {}", config_str);
            tera.add_raw_template("config", &config_str).unwrap();
        } else {
            println!("Event config is None");
            //TODO: this should be an error
        }

        // Render the template with your context
        let rendered_config = tera
            .render("config", &context)
            .expect("Failed to render config");

        println!("Rendered config: {}", rendered_config);

        // Convert the rendered config to a Value
        let config_value: Value =
            serde_json::from_str(&rendered_config).expect("Failed to convert config to Value");

        config_value
    }

    fn read_env_file(&self, config: &AnythingConfig) -> io::Result<HashMap<String, String>> {
        let runtime_config = config.runtime_config().clone();
        
        //TODO: give access to "ASSET_FOLDER"
        //FLOW_FOLDER
        //FLOW_SESSION_ID
        //TRIGGER_SESSION_ID

        let root_dir = runtime_config.base_dir.expect("Base directory is not set");

        println!("Root directory: {}", root_dir.display());
        let env_path = root_dir.join(".store").join("anything").join(".env");
        println!("Env file path: {}", env_path.display());
        let env_path_str = env_path
            .to_str()
            .expect("Failed to convert PathBuf to &str");

        let file = File::open(env_path_str)?;

        let env_file_reader = BufReader::new(file);
        let mut env_vars = HashMap::new();

        for line in env_file_reader.lines() {
            let line = line?;
            // Ignore empty lines and comments
            if line.trim().is_empty() || line.starts_with('#') {
                continue;
            }
            // Parse key-value pairs
            if let Some((key, value)) = line.split_once('=') {
                println!("ENV Key: {}", key.trim());
                println!("ENV Value: {}", value.trim());
                env_vars.insert(key.trim().to_string(), value.trim().to_string());
            }
        }

        Ok(env_vars)
    }
}
