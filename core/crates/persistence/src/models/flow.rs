use crate::{
    error::{PersistenceError, PersistenceResult},
    models::model_types::default_bool,
};
use anything_common::tracing;
use anything_graph::Flowfile;
use anything_store::FileStore;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{sqlite::SqliteRow, Column, FromRow, Row};

pub type FlowId = String;
pub type FlowVersionId = String;

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct StoredFlow {
    pub flow_id: String,
    #[serde(rename = "name")]
    pub flow_name: String,
    pub latest_version_id: FlowVersionId,
    pub active: bool,
    pub updated_at: DateTime<Utc>,
    pub versions: Vec<FlowVersion>,
}

// SQLITE handling
impl FromRow<'_, SqliteRow> for StoredFlow {
    fn from_row(row: &'_ SqliteRow) -> Result<Self, sqlx::Error> {
        let flow_id = row.get::<'_, String, &str>("flow_id");
        let flow_name = row.get::<'_, String, &str>("flow_name");
        let latest_version_id = row.get::<'_, String, &str>("latest_version_id");
        let active = row.get::<'_, bool, &str>("active");
        let updated_at = row.get::<'_, DateTime<Utc>, &str>("updated_at");

        let mut versions = Vec::default();
        let column_names = row
            .columns()
            .iter()
            .map(|c| c.name().to_owned())
            .collect::<Vec<String>>();

        if column_names.contains(&"fv_flow_definition".to_string()) {
            let flow_def = row.get::<'_, String, &str>("fv_flow_definition");
            let flow_version = FlowVersion {
                flow_id: flow_id.clone(),
                flow_version: row.get::<'_, String, &str>("fv_flow_version"),
                description: row.get::<'_, Option<String>, &str>("fv_description"),
                flow_definition: serde_json::from_str(&flow_def).unwrap(),
                checksum: row.get::<'_, String, &str>("fv_checksum"),
                published: row.get::<'_, bool, &str>("fv_published"),
                updated_at: row.get::<'_, Option<DateTime<Utc>>, &str>("fv_updated_at"),
            };
            versions.push(flow_version);
        }

        // let flow = anything_coordinator::Manager::default().get_flow(&flow_name).unwrap();

        Ok(Self {
            flow_id,
            flow_name,
            latest_version_id,
            active,
            updated_at,
            versions,
        })
    }
}

impl From<anything_graph::Flow> for StoredFlow {
    fn from(value: anything_graph::Flow) -> Self {
        Self {
            flow_id: value.flow_id,
            flow_name: value.name,
            latest_version_id: value.version,
            active: false,
            updated_at: Utc::now(),
            versions: Vec::default(),
        }
    }
}

// To support create_or_update, we need to convert a StoredFlow into a CreateFlow
impl Into<CreateFlow> for StoredFlow {
    fn into(self) -> CreateFlow {
        CreateFlow {
            name: self.flow_name,
            active: self.active,
            version: Some(self.latest_version_id),
        }
    }
}

impl Into<UpdateFlowArgs> for StoredFlow {
    fn into(self) -> UpdateFlowArgs {
        UpdateFlowArgs {
            flow_name: self.flow_name,
            active: self.active,
            version: Some(self.latest_version_id),
        }
    }
}

impl Into<StoredFlow> for CreateFlow {
    fn into(self) -> StoredFlow {
        StoredFlow {
            flow_id: "".to_string(),
            flow_name: self.name,
            latest_version_id: "".to_string(),
            active: self.active,
            updated_at: Utc::now(),
            versions: Vec::default(),
        }
    }
}

