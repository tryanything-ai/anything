use std::fmt::Debug;

use anything_common::tracing;
use anything_graph::Flow;
use chrono::Utc;
use sqlx::Row;

use crate::datastore::{DatabaseTransaction, RepoImpl};
use crate::models::flow::{CreateFlow, StoredFlow};
use crate::{
    datastore::Datastore,
    error::{PersistenceError, PersistenceResult},
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
    async fn create_flow(&self, flow: Flow) -> PersistenceResult<()>;
    async fn get_flows(&self) -> PersistenceResult<Vec<Flow>>;
}

#[derive()]
pub struct FlowRepoImpl<DB>
where
    DB: sqlx::Database,
{
    pub datastore: Box<dyn Datastore<DB>>,
}

impl<DB> Debug for FlowRepoImpl<DB>
where
    DB: sqlx::Database,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Implement the Debug trait for FlowRepoImpl as needed.
        // You can format the struct fields or customize the output here.
        write!(f, "FlowRepoImpl {{ /* Format your fields here */ }}")
    }
}

#[async_trait::async_trait]
impl<DB> RepoImpl<DB> for FlowRepoImpl<DB>
where
    DB: sqlx::Database,
{
    fn new_with_datastore(datastore: Box<dyn Datastore<DB>>) -> PersistenceResult<Self> {
        Ok(FlowRepoImpl { datastore })
    }
}

impl<DB> FlowRepoImpl<DB>
where
    DB: sqlx::Database + Send + Sync,
{
    async fn create_flow(&self, create_flow: CreateFlow) -> PersistenceResult<StoredFlow> {
        let tx = (*self.datastore).begin_transaction().await?;
        let mut tx = tx.sqlite().unwrap().clone();

        let flow_name = create_flow.name.clone();
        let version_id = uuid::Uuid::new_v4().to_string();

        let saved_flow_id = self.save(tx, flow_name, version_id, create_flow).await?;

        let res = tx
            .commit()
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

    async fn get_flow_by_id(&self, flow_id: String) -> PersistenceResult<StoredFlow> {
        let flow =
            sqlx::query_as::<_, StoredFlow>(&format!("{} WHERE f.flow_id = ?1", GET_FLOW_SQL))
                .bind(flow_id)
                .fetch_one(&self.datastore.get_pool())
                .await?;

        Ok(flow)
    }
}

#[cfg(test)]
mod tests {
    use crate::test_helper::get_test_datastore;

    use super::*;

    #[tokio::test]
    async fn test_create_flow() {
        let datastore = get_test_datastore().await.unwrap();
        // let flow = Flow::default();
        let flow_repo = FlowRepoImpl::new_with_datastore(Box::new(datastore)).unwrap();

        let create_flow = CreateFlow {
            name: "test".to_string(),
            active: None,
            version: None,
        };

        let res = flow_repo.create_flow(create_flow).await;
        assert!(res.is_ok());

        let stored_flow = res.unwrap();

        println!("stored_flow: {:?}", stored_flow);
    }
}
