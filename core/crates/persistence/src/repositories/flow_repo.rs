use std::fmt::Debug;

use anything_common::{hashing::hash_string_sha256, tracing};
use chrono::Utc;
use sqlx::Row;

use crate::datastore::{Datastore, DatastoreTrait, RepoImpl};
use crate::error::{PersistenceError, PersistenceResult};
use crate::models::flow::{
    CreateFlow, CreateFlowVersion, FlowId, FlowVersion, FlowVersionId, StoredFlow, UpdateFlowArgs,
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
    async fn create_or_update_flow(&self, flow: StoredFlow) -> PersistenceResult<StoredFlow>;
    async fn get_flow_by_id(&self, flow_id: FlowId) -> PersistenceResult<StoredFlow>;
    async fn get_flows(&self) -> PersistenceResult<Vec<StoredFlow>>;
    async fn get_flow_by_name(&self, name: String) -> PersistenceResult<StoredFlow>;
    async fn create_flow_version(
        &self,
        flow_id: String,
        flow_version: CreateFlowVersion,
    ) -> PersistenceResult<FlowVersion>;
    async fn get_flow_versions(&self, flow_id: FlowId) -> PersistenceResult<Vec<FlowVersion>>;
    async fn get_flow_version_by_id(
        &self,
        flow_id: FlowId,
        flow_version: FlowVersionId,
    ) -> PersistenceResult<FlowVersion>;
    async fn update_flow(
        &self,
        flow_id: FlowId,
        update_flow: UpdateFlowArgs,
    ) -> PersistenceResult<StoredFlow>;
    async fn update_flow_version(
        &self,
        flow_id: FlowId,
        version_id: FlowVersionId,
        update_flow_version: UpdateFlowVersion,
    ) -> PersistenceResult<FlowVersion>;
    async fn delete_flow(&self, name: String) -> PersistenceResult<FlowId>;
    async fn delete_flow_version(&self, version_id: FlowVersionId) -> PersistenceResult<bool>;
    async fn reset(&self) -> PersistenceResult<()>;
}

#[derive(Clone)]
pub struct FlowRepoImpl {
    pub datastore: Datastore,
}

