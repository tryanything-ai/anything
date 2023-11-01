use std::fmt::Debug;

use anything_common::{hashing::hash_string_sha256, tracing};
use chrono::Utc;
use sqlx::Row;

use crate::datastore::{Datastore, DatastoreTrait, RepoImpl};
use crate::error::{PersistenceError, PersistenceResult};
use crate::models::flow::{
    CreateFlow, CreateFlowVersion, FlowId, FlowVersion, FlowVersionId, StoredFlow, UpdateFlow,
    UpdateFlowVersion,
};

const GET_FLOW_SQL: &str = r#"
SELECT  f.flow_id, 
        f.flow_name, 
        f.latest_version_id,
        f.active, 
        f.updated_at,
        fv.flow_version AS fv_flow_version,
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
    async fn get_flow_version_by_id(
        &self,
        flow_id: FlowId,
        flow_version: FlowVersionId,
    ) -> PersistenceResult<FlowVersion>;
    async fn update_flow(
        &self,
        flow_id: FlowId,
        update_flow: UpdateFlow,
    ) -> PersistenceResult<StoredFlow>;
    async fn update_flow_version(
        &self,
        flow_id: FlowId,
        version_id: FlowVersionId,
        update_flow_version: UpdateFlowVersion,
    ) -> PersistenceResult<FlowVersion>;
}

#[derive(Clone)]
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
        let flow_version = "v0.0.0".to_string();

        let saved_flow_id = self
            .save(
                &mut tx,
                flow_name.clone(),
                flow_version.clone(),
                create_flow.clone(),
            )
            .await?;

        let save_flow_version: CreateFlowVersion = create_flow.into();

        self.save_flow_version(&mut tx, flow_name, save_flow_version)
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

    /// The function `create_flow_version` inserts a new flow version into a database table and returns
    /// the generated version ID.
    ///
    /// Arguments:
    ///
    /// * `flow_id`: The `flow_id` parameter is a `String` that represents the ID of the flow for which
    /// a new version is being created.
    /// * `flow_version`: The `flow_version` parameter is of type `CreateFlowVersion`, which is a struct
    /// or object that contains the following fields:
    ///
    /// Returns:
    ///
    /// a `PersistenceResult<String>`.
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

        let row = sqlx::query(r#"
        INSERT INTO flow_versions (flow_id, flow_version, description, checksum, flow_definition, updated_at)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6)
            RETURNING flow_version
        "#)
        .bind(flow_id)
        .bind(create_flow_version.version.unwrap_or("0.0.1".to_string()))
        .bind(create_flow_version.description.unwrap_or_default())
        .bind(checksum)
        .bind(flow_version.flow_definition)
        .bind(Utc::now().timestamp())
        .fetch_one(pool).await.map_err(|e| PersistenceError::DatabaseError(e))?;

        Ok(row.get("flow_version"))
    }

    /// The function `get_flow_versions` retrieves all versions of a flow from a database using SQL.
    ///
    /// Arguments:
    ///
    /// * `flow_id`: The `flow_id` parameter is the identifier of a flow. It is used to query the
    /// database for all versions of the flow with the specified `flow_id`.
    ///
    /// Returns:
    ///
    /// The function `get_flow_versions` returns a `PersistenceResult` which is a result type that can
    /// either be an `Ok` variant containing a `Vec<FlowVersion>` or an `Err` variant containing a
    /// `PersistenceError`.
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

    /// The function `get_flow_version_by_id` retrieves a specific version of a flow from a database
    /// based on the flow ID and version ID.
    ///
    /// Arguments:
    ///
    /// * `flow_id`: The `flow_id` parameter is the identifier of a specific flow. It is used to filter
    /// the query and retrieve the flow version associated with this flow ID.
    /// * `flow_version`: The `flow_version` parameter is the version number of a specific flow. It is
    /// used to identify a particular version of a flow in the database.
    ///
    /// Returns:
    ///
    /// a `PersistenceResult<FlowVersion>`.
    async fn get_flow_version_by_id(
        &self,
        flow_id: FlowId,
        flow_version: FlowVersionId,
    ) -> PersistenceResult<FlowVersion> {
        let pool = self.datastore.get_pool();

        let flow_version = sqlx::query_as::<_, FlowVersion>(
            r#"
        SELECT * FROM flow_versions WHERE flow_id = ?1 AND flow_version = ?2
            "#,
        )
        .bind(flow_id.clone())
        .bind(flow_version)
        .fetch_one(pool)
        .await
        .map_err(|e| PersistenceError::DatabaseError(e))?;

        Ok(flow_version)
    }

    /// The function `update_flow` updates the name and updated timestamp of a flow in a database and
    /// returns the updated flow.
    ///
    /// Arguments:
    ///
    /// * `flow_id`: The `flow_id` parameter represents the unique identifier of the flow that needs to
    /// be updated.
    /// * `update_flow`: The `update_flow` parameter is of type `UpdateFlow`, which is a struct or
    /// object containing the updated flow information. It likely has a field called `flow_name` which
    /// represents the new name of the flow.
    ///
    /// Returns:
    ///
    /// The function `update_flow` returns a `PersistenceResult<StoredFlow>`.
    async fn update_flow(
        &self,
        flow_id: FlowId,
        update_flow: UpdateFlow,
    ) -> PersistenceResult<StoredFlow> {
        let pool = self.datastore.get_pool();

        let flow = sqlx::query_as::<_, StoredFlow>(
            r#"
            UPDATE flows SET flow_name = ?1, updated_at = ?2 WHERE flow_id = ?3
            RETURNING *
            "#,
        )
        .bind(update_flow.flow_name)
        .bind(Utc::now().timestamp())
        .bind(flow_id.clone())
        .fetch_one(pool)
        .await
        .map_err(|e| {
            tracing::error!("Unable to update flow by id: {:?}", flow_id.clone());
            PersistenceError::DatabaseError(e)
        })?;

        let flow_id = flow.flow_id;

        self.get_flow_by_id(flow_id).await
    }

    async fn update_flow_version(
        &self,
        flow_id: FlowId,
        flow_version: FlowVersionId,
        update_flow_version: UpdateFlowVersion,
    ) -> PersistenceResult<FlowVersion> {
        let pool = self.datastore.get_pool();

        let current_flow_version = sqlx::query_as::<_, FlowVersion>(
            r#"
            SELECT * FROM flow_versions WHERE flow_id = ?1 AND flow_version = ?2
            "#,
        )
        .bind(flow_id.clone())
        .bind(flow_version.clone())
        .fetch_one(pool)
        .await
        .map_err(|e| PersistenceError::DatabaseError(e))?;

        let definition = match update_flow_version.flow_definition {
            Some(d) => d,
            None => current_flow_version.description.clone().unwrap_or_default(),
        };

        let version = match update_flow_version.version {
            Some(v) => v,
            None => current_flow_version.flow_version.clone(),
        };

        let description = match update_flow_version.description {
            Some(d) => d,
            None => current_flow_version.description.clone().unwrap_or_default(),
        };

        let published = match update_flow_version.published {
            Some(p) => p,
            None => current_flow_version.published,
        };

        let checksum = hash_string_sha256(definition.as_str())?;

        let row = sqlx::query(
            r#"
            UPDATE flow_versions SET 
                description = ?1, updated_at = ?2, published = ?3, checksum = ?4, flow_definition = ?5
            WHERE flow_version = ?6 AND flow_id = ?7
            RETURNING flow_version
            "#,
        )
        .bind(description)
        .bind(Utc::now().timestamp())
        .bind(published)
        .bind(checksum)
        .bind(definition)
        .bind(flow_version.clone())
        .bind(flow_id.clone())
        .fetch_one(pool)
        .await
        .map_err(|e| PersistenceError::DatabaseError(e))?;

        let version_id = row.get("flow_version");
        self.get_flow_version_by_id(flow_id, version_id).await
    }
}

