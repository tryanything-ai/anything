use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(schema_name = "anything", table_name = "tasks")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub task_id: Uuid,
    pub account_id: Uuid,
    pub task_status: String,
    pub flow_id: Uuid,
    pub flow_version_id: Uuid,
    pub action_label: String,
    pub trigger_id: String,
    pub trigger_session_id: String,
    pub trigger_session_status: String,
    pub flow_session_id: String,
    pub flow_session_status: String,
    pub action_id: String,
    pub r#type: String,
    pub plugin_name: Option<String>,
    pub plugin_version: Option<String>,
    pub stage: String,
    pub test_config: Option<Json>,
    pub config: Json,
    pub context: Option<Json>,
    pub started_at: Option<DateTimeWithTimeZone>,
    pub ended_at: Option<DateTimeWithTimeZone>,
    pub completed_at: Option<DateTimeWithTimeZone>,
    pub debug_result: Option<Json>,
    pub result: Option<Json>,
    pub output: Option<Json>,
    pub processing_order: i32,
    pub error: Option<Json>,
    pub error_message: Option<String>,
    pub execution_time_ms: Option<i64>,
    pub retry_count: Option<i32>,
    pub archived: bool,
    pub updated_at: Option<DateTimeWithTimeZone>,
    pub created_at: Option<DateTimeWithTimeZone>,
    pub updated_by: Option<Uuid>,
    pub created_by: Option<Uuid>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::flows::Entity",
        from = "Column::FlowId",
        to = "super::flows::Column::FlowId"
    )]
    Flows,
    #[sea_orm(
        belongs_to = "super::flow_versions::Entity",
        from = "Column::FlowVersionId",
        to = "super::flow_versions::Column::FlowVersionId"
    )]
    FlowVersions,
}

impl Related<super::flows::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Flows.def()
    }
}

impl Related<super::flow_versions::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::FlowVersions.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

// Task status enums
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaskStatus {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "running")]
    Running,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "cancelled")]
    Cancelled,
}

impl ToString for TaskStatus {
    fn to_string(&self) -> String {
        match self {
            TaskStatus::Pending => "pending".to_string(),
            TaskStatus::Running => "running".to_string(),
            TaskStatus::Completed => "completed".to_string(),
            TaskStatus::Failed => "failed".to_string(),
            TaskStatus::Cancelled => "cancelled".to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FlowSessionStatus {
    #[serde(rename = "running")]
    Running,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "cancelled")]
    Cancelled,
}

impl ToString for FlowSessionStatus {
    fn to_string(&self) -> String {
        match self {
            FlowSessionStatus::Running => "running".to_string(),
            FlowSessionStatus::Completed => "completed".to_string(),
            FlowSessionStatus::Failed => "failed".to_string(),
            FlowSessionStatus::Cancelled => "cancelled".to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TriggerSessionStatus {
    #[serde(rename = "running")]
    Running,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "cancelled")]
    Cancelled,
}

impl ToString for TriggerSessionStatus {
    fn to_string(&self) -> String {
        match self {
            TriggerSessionStatus::Running => "running".to_string(),
            TriggerSessionStatus::Completed => "completed".to_string(),
            TriggerSessionStatus::Failed => "failed".to_string(),
            TriggerSessionStatus::Cancelled => "cancelled".to_string(),
        }
    }
}
