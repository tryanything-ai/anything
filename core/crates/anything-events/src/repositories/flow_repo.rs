use std::io::BufReader;

use anything_core::utils::sha256_digest;
use anything_graph::flow::flowfile::Flowfile;
use chrono::Utc;
use sqlx::{Row, SqlitePool};

use crate::{
    errors::{EventsError, EventsResult},
    models::flow::{CreateFlow, Flow, FlowId, FlowVersion, FlowVersionId},
};

#[async_trait::async_trait]
pub trait FlowRepo {
    async fn create_flow(&self, create_flow: CreateFlow) -> EventsResult<FlowId>;
    async fn get_flows(&self) -> EventsResult<Vec<(Flow, FlowVersion)>>;
    async fn get_flow_by_id(&self, flow_id: FlowId) -> EventsResult<(Flow, FlowVersion)>;
}

#[derive(Debug, Clone)]
pub struct FlowRepoImpl {
    #[cfg(debug_assertions)]
    pub pool: SqlitePool,
}

impl FlowRepoImpl {
    pub fn new(pool: &SqlitePool) -> Self {
        Self { pool: pool.clone() }
    }

    pub async fn save_flow(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
        flow_id: String,
        version_id: String,
        create_flow: CreateFlow,
    ) -> EventsResult<FlowId> {
        let row = sqlx::query(
            r#"
        INSERT INTO flows (flow_id, flow_name, active, latest_version_id, updated_at)
        VALUES (?1, ?2, ?3, ?4, ?5)
        RETURNING flow_id
        "#,
        )
        .bind(flow_id.clone())
        .bind(create_flow.flow_name)
        .bind(create_flow.active)
        .bind(version_id.clone())
        .bind(Utc::now().timestamp())
        .fetch_one(&mut **tx)
        .await
        .map_err(|e| {
            EventsError::DatabaseError(crate::errors::DatabaseError::DBError(Box::new(e)))
        })?;

        Ok(row.get("flow_id"))
    }

    async fn save_flow_version(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
        flow_id: String,
        version_id: String,
        create_flow: CreateFlow,
    ) -> EventsResult<FlowVersionId> {
        let create_flow_clone = create_flow.clone();
        // TODO: decide if this is how we want to handle the input or not
        let input = format!(
            r#"
            id = "{}"
            name = "{}"
            version = "{}"
            description = "{}"
            
        "#,
            flow_id.clone(),
            create_flow.flow_name.clone(),
            create_flow.version.unwrap_or("0.0.1".to_string()),
            create_flow.description.unwrap_or_default()
        );
        let reader = BufReader::new(input.as_bytes());
        let checksum = sha256_digest(reader)?;
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
        .bind(checksum.as_ref())
        .bind(input)
        .bind(Utc::now().timestamp())
        .fetch_one(&mut **tx).await.map_err(|e| {
            EventsError::DatabaseError(crate::errors::DatabaseError::DBError(Box::new(e)))
        })?;

        Ok(row.get("version_id"))
    }

    pub async fn get_latest_version_for(&self, flow_id: String) -> EventsResult<FlowVersion> {
        let row = sqlx::query_as::<_, FlowVersion>(
            r#"
                SELECT * FROM flow_versions WHERE version_id IN (SELECT latest_version_id FROM flows WHERE flow_id = ?1)
            "#,
        ).bind(flow_id.clone()).fetch_one(&self.pool).await.map_err(|e| {
            EventsError::DatabaseError(crate::errors::DatabaseError::DBError(Box::new(e)))
        })?;

        Ok(row)
    }
}

#[async_trait::async_trait]
impl FlowRepo for FlowRepoImpl {
    /// Create a new flow
    async fn create_flow(&self, create_flow: CreateFlow) -> EventsResult<FlowId> {
        // Create a new flow
        let mut tx = self.pool.begin().await.map_err(|e| {
            EventsError::DatabaseError(crate::errors::DatabaseError::DBError(Box::new(e)))
        })?;
        let cloned_create_flow = create_flow.clone();
        // TODO: decide if w're keeping flow or just creating it to ensure it can be created
        let flow = match create_flow.description {
            None => Flowfile::default(),
            Some(d) => Flowfile::from_string(d).unwrap(),
        };
        let flow_id = flow.flow.id.clone();
        let version_id = uuid::Uuid::new_v4().to_string();
        // Create a new flow
        let saved_flow_id = self
            .save_flow(
                &mut tx,
                flow_id.clone(),
                version_id.clone(),
                cloned_create_flow.clone(),
            )
            .await?;
        self.save_flow_version(&mut tx, flow_id, version_id, cloned_create_flow)
            .await?;
        tx.commit().await.map_err(|e| {
            EventsError::DatabaseError(crate::errors::DatabaseError::DBError(Box::new(e)))
        })?;

        Ok(saved_flow_id)
    }

