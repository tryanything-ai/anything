use anything_core::hashing::hash_string_sha256;
use anything_graph::flow::flowfile::Flowfile;
use chrono::Utc;

use sqlx::{Row, SqlitePool};

use crate::{
    errors::{EventsError, EventsResult},
    models::flow::{
        CreateFlow, Flow, FlowId, FlowVersion, FlowVersionId, UpdateFlow, UpdateFlowVersion,
    },
};

const GET_FLOW_SQL: &str = r#"
SELECT f.flow_id, f.flow_name, f.latest_version_id,
        f.active, f.updated_at,
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
    async fn create_flow(&self, create_flow: CreateFlow) -> EventsResult<FlowId>;
    async fn get_flows(&self) -> EventsResult<Vec<Flow>>;
    async fn get_flow_by_id(&self, flow_id: FlowId) -> EventsResult<Flow>;
    async fn update_flow(&self, flow_id: FlowId, update_flow: UpdateFlow) -> EventsResult<FlowId>;
    async fn update_flow_version(
        &self,
        flow_id: FlowId,
        version_id: FlowVersionId,
        update_flow_version: UpdateFlowVersion,
    ) -> EventsResult<FlowVersionId>;
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
        .map_err(|e| EventsError::DatabaseError(e))?;

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
        .map_err(|e| EventsError::DatabaseError(e))?;

        Ok(row.get("version_id"))
    }

    pub async fn get_latest_version_for(&self, flow_id: String) -> EventsResult<FlowVersion> {
        let row = sqlx::query_as::<_, FlowVersion>(
            r#"
                SELECT * FROM flow_versions WHERE version_id IN (SELECT latest_version_id FROM flows WHERE flow_id = ?1)
            "#,
        ).bind(flow_id.clone()).fetch_one(&self.pool).await
        .map_err(|e| {
            EventsError::DatabaseError(e)
        })?;

        Ok(row)
    }
}

#[async_trait::async_trait]
impl FlowRepo for FlowRepoImpl {
    /// Create a new flow
    async fn create_flow(&self, create_flow: CreateFlow) -> EventsResult<FlowId> {
        // Create a new flow
        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(|e| EventsError::DatabaseError(e))?;
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
        tx.commit()
            .await
            .map_err(|e| EventsError::DatabaseError(e))?;

        Ok(saved_flow_id)
    }

    async fn get_flow_by_id(&self, flow_id: FlowId) -> EventsResult<Flow> {
        let flow = sqlx::query_as::<_, Flow>(&format!("{} WHERE f.flow_id = ?1", GET_FLOW_SQL))
            .bind(flow_id)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| EventsError::DatabaseError(e))?;

        // let version = self
        //     .get_latest_version_for(row.flow_id.clone())
        //     .await
        //     .map_err(|e| {
        //         EventsError::DatabaseError(e)
        //     })?;

