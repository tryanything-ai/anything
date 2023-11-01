use std::fmt::Debug;

use anything_common::{hashing::hash_string_sha256, tracing};
use chrono::Utc;
use sqlx::Row;

use crate::datastore::{Datastore, DatastoreTrait, RepoImpl};
use crate::error::{PersistenceError, PersistenceResult};
use crate::models::flow::{
    CreateFlow, CreateFlowVersion, FlowId, FlowVersion, FlowVersionId, StoredFlow,
};

const GET_FLOW_SQL: &str = r#"
SELECT  f.flow_id, 
        f.flow_name, 
        f.latest_version_id,
        f.active, 
        f.updated_at,
        fv.version_id AS fv_id,
        fv.description AS fv_description,
        fv.flow_version AS fv_version,
        fv.checksum AS fv_checksum,
        fv.updated_at AS fv_updated_at,
        fv.published AS fv_published,
        fv.flow_definition AS fv_flow_definition
FROM flows f
INNER JOIN flow_versions fv ON fv.flow_id = f.flow_id
"#;

#[async_trait::async_trait]
pub trait FlowRepo {
    async fn create_flow(&self, flow: CreateFlow) -> PersistenceResult<StoredFlow>;
    async fn get_flow_by_id(&self, flow_id: FlowId) -> PersistenceResult<StoredFlow>;
    async fn get_flows(&self) -> PersistenceResult<Vec<StoredFlow>>;
    async fn get_flow_by_name(&self, name: String) -> PersistenceResult<StoredFlow>;
    async fn create_flow_version(
        &self,
        flow_id: String,
        flow_version: CreateFlowVersion,
    ) -> PersistenceResult<String>;
    async fn get_flow_versions(&self, flow_id: FlowId) -> PersistenceResult<Vec<FlowVersion>>;
}

#[derive()]
pub struct FlowRepoImpl {
    pub datastore: Datastore,
}

impl Debug for FlowRepoImpl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Implement the Debug trait for FlowRepoImpl as needed.
        // You can format the struct fields or customize the output here.
        write!(f, "FlowRepoImpl {{ /* Format your fields here */ }}")
    }
}

#[cfg(feature = "sqlite")]
#[async_trait::async_trait]
impl RepoImpl<sqlx::Sqlite> for FlowRepoImpl {
    fn new_with_datastore(datastore: Datastore) -> PersistenceResult<Self> {
        Ok(FlowRepoImpl { datastore })
    }

    async fn get_transaction<'a>(&self) -> PersistenceResult<sqlx::Transaction<'a, sqlx::Sqlite>> {
        let pool = self.datastore.get_pool();
        let tx = pool
            .begin()
            .await
            .map_err(|e| PersistenceError::DatabaseError(e))?;

        Ok(tx)
    }
}

#[async_trait::async_trait]
impl FlowRepo for FlowRepoImpl {
    /// The function `create_flow` saves a new flow and its version to a database transaction and
    /// returns the saved flow.
    ///
    /// Arguments:
    ///
    /// * `create_flow`: The `create_flow` parameter is of type `CreateFlow`, which is a struct or
    /// object that contains the necessary information to create a new flow. It likely includes
    /// properties such as the name of the flow, its version, and any other relevant data needed for the
    /// flow creation process.
    ///
    /// Returns:
    ///
    /// a `PersistenceResult<StoredFlow>`.
    async fn create_flow(&self, create_flow: CreateFlow) -> PersistenceResult<StoredFlow> {
        let mut tx = self.get_transaction().await?;

        let flow_name = create_flow.name.clone();
        let version_id = uuid::Uuid::new_v4().to_string();

        let saved_flow_id = self
            .save(
                &mut tx,
                flow_name.clone(),
                version_id.clone(),
                create_flow.clone(),
            )
            .await?;

        let save_flow_version: CreateFlowVersion = create_flow.into();

        self.save_flow_version(&mut tx, flow_name, version_id, save_flow_version)
            .await?;

        tx.commit()
            .await
            .map_err(|e| PersistenceError::DatabaseError(e))?;

        let saved_flow = match self.get_flow_by_id(saved_flow_id.clone()).await {
            Ok(f) => f,
            Err(e) => return Err(e),
        };

        Ok(saved_flow)
    }

    /// The function `get_flow_by_id` retrieves a stored flow from a database using its ID.
    ///
    /// Arguments:
    ///
    /// * `flow_id`: The `flow_id` parameter is the unique identifier of the flow that you want to
    /// retrieve from the datastore.
    ///
    /// Returns:
    ///
    /// The function `get_flow_by_id` returns a `PersistenceResult<StoredFlow>`.
    async fn get_flow_by_id(&self, flow_id: FlowId) -> PersistenceResult<StoredFlow> {
        let pool = self.datastore.get_pool();

        let flow =
            sqlx::query_as::<_, StoredFlow>(&format!("{} WHERE f.flow_id = ?1", GET_FLOW_SQL))
                .bind(flow_id)
                .fetch_one(pool)
                .await
                .map_err(|e| PersistenceError::DatabaseError(e))?;

        Ok(flow)
    }

