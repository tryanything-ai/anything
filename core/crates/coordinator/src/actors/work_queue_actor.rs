use anything_common::AnythingConfig;
use anything_persistence::{EventRepo, EventRepoImpl, FlowRepoImpl, StoreEvent};
use anything_runtime::{ExecuteConfigBuilder, PluginManager, Scope};
use anything_store::FileStore;
use serde_json::Value;
use std::collections::HashMap;
use uuid::Uuid;

use anything_persistence::FlowRepo;

use std::fs::File;
use std::io::{self, BufRead, BufReader};
use tera::{Context, Tera};

use ractor::{async_trait, Actor, ActorProcessingErr, ActorRef};

use crate::CoordinatorResult;

// Messages for Work Queue Actor
#[derive(Debug, Clone)]
pub enum WorkQueueActorMessage {
    StartWorkQueue,
    WorkCompleted(String),
    ExecuteFlow {
        flow_id: String,
        flow_version_id: String,
        session_id: Option<String>,
        stage: Option<String>,
    },
}

pub struct WorkQueueActorState {
    pub processing: bool,
    pub event_repo: EventRepoImpl,
    pub flow_repo: FlowRepoImpl,
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
            WorkQueueActorMessage::ExecuteFlow {
                flow_id,
                flow_version_id,
                session_id,
                stage,
            } => {
                // Implementation for executing a flow goes here
                self.execute_flow(
                    state,
                    myself,
                    flow_id.clone(),
                    flow_version_id.clone(),
                    session_id,
                    stage,
                )
                .await?;
                tracing::debug!("Execute Flow: {} {}", flow_id, flow_version_id);
                println!("println: Execute Flow: {} {}", flow_id, flow_version_id);
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
            let extension_id = event.extension_id.clone();

            //BUndle .env and results from previous actions in the context
            match self
                .create_bundled_context(event, &state.anything_config.clone(), state)
                .await
            {
                Ok(bundled_context) => {
                    state
                        .event_repo
                        .store_event_context(event_id.clone(), bundled_context.clone())
                        .await?;

                    if extension_id != "trigger" {
                        let extension = state.plugin_manager.get_plugin(&extension_id).unwrap();

                        let config = ExecuteConfigBuilder::default()
                            .plugin_name(extension_id)
                            .runtime("bash")
                            .context(bundled_context)
                            .build()
                            .unwrap();

                        // extension.register_action();
                        let result = extension.execute(&Scope::default(), &config);
                        match result {
                            Ok(execution_result) => {
                                //Save Result to DB
                                state
                                    .event_repo
                                    .store_execution_result(
                                        event_id.clone(),
                                        execution_result.result,
                                    )
                                    .await?;

                                //Mark as complete
                                let _event = state
                                    .event_repo
                                    .mark_event_as_complete(event_id.clone())
                                    .await?;
                            }
                            Err(e) => {
                                println!("Error occurred while executing the engine: {:?}", e);
                                state
                                    .event_repo
                                    .store_execution_error_result_and_cancel_remaining(
                                        event_id.clone(),
                                        serde_json::json!({ "error": e.to_string() }),
                                    )
                                    .await?;
                            }
                        }
                    } else {
                        //marking trigger as complete
                        let _event = state
                            .event_repo
                            .mark_event_as_complete(event_id.clone())
                            .await?;
                        println!("Not running action. Event is a trigger.");
                    }
                }
                Err(e) => {
                    println!("Failed to create bundled context: {:?}", e);
                    state
                        .event_repo
                        .store_execution_error_result_and_cancel_remaining(
                            event_id.clone(),
                            serde_json::json!({ "error": e.to_string() }),
                        )
                        .await?;
                }
            }

            let _ = myself.send_message(WorkQueueActorMessage::WorkCompleted(event_id.clone()));
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
        _event_id: String,
        state: &mut <WorkQueueActor as Actor>::State,
        myself: ActorRef<WorkQueueActorMessage>, // Add the myself parameter
    ) -> Result<(), ActorProcessingErr> {
        println!("Event Processed");
        self.process_next(state, myself).await?;
        Ok(())
    }

    async fn create_bundled_context(
        &self,
        event: StoreEvent,
        config: &AnythingConfig,
        state: &mut <WorkQueueActor as Actor>::State,
    ) -> Result<Value, anyhow::Error> {
        let mut context = Context::new();

        // Fetch .env data with error handling
        let env_vars = self.read_env_file(config).map_err(|e| {
            println!("Failed to read env file: {}", e);
            e
        })?;

        // Create your context and insert the `env` map.
        context.insert("env", &env_vars);

        // Retrieve events from state
        let events = state
            .event_repo
            .get_completed_events_for_session(event.flow_session_id.unwrap_or_default())
            .await
            .map_err(|e| {
                println!("Error fetching completed session events: {}", e);
                e
            })?;

        println!("Completed Session Events query result: {:?}", events);

        // Add results to context by node_id
        for event in events {
            if let Some(result) = event.result {
                context.insert(event.node_id, &result);
            }
        }

        // Prepare the Tera template engine
        let mut tera = Tera::default();
        if let Some(config) = &event.config {
            let config_str = config.to_string();
            tera.add_raw_template("config", &config_str).map_err(|e| {
                println!("Failed to add raw template to Tera: {}", e);
                e
            })?;
        } else {
            return Err(anyhow::anyhow!("Event config is None"));
        }

        // Render the Tera template with context
        let rendered_config = tera.render("config", &context).map_err(|e| {
            println!("Failed to render config with Tera: {}", e);
            e
        })?;

        println!("Rendered config: {}", rendered_config);

        serde_json::from_str::<Value>(&rendered_config).map_err(|e| {
            println!("Failed to convert rendered config to Value: {}", e);
            anyhow::Error::new(e) // Explicitly convert the serde_json error to anyhow::Error
        })
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

    pub async fn execute_flow(
        &self,
        state: &mut <WorkQueueActor as Actor>::State,
        myself: ActorRef<WorkQueueActorMessage>,
        flow_id: String,
        flow_version_id: String, //TODO: add trigger_id and context
        session_id: Option<String>,
        stage: Option<String>,
    ) -> CoordinatorResult<String> {
        println!("flow_id: {}", flow_id);
        println!("flow_version_id: {}", flow_version_id);

        let flow = state
            .flow_repo
            .get_flow_version_by_id(flow_id, flow_version_id)
            .await?;

        //create flow session id if one was not passed
        let flow_session_id = if session_id.is_none() {
            Uuid::new_v4().to_string()
        } else {
            session_id.unwrap()
        };

        //BFS over the flow to get the execution plan
        let worklist =
            anything_carl::flow::create_execution_plan(flow, flow_session_id.clone(), stage);

        println!("worklist in manager: {:?}", worklist);

        //create all the events in the database
        for event in worklist {
            state.event_repo.save_event(event).await?;
        }

        //TODO: send start message to self
        let _ = myself.send_message(WorkQueueActorMessage::StartWorkQueue);

        Ok(flow_session_id.clone())
    }
}
