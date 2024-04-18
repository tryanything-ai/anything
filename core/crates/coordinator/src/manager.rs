use anything_carl;
use anything_common::AnythingConfig;
use std::process::Command;

use anything_persistence::datastore::RepoImpl;
use anything_persistence::{
    create_sqlite_datastore_from_config_and_file_store, CreateFlow, CreateFlowVersion, EventRepo,
    EventRepoImpl, FlowRepo, FlowRepoImpl, FlowVersion, UpdateFlowArgs, UpdateFlowVersion,
};
use anything_runtime::{PluginManager, RuntimeConfig};
use anything_store::FileStore;
use ractor::{cast, Actor, ActorRef};
use std::{env::temp_dir, sync::Arc};

use tokio::sync::{
    mpsc::{self},
    Mutex,
};

use uuid::Uuid;

// use crate::actors::flow_actors::{FlowActor, FlowActorState, FlowMessage};
use crate::actors::system_actors::{SystemActor, SystemActorState, SystemMessage};
use crate::actors::trigger_actor::{TriggerActor, TriggerActorState, TriggerMessage};
// use crate::actors::update_actor::{UpdateActor, UpdateActorMessage, UpdateActorState};
use crate::actors::work_queue_actor::{WorkQueueActor, WorkQueueActorMessage, WorkQueueActorState};
use crate::error::CoordinatorResult;
use crate::CoordinatorError;

#[derive(Debug, Clone)]
pub struct Repositories {
    pub flow_repo: anything_persistence::FlowRepoImpl,
    pub event_repo: anything_persistence::EventRepoImpl,
    // pub trigger_repo: anything_persistence::TriggerRepoImpl,
}

#[derive(Debug, Clone)]
pub struct ActorRefs {
    pub system_actor: ActorRef<SystemMessage>,
    // pub flow_actor: ActorRef<FlowMessage>,
    // pub update_actor: ActorRef<UpdateActorMessage>,
    pub work_queue_actor: ActorRef<WorkQueueActorMessage>,
    pub trigger_actor: ActorRef<TriggerMessage>,
}

#[derive(Debug, Clone)]
pub struct Manager {
    pub file_store: FileStore,
    pub config: AnythingConfig,
    // pub runner: Runner,
    // pub shutdown_sender: Sender<()>,
    pub repositories: Option<Repositories>,
    pub actor_refs: Option<ActorRefs>,
}

impl Default for Manager {
    fn default() -> Self {
        let mut runtime_config = RuntimeConfig::default();
        let temp_dir = temp_dir();
        runtime_config.base_dir = Some(temp_dir);
        let anything_config = AnythingConfig::new(runtime_config);
        Self::new(anything_config)
    }
}

pub async fn start(
    config: AnythingConfig,
    shutdown_rx: mpsc::Receiver<()>,
    ready_tx: mpsc::Sender<Arc<Manager>>,
) -> CoordinatorResult<()> {
    let mut manager = Manager::new(config);

    manager.start(shutdown_rx, ready_tx).await?;
    Ok(())
}