impl Debug for FlowRepoImpl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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

        let flow_id = uuid::Uuid::new_v4().to_string();
        let flow_version = "0.0.0".to_string();

        let saved_flow = self
            .internal_save(&mut tx, flow_id, flow_version.clone(), create_flow.into())
            .await?;

        tx.commit()
            .await
            .map_err(|e| PersistenceError::DatabaseError(e))?;

        Ok(saved_flow)
    }

    async fn create_or_update_flow(&self, flow: StoredFlow) -> PersistenceResult<StoredFlow> {
        let mut tx = self.get_transaction().await?;

        let flow_id = flow.flow_id.clone();
        let res = self
            .internal_save(
                &mut tx,
                flow_id.clone(),
                flow.latest_version_id.clone(),
                flow,
            )
            .await;

        tx.commit()
            .await
            .map_err(|e| PersistenceError::DatabaseError(e))?;
        res
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
        let mut tx = self.get_transaction().await?;

        let res = match self
            .internal_find_existing_flow_by_id(&mut tx, flow_id.clone())
            .await
        {
            Some(existing_flow) => Ok(existing_flow),
            None => Err(PersistenceError::FlowNotFound(flow_id)),
        };

        tx.commit()
            .await
            .map_err(|e| PersistenceError::DatabaseError(e))?;

        res
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

        tracing::trace!("Found flows: {:#?}", flows);

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
        // let pool = self.datastore.get_pool();
        let mut tx = self.get_transaction().await?;

        let res = self
            .internal_find_existing_flow_by_name(&mut tx, name.clone())
            .await;

        tx.commit()
            .await
            .map_err(|e| PersistenceError::DatabaseError(e))?;

        match res {
            Some(existing_flow) => Ok(existing_flow),
            None => Err(PersistenceError::FlowNotFound(name)),
        }
        // let flow = sqlx::query_as("SELECT * FROM flows WHERE flow_name = ?1")
        //     .bind(name)
        //     .fetch_one(pool)
        //     .await
        //     .map_err(|e| PersistenceError::DatabaseError(e))?;
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
    ) -> PersistenceResult<FlowVersion> {
        let mut tx = self.get_transaction().await?;

        let flow_version_id = self
            .internal_save_flow_version(&mut tx, flow_id.clone(), flow_version)
            .await?;

        let flow_version = self
            .internal_find_existing_flow_version_by_id(&mut tx, flow_id, flow_version_id)
            .await?;

        tx.commit()
            .await
            .map_err(|e| PersistenceError::DatabaseError(e))?;

        Ok(flow_version)
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
        args: UpdateFlowArgs,
    ) -> PersistenceResult<StoredFlow> {
        let mut tx = self.get_transaction().await?;

        let res = self
            .internal_rename_existing_flow(&mut tx, flow_id, args)
            .await;

        tx.commit()
            .await
            .map_err(|e| PersistenceError::DatabaseError(e))?;

        res
    }

    async fn update_flow_version(
        &self,
        flow_id: FlowId,
        flow_version_id: FlowVersionId,
        update_flow: UpdateFlowVersion,
    ) -> PersistenceResult<FlowVersion> {
        let mut tx = self.get_transaction().await?;

        let res = self
            .internal_update_existing_flow_version(&mut tx, flow_id, flow_version_id, update_flow)
            .await?;

        tx.commit().await?;

        Ok(res)
    }

    async fn delete_flow(&self, flow_id: String) -> PersistenceResult<FlowId> {
        let mut tx = self.get_transaction().await?;

        let res = match self
            .internal_find_existing_flow_by_id(&mut tx, flow_id.clone())
            .await
        {
            Some(stored_flow) => {
                self.internal_delete_flow_by_id(&mut tx, stored_flow.flow_id)
                    .await
            }
            None => Err(PersistenceError::FlowNotFound(flow_id)),
        };

        tx.commit().await?;

        res
    }

    async fn delete_flow_version(&self, version_id: FlowVersionId) -> PersistenceResult<bool> {
        let mut tx = self.get_transaction().await?;

        let res = self
            .internal_delete_flow_version_by_id(&mut tx, version_id)
            .await?;
        tx.commit().await?;

        Ok(res)
    }

    async fn reset(&self) -> PersistenceResult<()> {
        let mut tx = self.get_transaction().await?;

        let _res = self.internal_reset(&mut tx).await?;
        tx.commit().await?;

        Ok(())
    }
}

impl FlowRepoImpl {
    /// Save a flow
    ///
    /// If a flow exists already, this will update the flow with the StoredFlow parameters
    /// otherwise it will create a new flow
    async fn internal_save(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
        flow_id: String,
        flow_version: String,
        flow: StoredFlow,
    ) -> PersistenceResult<StoredFlow> {
        match self
            .internal_find_existing_flow_by_id(tx, flow_id.clone())
            .await
        {
            Some(mut existing_flow) => {
                existing_flow.latest_version_id = flow_version.clone();
                let update_flow: UpdateFlowArgs = flow.clone().into();
                self.internal_rename_existing_flow(tx, flow_id, update_flow)
                    .await
            }
            None => {
                let mut create_flow: CreateFlow = flow.into();
                create_flow.version = Some(flow_version);
                self.internal_create_new_flow(tx, flow_id, create_flow)
                    .await
            }
        }
    }

    /// Create a new flow in the database
    /// Also triggers creating a version of the flow
    async fn internal_create_new_flow(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
        flow_id: String,
        create_flow: CreateFlow,
    ) -> PersistenceResult<StoredFlow> {
        let create_flow_clone = create_flow.clone();
        let flow_name: String = create_flow_clone.name.clone();
        let row = sqlx::query(
            r#"
            INSERT INTO flows (flow_id, flow_name, active, latest_version_id, updated_at)
            VALUES (?1, ?2, ?3, ?4, ?5)
            RETURNING flow_id
            "#,
        )
        .bind(flow_id.clone())
        .bind(create_flow_clone.name)
        .bind(create_flow_clone.active)
        .bind(create_flow_clone.version.clone())
        .bind(Utc::now().timestamp())
        .fetch_one(&mut **tx)
        .await
        .map_err(|e| PersistenceError::DatabaseError(e))?;

        tracing::debug!("Inserted new flow, now saving flow version");

        let flow_id: String = row.get("flow_id");

        let save_flow_version: CreateFlowVersion = create_flow.into();

        self.internal_save_flow_version(tx, flow_id.clone(), save_flow_version)
            .await?;

        match self.internal_find_existing_flow_by_id(tx, flow_id).await {
            Some(existing_flow) => Ok(existing_flow),
            None => Err(PersistenceError::FlowNotFound(flow_name)),
        }
    }

