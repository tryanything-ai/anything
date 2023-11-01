use crate::datastore::types::DatastoreTrait;
use crate::models::flow::{CreateFlow, CreateFlowVersion, FlowVersion, StoredFlow};
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
}