// TODO: Move to use repositories instead of models
impl Manager {
    pub fn new(config: AnythingConfig) -> Self {
        let mut runtime_config = config.runtime_config().clone();
        //manages plugins and deno stuff i think
        // let runner = Runner::new(runtime_config.clone());

        //Make a dir if we don't have one
        let root_dir = match runtime_config.base_dir {
            Some(v) => v.clone(),
            None => tempfile::tempdir().unwrap().path().to_path_buf(),
        };
        runtime_config.base_dir = Some(root_dir.clone());

        //Deal with local file system
        let file_store = FileStore::create(root_dir.as_path(), &["anything"]).unwrap();

        // Create all the base directories required
        file_store.create_base_dir().unwrap();
        for dir in &["flows", "database", "actions", "assets"] {
            file_store.create_directory(&[dir]).unwrap();
        }

        // Create files if they don't exist
        let file_paths = vec![vec![".env"]];
        let file_content = b"";
        for file_path in file_paths {
            if !file_store.file_exists(&file_path) {
                file_store.write_file(&file_path, file_content).unwrap();
            }
        }

        // Initialize git repository if it doesn't exist
        let git_dir = root_dir.join(".store").join("anything").join(".git");
        if !git_dir.exists() {
            let output = Command::new("git")
                .arg("init")
                .current_dir(&git_dir.parent().unwrap())
                .output()
                .expect("Failed to execute git init");

            //make a .gitignore file
            let gitignore_content = r#"
# Ignore all .env files
.env

# Ignore databases
database/
"#;

            file_store
                .write_file(&[".gitignore"], gitignore_content.as_bytes())
                .unwrap();

            if !output.status.success() {
                eprintln!(
                    "git init failed: {}",
                    String::from_utf8_lossy(&output.stderr)
                );
            }
        } else {
            println!("Git repository already exists");
        }

        Manager {
            file_store,
            // runner,
            config: config.clone(),
            repositories: None,
            actor_refs: None,
        }
    }

    pub async fn start(
        &mut self,
        mut shutdown_rx: mpsc::Receiver<()>,
        ready_tx: mpsc::Sender<Arc<Self>>,
    ) -> CoordinatorResult<()> {
        // Setup sqlite db
        let datastore = create_sqlite_datastore_from_config_and_file_store(
            self.config.clone(),
            self.file_store.clone(),
        )
        .await
        .unwrap();

        //Stores The Flows in SQLite
        let flow_repo = FlowRepoImpl::new_with_datastore(datastore.clone())
            .expect("unable to create flow repo");
        //Stores The Events in SQLite
        let event_repo = EventRepoImpl::new_with_datastore(datastore.clone())
            .expect("unable to create event repo");

        self.repositories = Some(Repositories {
            flow_repo: flow_repo.clone(),
            event_repo: event_repo.clone(),
        });

        // startup System Actor in charge of watching files changes for flows to syncronize
        let (system_actor, _handle) = Actor::spawn(
            None,
            SystemActor,
            SystemActorState {
                file_store: self.file_store.clone(),
                flow_repo: flow_repo.clone(),
            },
        )
        .await
        .unwrap();

        //Start carls work queue actor
        let (work_queue_actor, _handle) = Actor::spawn(
            None,
            WorkQueueActor,
            WorkQueueActorState {
                processing: false,
                event_repo: event_repo.clone(),
                flow_repo: flow_repo.clone(),
                plugin_manager: PluginManager::new(self.config.runtime_config()),
                file_store: self.file_store.clone(),
                anything_config: self.config.clone(),
            },
        )
        .await
        .unwrap();

        let (trigger_actor, _handle) = Actor::spawn(
            None,
            TriggerActor,
            TriggerActorState {
                flow_repo: flow_repo.clone(),
                triggers: Arc::new(std::sync::Mutex::new(vec![])),
                // config: self.config.clone(),
                work_queue_actor: work_queue_actor.clone(), // execute_flow: self.execute_flow.clone(),
                                                            // execute_flow: self.exec
            },
        )
        .await
        .unwrap();

        self.actor_refs = Some(ActorRefs {
            system_actor,
            work_queue_actor: work_queue_actor.clone(),
            trigger_actor,
        });

        // Setup listeners and action-takers
        self.setup_file_handler().await;

        // Return with ready
        ready_tx.send(Arc::new(self.clone())).await.unwrap();

        // never quit -> this i think talks to tauri runtime. we signal ready to tauri with the read_rx
        loop {
            // Never quit
            tokio::select! {

                _ = shutdown_rx.recv() => {
                    break;
                }
            }
        }
        tracing::debug!("shutting down");

        Ok(())
    }

