use chrono::{DateTime, Utc};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::FromRow;

use crate::generated::flows::{
    Flow as ProtoFlow, FlowVersion as ProtoFlowVersion, Node as ProtoNode,
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
    pub latest_active_version: FlowVersionId,
    pub published: bool,
    pub updated_at: DateTime<Utc>,
    pub description: Option<String>,
    pub versions: Vec<FlowVersion>,
    pub nodes: Vec<Node>,
}

impl Into<ProtoFlow> for Flow {
    // use crate::generated::
    fn into(self) -> ProtoFlow {
        use crate::generated::flows::flow::Description;
        ProtoFlow {
            flow_id: self.flow_id,
            flow_name: self.flow_name,
            version: self.latest_active_version,
            published: self.published,
            nodes: self.nodes.into_iter().map(|n| n.into()).collect(),
            description: self.description.map(Description::Present),
            versions: self.versions.into_iter().map(|v| v.into()).collect(),
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
    pub latest_active_version: String,
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
            flow_id: self.flow_id,
            version: self.flow_version,
            description: self.description.map(Description::Present),
            flow_definition: self.flow_definition.to_string(),
            published: self.published,
            updated_at,
        }
    }
}
