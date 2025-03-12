use chrono::Utc;
use node_semver::Version;
use serde_json::Value;
use std::sync::Arc;
use tokio::sync::oneshot;
use uuid::Uuid;

use crate::processor::PathProcessingContext;

use crate::{
    processor::ProcessorMessage,
    types::{
        action_types::{ActionType, PluginName},
        task_types::{
            CreateTaskInput, FlowSessionStatus, Stage, Task, TaskConfig, TaskStatus,
            TriggerSessionStatus,
        },
    },
    AppState,
};

pub async fn process_subflow_task(
    state: Arc<AppState>,
    inputs: &Value,
    config: &Value,
    parent_task: &Task,
    ctx: &PathProcessingContext,
) -> Result<Option<Value>, Box<dyn std::error::Error + Send + Sync>> {
    // Extract workflow ID from config
    let workflow_id = config["workflow_id"]
        .as_str()
        .ok_or("No workflow_id specified")?;

    // Create a channel for receiving the result
    let (tx, rx) = oneshot::channel();
    let flow_session_id = Uuid::new_v4();

    // Store the response channel in the state
    {
        let mut result_channels = state.subflow_result_channels.write().await;
        result_channels.insert(flow_session_id, tx);
    }

    // Create processor message for subflow
    let trigger_session_id = Uuid::new_v4();
    let trigger_task = CreateTaskInput {
        account_id: parent_task.account_id.to_string(),
        processing_order: 0,
        task_status: TaskStatus::Running.as_str().to_string(),
        flow_id: workflow_id.to_string(),
        flow_version_id: parent_task.flow_version_id.to_string(),
        action_label: "Subflow Trigger".to_string(),
        trigger_id: parent_task.task_id.to_string(),
        trigger_session_id: trigger_session_id.to_string(),
        trigger_session_status: TriggerSessionStatus::Running.as_str().to_string(),
        flow_session_id: flow_session_id.to_string(),
        flow_session_status: FlowSessionStatus::Running.as_str().to_string(),
        action_id: Uuid::new_v4().to_string(),
        r#type: ActionType::Trigger,
        plugin_name: PluginName::new("@anything/subflow".to_string())?,
        plugin_version: Version::new(0, 1, 0),
        stage: parent_task.stage.as_str().to_string(),
        config: TaskConfig {
            inputs: Some(inputs.clone()),
            inputs_schema: None,
            plugin_config: Some(config.clone()),
            plugin_config_schema: None,
        },
        result: None,
        error: None,
        test_config: None,
        started_at: Some(Utc::now()),
    };

    let processor_message = ProcessorMessage {
        workflow_id: Uuid::parse_str(workflow_id)?,
        version_id: None,
        flow_session_id,
        trigger_session_id,
        trigger_task: Some(trigger_task),
        subflow_depth: ctx.subflow_depth + 1,
    };

    // Send message to processor
    state.processor_sender.send(processor_message).await?;

    // Wait for result
    let result = rx.await?;

    Ok(Some(result))
}