    /// The function `get_flows` returns a result containing a vector of `anything_persistence::StoredFlow` objects.
    ///
    /// Returns:
    ///
    /// The function `get_flows` returns a `CoordinatorResult` containing a `Vec` of
    /// `anything_persistence::StoredFlow` objects.
    pub async fn get_flows(&self) -> CoordinatorResult<Vec<anything_persistence::StoredFlow>> {
        let flow_repo = self.flow_repo()?;
        // let mut file_store = self.file_store.clone();
        let flows = flow_repo.get_flows().await.map_err(|e| {
            tracing::error!("error when getting flows: {:#?}", e);
            CoordinatorError::PersistenceError(e)
        })?;
        // let mut graph_flows: Vec<anything_graph::Flow> = vec![];
        // for flow in flows.iter() {
        //     let flow = flow.get_flow(&mut file_store).await.map_err(|e| {
        //         tracing::error!("error when getting flow for flow {:#?}: {:#?}", flow, e);
        //         CoordinatorError::PersistenceError(e)
        //     })?;
        //     graph_flows.push(flow.into());
        // }
        Ok(flows)
    }

    /// The function `get_flow` retrieves a flow by name and returns it as a result, or returns an error
    /// if the flow is not found.
    ///
    /// Arguments:
    ///
    /// * `name`: A string representing the name of the flow to retrieve.
    ///
    /// Returns:    
    ///
    /// The function `get_flow` returns a `CoordinatorResult` which can either be an `Ok` variant
    /// containing a `anything_persistence::StoredFlow` or an `Err` variant containing a
    /// `CoordinatorError::FlowNotFound` with the name of the flow as a string.
    pub async fn get_flow(
        &self,
        name: String,
    ) -> CoordinatorResult<anything_persistence::StoredFlow> {
        let flow_repo = self.flow_repo()?;
        // let mut file_store = self.file_store.clone();
        tracing::trace!("Get flow by name called in the manager: {:?}", name.clone());
        // Look for stored flow in database
        let flow = flow_repo.get_flow_by_name(name).await.map_err(|e| {
            tracing::error!("error when getting flow: {:#?}", e);
            CoordinatorError::PersistenceError(e)
        })?;
        // tracing::info!("db_flow: {:#?}", flow);
        // Get the flow from disk
        // let flow = flow.get_flow(&mut file_store).await.map_err(|e| {
        //     tracing::error!("error when getting flow: {:#?}", e);
        //     CoordinatorError::PersistenceError(e)
        // })?;
        // tracing::info!("file_flow: {:#?}", flow);
        Ok(flow.into())
    }

    pub async fn fetch_session_events(
        &self,
        session_id: String,
    ) -> CoordinatorResult<anything_persistence::EventList> {
        let event_repo = self.event_repo()?;
        // let mut file_store = self.file_store.clone();
        tracing::trace!(
            "Get events by session_id called in the manager: {:?}",
            session_id.clone()
        );
        // Look for stored flow in database
        let events = event_repo
            .get_events_for_session(session_id)
            .await
            .map_err(|e| {
                tracing::error!("error when getting events for session: {:#?}", e);
                CoordinatorError::PersistenceError(e)
            })?;

        Ok(events)
    }

    pub async fn get_event(
        &self,
        event_id: String,
    ) -> CoordinatorResult<anything_persistence::StoreEvent> {
        let event_repo = self.event_repo()?;

        tracing::trace!(
            "Get event by event_id called in the manager: {:?}",
            event_id.clone()
        );
        // Look for stored flow in database
        let event = event_repo.find_by_id(event_id).await.map_err(|e| {
            tracing::error!("error when getting event by id: {:#?}", e);
            CoordinatorError::PersistenceError(e)
        })?;

        Ok(event)
    }

