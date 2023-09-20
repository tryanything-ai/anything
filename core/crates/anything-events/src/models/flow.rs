use chrono::{DateTime, Utc};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::FromRow;

use crate::generated::flows::{
    CreateFlow as ProtoCreateFlow, Flow as ProtoFlow, FlowVersion as ProtoFlowVersion,
    Node as ProtoNode, UpdateFlow as ProtoUpdateFlow, UpdateFlowVersion as ProtoUpdateFlowVersion,
    Variable as ProtoVariable,
};

pub type FlowId = String;
pub type FlowVersionId = String;

/*
string key = 1;
    string value = 2;
*/
#[derive(Debug, Serialize, Deserialize, Clone, Builder)]
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

#[derive(FromRow, Debug, Serialize, Deserialize, Clone, Builder)]
pub struct Flow {
    pub flow_id: FlowId,
    pub flow_name: String,
    pub latest_version_id: FlowVersionId,
    pub active: bool,
    pub updated_at: DateTime<Utc>,
    // pub description: Option<String>,
    // pub versions: Vec<FlowVersion>,
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
            // description: self.description.map(Description::Present),
            // nodes: self.nodes.into_iter().map(|n| n.into()).collect(),
            // versions: self.versions.into_iter().map(|v| v.into()).collect(),
        }
    }
}

#[derive(FromRow, Debug, Serialize, Deserialize, Clone, Builder)]
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
    pub active: bool,
    pub version: Option<String>,
    pub description: Option<String>,
}

impl Into<ProtoCreateFlow> for CreateFlow {
    fn into(self) -> ProtoCreateFlow {
        use crate::generated::flows::create_flow::{Description, Version};
        ProtoCreateFlow {
            flow_id: uuid::Uuid::new_v4().to_string(),
            flow_name: self.flow_name,
            version: Some(self.version.map_or_else(
                || Version::VersionString("0.0.1".to_string()),
                Version::VersionString,
            )),
            description: self.description.map(Description::Present),
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
        use crate::generated::flows::update_flow::{Description, Version};
        ProtoUpdateFlow {
            flow_name: self.flow_name,
            version: Some(self.version.map_or_else(
                || Version::VersionString("0.0.1".to_string()),
                Version::VersionString,
            )),
            description: self.description.map(Description::Present),
            active: self.active,
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
        use crate::generated::flows::update_flow_version::{
            Description, FlowDefinition, Published, Version,
        };
        ProtoUpdateFlowVersion {
            version: Some(self.version.map_or_else(
                || Version::VersionString("0.0.1".to_string()),
                Version::VersionString,
            )),
            flow_definition: self
                .flow_definition
                .map(FlowDefinition::FlowDefinitionString),
            published: self.published.map(Published::PublishedBool),
            description: self.description.map(Description::Present),
        }
    }
}
