use std::fmt::Debug;

use anything_common::{hashing::hash_string_sha256, tracing};
use anything_graph::Flow;
use chrono::Utc;
use sqlx::{Row, Sqlite};

use crate::datastore::{Datastore, DatastoreTrait, RepoImpl};
use crate::error::{PersistenceError, PersistenceResult};
use crate::models::flow::{CreateFlow, CreateFlowVersion, FlowId, FlowVersionId, StoredFlow};

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
    async fn create_flow(&self, flow: Flow) -> PersistenceResult<()>;
    async fn get_flows(&self) -> PersistenceResult<Vec<Flow>>;
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

impl FlowRepoImpl {
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

        tx.commit()
            .await
            .map_err(|e| PersistenceError::DatabaseError(e))?;

        let saved_flow = match self.get_flow_by_id(saved_flow_id.clone()).await {
            Ok(f) => f,
            Err(e) => return Err(e),
        };

        Ok(saved_flow)
    }

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

    async fn get_flow_by_id(&self, flow_id: FlowId) -> PersistenceResult<StoredFlow> {
        let pool = self.datastore.get_pool();
        println!("get_flow_by_id: {:?}", flow_id);

        let flow =
            sqlx::query_as::<_, StoredFlow>(&format!("{} WHERE f.flow_id = ?1", GET_FLOW_SQL))
                .bind(flow_id)
                .fetch_one(pool)
                .await
                .map_err(|e| PersistenceError::DatabaseError(e))?;

        Ok(flow)
    }
}

#[cfg(test)]
mod tests {
    use crate::test_helper::{get_test_datastore, select_all_flows};

    use super::*;

    #[tokio::test]
    async fn test_create_flow() {
        let datastore = get_test_datastore().await.unwrap();
        // let flow = Flow::default();
        let flow_repo = FlowRepoImpl::new_with_datastore(datastore).unwrap();

        let create_flow = CreateFlow {
            name: "test".to_string(),
            active: false,
            version: None,
        };

        let res = flow_repo.create_flow(create_flow).await;

        let all = select_all_flows(&flow_repo.datastore.get_pool())
            .await
            .unwrap();
        println!("all: {:?}", all);
        println!("res: {:?}", res);
        assert!(res.is_ok());

        let stored_flow = res.unwrap();

        println!("stored_flow: {:?}", stored_flow);
    }
}
