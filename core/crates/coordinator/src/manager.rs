use anything_common::{spawn_or_crash, AnythingConfig};
use anything_graph::Flowfile;
use anything_mq::new_client;
use anything_persistence::datastore::RepoImpl;
use anything_persistence::{
    create_sqlite_datastore_from_config_and_file_store, EventRepoImpl, FlowRepoImpl,
    TriggerRepoImpl,
};
use anything_runtime::{Runner, RuntimeConfig};
use anything_store::FileStore;
use std::{env::temp_dir, sync::Arc};

use tokio::sync::{
    mpsc::{self, Sender},
    Mutex,
};

use crate::CoordinatorError;
use crate::{
    error::CoordinatorResult,
    events::StoreChangesPublisher,
    handlers,
    models::{Models, MODELS},
};

#[derive(Debug, Clone)]
pub struct Repositories {
    pub flow_repo: anything_persistence::FlowRepoImpl,
    pub event_repo: anything_persistence::EventRepoImpl,
    pub trigger_repo: anything_persistence::TriggerRepoImpl,
}

#[derive(Debug, Clone)]
pub struct Manager {
    pub file_store: FileStore,
    pub config: AnythingConfig,
    pub executor: Option<Runner>,
    pub shutdown_sender: Sender<()>,
    pub repositories: Option<Repositories>,
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

// TODO: Move to use repositories instead of models
impl Manager {
    pub fn new(config: AnythingConfig) -> Self {
        let mut runtime_config = config.runtime_config().clone();
        let executor = Runner::new(runtime_config.clone());

        let root_dir = match runtime_config.base_dir {
            Some(v) => v.clone(),
            None => tempfile::tempdir().unwrap().path().to_path_buf(),
        };
        runtime_config.base_dir = Some(root_dir.clone());
        let (shutdown_sender, _) = tokio::sync::mpsc::channel(1);

        let file_store = FileStore::create(root_dir.as_path(), &["anything"]).unwrap();

        // Create all the base directories required
        file_store.create_base_dir().unwrap();
        for dir in &["flows", "db"] {
            file_store.create_directory(&[dir]).unwrap();
        }

        Manager {
            file_store,
            config: config.clone(),
            executor: Some(executor),
            shutdown_sender,
            repositories: None, // post_office: PostOffice::open(),
        }
    }

    pub async fn start(&mut self, mut shutdown_rx: mpsc::Receiver<()>) -> CoordinatorResult<()> {
        // Setup persistence
        let datastore = create_sqlite_datastore_from_config_and_file_store(
            self.config.clone(),
            self.file_store.clone(),
        )
        .await
        .unwrap();
        let repositories = Repositories {
            flow_repo: FlowRepoImpl::new_with_datastore(datastore.clone())
                .expect("unable to create flow repo"),
            event_repo: EventRepoImpl::new_with_datastore(datastore.clone())
                .expect("unable to create event repo"),
            trigger_repo: TriggerRepoImpl::new_with_datastore(datastore.clone())
                .expect("unable to create trigger repo"),
        };

        let system = actix::System::new();

        // spawn_or_crash(
        //     "internal_events",
        //     self.clone(),
        //     handlers::system_handler::process_system_events,
        // );

        // spawn_or_crash(
        //     "flow_events",
        //     self.clone(),
        //     handlers::flow_handler::process_flows,
        // );

        // spawn_or_crash(
        //     "store_events",
        //     self.clone(),
        //     handlers::store_handler::process_store_events,
        // );

        self.setup_file_handler().await;

        // never quit
        let _res = system.run();
        // loop {
        //     // Never quit
        //     tokio::select! {

        //         _ = shutdown_rx.recv() => {
        //             break;
        //         }
        //     }
        // }
        tracing::debug!("shutting down");

        Ok(())
    }

    pub async fn refresh_flows(&self) -> CoordinatorResult<()> {
        let mut models = MODELS.get().unwrap().lock().await;
        models.reload_flows().await;
        Ok(())
    }

