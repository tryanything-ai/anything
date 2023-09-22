use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::sqlite::SqliteRow;
use sqlx::{Column, FromRow};
use sqlx::{Error, Row};

use crate::generated::flows::{
    CreateFlow as ProtoCreateFlow, CreateFlowVersion as ProtoCreateFlowVersion, Flow as ProtoFlow,
    FlowVersion as ProtoFlowVersion, Node as ProtoNode, UpdateFlow as ProtoUpdateFlow,
    UpdateFlowVersion as ProtoUpdateFlowVersion, Variable as ProtoVariable,
};

pub type FlowId = String;
pub type FlowVersionId = String;

/*
string key = 1;
    string value = 2;
*/
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Variable {
    pub key: String,
    pub value: Value,
}

impl Into<ProtoVariable> for Variable {
    fn into(self) -> ProtoVariable {
        ProtoVariable {
            key: self.key,
            value: self.value.to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Node {
    pub id: String,
    pub name: String,
    pub label: String,
    pub action: String,
    pub dependencies: Vec<Self>,
    pub variables: Vec<Variable>,
}
impl Into<ProtoNode> for Node {
    fn into(self) -> ProtoNode {
        ProtoNode {
            id: self.id,
            name: self.name,
            label: self.label,
            action: self.action,
            dependencies: self.dependencies.into_iter().map(|d| d.into()).collect(),
            variables: self.variables.into_iter().map(|v| v.into()).collect(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Flow {
    pub flow_id: FlowId,
    pub flow_name: String,
    pub latest_version_id: FlowVersionId,
    pub active: bool,
    pub updated_at: DateTime<Utc>,
    pub versions: Vec<FlowVersion>,
    // pub versions: Vec<FlowVersion>,
    // pub description: Option<String>,
    // pub nodes: Vec<Node>,
}

impl Into<ProtoFlow> for Flow {
    // use crate::generated::
    fn into(self) -> ProtoFlow {
        ProtoFlow {
            flow_id: self.flow_id,
            flow_name: self.flow_name,
            version: self.latest_version_id,
            active: self.active,
            flow_versions: Vec::default(),
            // versions: self.versions.into_iter().map(|v| v.into()).collect(),
            // description: self.description.map(Description::Present),
            // nodes: self.nodes.into_iter().map(|n| n.into()).collect(),
        }
    }
}

impl From<ProtoFlow> for Flow {
    fn from(value: ProtoFlow) -> Self {
        Self {
            flow_id: value.flow_id,
            flow_name: value.flow_name,
            latest_version_id: value.version,
            active: value.active,
            updated_at: Utc::now(),
            versions: Vec::default(),
        }
    }
}

impl FromRow<'_, SqliteRow> for Flow {
    fn from_row(row: &'_ SqliteRow) -> Result<Self, Error> {
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
                flow_version: row.get::<'_, String, &str>("fv_version"),
                description: row.get::<'_, Option<String>, &str>("fv_description"),
                flow_definition: serde_json::from_str(&flow_def).unwrap(),
                checksum: row.get::<'_, String, &str>("fv_checksum"),
                version_id: row.get::<'_, String, &str>("fv_id"),
                published: row.get::<'_, bool, &str>("fv_published"),
                updated_at: row.get::<'_, Option<DateTime<Utc>>, &str>("fv_updated_at"),
            };
            versions.push(flow_version);
        }
        Ok(Self {
            flow_id,
            flow_name,
            latest_version_id,
            active,
            updated_at,
            versions,
        })
    }

    // fn from_row(row: &'_ SqliteRow) -> std::result::Result<Self, Error> {
    //     Ok(VerificationRecord {
    //         rowid: row.get::<'_, i64, &str>("id") as u64,
    //         name,
    //         address: hex::encode(row.get::<'_, Vec<u8>, &str>("pub_key")),
    //         event: hex::encode(row.get::<'_, Vec<u8>, &str>("event_id")),
    //         event_created: row.get::<'_, DateTime<Utc>, &str>("created_at").timestamp() as u64,
    //         last_success: match row.try_get::<'_, DateTime<Utc>, &str>("verified_at") {
    //             Ok(x) => Some(x.timestamp() as u64),
    //             _ => None,
    //         },
    //         last_failure: match row.try_get::<'_, DateTime<Utc>, &str>("failed_at") {
    //             Ok(x) => Some(x.timestamp() as u64),
    //             _ => None,
    //         },
    //         failure_count: row.get::<'_, i32, &str>("fail_count") as u64,
    //     })
    // }
}

#[derive(FromRow, Debug, Serialize, Deserialize, Clone, Default)]
pub struct FlowVersion {
    pub flow_id: FlowId,
    pub flow_version: String,
    pub description: Option<String>,
    pub flow_definition: Value,
    pub checksum: String,
    pub version_id: String,
    pub published: bool,
    pub updated_at: Option<DateTime<Utc>>,
}

impl Into<ProtoFlowVersion> for FlowVersion {
    fn into(self) -> ProtoFlowVersion {
        use crate::generated::flows::flow_version::Description;
        let updated_at = match self.updated_at {
            None => Utc::now().timestamp(),
            Some(t) => t.timestamp(),
        };
        ProtoFlowVersion {
            version_id: self.version_id,
            flow_id: self.flow_id,
            version: self.flow_version,
            description: self.description.map(Description::Present),
            flow_definition: self.flow_definition.to_string(),
            published: self.published,
            updated_at,
        }
    }
}

/*
       flow_id TEXT PRIMARY KEY NOT NULL,
    flow_version TEXT NOT NULL,
    description TEXT NOT NULL,
    checksum TEXT NOT NULL,
    updated_at timestamp with time zone DEFAULT (CURRENT_TIMESTAMP),
    published BOOLEAN NOT NULL DEFAULT FALSE,
    flow_definition json NOT NULL,
*/
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CreateFlow {
    pub flow_name: String,
    pub active: Option<bool>,
    pub version: Option<String>,
}

impl Into<ProtoCreateFlow> for CreateFlow {
    fn into(self) -> ProtoCreateFlow {
        use crate::generated::flows::create_flow::Version;
        // oneof version { string version_string = 1;};
        // string flow_name = 2;
        // optional bool active = 3;
        ProtoCreateFlow {
            flow_name: self.flow_name,
            version: Some(self.version.map_or_else(
                || Version::VersionString("0.0.1".to_string()),
                Version::VersionString,
            )),
            active: self.active,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UpdateFlow {
    pub flow_name: String,
    pub active: bool,
    pub version: Option<String>,
    pub description: Option<String>,
}

impl Into<ProtoUpdateFlow> for UpdateFlow {
    fn into(self) -> ProtoUpdateFlow {
        // use crate::generated::flows::update_flow::{Description, Version};
        ProtoUpdateFlow {
            flow_name: self.flow_name,
            version: Some(self.version.unwrap_or("0.0.1".to_string())),
            description: Some(self.description.unwrap_or("".to_string())),
            active: self.active,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CreateFlowVersion {
    pub flow_id: FlowId,
    pub version: Option<String>,
    pub flow_definition: String,
    pub published: Option<bool>,
    pub description: Option<String>,
}

impl Default for CreateFlowVersion {
    fn default() -> Self {
        Self {
            flow_id: "".to_string(),
            version: Some("0.0.1".to_string()),
            flow_definition: "{}".to_string(),
            published: Some(false),
            description: None,
        }
    }
}

impl Into<ProtoCreateFlowVersion> for CreateFlowVersion {
    fn into(self) -> ProtoCreateFlowVersion {
        use crate::generated::flows::create_flow_version::{Description, Version};
        // oneof version { string version_string = 1;};
        // string flow_name = 2;
        // optional bool active = 3;
        ProtoCreateFlowVersion {
            flow_id: self.flow_id,
            version: Some(self.version.map_or_else(
                || Version::VersionString("0.0.1".to_string()),
                Version::VersionString,
            )),
            published: self.published,
            description: self.description.map(Description::Present),
            flow_definition: self.flow_definition,
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

impl Into<ProtoUpdateFlowVersion> for UpdateFlowVersion {
    fn into(self) -> ProtoUpdateFlowVersion {
        ProtoUpdateFlowVersion {
            version: Some(self.version.unwrap_or_default()),
            flow_definition: Some(self.flow_definition.unwrap_or_default()),
            published: self.published,
            description: self.description,
        }
    }
}