    /// The function `create_flow` creates a new flow, saves it to a file, and returns the created flow.
    ///
    /// Arguments:
    ///
    /// * `flow_name`: A string representing the name of the flow to be created.
    /// * `flow_id`: The `flow_id` parameter is a unique identifier for the flow. It is used to
    /// distinguish one flow from another and ensure that each flow has a unique identity.
    ///
    /// Returns:
    ///
    /// a `CoordinatorResult` containing a `anything_persistence::StoredFlow` object.
    pub async fn create_flow(
        &mut self,
        flow_name: String,
    ) -> CoordinatorResult<anything_persistence::StoredFlow> {
        // Create flow model
        let create_flow = CreateFlow {
            name: flow_name.clone(),
            active: false,
            version: None,
        };

        // tracing::debug!("Creating flow: {:#?}", create_flow);

        let flow = self.flow_repo()?.create_flow(create_flow).await?;

        // tracing::debug!("Created flow in the repo: {:#?}", flow);

        // let new_directory = self
        //     .file_store
        //     .create_directory(&["flows", &flow.flow_name])
        //     .expect("unable to create flow directory");

        // tracing::debug!("Created flow directory: {:#?}", new_directory);

        // let flow: Flow = flow.clone().get_flow(&mut self.file_store).await.unwrap();
        // let flowfile: Flowfile = flow.clone().into();
        // let toml_repr = toml::to_string(&flow).expect("unable to convert StoredFlow into a string");
        // tracing::debug!("Saving flow toml representation: {:#?}", toml_repr);
        // let flowfile =
        //     Flowfile::from_string(toml_repr).expect("unable to create flow file for a new flow");
        // let flow_str: String = flowfile.clone().into();

        //TODO: why lowercase? folders are normal uppercase
        // let lowercased_flow_name = flow_name.to_lowercase();
        // let new_dir_str = new_directory
        //     .to_str()
        //     .expect("unable to create new directory string");

        // tracing::debug!("new_dir_str: {:#?}", new_dir_str);

        // self.file_store
        //     .write_file(
        //         &["flows", new_dir_str, &format!("flow.toml")],
        //         flow_str.as_bytes(),
        //     )
        //     .expect("unable to write basic flow string");

        // tracing::debug!(
        //     "wrote flow file at {:#?}",
        //     &[
        //         "flows",
        //         new_dir_str,
        //         &format!("{}.toml", lowercased_flow_name),
        //     ]
        // );
        // let flow: Flow = flowfile.into();
        Ok(flow)
    }

    /// The function `delete_flow` deletes a flow and its associated files.
    ///
    /// Arguments:
    ///
    /// * `flow_name`: The `flow_name` parameter is a `String` that represents the name of the flow to
    /// be deleted.
    ///
    /// Returns:
    ///
    /// a `CoordinatorResult` containing a `anything_graph::Flow` object.
    pub async fn delete_flow(&self, flow_id: String) -> CoordinatorResult<String> {
        let flow_name = self.flow_repo()?.delete_flow(flow_id).await?;

        // let _ = self
        //     .file_store
        //     .delete_directory(&["flows", &flow_name])
        //     .unwrap();

        Ok(flow_name)
    }

    /// The function `update_flow` updates a flow with the given name and returns the updated flow.
    ///
    /// Arguments:
    ///
    /// * `flow_name`: The `flow_name` parameter is a `String` that represents the name of the flow that
    /// needs to be updated.
    ///
    /// Returns:
    ///
    /// a `CoordinatorResult` containing a value of type `anything_persistence::StoredFlow`.
    pub async fn update_flow(
        &mut self,
        flow_id: String,
        args: UpdateFlowArgs,
    ) -> CoordinatorResult<anything_persistence::StoredFlow> {
        tracing::trace!("Update flow with {flow_id} and {:#?}", args);
        // let new_flow_name = args.flow_name.clone();
        // let mut original_flow = self.flow_repo()?.get_flow_by_id(flow_id.clone()).await?;
        // let original_flow_name = original_flow.flow_name.clone();

        // tracing::trace!("original_flow: {:#?}", original_flow);

        // self.flow_repo()?.delete_flow(flow_id.clone()).await?;

        let stored_flow = self.flow_repo()?.update_flow(flow_id.clone(), args).await?;

        // original_flow.flow_name = stored_flow.flow_name.clone();
        // self.file_store
        //     .rename_directory(&["flows", &original_flow_name], &["flows", &new_flow_name])
        //     .expect("unable to rename flow directory");

        // let flow_str: String = toml::to_string(&stored_flow).expect("unable to convert to string");

        // self.file_store
        //     .write_file(
        //         &["flows", &new_flow_name, &format!("flow.toml")],
        //         flow_str.as_bytes(),
        //     )
        //     .expect("unable to write basic flow string");
        // let mut file_store = self.file_store.clone();

        // let flow = stored_flow.get_flow(&mut file_store).await?;
        Ok(stored_flow)
    }