    /// The function `get_flows` retrieves all stored flows from a database and returns them as a vector
    /// of `StoredFlow` objects.
    ///
    /// Returns:
    ///
    /// The function `get_flows` returns a `PersistenceResult` which is a result type containing either
    /// a `Vec<StoredFlow>` if the query is successful, or a `PersistenceError::DatabaseError` if there
    /// is an error during the database operation.
    async fn get_flows(&self) -> PersistenceResult<Vec<StoredFlow>> {
        let pool = self.datastore.get_pool();

        let flows = sqlx::query_as::<_, StoredFlow>("SELECT * from flows")
            .fetch_all(pool)
            .await
            .map_err(|e| PersistenceError::DatabaseError(e))?;

        Ok(flows)
    }

    /// The function `get_flow_by_name` retrieves a stored flow from a database by its name.
    ///
    /// Arguments:
    ///
    /// * `name`: A string representing the name of the flow to retrieve.
    ///
    /// Returns:
    ///
    /// The function `get_flow_by_name` returns a `PersistenceResult<StoredFlow>`.
    async fn get_flow_by_name(&self, name: String) -> PersistenceResult<StoredFlow> {
        let pool = self.datastore.get_pool();

        let flow = sqlx::query_as("SELECT * FROM flows WHERE flow_name = ?1")
            .bind(name)
            .fetch_one(pool)
            .await
            .map_err(|e| PersistenceError::DatabaseError(e))?;

        Ok(flow)
    }

    async fn create_flow_version(
        &self,
        flow_id: String,
        flow_version: CreateFlowVersion,
    ) -> PersistenceResult<String> {
        let pool = self.datastore.get_pool();

        let create_flow_version = flow_version.clone();
        let input = format!(
            r#"{{"id": "{}", "version": "{}", "description": "{}"}}"#,
            flow_id.clone(),
            flow_version.version.unwrap_or("0.0.1".to_string()),
            flow_version.description.unwrap_or_default()
        );
        let checksum = hash_string_sha256(input.as_str())?;
        let unique_version_id = uuid::Uuid::new_v4().to_string();

        let row = sqlx::query(r#"
        INSERT INTO flow_versions (flow_id, version_id, flow_version, description, checksum, flow_definition, updated_at)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
            RETURNING version_id
        "#)
        .bind(flow_id)
        .bind(unique_version_id)
        .bind(create_flow_version.version.unwrap_or("0.0.1".to_string()))
        .bind(create_flow_version.description.unwrap_or_default())
        .bind(checksum)
        .bind(flow_version.flow_definition)
        .bind(Utc::now().timestamp())
        .fetch_one(pool).await.map_err(|e| PersistenceError::DatabaseError(e))?;

        Ok(row.get("version_id"))
    }

    async fn get_flow_versions(&self, flow_id: FlowId) -> PersistenceResult<Vec<FlowVersion>> {
        let pool = self.datastore.get_pool();
        let flow_versions = sqlx::query_as::<_, FlowVersion>(
            r#"
        SELECT * FROM flow_versions WHERE flow_id = ?1
        "#,
        )
        .bind(flow_id.clone())
        .fetch_all(pool)
        .await
        .map_err(|e| PersistenceError::DatabaseError(e))?;

        Ok(flow_versions)
    }
}

impl FlowRepoImpl {
    async fn save(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
        flow_id: String,
        version_id: String,
        flow: CreateFlow,
    ) -> PersistenceResult<String> {
        // let mut tx = tx;
        let row = sqlx::query(
            r#"
            INSERT INTO flows (flow_id, flow_name, active, latest_version_id, updated_at)
            VALUES (?1, ?2, ?3, ?4, ?5)
            RETURNING flow_id
            "#,
        )
        .bind(flow_id.clone())
        .bind(flow.name)
        .bind(flow.active)
        .bind(version_id.clone())
        .bind(Utc::now().timestamp())
        .fetch_one(&mut **tx)
        .await
        .map_err(|e| PersistenceError::DatabaseError(e))?;

        Ok(row.get("flow_id"))
    }