impl FlowRepoImpl {
    async fn save(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
        flow_id: String,
        flow_version: String,
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
        .bind(flow_version.clone())
        .bind(Utc::now().timestamp())
        .fetch_one(&mut **tx)
        .await
        .map_err(|e| PersistenceError::DatabaseError(e))?;

        Ok(row.get("flow_id"))
    }

    /// The function `save_flow_version` saves a new version of a flow in a SQLite database.
    ///
    /// Arguments:
    ///
    /// * `tx`: `tx` is a mutable reference to a `sqlx::Transaction` object. It is used to perform
    /// database operations within a transaction.
    /// * `flow_id`: The `flow_id` parameter is a `String` that represents the ID of the flow for which
    /// a new version is being created.
    /// * `create_flow`: The `create_flow` parameter is of type `CreateFlowVersion`, which is a struct
    /// or object that contains the following fields:
    ///
    /// Returns:
    ///
    /// a `PersistenceResult<FlowVersionId>`.
    async fn save_flow_version(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
        flow_id: String,
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
        INSERT INTO flow_versions (flow_id, flow_version, description, checksum, flow_definition, updated_at)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6)
        RETURNING flow_version
            "#,
        )
        .bind(flow_id.clone())
        .bind(create_flow_clone.version)
        .bind(create_flow_clone.description)
        .bind(checksum)
        .bind(input)
        .bind(Utc::now().timestamp())
        .fetch_one(&mut **tx).await
        .map_err(|e| PersistenceError::DatabaseError(e))?;

        Ok(row.get("flow_version"))
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
            stored_flow.versions[0].flow_version
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
    async fn test_can_create_a_flow_version() {
        let datastore = get_test_datastore().await.unwrap();
        let test_helper = TestFlowHelper::new(datastore.clone());
        let flow_repo = FlowRepoImpl::new_with_datastore(datastore).unwrap();

        let flow_name = "test".to_string();
        let create_flow = test_helper.make_create_flow(flow_name.clone()).await;
        let res = flow_repo.create_flow(create_flow.clone()).await;
        assert!(res.is_ok());

        let create_flow_version = test_helper
            .make_flow_version(flow_name.clone(), format!("v0.0.{}", 1))
            .await;
        let res = flow_repo
            .create_flow_version(flow_name.clone(), create_flow_version.clone())
            .await;
        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn test_can_fetch_a_flow_version_by_id() {
        let datastore = get_test_datastore().await.unwrap();
        let test_helper = TestFlowHelper::new(datastore.clone());
        let flow_repo = FlowRepoImpl::new_with_datastore(datastore).unwrap();

        let flow_name = "test".to_string();
        let create_flow = test_helper.make_create_flow(flow_name.clone()).await;
        let res = flow_repo.create_flow(create_flow.clone()).await;
        assert!(res.is_ok());

        let create_flow_version = test_helper
            .make_flow_version(flow_name.clone(), format!("v0.0.{}", 1))
            .await;
        let res = flow_repo
            .create_flow_version(flow_name.clone(), create_flow_version.clone())
            .await;
        assert!(res.is_ok());

        let flow_version = flow_repo
            .get_flow_version_by_id(flow_name.clone(), res.unwrap())
            .await;
        assert!(flow_version.is_ok());
        let flow_version = flow_version.unwrap();
        assert_eq!(flow_version.flow_id, flow_name);
        assert_eq!(flow_version.flow_version, "v0.0.1");

        let flow_version = flow_repo
            .get_flow_version_by_id(flow_name.clone(), "v0.0.0".to_string())
            .await;
        assert!(flow_version.is_ok());
        let flow_version = flow_version.unwrap();
        assert_eq!(flow_version.flow_id, flow_name);
        assert_eq!(flow_version.flow_version, "v0.0.0");
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
        let mut created_flow_flow_version = vec![];
        for i in 1..5 {
            let create_flow_version = test_helper
                .make_flow_version(flow_name.clone(), format!("v0.0.{}", i))
                .await;
            let res = flow_repo
                .create_flow_version(flow_name.clone(), create_flow_version.clone())
                .await;
            assert!(res.is_ok());
            created_flow_flow_version.push(res.unwrap());
        }

        let flows = flow_repo.get_flow_versions("test".to_string()).await;
        assert!(flows.is_ok());
        let flows = flows.unwrap();
        assert_eq!(flows.len(), 5);
        assert_eq!(flows[4].flow_id, "test");
    }

    #[tokio::test]
    async fn test_can_update_flow() {
        let datastore = get_test_datastore().await.unwrap();
        let test_helper = TestFlowHelper::new(datastore.clone());
        let flow_repo = FlowRepoImpl::new_with_datastore(datastore).unwrap();

        let create_flow = test_helper.make_create_flow("test".to_string()).await;
        let res = flow_repo.create_flow(create_flow.clone()).await;
        assert!(res.is_ok());

        let update_flow = UpdateFlow {
            flow_name: "test2".to_string(),
            version: None,
        };

        let res = flow_repo
            .update_flow(create_flow.name.clone(), update_flow)
            .await;
        assert!(res.is_ok());
        let res = res.unwrap();
        assert_eq!(res.flow_name, "test2");
    }

    #[tokio::test]
    async fn test_update_flow_version() -> Result<(), ()> {
        let datastore = get_test_datastore().await.unwrap();
        let test_helper = TestFlowHelper::new(datastore.clone());
        let flow_repo = FlowRepoImpl::new_with_datastore(datastore).unwrap();

        // Create the flow
        let flow_name = "test".to_string();
        let create_flow = test_helper.make_create_flow(flow_name.clone()).await;
        let res = flow_repo.create_flow(create_flow.clone()).await;
        assert!(res.is_ok());

        // Create a new flow version (not the default, altough that would work as well)
        let create_flow_version = test_helper
            .make_flow_version(flow_name.clone(), format!("v0.0.{}", 1))
            .await;
        let res = flow_repo
            .create_flow_version(flow_name.clone(), create_flow_version.clone())
            .await;
        assert!(res.is_ok());

        // BEFORE UPDATE
        let flow_version = flow_repo
            .get_flow_version_by_id(flow_name.clone(), "v0.0.1".to_string())
            .await;
        assert!(flow_version.is_ok());
        let original_flow_version = flow_version.unwrap();

        let flow_description = original_flow_version.description;
        let original_checksum = original_flow_version.checksum;

        let update_flow_version = UpdateFlowVersion {
            version: None,
            description: Some("new description".to_string()),
            flow_definition: Some("{}".to_string()),
            published: Some(true),
        };

        let res = flow_repo
            .update_flow_version(flow_name.clone(), "v0.0.1".to_string(), update_flow_version)
            .await;
        assert!(res.is_ok());
        let new_flow_version = res.unwrap();
        let version_id = new_flow_version.flow_version.clone();

        // AFTER UPDATE
        // Flow version doesn't change, just attributes
        assert_eq!(new_flow_version.flow_version, version_id);
        assert_ne!(original_checksum, new_flow_version.checksum);
        assert_eq!(
            new_flow_version.description,
            Some("new description".to_string())
        );
        assert_eq!(new_flow_version.published, true);

        Ok(())
    }
}