impl StoredFlow {
    pub async fn get_flow(
        &self,
        file_store: &mut FileStore,
    ) -> PersistenceResult<anything_graph::Flow> {
        let flow_path = file_store
            .store_path(&["flows"])
            .join(self.flow_name.clone());

        tracing::trace!("flow_path: {:?}", flow_path);

        let files_in_flow_dir = file_store
            .get_files_in_dir(&[&flow_path.as_os_str().to_str().unwrap()], &["toml"])
            .await
            .map_err(|e| PersistenceError::StoreError(e))?;

        let flow_file_path = files_in_flow_dir.iter().find(|f| {
            tracing::debug!("{:?}", f.display());
            f.file_name().unwrap().to_str().unwrap().starts_with("flow")
        });

        match flow_file_path {
            Some(flow_file) => {
                let flow_file = Flowfile::from_file(flow_file.to_path_buf()).expect(
                    format!(
                        "unable to create a Flowfile from file: {}",
                        flow_file.display()
                    )
                    .as_str(),
                );
                let flow = flow_file.into();
                Ok(flow)
            }
            None => Err(PersistenceError::FlowNotFound(format!(
                "Flow not found in path: {:?}",
                flow_path
            ))),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, sqlx::FromRow)]
pub struct CreateFlow {
    pub name: String,
    #[serde(default = "default_bool::<false>")]
    pub active: bool,
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct FlowVersion {
    pub flow_id: FlowId,
    pub flow_version: String,
    pub description: Option<String>,
    pub flow_definition: serde_json::Value,
    pub checksum: String,
    pub published: bool,
    pub updated_at: Option<DateTime<Utc>>,
}

impl FromRow<'_, SqliteRow> for FlowVersion {
    fn from_row(row: &'_ SqliteRow) -> Result<Self, sqlx::Error> {
        let flow_id = row.get::<'_, String, &str>("flow_id");
        let flow_version = row.get::<'_, String, &str>("flow_version");

        let flow_definition = row.get::<'_, String, &str>("flow_definition");
        let description = row.get::<'_, Option<String>, &str>("description");
        let checksum = row.get::<'_, String, &str>("checksum");
        let published = row.get::<'_, bool, &str>("published");
        let updated_at = row.get::<'_, Option<DateTime<Utc>>, &str>("updated_at");

        Ok(FlowVersion {
            flow_id,
            flow_version,
            flow_definition: serde_json::from_str(&flow_definition).unwrap(),
            description,
            checksum,
            published,
            updated_at,
        })
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CreateFlowVersion {
    pub flow_id: FlowId,
    pub version: Option<String>,
    pub flow_definition: serde_json::Value,
    pub published: Option<bool>,
    pub description: Option<String>,
}

impl Default for CreateFlowVersion {
    fn default() -> Self {
        Self {
            flow_id: "".to_string(),
            version: Some("0.0.1".to_string()),
            flow_definition: serde_json::json!("{}"),
            published: Some(false),
            description: None,
        }
    }
}

impl Into<CreateFlowVersion> for CreateFlow {
    fn into(self) -> CreateFlowVersion {
        CreateFlowVersion {
            flow_id: self.name.clone(),
            version: Some("0.0.0".to_string()),
            flow_definition: serde_json::json!("{}"),
            published: Some(false),
            description: None,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UpdateFlowArgs {
    pub flow_name: String,
    pub active: bool,
    pub version: Option<String>,
}

impl UpdateFlowArgs {
    pub fn new(flow_name: String) -> Self {
        Self {
            flow_name,
            active: false,
            version: None,
        }
    }
}

impl From<anything_graph::Flow> for UpdateFlowArgs {
    fn from(value: anything_graph::Flow) -> Self {
        Self {
            flow_name: value.name,
            active: false,
            version: Some(value.version),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UpdateFlowVersion {
    pub version: Option<String>,
    pub flow_definition: Option<String>,
    pub published: Option<bool>,
    pub description: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conversion_from_graph_flow_into_stored_flow() {
        let flow = anything_graph::FlowBuilder::default()
            .name("some-flow".to_string())
            .version("v0.1.1".to_string())
            .build()
            .unwrap();

        let stored_flow: StoredFlow = flow.into();
        assert_eq!(stored_flow.flow_name, "some-flow".to_string());
        assert_eq!(stored_flow.latest_version_id, "v0.1.1".to_string());
    }
}
