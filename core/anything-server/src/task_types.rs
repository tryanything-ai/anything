use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum Stage {
    Production,
    Testing,
}

impl Stage {
    pub fn as_str(&self) -> &str {
        match self {
            Stage::Production => "production",
            Stage::Testing => "testing",
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub enum TaskStatus {
    Pending,    // Task is created but not yet started
    Waiting,    // Task is waiting for correct time to run. Allows pause and HITL stuff we will do later
    Running,    // Task is running
    Completed,  // Task is completed
    Failed,     // Task failed
    Canceled    // Task was canceled usually because task ahead failed
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
#[derive(Debug, Deserialize, Serialize)]
pub enum FlowSessionStatus {
    Pending,    // Flow is created but not yet started
    Waiting,    // Flow is waiting for correct time to run. Allows pause and HITL stuff we will do later
    Running,    // Flow is running
    Completed,  // Flow is completed
    Failed,     // Flow failed
    Canceled    // Flow was canceled usually because task ahead failed. Maybe if we delete a workflow and their is uncompleted work
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

#[derive(Debug, Deserialize, Serialize)]
pub enum TriggerSessionStatus {
    Pending,    // Trigger is created but not yet started
    Waiting,    // Trigger is waiting for correct time to run. Allows pause and HITL stuff we will do later
    Running,    // Trigger is running
    Completed,  // Trigger is completed
    Failed,     // Trigger failed
    Canceled    // Trigger was canceled usually because task ahead failed. Maybe if we delete a workflow and their is uncompleted work
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
#[serde(rename_all = "lowercase")]
pub enum ActionType {
    Input,      // Input action
    Trigger,    // Trigger action
    Action,     // General action
    Loop,       // Loop action
    Decision,   // Decision actiond
    Filter,     // Filter action
    Output      // Output action
}

impl ActionType {
    pub fn as_str(&self) -> &str {
        match self {
            ActionType::Input => "input",
            ActionType::Trigger => "trigger",
            ActionType::Action => "action",
            ActionType::Loop => "loop",
            ActionType::Decision => "decision",
            ActionType::Filter => "filter",
            ActionType::Output => "output",
        }
    }
}