    async fn internal_rename_existing_flow(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
        flow_id: String,
        args: UpdateFlowArgs,
    ) -> PersistenceResult<StoredFlow> {
        let flow = sqlx::query_as::<_, StoredFlow>(
            r#"
            UPDATE flows SET flow_name = ?1, updated_at = ?2, active = ?3 WHERE flow_id = ?4
            RETURNING *
            "#,
        )
        .bind(args.flow_name)
        .bind(Utc::now().timestamp())
        .bind(args.active)
        .bind(flow_id.clone())
        .fetch_one(&mut **tx)
        .await
        .map_err(|e| {
            tracing::error!("Unable to update flow by id: {:?}", flow_id.clone());
            PersistenceError::DatabaseError(e)
        })?;

        let flow_id = flow.flow_id;

        match self
            .internal_find_existing_flow_by_id(tx, flow_id.clone())
            .await
        {
            Some(existing_flow) => Ok(existing_flow),
            None => Err(PersistenceError::FlowNotFound(flow_id)),
        }
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
    async fn internal_save_flow_version(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
        flow_id: String,
        create_flow_version: CreateFlowVersion,
    ) -> PersistenceResult<FlowVersionId> {
        let create_flow_clone = create_flow_version.clone();
        // TODO: decide if this is how we want to handle the input or not
        let input = format!(
            r#"{{"id": "{}", "version": "{}", "description": "{}"}}"#,
            flow_id.clone(),
            create_flow_version.version.unwrap_or("0.0.1".to_string()),
            create_flow_version.description.unwrap_or_default()
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

    async fn internal_update_existing_flow_version(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
        flow_id: FlowId,
        flow_version_id: FlowVersionId,
        update_flow_version: UpdateFlowVersion,
    ) -> PersistenceResult<FlowVersion> {

        println!("Flow ID: {:?}", flow_id);
        println!("Flow Version ID: {:?}", flow_version_id);
        println!("Update Flow Version: {:?}", update_flow_version);

        let current_flow_version = sqlx::query_as::<_, FlowVersion>(
            r#"
            SELECT * FROM flow_versions WHERE flow_id = ?1 AND flow_version = ?2
            "#,
        )
        .bind(flow_id.clone())
        .bind(flow_version_id.clone())
        .fetch_one(&mut **tx)
        .await
        .map_err(|e| PersistenceError::DatabaseError(e))?;

        println!("Response from SQLx query: {:?}", current_flow_version);

        let definition = match update_flow_version.flow_definition {
            Some(d) => d,
            None => current_flow_version.description.clone().unwrap_or_default(),
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
        .bind(flow_version_id.clone())
        .bind(flow_id.clone())
        .fetch_one(&mut **tx)
        .await
        .map_err(|e| PersistenceError::DatabaseError(e))?;

        let version_id = row.get("flow_version");
        self.internal_find_existing_flow_version_by_id(tx, flow_id, version_id)
            .await
    }

    async fn internal_find_existing_flow_version_by_id(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
        flow_id: FlowId,
        flow_version: FlowVersionId,
    ) -> PersistenceResult<FlowVersion> {
        let flow_version = sqlx::query_as::<_, FlowVersion>(
            r#"
        SELECT * FROM flow_versions WHERE flow_id = ?1 AND flow_version = ?2
            "#,
        )
        .bind(flow_id.clone())
        .bind(flow_version)
        .fetch_one(&mut **tx)
        .await
        .map_err(|e| PersistenceError::DatabaseError(e))?;

        Ok(flow_version)
    }

    async fn internal_find_existing_flow_by_id(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
        flow_id: String,
    ) -> Option<StoredFlow> {
        match sqlx::query_as::<_, StoredFlow>(&format!("{} WHERE f.flow_id = ?1", GET_FLOW_SQL))
            .bind(flow_id.clone())
            .fetch_one(&mut **tx)
            .await
        {
            Ok(existing_flow) => Some(existing_flow),
            Err(e) => {
                tracing::debug!("Error finding existing flow by id {:?}: {:?}", flow_id, e);
                None
            }
        }
    }

    #[inline]
    #[allow(unused)]
    async fn internal_find_existing_flow_by_name(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
        flow_id: String,
    ) -> Option<StoredFlow> {
        match sqlx::query_as::<_, StoredFlow>(&format!("{} WHERE f.flow_name = ?1", GET_FLOW_SQL))
            .bind(flow_id)
            .fetch_one(&mut **tx)
            .await
        {
            Ok(existing_flow) => Some(existing_flow),
            Err(_e) => None,
        }
    }

    async fn internal_delete_flow_by_id(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
        flow_id: String,
    ) -> PersistenceResult<String> {
        let row = sqlx::query(
            r#"
            DELETE FROM flows WHERE flow_id = ?1
            RETURNING flow_id
            "#,
        )
        .bind(flow_id)
        .fetch_one(&mut **tx)
        .await
        .map_err(|e| PersistenceError::DatabaseError(e))?;

        let flow_id = row.get("flow_id");

        Ok(flow_id)
    }

    async fn internal_delete_flow_version_by_id(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
        version_id: FlowVersionId,
    ) -> PersistenceResult<bool> {
        let row = sqlx::query(
            r#"
            DELETE FROM flow_versions WHERE flow_version = ?1
            "#,
        )
        .bind(version_id)
        .execute(&mut **tx)
        .await
        .map_err(|e| PersistenceError::DatabaseError(e))?;

        let rows_affected = row.rows_affected();

        Ok(rows_affected == 1)
    }

    async fn internal_reset(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
    ) -> PersistenceResult<()> {
        sqlx::query("DELETE FROM flows")
            .execute(&mut **tx)
            .await
            .map_err(|e| PersistenceError::DatabaseError(e))?;

        sqlx::query("DELETE FROM flow_versions")
            .execute(&mut **tx)
            .await
            .map_err(|e| PersistenceError::DatabaseError(e))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use anything_store::FileStore;

    use crate::test_helper::{add_flow_directory, get_test_datastore, TestFlowHelper};

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
    async fn test_can_create_flow_with_create_or_update_flow_when_it_does_not_exist() {
        let datastore = get_test_datastore().await.unwrap();
        let test_helper = TestFlowHelper::new(datastore.clone());
        let flow_repo = FlowRepoImpl::new_with_datastore(datastore).unwrap();

        let stored_flow = test_helper.make_unique_stored_flow();
        let flow_id = stored_flow.flow_id.clone();

        let res = flow_repo.create_or_update_flow(stored_flow).await;
        assert!(res.is_ok());
        let saved_flow = res.unwrap();
        let stored_flow = test_helper.get_flow_by_id(flow_id.clone()).await;
        assert!(stored_flow.is_some());
        assert_eq!(stored_flow.unwrap().flow_name, saved_flow.flow_name);
    }

    #[tokio::test]
    async fn test_can_create_flow_with_create_or_update_flow_when_it_already_exists() {
        let datastore = get_test_datastore().await.unwrap();
        let test_helper = TestFlowHelper::new(datastore.clone());
        let flow_repo = FlowRepoImpl::new_with_datastore(datastore).unwrap();

        // Create a new flow
        let stored_flow = test_helper.make_unique_stored_flow();
        let flow_name = stored_flow.flow_name.clone();
        let res = flow_repo
            .create_flow(stored_flow.clone().into())
            .await
            .unwrap();
        let flow_id = res.flow_id.clone();

        // confirm it already exists
        let found_existing_flow = flow_repo.get_flow_by_id(flow_id.clone()).await.unwrap();
        assert_eq!(res.flow_name, found_existing_flow.flow_name);
        assert!(!res.active);

        // Now update it and change status to active
        let mut stored_flow = test_helper.make_unique_stored_flow();
        stored_flow.flow_id = flow_id.clone();
        stored_flow.flow_name = flow_name.clone();
        stored_flow.active = true;

        let res = flow_repo.create_or_update_flow(stored_flow).await;
        assert!(res.is_ok());

        let saved_flow = res.unwrap();
        let stored_flow = test_helper.get_flow_by_id(flow_id.clone()).await;
        assert!(stored_flow.is_some());
        let stored_flow = stored_flow.unwrap();
        assert_eq!(stored_flow.flow_name, saved_flow.flow_name);
        assert!(stored_flow.active);
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
        let flow_version = flow_repo
            .create_flow_version(flow_name.clone(), create_flow_version.clone())
            .await;
        assert!(flow_version.is_ok());

        let flow_version = flow_version.unwrap();
        assert_eq!(flow_version.flow_id, flow_name);
        assert_eq!(flow_version.flow_version, "v0.0.1");

        let flow_version = flow_repo
            .get_flow_version_by_id(flow_name.clone(), "0.0.0".to_string())
            .await;
        assert!(flow_version.is_ok());
        let flow_version = flow_version.unwrap();
        assert_eq!(flow_version.flow_id, flow_name);
        assert_eq!(flow_version.flow_version, "0.0.0");
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

        let update_flow = UpdateFlowArgs {
            flow_name: "test2".to_string(),
            active: false,
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

    #[tokio::test]
    async fn test_get_flow_from_stored_flow() {
        let datastore = get_test_datastore().await.unwrap();
        let test_helper = TestFlowHelper::new(datastore.clone());
        let flow_repo = FlowRepoImpl::new_with_datastore(datastore).unwrap();

        let root_dir = tempfile::tempdir().unwrap().path().to_path_buf();
        let mut file_store = FileStore::create(root_dir.as_path(), &["anything"]).unwrap();

        // Create the flow on the file system and in the database
        let flow_name = "test".to_string();
        let create_flow = test_helper.make_create_flow(flow_name.clone()).await;
        let _res = flow_repo.create_flow(create_flow.clone()).await;

        // Two files in the directory
        add_flow_directory(file_store.store_path(&["flows"]), flow_name.as_str());
        let _ = file_store.write_file(
            &["flows", "some-flow", "note.txt"],
            "just a note".as_bytes(),
        );

        // Now get the flow from the database
        let stored_flow = flow_repo.get_flow_by_name(flow_name.clone()).await.unwrap();
        let flow = stored_flow.get_flow(&mut file_store).await.unwrap();

        assert_eq!(flow.name, "test".to_string());
        assert_eq!(flow.version, "v0.0.1".to_string());
        assert_eq!(flow.description, "test flow".to_string());
    }

    #[tokio::test]
    async fn test_can_delete_a_flow() {
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

        let all_flows = test_helper
            .select_all_flows()
            .await
            .expect("unable to select all flows");
        assert_eq!(all_flows.len(), 2);

        let res = flow_repo.delete_flow("alpha".to_string()).await;
        assert!(res.is_ok());

        let all_flows = test_helper
            .select_all_flows()
            .await
            .expect("unable to select all flows");
        assert_eq!(all_flows.len(), 1);
    }

    #[tokio::test]
    async fn test_can_delete_a_flow_version() {
        let datastore = get_test_datastore().await.unwrap();
        let test_helper = TestFlowHelper::new(datastore.clone());
        let flow_repo = FlowRepoImpl::new_with_datastore(datastore).unwrap();

        let flow_name = "test".to_string();
        let create_flow = test_helper.make_create_flow(flow_name.clone()).await;
        let res = flow_repo.create_flow(create_flow.clone()).await;
        assert!(res.is_ok());

        // Create first version
        let create_flow_version = test_helper
            .make_flow_version(flow_name.clone(), format!("v0.0.{}", 1))
            .await;
        let res = flow_repo
            .create_flow_version(flow_name.clone(), create_flow_version.clone())
            .await;
        assert!(res.is_ok());

        // Create a second version
        let create_flow_version = test_helper
            .make_flow_version(flow_name.clone(), format!("v0.0.{}", 2))
            .await;
        let res = flow_repo
            .create_flow_version(flow_name.clone(), create_flow_version.clone())
            .await;
        assert!(res.is_ok());

        // Confirm there are two versions
        let flow_versions = test_helper
            .select_all_flow_versions(flow_name.clone())
            .await;

        assert!(flow_versions.is_ok());
        assert_eq!(flow_versions.unwrap().len(), 3);

        // Delete the first version
        let res = flow_repo
            .delete_flow_version("v0.0.1".to_string())
            .await
            .unwrap();
        assert!(res);

        // Confirm there is one version
        let flow_versions = test_helper
            .select_all_flow_versions(flow_name.clone())
            .await;

        assert!(flow_versions.is_ok());
        assert_eq!(flow_versions.unwrap().len(), 2);
    }
}