    pub async fn create_flow_version(
        &mut self,
        flow_id: String,
        flow_version: CreateFlowVersion,
    ) -> CoordinatorResult<FlowVersion> {
        let stored_flow_version = self
            .flow_repo()?
            .create_flow_version(flow_id, flow_version)
            .await?;
        Ok(stored_flow_version)
    }

    pub async fn update_flow_version(
        &mut self,
        flow_id: String,
        flow_version_id: String,
        update_flow: UpdateFlowVersion,
    ) -> CoordinatorResult<FlowVersion> {
        let db_flow_version = self
            .flow_repo()?
            .update_flow_version(flow_id, flow_version_id, update_flow)
            .await?;
        Ok(db_flow_version)
    }

    pub async fn execute_flow(
        &self,
        flow_id: String,
        flow_version_id: String, //TODO: add trigger_id and context
        session_id: Option<String>,
        stage: Option<String>,
    ) -> CoordinatorResult<String> {
        println!("Execute flow called in the manager");
        println!("flow_id: {}", flow_id);
        println!("flow_version_id: {}", flow_version_id);

        // let flow = self
        //     .flow_repo()?
        //     .get_flow_version_by_id(flow_id, flow_version_id)
        //     .await?;

        // //create flow session id if one was not passed
        let flow_session_id = if session_id.is_none() {
            Uuid::new_v4().to_string()
        } else {
            session_id.unwrap()
        };

        //BFS over the flow to get the execution plan
        // let worklist =
        //     anything_carl::flow::create_execution_plan(flow, flow_session_id.clone(), stage);

        // println!("worklist in manager: {:?}", worklist);

        // //create all the events in the database
        // for event in worklist {
        //     self.event_repo()?.save_event(event).await?;
        // }

        //start execution
        // Check if the actor_refs and work_queue_actor are available
        if let Some(ref actor_refs) = self.actor_refs {
            // Send the StartWorkQueue message to the work_queue_actor
            let result =
                actor_refs
                    .work_queue_actor
                    .send_message(WorkQueueActorMessage::ExecuteFlow {
                        flow_id: flow_id,
                        flow_version_id: flow_version_id,
                        session_id: Some(flow_session_id.clone()),
                        stage: stage,
                    });
            match result {
                Ok(_) => {
                    // Handle success case
                    println!("Message Sent to Work Queue Actor")
                }
                Err(e) => {
                    // Handle error case
                    println!("Error sending StartWorkQueue message: {:?}", e);
                }
            }

            //TODO: send message to update triggers ( ACTUALLLY do this in the CRUD functions for triggers)
        } else {
            return Err(CoordinatorError::ActorNotInitialized(String::from(
                "work_queue_actor",
            )));
        }

        Ok(flow_session_id.clone())
    }

    pub fn flow_repo(&self) -> CoordinatorResult<FlowRepoImpl> {
        match &self.repositories {
            Some(repositories) => Ok(repositories.flow_repo.clone()),
            None => Err(CoordinatorError::RepoNotInitialized),
        }
    }

    pub fn event_repo(&self) -> CoordinatorResult<EventRepoImpl> {
        match &self.repositories {
            Some(repositories) => Ok(repositories.event_repo.clone()),
            None => Err(CoordinatorError::RepoNotInitialized),
        }
    }

    pub fn system_actor(&self) -> CoordinatorResult<ActorRef<SystemMessage>> {
        self.actor_refs
            .as_ref()
            .ok_or(CoordinatorError::ActorNotInitialized(String::from(
                "system_actor",
            )))
            .map(|refs| refs.system_actor.clone())
    }

