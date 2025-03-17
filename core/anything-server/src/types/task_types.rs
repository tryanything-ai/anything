use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

use crate::types::action_types::{ActionType, PluginName};
use crate::types::json_schema::JsonSchema;
use node_semver::Version;

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
    pub trigger_session_id: Uuid,
    pub trigger_session_status: TriggerSessionStatus,
    pub flow_session_id: Uuid,
    pub flow_session_status: FlowSessionStatus,
    pub action_id: String,
    pub r#type: ActionType, //Needed for UI to know what type of thing to show. ( loops vs forks vs triggers vs actions etc )
    // pub plugin_id: Option<String>, //Needed for plugin engine to process it with a plugin.
    pub plugin_name: Option<PluginName>,
    pub plugin_version: Option<Version>,
    pub stage: Stage,
    pub test_config: Option<Value>,
    pub config: TaskConfig,
    pub context: Option<Value>,
    pub started_at: Option<DateTime<Utc>>,
    pub ended_at: Option<DateTime<Utc>>,
    pub debug_result: Option<Value>,
    pub result: Option<Value>,
    pub error: Option<Value>,
    pub archived: bool,
    pub updated_at: Option<DateTime<Utc>>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_by: Option<Uuid>,
    pub created_by: Option<Uuid>,
    pub processing_order: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TaskConfig {
    pub inputs: Option<Value>,
    pub inputs_schema: Option<JsonSchema>,
    pub plugin_config: Option<Value>,
    pub plugin_config_schema: Option<JsonSchema>,
}

// #[derive(Debug, Deserialize, Serialize, Clone)]
// pub struct CreateTaskInput {
//     pub task_id: Uuid,
//     pub account_id: String,
//     pub task_status: String,
//     pub started_at: Option<DateTime<Utc>>,
//     pub flow_id: String,
//     pub flow_version_id: String,
//     pub action_label: String,
//     pub trigger_id: String,
//     pub trigger_session_id: String,
//     pub trigger_session_status: String,
//     pub flow_session_id: String,
//     pub flow_session_status: String,
//     pub action_id: String,
//     pub r#type: ActionType,
//     pub plugin_name: PluginName,
//     pub plugin_version: Version,
//     pub stage: String,
//     pub config: TaskConfig,
//     pub result: Option<Value>,
//     pub error: Option<Value>,
//     pub test_config: Option<Value>, // deprecate
//     pub processing_order: i32,
// }

#[derive(Serialize, Deserialize, Debug)]
pub struct TestConfig {
    pub action_id: Option<String>, //if action_id is present, then we are testing just an action
    pub variables: Value,
    pub inputs: Value,
}

#[derive(Debug)]
pub struct TaskBuilder {
    task_id: Option<Uuid>,
    account_id: Option<Uuid>,
    task_status: Option<TaskStatus>,
    flow_id: Option<Uuid>,
    flow_version_id: Option<Uuid>,
    action_label: Option<String>,
    trigger_id: Option<String>,
    trigger_session_id: Option<Uuid>,
    trigger_session_status: Option<TriggerSessionStatus>,
    flow_session_id: Option<Uuid>,
    flow_session_status: Option<FlowSessionStatus>,
    action_id: Option<String>,
    r#type: Option<ActionType>,
    plugin_name: Option<PluginName>,
    plugin_version: Option<Version>,
    stage: Option<Stage>,
    test_config: Option<Value>,
    config: Option<TaskConfig>,
    context: Option<Value>,
    started_at: Option<DateTime<Utc>>,
    ended_at: Option<DateTime<Utc>>,
    debug_result: Option<Value>,
    result: Option<Value>,
    error: Option<Value>,
    archived: bool,
    updated_at: Option<DateTime<Utc>>,
    created_at: Option<DateTime<Utc>>,
    updated_by: Option<Uuid>,
    created_by: Option<Uuid>,
    processing_order: Option<i32>,
}

impl TaskBuilder {
    pub fn new() -> Self {
        Self {
            task_id: Some(Uuid::new_v4()),
            account_id: None,
            task_status: Some(TaskStatus::Running),
            flow_id: None,
            flow_version_id: None,
            action_label: None,
            trigger_id: None,
            trigger_session_id: Some(Uuid::new_v4()),
            trigger_session_status: Some(TriggerSessionStatus::Running),
            flow_session_id: Some(Uuid::new_v4()),
            flow_session_status: Some(FlowSessionStatus::Running),
            action_id: None,
            r#type: None,
            plugin_name: None,
            plugin_version: None,
            stage: Some(Stage::Production),
            test_config: None,
            config: None,
            context: None,
            started_at: Some(Utc::now()),
            ended_at: None,
            debug_result: None,
            result: None,
            error: None,
            archived: false,
            updated_at: None,
            created_at: Some(Utc::now()),
            updated_by: None,
            created_by: None,
            processing_order: Some(0),
        }
    }

