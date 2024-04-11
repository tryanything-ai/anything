#![allow(unused)]
use std::path::PathBuf;

use crate::datastore::types::DatastoreTrait;
use crate::datastore::types::RepoImpl;
use crate::models::event::StoreEvent;
use crate::models::flow::{CreateFlow, CreateFlowVersion, FlowVersion, StoredFlow};
use crate::models::trigger::StoredTrigger;
use crate::repositories::flow_repo::FlowRepo;
use crate::repositories::flow_repo::FlowRepoImpl;
use crate::{
    datastore::sqlite::SqliteDatastore,
    error::{PersistenceError, PersistenceResult},
};
use anything_common::tracing;
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::SqlitePool;

#[cfg(feature = "sqlite")]
#[allow(unused)]
pub async fn get_test_datastore() -> PersistenceResult<SqliteDatastore> {
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect("sqlite::memory:")
        .await
        .map_err(|e| PersistenceError::DatabaseError(e))?;

    let res = sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .map_err(|e| {
            tracing::error!("Error running migrations: {}", e);
            PersistenceError::MigrationError(e)
        });

    assert!(res.is_ok());

    let ds = SqliteDatastore::new_with_pool(pool).await.unwrap();

    Ok(ds)
}

pub struct TestFlowHelper {
    pub datastore: SqliteDatastore,
}

impl TestFlowHelper {
    pub fn new(datastore: SqliteDatastore) -> Self {
        Self { datastore }
    }
    pub async fn select_all_flows(&self) -> PersistenceResult<Vec<StoredFlow>> {
        let pool = self.datastore.get_pool();
        let query = sqlx::query_as::<_, StoredFlow>(r#"SELECT * FROM flows"#);

        let result = query.fetch_all(pool).await.map_err(|e| {
            tracing::error!("Error fetching flows: {}", e);
            PersistenceError::DatabaseError(e)
        })?;

        Ok(result)
    }

    pub async fn select_all_flow_versions(
        &self,
        flow_id: String,
    ) -> PersistenceResult<Vec<FlowVersion>> {
        let pool = self.datastore.get_pool();
        let query =
            sqlx::query_as::<_, FlowVersion>(r#"SELECT * FROM flow_versions WHERE flow_id = ?1"#)
                .bind(flow_id);

        let result = query.fetch_all(pool).await.map_err(|e| {
            tracing::error!("Error fetching flows: {}", e);
            PersistenceError::DatabaseError(e)
        })?;

        Ok(result)
    }

    pub async fn make_create_flows(&self, names: Vec<String>) -> Vec<CreateFlow> {
        let mut flows = Vec::default();
        for name in names {
            flows.push(self.make_create_flow(name).await);
        }
        flows
    }

    pub async fn make_create_flow(&self, name: String) -> CreateFlow {
        CreateFlow {
            name,
            active: false,
            version: None,
        }
    }

    pub async fn make_flow_version(
        &self,
        flow_id: String,
        flow_version: String,
    ) -> CreateFlowVersion {
        CreateFlowVersion {
            flow_id,
            description: None,
            flow_definition: serde_json::json!("{}"),
            published: None,
            version: Some(flow_version),
        }
    }

    pub fn make_unique_stored_flow(&self) -> StoredFlow {
        let mut stored_flow = StoredFlow::default();
        stored_flow.flow_name = uuid::Uuid::new_v4().to_string();
        stored_flow
    }

    pub async fn create_flow(&self, create_flow: CreateFlow) -> StoredFlow {
        let flow_repo = FlowRepoImpl::new_with_datastore(self.datastore.clone()).unwrap();
        let res = flow_repo.create_flow(create_flow).await;
        assert!(res.is_ok());
        res.unwrap()
    }

    pub async fn create_flow_version(
        &self,
        flow_id: String,
        create_flow_version: CreateFlowVersion,
    ) -> FlowVersion {
        let flow_repo = FlowRepoImpl::new_with_datastore(self.datastore.clone()).unwrap();
        let res = flow_repo
            .create_flow_version(flow_id, create_flow_version)
            .await;
        assert!(res.is_ok());
        res.unwrap()
    }

    pub async fn get_flow_by_id(&self, name: String) -> Option<StoredFlow> {
        let flow_repo = FlowRepoImpl::new_with_datastore(self.datastore.clone()).unwrap();
        match flow_repo.get_flow_by_id(name).await {
            Ok(flow) => Some(flow),
            Err(_) => None,
        }
    }

    pub async fn find_flow_by_name(&self, name: String) -> Option<StoredFlow> {
        let flow_repo = FlowRepoImpl::new_with_datastore(self.datastore.clone()).unwrap();
        match flow_repo.get_flow_by_name(name).await {
            Ok(flow) => Some(flow),
            Err(_) => None,
        }
    }
}

pub struct TestTriggerHelper {
    pub datastore: SqliteDatastore,
}

impl TestTriggerHelper {
    pub fn new(datastore: SqliteDatastore) -> Self {
        Self { datastore }
    }

    // pub async fn get_trigger_by_id(&self, trigger_id: String) -> StoredTrigger {
    //     let pool = self.datastore.get_pool();
    //     let trigger =
    //         sqlx::query_as::<_, StoredTrigger>(r#"SELECT * from triggers WHERE trigger_id = ?1"#)
    //             .bind(&trigger_id)
    //             .fetch_one(pool)
    //             .await
    //             .map_err(|e| PersistenceError::DatabaseError(e))
    //             .unwrap();

    //     trigger
    // }
}

pub struct TestEventHelper {
    pub datastore: SqliteDatastore,
}

impl TestEventHelper {
    pub fn new(datastore: SqliteDatastore) -> Self {
        Self { datastore }
    }

    pub async fn get_event_by_id(&self, event_id: String) -> StoreEvent {
        let pool = self.datastore.get_pool();
        let row = sqlx::query_as::<_, StoreEvent>("SELECT * from events WHERE id = ?1")
            .bind(event_id)
            .fetch_one(pool)
            .await
            .map_err(|e| PersistenceError::DatabaseError(e))
            .expect("unable to get event by id");

        row
    }
}

pub(crate) fn add_flow_directory(path: PathBuf, name: &str) {
    let mut flow_path = path.clone();
    flow_path.push(name);
    std::fs::create_dir_all(flow_path.clone()).unwrap();

    add_flow_file_into_directory(flow_path, name);
}

pub(crate) fn add_flow_file_into_directory(path: PathBuf, name: &str) {
    let mut flow_path = path.clone();
    // flow_path.push("flows");
    std::fs::create_dir_all(flow_path.clone()).unwrap();
    flow_path.push("flow.toml");
    let toml = format!(
        r#"
    name = "{}"
    version = "0.0.1"
    description = "test flow"

    [[nodes]]
    name = "echo"

    [nodes.engine]
    engine = "bash"
    args = ["echo", "hello world"]

    "#,
        name
    );
    std::fs::write(flow_path, toml).unwrap();
}