    /*
    INTERNAL FUNCTIONS
    */

    // Internal
    async fn setup_file_handler(&mut self) {
        let (tx, mut rx) = tokio::sync::mpsc::channel(4096);
        let file_store = Arc::new(Mutex::new(self.file_store.clone()));

        // Listen for changes on the file system
        let _t1 = tokio::spawn(async move {
            let mut fs = file_store.try_lock().expect("should be unlockable");
            fs.notify_changes(tx.clone()).await.unwrap();
        });

        let actor = self.actor_refs.as_ref().unwrap().system_actor.clone();

        // Send changes to the coordinator
        let _t2 = tokio::spawn(async move {
            while let Some(msg) = rx.recv().await {
                cast!(actor.clone(), SystemMessage::StoreChanged(msg)).unwrap();
            }
        });
    }

    pub async fn stop(self) -> CoordinatorResult<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use std::{path::PathBuf, time::Duration};

    use crate::{
        events::{FlowPublisher, NewFlowPublisher, StringPublisher},
        processing::processor::ProcessorMessage,
        test_helper::add_flow_directory,
    };
    use anything_graph::Flowfile;
    use anything_mq::new_client;
    use anything_runtime::{EngineKind, EngineOption, PluginEngine};
    use ractor::call;
    use tokio::time::{sleep, timeout};
    const SLEEP_TIME: u64 = 600;

    use super::*;

    #[tokio::test]
    async fn test_can_subscribe() {
        let _manager = Manager::default();

        let client = new_client().await.unwrap();

        let sub = client.subscribe("not-used-yet").await.unwrap();
        let tx = client.publisher().await.unwrap();

        let msg = StringPublisher("A new string, bruh".to_string());
        let res = tx.send(msg.clone()).await;
        assert!(res.is_ok());

        let StringPublisher(payload) = sub.recv().await.unwrap();
        assert_eq!("A new string, bruh".to_string(), payload);
    }

    #[tokio::test]
    async fn test_can_subscribe_to_a_new_flows_publisher() {
        let _manager = Manager::default();

        // let file = get_fixtures_directory().join("simple.toml");

        let toml = r#"
            name = "SimpleFlow"
            version = "0.0.1"
            description = "A simple flow that echos holiday cheer"

            [[nodes]]
            name = "echo-cheer"
            label = "Holiday cheers"
            depends_on = []
            variables = { cheers = "Jingle Bells" }

            [nodes.engine]
            interpreter = "deno"
            args = ["export default function() { return 'hello {{cheers}}' }"]

            "#
        .to_string();
        let test_flow = Flowfile::from_string(toml);
        let test_flow = test_flow.unwrap();

        // let _ = join_handle.await;
        let client1 = new_client().await.unwrap();
        let client2 = new_client().await.unwrap();

        let sub = client1.subscribe("new").await.unwrap();

        let test_flow_name = test_flow.clone().name;
        let msg = FlowPublisher::NewFlow(NewFlowPublisher { flow: test_flow });

        let res = client2.publish("new", msg.clone()).await;
        assert!(res.is_ok());
        // po_sender.send(()).unwrap();

        let res = sub.recv().await;
        let payload: crate::events::FlowPublisher = res.unwrap();
        let payload = match payload {
            FlowPublisher::NewFlow(inner_payload) => inner_payload,
            _ => unreachable!(),
        };
        assert_eq!(payload.flow.name, test_flow_name);

        let first_node = payload.flow.nodes.first().unwrap();
        let runtime = first_node.run_options.clone();

        let mut deno_engine = PluginEngine::default();
        deno_engine.engine = "deno".to_string();
        deno_engine.args = Some(vec![
            "export default function() { return 'hello {{cheers}}' }".to_string(),
        ]);
        assert_eq!(runtime.engine, Some(EngineKind::PluginEngine(deno_engine)));
    }

