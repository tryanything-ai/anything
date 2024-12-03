use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

use super::action_types::ActionType;

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Stage {
    Production,
    Testing,
}

impl ToString for Stage {
    fn to_string(&self) -> String {
        match self {
            Stage::Production => "production".to_string(),
            Stage::Testing => "testing".to_string(),
        }
    }
}

impl Stage {
    pub fn as_str(&self) -> &str {
        match self {
            Stage::Production => "production",
            Stage::Testing => "testing",
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TaskStatus {
    Pending,   // Task is created but not yet started
    Waiting, // Task is waiting for correct time to run. Allows pause and HITL stuff we will do later
    Running, // Task is running
    Completed, // Task is completed
    Failed,  // Task failed
    Canceled, // Task was canceled usually because task ahead failed
}

impl TaskStatus {
    pub fn as_str(&self) -> &str {
        match self {
            TaskStatus::Pending => "pending",
            TaskStatus::Waiting => "waiting",
            TaskStatus::Running => "running",
            TaskStatus::Completed => "completed",
            TaskStatus::Failed => "failed",
            TaskStatus::Canceled => "canceled",
        }
    }
}

//Used to determine if whole workflow is completed or what happened
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum FlowSessionStatus {
    Pending,   // Flow is created but not yet started
    Waiting, // Flow is waiting for correct time to run. Allows pause and HITL stuff we will do later
    Running, // Flow is running
    Completed, // Flow is completed
    Failed,  // Flow failed
    Canceled, // Flow was canceled usually because task ahead failed. Maybe if we delete a workflow and their is uncompleted work
}

//Used to determine if whole workflow is completed or what happened especially with nested flows where we want to trace
//What the whole status is of a trigger that is not from this flow
impl FlowSessionStatus {
    pub fn as_str(&self) -> &str {
        match self {
            FlowSessionStatus::Pending => "pending",
            FlowSessionStatus::Waiting => "waiting",
            FlowSessionStatus::Running => "running",
            FlowSessionStatus::Completed => "completed",
            FlowSessionStatus::Failed => "failed",
            FlowSessionStatus::Canceled => "canceled",
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum TriggerSessionStatus {
    Pending,   // Trigger is created but not yet started
    Waiting, // Trigger is waiting for correct time to run. Allows pause and HITL stuff we will do later
    Running, // Trigger is running
    Completed, // Trigger is completed
    Failed,  // Trigger failed
    Canceled, // Trigger was canceled usually because task ahead failed. Maybe if we delete a workflow and their is uncompleted work
}

impl TriggerSessionStatus {
    pub fn as_str(&self) -> &str {
        match self {
            TriggerSessionStatus::Pending => "pending",
            TriggerSessionStatus::Waiting => "waiting",
            TriggerSessionStatus::Running => "running",
            TriggerSessionStatus::Completed => "completed",
            TriggerSessionStatus::Failed => "failed",
            TriggerSessionStatus::Canceled => "canceled",
        }
    }
}



#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Task {
    pub task_id: Uuid,
    pub account_id: Uuid,
    pub task_status: TaskStatus,
    pub flow_id: Uuid,
    pub flow_version_id: Uuid,
    pub action_label: String,
    pub trigger_id: String,
    pub trigger_session_id: String,
    pub trigger_session_status: TriggerSessionStatus,
    pub flow_session_id: String,
    pub flow_session_status: FlowSessionStatus,
    pub action_id: String,
    pub r#type: String, //Needed for UI to know what type of thing to show. ( loops vs forks vs triggers vs actions etc )
    pub plugin_id: Option<String>, //Needed for plugin engine to process it with a plugin.
    pub stage: Stage,
    pub test_config: Option<Value>,
    pub config: Value,
    pub context: Option<Value>,
    pub started_at: Option<DateTime<Utc>>,
    pub ended_at: Option<DateTime<Utc>>,
    pub debug_result: Option<Value>,
    pub result: Option<Value>,
    pub archived: bool,
    pub updated_at: Option<DateTime<Utc>>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_by: Option<Uuid>,
    pub created_by: Option<Uuid>,
    pub processing_order: i32,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TaskConfig {
    pub variables: Option<Value>,
    pub variables_schema: Option<Value>,
    pub input: Option<Value>,
    pub input_schema: Option<Value>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CreateTaskInput {
    pub account_id: String,
    pub task_status: String,
    pub started_at: Option<DateTime<Utc>>,
    pub flow_id: String,
    pub flow_version_id: String,
    pub action_label: String,
    pub trigger_id: String,
    pub trigger_session_id: String,
    pub trigger_session_status: String,
    pub flow_session_id: String,
    pub flow_session_status: String,
    pub action_id: String,
    pub r#type: ActionType,
    pub plugin_id: String,
    pub stage: String,
    pub config: TaskConfig,
    pub result: Option<Value>,
    pub test_config: Option<Value>, // deprecate
    pub processing_order: i32,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct TestConfig {
    pub action_id: Option<String>, //if action_id is present, then we are testing just an action
    pub variables: Value,
    pub inputs: Value,
}