    /// The function `get_flows` returns a result containing a vector of `anything_graph::Flow` objects.
    ///
    /// Returns:
    ///
    /// The function `get_flows` returns a `CoordinatorResult` containing a `Vec` of
    /// `anything_graph::Flow` objects.
    pub async fn get_flows(&self) -> CoordinatorResult<Vec<anything_graph::Flow>> {
        let flows = MODELS.get().unwrap().lock().await.get_flows();
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
    /// containing a `anything_graph::Flow` or an `Err` variant containing a
    /// `CoordinatorError::FlowNotFound` with the name of the flow as a string.
    pub async fn get_flow(&self, name: &str) -> CoordinatorResult<anything_graph::Flow> {
        let flow = MODELS.get().unwrap().lock().await.get_flow(name);
        match flow {
            Some(flow) => Ok(flow),
            None => Err(crate::error::CoordinatorError::FlowNotFound(
                name.to_string(),
            )),
        }
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
    /// a `CoordinatorResult` containing a `anything_graph::Flow` object.
    pub async fn create_flow(
        &self,
        flow_name: String,
        flow_id: String,
    ) -> CoordinatorResult<anything_graph::Flow> {
        let flow = MODELS
            .get()
            .unwrap()
            .lock()
            .await
            .create_flow(flow_name, flow_id)?;

        let new_directory = self
            .file_store
            .create_directory(&["flows", &flow.name])
            .unwrap();

        let flowfile: Flowfile = flow.clone().into();
        let flow_str: String = flowfile.into();

        self.file_store
            .write_file(
                &[
                    "flows",
                    new_directory
                        .as_os_str()
                        .to_os_string()
                        .as_os_str()
                        .to_str()
                        .unwrap(),
                    &format!("{}.toml", flow.name),
                ],
                flow_str.as_bytes(),
            )
            .unwrap();

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
    pub async fn delete_flow(&self, flow_name: String) -> CoordinatorResult<anything_graph::Flow> {
        let flow = MODELS
            .get()
            .unwrap()
            .lock()
            .await
            .delete_flow(flow_name)
            .unwrap();

        let _ = self
            .file_store
            .delete_directory(&["flows", &flow.name])
            .unwrap();

        Ok(flow)
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
    /// a `CoordinatorResult` containing a value of type `anything_graph::Flow`.
    pub async fn update_flow(&self, flow_name: String) -> CoordinatorResult<anything_graph::Flow> {
        let flow = MODELS
            .get()
            .unwrap()
            .lock()
            .await
            .update_flow(&flow_name)
            .unwrap();

        Ok(flow)
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

    pub fn trigger_repo(&self) -> CoordinatorResult<TriggerRepoImpl> {
        match &self.repositories {
            Some(repositories) => Ok(repositories.trigger_repo.clone()),
            None => Err(CoordinatorError::RepoNotInitialized),
        }
    }

    /*
    INTERNAL FUNCTIONS
     */

    // Internal
    async fn setup_file_handler(&mut self) {
        let (tx, mut rx) = tokio::sync::mpsc::channel(4096);
        let file_store = Arc::new(Mutex::new(self.file_store.clone()));

        let client = new_client::<StoreChangesPublisher>().await.unwrap();

        // Listen for changes on the file system
        let _t1 = tokio::spawn(async move {
            let mut fs = file_store.try_lock().expect("should be unlockable");
            fs.notify_changes(tx.clone()).await.unwrap();
        });

        // Send changes to the coordinator
        let _t2 = tokio::spawn(async move {
            while let Some(msg) = rx.recv().await {
                let _ = client
                    .publish(
                        "file-system-change",
                        StoreChangesPublisher::ChangeMessage(msg),
                    )
                    .await;
                // let _ = sender.send(StoreChangesPublisher::ChangeMessage(msg)).await;
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
        events::{FlowPublisher, InternalEventsPublisher, NewFlowPublisher, StringPublisher},
        test_helper::add_flow_directory,
    };
    use anything_graph::Flowfile;
    use anything_mq::new_client;
    use anything_runtime::{EngineKind, PluginEngine};
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
        version = "0.1"
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

        let mut deno_engine = PluginEngine::default();
        deno_engine.engine = "system-shell".to_string();
        deno_engine.args = Some(vec!["echo 'hello {{cheers}}'".to_string()]);
        assert_eq!(runtime.engine, Some(EngineKind::PluginEngine(deno_engine)));

        // po_sender.send(()).unwrap();
    }

    #[actix::test]
    async fn test_subscribe_to_store_changes() {
        let manager = Manager::default();
        let config = AnythingConfig::default();
        let (stop_tx, stop_rx) = mpsc::channel(1);

        let mut cloned_manager = manager.clone();
        // arc_m.start(stop_rx, ready_tx).await;
        actix_rt::spawn(async move {
            let _res = cloned_manager.start(stop_rx).await;
        });

        let store = manager.file_store.clone();
        // let server_task = tokio::spawn(async move {
        //     let _v = ready_rx.recv().await;
        //     sleep(Duration::from_millis(100)).await;
        //     let res = store.write_file(&["just_a_test.txt"], "test".as_bytes());
        //     let _ = sleep(Duration::from_millis(100));
        //     assert!(res.is_ok());
        //     stop_tx.send(()).await.unwrap();
        // });

        // let res = timeout(Duration::from_secs(1), server_task).await;
        // assert!(res.is_ok(), "server task did not quit");
    }

    // #[tokio::test]
    // async fn test_started_manager_receives_system_events() {
    //     let _manager = Manager::default();
    //     let config = AnythingConfig::default();
    //     let (stop_tx, stop_rx) = mpsc::channel(1);
    //     let (ready_tx, _ready_rx) = mpsc::channel(1);

    //     let client = new_client().await.unwrap();

    //     tokio::spawn(async move {
    //         start(config.clone(), stop_rx, ready_tx).await.unwrap();
    //     });

    //     let server_task = tokio::spawn(async move {
    //         client
    //             .publish("ping", InternalEventsPublisher::Ping)
    //             .await
    //             .unwrap();
    //         sleep(Duration::from_millis(600)).await;
    //         stop_tx.send(()).await.unwrap();
    //     });

    //     let res = timeout(Duration::from_secs(10), server_task).await;
    //     assert!(res.is_ok(), "Server task did not quit");
    // }

    // #[tokio::test]
    // async fn test_started_manager_receives_system_events_and_shutsdown_the_system() {
    //     let _manager = Manager::default();
    //     let config = AnythingConfig::default();
    //     let (stop_tx, stop_rx) = mpsc::channel(1);
    //     let (ready_tx, _ready_rx) = mpsc::channel(1);

    //     let client = new_client().await.unwrap();

    //     tokio::spawn(async move {
    //         start(config.clone(), stop_rx, ready_tx).await.unwrap();
    //     });

    //     let server_task = tokio::spawn(async move {
    //         client
    //             .publish("stop", InternalEventsPublisher::Shutdown)
    //             .await
    //             .unwrap();
    //         stop_tx.send(()).await.unwrap();
    //     });

    //     let res = timeout(Duration::from_secs(10), server_task).await;
    //     assert!(res.is_ok(), "Server task did not quit");
    // }

    // #[tokio::test]
    // #[ignore]
    // async fn test_process_flow_store_change() {
    //     let config = get_unique_config();

    //     let (stop_tx, stop_rx) = mpsc::channel(1);
    //     let (ready_tx, mut ready_rx) = mpsc::channel(1);

    //     let _listener_task = tokio::spawn(async move {
    //         start(config.clone(), stop_rx, ready_tx).await.unwrap();
    //     });

    //     let manager = ready_rx.recv().await.unwrap();
    //     let rpath = manager.file_store.store_path(&["flows"]);

    //     let manager_clone = manager.clone();

    //     add_flow_directory(rpath.clone(), "some-simple-flow");

    //     let manager = manager_clone;
    //     sleep(Duration::from_millis(SLEEP_TIME)).await;
    //     let flows = manager.get_flows().await.unwrap();
    //     assert_eq!(flows.len(), 1);

    //     sleep(Duration::from_millis(SLEEP_TIME)).await;
    //     add_flow_directory(rpath.clone(), "one-other-flow");
    //     sleep(Duration::from_millis(SLEEP_TIME)).await;
    //     let flows = manager.get_flows().await.unwrap();
    //     assert_eq!(flows.len(), 2);

    //     manager.file_store.cleanup_base_dir().unwrap();

    //     stop_tx.send(()).await.unwrap();
    // }

    // #[tokio::test]
    // #[ignore]
    // async fn test_process_flow_can_fetch_loaded_flow() {
    //     let config = get_unique_config();

    //     let (stop_tx, stop_rx) = mpsc::channel(1);
    //     let (ready_tx, mut ready_rx) = mpsc::channel(1);

    //     let _listener_task = tokio::spawn(async move {
    //         start(config.clone(), stop_rx, ready_tx).await.unwrap();
    //     });

    //     let manager = ready_rx.recv().await.unwrap();
    //     let rpath = manager.file_store.store_path(&["flows"]);

    //     add_flow_directory(rpath.clone(), "some-simple-flow");
    //     sleep(Duration::from_millis(SLEEP_TIME)).await;
    //     let flow = manager.get_flow("some-simple-flow").await.unwrap();
    //     assert_eq!(flow.name, "some-simple-flow");

    //     manager.file_store.cleanup_base_dir().unwrap();

    //     stop_tx.send(()).await.unwrap();
    // }

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