    #[tokio::test]
    async fn test_subscribe_to_execute_flow() {
        let _manager = Manager::default();

        let file = get_fixtures_directory().join("simple.toml");
        let test_flow = Flowfile::from_file(file).unwrap();

        let client1 = new_client().await.unwrap();
        let client2 = new_client().await.unwrap();

        let sub = client1.subscribe("new").await.unwrap();

        let test_flow_name = test_flow.clone().name;
        let msg = FlowPublisher::NewFlow(NewFlowPublisher { flow: test_flow });

        let res = client2.publish("new", msg.clone()).await;
        assert!(res.is_ok());
        // po_sender.send(()).unwrap();

        let res = sub.recv().await;
        let payload: crate::events::FlowPublisher = res.unwrap();
        let payload = match payload {
            FlowPublisher::NewFlow(inner_payload) => inner_payload,
            _ => unreachable!(),
        };
        assert_eq!(payload.flow.name, test_flow_name);

        let first_node = payload.flow.nodes.first().unwrap();
        let runtime = first_node.run_options.clone();

        let mut system_plugin_engine = PluginEngine::default();
        system_plugin_engine.engine = "system-shell".to_string();
        system_plugin_engine.args = Some(vec!["echo 'hello {{cheers}}'".to_string()]);
        system_plugin_engine.options = indexmap::indexmap! {
            "shell".to_string() => EngineOption::from("bash".to_string())
        };
        assert_eq!(
            runtime.engine,
            Some(EngineKind::PluginEngine(system_plugin_engine))
        );

        // po_sender.send(()).unwrap();
    }

    #[tokio::test]
    async fn test_subscribe_to_store_changes() {
        let config = AnythingConfig::default();

        let (shutdown_tx, shutdown_rx) = mpsc::channel(1);
        let (ready_tx, mut ready_rx) = mpsc::channel(1);

        tokio::spawn(async move {
            start(config, shutdown_rx, ready_tx).await.unwrap();
        });

        let manager = ready_rx.recv().await.unwrap();
        // let store = manager.file_store.clone();

        let rpath = manager.file_store.store_path(&["flows"]);

        let server_task = tokio::spawn(async move {
            add_flow_directory(rpath.clone(), "some-simple-flow");
            // let res = store.write_file(&["just_a_test.txt"], "test".as_bytes());
            let _ = sleep(Duration::from_millis(SLEEP_TIME)).await;
            // Get the flow to ensure it changed in the database
            let _found_flow = shutdown_tx.send(()).await.unwrap();
        });

        let res = timeout(Duration::from_secs(5), server_task).await;
        assert!(res.is_ok(), "server task did not quit");
    }

    #[tokio::test]
    async fn test_can_trigger_simple_flow_run() {
        let config = AnythingConfig::default();

        // Channels for management
        let (shutdown_tx, shutdown_rx) = mpsc::channel(1);
        let (ready_tx, mut ready_rx) = mpsc::channel(1);

        // Start off the manager
        tokio::spawn(async move {
            start(config, shutdown_rx, ready_tx).await.unwrap();
        });

        let manager = ready_rx.recv().await.unwrap();

        // Add a simple flow
        let rpath = manager.file_store.store_path(&["flows"]);
        add_flow_directory(rpath.clone(), "some-simple-flow");
        let _ = sleep(Duration::from_millis(SLEEP_TIME)).await;

        // the actual test
        let server_task = tokio::spawn(async move {
            let flow = manager
                .get_flow("some-simple-flow".to_string())
                .await
                .unwrap();

            let flow_actor = manager.flow_actor().unwrap();
            // Send the execute flow message
            cast!(flow_actor.clone(), FlowMessage::ExecuteFlow(flow)).unwrap();
            // Give the flow a few milliseconds to execute
            let _ = sleep(Duration::from_millis(SLEEP_TIME)).await;

            let update_actor_ref = manager.actor_refs.as_ref().unwrap().update_actor.clone();

            let res = call!(
                update_actor_ref,
                UpdateActorMessage::GetLatestProcessorMessages
            )
            .unwrap();

            assert_eq!(res.len(), 1);
            let msg = res.first().unwrap();
            match msg {
                ProcessorMessage::FlowTaskFinishedSuccessfully(task_name, result) => {
                    assert_eq!(task_name, "echo");
                    assert_eq!(result, "hello world");
                }
                _ => assert!(false, "unexpected message type"),
            };

            // update_actor_re

            // Get the flow to ensure it changed in the database
            let _found_flow = shutdown_tx.send(()).await.unwrap();
        });

        let res = timeout(Duration::from_secs(5), server_task).await;
        assert!(res.is_ok(), "server task did not quit");
    }