    async fn get_flow_by_id(&self, flow_id: FlowId) -> EventsResult<(Flow, FlowVersion)> {
        let row = sqlx::query_as::<_, Flow>("SELECT * from flows WHERE flow_id = ?1")
            .bind(flow_id)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| {
                EventsError::DatabaseError(crate::errors::DatabaseError::DBError(Box::new(e)))
            })?;

        let version = self
            .get_latest_version_for(row.flow_id.clone())
            .await
            .map_err(|e| {
                EventsError::DatabaseError(crate::errors::DatabaseError::DBError(Box::new(e)))
            })?;

        Ok((row, version))
    }

    async fn get_flows(&self) -> EventsResult<Vec<(Flow, FlowVersion)>> {
        let flows = sqlx::query_as::<_, Flow>("SELECT * from flows")
            .fetch_all(&self.pool)
            .await
            .map_err(|e| {
                EventsError::DatabaseError(crate::errors::DatabaseError::DBError(Box::new(e)))
            })?;

        let mut flows_and_versions: Vec<(Flow, FlowVersion)> = vec![];

        for flow in flows.into_iter() {
            let version = self
                .get_latest_version_for(flow.flow_id.clone())
                .await
                .map_err(|e| {
                    EventsError::DatabaseError(crate::errors::DatabaseError::DBError(Box::new(e)))
                })?;
            flows_and_versions.push((flow, version));
        }

        Ok(flows_and_versions)
    }
}

#[cfg(test)]
mod tests {
    use crate::internal::test_helper::TestFlowRepo;

    use super::*;
    use anyhow::Result;
    use sqlx::Row;

    #[tokio::test]
    async fn test_create_flow() -> Result<()> {
        let test = TestFlowRepo::new().await;
        let dummy_create = test.dummy_create_flow();

        let res = test.flow_repo.create_flow(dummy_create.clone()).await;
        assert!(res.is_ok());

        let flow_id = res.unwrap();

        // Get the flow from the database
        let row = sqlx::query("SELECT * FROM flows WHERE flow_id = ?1")
            .bind(flow_id.clone())
            .fetch_one(&test.pool)
            .await?;

        assert_eq!(row.get::<String, _>("flow_name"), dummy_create.flow_name);

        // Get the flow version
        let version_id = row.get::<String, _>("latest_version_id");
        let row = sqlx::query("SELECT * FROM flow_versions WHERE version_id = ?1")
            .bind(version_id)
            .fetch_one(&test.pool)
            .await?;

        assert_eq!(row.get::<String, _>("flow_id"), flow_id);

        let row = sqlx::query(
            r#"
            SELECT * FROM flow_versions WHERE version_id IN (SELECT latest_version_id FROM flows WHERE flow_id = ?1)
            "#,
        ).bind(flow_id.clone()).fetch_one(&test.pool).await?;

        assert_eq!(row.get::<String, _>("flow_id"), flow_id);
        assert_eq!(row.get::<String, _>("flow_version"), "0.0.1".to_string());

        Ok(())
    }

    #[tokio::test]
    async fn test_get_flows() -> Result<()> {
        let test = TestFlowRepo::new().await;
        let dummy_create = test.dummy_create_flow();
        let dummy_create2 = test.dummy_create_flow();

        let (flow_id, version_id) = test.insert_create_flow(dummy_create.clone()).await?;
        test.insert_create_flow_version(flow_id.clone(), version_id.clone(), dummy_create.clone())
            .await?;
        let (flow_id2, version_id2) = test.insert_create_flow(dummy_create2.clone()).await?;
        test.insert_create_flow_version(
            flow_id2.clone(),
            version_id2.clone(),
            dummy_create2.clone(),
        )
        .await?;

        let res = test.flow_repo.get_flows().await;
        assert!(res.is_ok());
        let flows = res.unwrap();

        assert_eq!(flows.len(), 2);
        let first = flows.first();
        assert_eq!(first.is_some(), true);
        let (first_flow, flow_version) = first.unwrap();
        assert_eq!(first_flow.flow_name, dummy_create.flow_name);
        assert_eq!(flow_version.flow_version, dummy_create.version.unwrap());

        Ok(())
    }

    #[tokio::test]
    async fn test_get_flow_by_id() -> Result<()> {
        let test = TestFlowRepo::new().await;
        let dummy_create = test.dummy_create_flow();

        let (flow_id, version_id) = test.insert_create_flow(dummy_create.clone()).await?;
        test.insert_create_flow_version(flow_id.clone(), version_id.clone(), dummy_create.clone())
            .await?;

        let res = test.flow_repo.get_flow_by_id(flow_id.clone()).await;
        assert!(res.is_ok());
        let (flow, flow_version) = res.unwrap();

        assert_eq!(flow.flow_name, dummy_create.flow_name);
        assert_eq!(flow_version.flow_version, dummy_create.version.unwrap());

        Ok(())
    }
}