        Ok(flow)
    }

    async fn get_flows(&self) -> EventsResult<Vec<Flow>> {
        let flows = sqlx::query_as::<_, Flow>(GET_FLOW_SQL)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| EventsError::DatabaseError(e))?;

        // let mut flows_and_versions: Vec<(Flow, FlowVersion)> = vec![];

        // for flow in flows.into_iter() {
        //     let version = self
        //         .get_latest_version_for(flow.flow_id.clone())
        //         .await
        //         .map_err(|e| EventsError::DatabaseError(e))?;
        //     flows_and_versions.push((flow, version));
        // }

        Ok(flows)
    }

    async fn update_flow(&self, flow_id: FlowId, update_flow: UpdateFlow) -> EventsResult<FlowId> {
        let flow = sqlx::query_as::<_, Flow>(
            r#"
            UPDATE flows SET flow_name = ?1, active = ?2, updated_at = ?3 WHERE flow_id = ?4
            RETURNING *
            "#,
        )
        .bind(update_flow.flow_name)
        .bind(update_flow.active)
        .bind(Utc::now().timestamp())
        .bind(flow_id.clone())
        .fetch_one(&self.pool)
        .await
        .map_err(|e| EventsError::DatabaseError(e))?;

        Ok(flow.flow_id)
    }

    async fn update_flow_version(
        &self,
        flow_id: FlowId,
        version_id: FlowVersionId,
        update_flow_version: UpdateFlowVersion,
    ) -> EventsResult<FlowVersionId> {
        let current_flow_version = sqlx::query_as::<_, FlowVersion>(
            r#"
            SELECT * FROM flow_versions WHERE flow_id = ?1 AND version_id = ?2
            "#,
        )
        .bind(flow_id.clone())
        .bind(version_id.clone())
        .fetch_one(&self.pool)
        .await
        .map_err(|e| EventsError::DatabaseError(e))?;

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
                flow_version = ?1, description = ?2, updated_at = ?3, published = ?4, checksum = ?5
            WHERE version_id = ?6 AND flow_id = ?7
            RETURNING version_id
            "#,
        )
        .bind(version)
        .bind(description)
        .bind(Utc::now().timestamp())
        .bind(published)
        .bind(checksum)
        .bind(version_id.clone())
        .bind(flow_id.clone())
        .fetch_one(&self.pool)
        .await
        .map_err(|e| EventsError::DatabaseError(e))?;

        Ok(row.get("version_id"))
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
        let first = first.unwrap();
        assert_eq!(first.flow_name, dummy_create.flow_name);
        assert_eq!(
            first.versions[0].flow_version,
            dummy_create.version.unwrap()
        );

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
        let flow = res.unwrap();

        assert_eq!(flow.flow_name, dummy_create.flow_name);
        assert_eq!(flow.versions[0].flow_version, dummy_create.version.unwrap());

        Ok(())
    }

    #[tokio::test]
    async fn test_update_flow() -> Result<()> {
        let test = TestFlowRepo::new().await;
        let dummy_create = test.dummy_create_flow();

        let (flow_id, version_id) = test.insert_create_flow(dummy_create.clone()).await?;
        test.insert_create_flow_version(flow_id.clone(), version_id.clone(), dummy_create.clone())
            .await?;

        let update_flow = UpdateFlow {
            flow_name: "new name".to_string(),
            active: true,
            version: Some("0.0.2".to_string()),
            description: Some("new description".to_string()),
        };

        // BEFORE UPDATE
        let row = sqlx::query("SELECT * FROM flows WHERE flow_id = ?1")
            .bind(flow_id.clone())
            .fetch_one(&test.pool)
            .await?;

        let flow_name: String = row.get("flow_name");

        assert_eq!(flow_name, dummy_create.flow_name);
        assert_eq!(row.get::<bool, _>("active"), false);

        let res = test
            .flow_repo
            .update_flow(flow_id.clone(), update_flow.clone())
            .await;
        assert!(res.is_ok());
        let flow_id = res.unwrap();

        // AFTER UPDATE
        let row = sqlx::query("SELECT * FROM flows WHERE flow_id = ?1")
            .bind(flow_id.clone())
            .fetch_one(&test.pool)
            .await?;

        let flow_name: String = row.get("flow_name");

        assert_eq!(flow_name, "new name".to_string());
        assert_eq!(row.get::<bool, _>("active"), true);

        Ok(())
    }

    #[tokio::test]
    async fn test_update_flow_version() -> Result<()> {
        let test = TestFlowRepo::new().await;
        let dummy_create = test.dummy_create_flow();

        let (flow_id, version_id) = test.insert_create_flow(dummy_create.clone()).await?;
        test.insert_create_flow_version(flow_id.clone(), version_id.clone(), dummy_create.clone())
            .await?;

        let update_flow_version = UpdateFlowVersion {
            version: None,
            description: Some("new description".to_string()),
            flow_definition: Some("{}".to_string()),
            published: Some(true),
        };

        // BEFORE UPDATE
        let row = sqlx::query("SELECT * FROM flow_versions WHERE flow_id = ?1 AND version_id = ?2")
            .bind(flow_id.clone())
            .bind(version_id.clone())
            .fetch_one(&test.pool)
            .await?;

        let flow_description: String = row.get("description");
        let original_checksum: String = row.get("checksum");

        assert_eq!(flow_description, "".to_string());
        assert_eq!(row.get::<bool, _>("published"), false);

        let res = test
            .flow_repo
            .update_flow_version(
                flow_id.clone(),
                version_id.clone(),
                update_flow_version.clone(),
            )
            .await;
        assert!(res.is_ok());
        let version_id = res.unwrap();

        // AFTER UPDATE
        let flow_version = sqlx::query_as::<_, FlowVersion>(
            "SELECT * FROM flow_versions WHERE flow_id = ?1 AND version_id = ?2",
        )
        .bind(flow_id.clone())
        .bind(version_id.clone())
        .fetch_one(&test.pool)
        .await?;

        assert_eq!(flow_version.flow_id, flow_id);
        assert_eq!(
            flow_version.description,
            Some("new description".to_string())
        );
        assert_eq!(flow_version.published, true);
        assert_ne!(flow_version.checksum, original_checksum);

        // assert_eq!(row.get::<bool, _>("active"), true);

        Ok(())
    }
}