    #[tokio::test]
    async fn test_can_trigger_flow_run() {
        let config = AnythingConfig::default();

        // Channels for management
        let (shutdown_tx, shutdown_rx) = mpsc::channel(1);
        let (ready_tx, mut ready_rx) = mpsc::channel(1);

        // Start off the manager
        tokio::spawn(async move {
            start(config, shutdown_rx, ready_tx).await.unwrap();
        });

        let manager = ready_rx.recv().await.unwrap();

        // Add a simple flow
        let file = get_fixtures_directory().join("simple.toml");
        let test_flow = Flowfile::from_file(file).unwrap();
        let flow: Flow = test_flow.into();
        let _ = sleep(Duration::from_millis(SLEEP_TIME)).await;

        // the actual test
        let server_task = tokio::spawn(async move {
            let flow_actor = manager.flow_actor().unwrap();
            // Send the execute flow message
            cast!(flow_actor.clone(), FlowMessage::ExecuteFlow(flow)).unwrap();
            // Give the flow a few milliseconds to execute
            let _ = sleep(Duration::from_millis(SLEEP_TIME)).await;

            let update_actor_ref = manager.actor_refs.as_ref().unwrap().update_actor.clone();

            let res = call!(
                update_actor_ref,
                UpdateActorMessage::GetLatestProcessorMessages
            )
            .unwrap();

            assert_eq!(res.len(), 3);
            let messages = res.iter().map(|m| m.clone()).collect::<Vec<_>>();
            match messages.get(0).unwrap() {
                ProcessorMessage::FlowTaskFinishedSuccessfully(task_name, result) => {
                    assert_eq!(task_name, "echo-cheer");
                    assert_eq!(result, "hello Jingle Bells");
                }
                _ => assert!(false, "unexpected message type"),
            };
            match messages.get(1).unwrap() {
                ProcessorMessage::FlowTaskFinishedSuccessfully(task_name, result) => {
                    assert_eq!(task_name, "say-cheers");
                    assert_eq!(result, "second Jingle Bells");
                }
                _ => assert!(false, "unexpected message type"),
            };

            match messages.get(2).unwrap() {
                ProcessorMessage::FlowTaskFinishedSuccessfully(task_name, result) => {
                    assert_eq!(task_name, "share");
                    assert_eq!(result, "cheers Jingle Bells to all");
                }
                _ => assert!(false, "unexpected message type"),
            };

            // update_actor_re

            // Get the flow to ensure it changed in the database
            let _found_flow = shutdown_tx.send(()).await.unwrap();
        });

        let res = timeout(Duration::from_secs(5), server_task).await;
        assert!(res.is_ok(), "server task did not quit");
    }

    #[allow(unused)]
    fn get_fixtures_directory() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("fixtures")
    }

    #[allow(unused)]
    fn get_unique_config() -> AnythingConfig {
        let mut config = AnythingConfig::default();
        let tmpdir = tempfile::tempdir()
            .unwrap()
            .path()
            .to_path_buf()
            .join(format!("test-{}", uuid::Uuid::new_v4()));
        let mut runtime_config = config.runtime_config().clone();
        runtime_config.base_dir = Some(tmpdir.clone());
        config.update_runtime_config(runtime_config);
        config
    }

    // async fn setup()
}