    async fn save_flow_version(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
        flow_id: String,
        version_id: String,
        create_flow: CreateFlowVersion,
    ) -> PersistenceResult<FlowVersionId> {
        let create_flow_clone = create_flow.clone();
        // TODO: decide if this is how we want to handle the input or not
        let input = format!(
            r#"{{"id": "{}", "version": "{}", "description": "{}"}}"#,
            flow_id.clone(),
            create_flow.version.unwrap_or("0.0.1".to_string()),
            create_flow.description.unwrap_or_default()
        );
        let checksum = hash_string_sha256(input.as_str())?;
        // Create flow version
        let row = sqlx::query(
            r#"
        INSERT INTO flow_versions (version_id, flow_id, flow_version, description, checksum, flow_definition, updated_at)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
        RETURNING version_id
            "#,
        )
        .bind(version_id.clone())
        .bind(flow_id.clone())
        .bind(create_flow_clone.version)
        .bind(create_flow_clone.description)
        .bind(checksum)
        .bind(input)
        .bind(Utc::now().timestamp())
        .fetch_one(&mut **tx).await
        .map_err(|e| PersistenceError::DatabaseError(e))?;

        Ok(row.get("version_id"))
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        datastore,
        test_helper::{get_test_datastore, TestFlowHelper},
    };

    use super::*;

    #[tokio::test]
    async fn test_create_flow() {
        let datastore = get_test_datastore().await.unwrap();
        let flow_repo = FlowRepoImpl::new_with_datastore(datastore).unwrap();

        let create_flow = CreateFlow {
            name: "test".to_string(),
            active: false,
            version: None,
        };

        let res = flow_repo.create_flow(create_flow).await;

        assert!(res.is_ok());

        let stored_flow = res.unwrap();

        assert_eq!(stored_flow.flow_name, "test");
        assert_eq!(stored_flow.active, false);
        assert_eq!(
            stored_flow.latest_version_id,
            stored_flow.versions[0].version_id
        );
    }

    #[tokio::test]
    async fn test_can_fetch_flows() {
        let datastore = get_test_datastore().await.unwrap();
        let test_helper = TestFlowHelper::new(datastore.clone());
        let flow_repo = FlowRepoImpl::new_with_datastore(datastore).unwrap();

        let create_flows = test_helper
            .make_create_flows(vec!["alpha".to_string(), "beta".to_string()])
            .await;

        for f in create_flows {
            let res = flow_repo.create_flow(f.clone()).await;
            assert!(res.is_ok());
        }

        let flows = flow_repo.get_flows().await.unwrap();
        assert!(flows.len() == 2);
        assert_eq!(flows[0].flow_name, "alpha");
        assert_eq!(flows[1].flow_name, "beta");
    }

    #[tokio::test]
    async fn test_can_fetch_flow_by_id() {
        let datastore = get_test_datastore().await.unwrap();
        let test_helper = TestFlowHelper::new(datastore.clone());
        let flow_repo = FlowRepoImpl::new_with_datastore(datastore).unwrap();

        let create_flows = test_helper
            .make_create_flows(vec!["alpha".to_string(), "beta".to_string()])
            .await;

        for f in create_flows {
            let res = flow_repo.create_flow(f.clone()).await;
            assert!(res.is_ok());
        }

        let flows = flow_repo.get_flow_by_id("alpha".to_string()).await;
        assert!(flows.is_ok());
        assert_eq!(flows.unwrap().flow_name, "alpha");
    }

    #[tokio::test]
    async fn test_can_fetch_flow_by_name() {
        let datastore = get_test_datastore().await.unwrap();
        let test_helper = TestFlowHelper::new(datastore.clone());
        let flow_repo = FlowRepoImpl::new_with_datastore(datastore).unwrap();

        let create_flows = test_helper
            .make_create_flows(vec!["alpha".to_string(), "beta".to_string()])
            .await;

        for f in create_flows {
            let res = flow_repo.create_flow(f.clone()).await;
            assert!(res.is_ok());
        }

        let flows = flow_repo.get_flow_by_name("beta".to_string()).await;
        assert!(flows.is_ok());
        assert_eq!(flows.unwrap().flow_name, "beta");
    }

    #[tokio::test]
    async fn test_can_fetch_flow_versions() {
        let datastore = get_test_datastore().await.unwrap();
        let test_helper = TestFlowHelper::new(datastore.clone());
        let flow_repo = FlowRepoImpl::new_with_datastore(datastore).unwrap();

        let create_flow = test_helper.make_create_flow("test".to_string()).await;

        let res = flow_repo.create_flow(create_flow.clone()).await;
        assert!(res.is_ok());

        let flow_name = create_flow.name.clone();
        let mut created_flow_version_id = vec![];
        for i in 1..5 {
            let create_flow_version = test_helper
                .make_flow_version(flow_name.clone(), format!("v0.0.{}", i))
                .await;
            let res = flow_repo
                .create_flow_version(flow_name.clone(), create_flow_version.clone())
                .await;
            assert!(res.is_ok());
            created_flow_version_id.push(res.unwrap());
        }

        let flows = flow_repo.get_flow_versions("test".to_string()).await;
        assert!(flows.is_ok());
        let flows = flows.unwrap();
        assert_eq!(flows.len(), 5);
        assert_eq!(flows[4].flow_id, "test");
    }
}