    // Required fields
    pub fn task_id(mut self, task_id: Uuid) -> Self {
        self.task_id = Some(task_id);
        self
    }

    pub fn account_id(mut self, account_id: Uuid) -> Self {
        self.account_id = Some(account_id);
        self
    }

    pub fn flow_id(mut self, flow_id: Uuid) -> Self {
        self.flow_id = Some(flow_id);
        self
    }

    // Optional fields with some examples (add more as needed)
    pub fn task_status(mut self, status: TaskStatus) -> Self {
        self.task_status = Some(status);
        self
    }

    pub fn action_label(mut self, label: String) -> Self {
        self.action_label = Some(label);
        self
    }

    pub fn config(mut self, config: TaskConfig) -> Self {
        self.config = Some(config);
        self
    }

    pub fn processing_order(mut self, processing_order: i32) -> Self {
        self.processing_order = Some(processing_order);
        self
    }

    pub fn flow_version_id(mut self, flow_version_id: Uuid) -> Self {
        self.flow_version_id = Some(flow_version_id);
        self
    }

    pub fn trigger_id(mut self, trigger_id: String) -> Self {
        self.trigger_id = Some(trigger_id);
        self
    }

    pub fn trigger_session_id(mut self, trigger_session_id: Uuid) -> Self {
        self.trigger_session_id = Some(trigger_session_id);
        self
    }

    pub fn trigger_session_status(mut self, trigger_session_status: TriggerSessionStatus) -> Self {
        self.trigger_session_status = Some(trigger_session_status);
        self
    }

    pub fn flow_session_id(mut self, flow_session_id: Uuid) -> Self {
        self.flow_session_id = Some(flow_session_id);
        self
    }

    pub fn flow_session_status(mut self, flow_session_status: FlowSessionStatus) -> Self {
        self.flow_session_status = Some(flow_session_status);
        self
    }

    pub fn action_id(mut self, action_id: String) -> Self {
        self.action_id = Some(action_id);
        self
    }

    pub fn r#type(mut self, r#type: ActionType) -> Self {
        self.r#type = Some(r#type);
        self
    }

    pub fn plugin_name(mut self, plugin_name: PluginName) -> Self {
        self.plugin_name = Some(plugin_name);
        self
    }

    pub fn plugin_version(mut self, plugin_version: Version) -> Self {
        self.plugin_version = Some(plugin_version);
        self
    }

    pub fn stage(mut self, stage: Stage) -> Self {
        self.stage = Some(stage);
        self
    }

    pub fn result(mut self, result: Value) -> Self {
        self.result = Some(result);
        self
    }

    pub fn error(mut self, error: Value) -> Self {
        self.error = Some(error);
        self
    }

    pub fn build(self) -> Result<Task, &'static str> {
        let task = Task {
            task_id: self.task_id.ok_or("task_id is required")?,
            account_id: self.account_id.ok_or("account_id is required")?,
            task_status: self.task_status.unwrap_or(TaskStatus::Running),
            flow_id: self.flow_id.ok_or("flow_id is required")?,
            flow_version_id: self.flow_version_id.ok_or("flow_version_id is required")?,
            action_label: self.action_label.ok_or("action_label is required")?,
            trigger_id: self.trigger_id.ok_or("trigger_id is required")?,
            trigger_session_id: self.trigger_session_id.unwrap_or_else(Uuid::new_v4),
            trigger_session_status: self
                .trigger_session_status
                .unwrap_or(TriggerSessionStatus::Running),
            flow_session_id: self.flow_session_id.unwrap_or_else(Uuid::new_v4),
            flow_session_status: self
                .flow_session_status
                .unwrap_or(FlowSessionStatus::Running),
            action_id: self.action_id.ok_or("action_id is required")?,
            r#type: self.r#type.ok_or("type is required")?,
            plugin_name: self.plugin_name,
            plugin_version: self.plugin_version,
            stage: self.stage.unwrap_or(Stage::Production),
            test_config: self.test_config,
            config: self.config.ok_or("config is required")?,
            context: self.context,
            started_at: self.started_at,
            ended_at: self.ended_at,
            debug_result: self.debug_result,
            result: self.result,
            error: self.error,
            archived: self.archived,
            updated_at: self.updated_at,
            created_at: self.created_at,
            updated_by: self.updated_by,
            created_by: self.created_by,
            processing_order: self.processing_order.unwrap_or(0),
        };
        Ok(task)
    }
}

impl Task {
    pub fn builder() -> TaskBuilder {
        TaskBuilder::new()
    }
}
