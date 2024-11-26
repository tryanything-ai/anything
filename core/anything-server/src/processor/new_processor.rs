use chrono::Utc;
use postgrest::Postgrest;
use serde_json::{json, Value};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::debug;
use uuid::Uuid;

use crate::processor::execute_task::execute_task;
use crate::processor::flow_session_cache::FlowSessionData;
use crate::processor::parsing_utils::get_trigger_node;
use crate::workflow_types::{CreateTaskInput, DatabaseFlowVersion};
use crate::AppState;

use crate::processor::db_calls::{
    create_task, get_workflow_definition, update_flow_session_status, update_task_status,
};

use crate::task_types::{
    ActionType, FlowSessionStatus, Stage, Task, TaskStatus, TriggerSessionStatus,
};

use super::ProcessorMessage;

// New processor implementation with better organization
struct WorkflowProcessor {
    state: Arc<AppState>,
    workflow_id: Uuid,
    version_id: Option<Uuid>,
    flow_session_id: Uuid,
    trigger_task: Option<CreateTaskInput>,
    workflow_definition: Option<DatabaseFlowVersion>,
    graph: HashMap<String, Vec<String>>,
    processing_order: usize,
    client: Arc<Postgrest>,
}

impl WorkflowProcessor {
    async fn new(
        state: Arc<AppState>,
        message: ProcessorMessage,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let ProcessorMessage {
            workflow_id,
            version_id,
            flow_session_id,
            trigger_task,
        } = message;

        Ok(Self {
            state: state.clone(),
            workflow_id,
            version_id,
            flow_session_id,
            trigger_task,
            workflow_definition: None,
            graph: HashMap::new(),
            processing_order: 0,
            client: state.anything_client.clone(),
        })
    }

    async fn load_workflow_definition(
        &mut self,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Try cache first
        {
            let cache = self.state.flow_session_cache.read().await;
            if let Some(session_data) = cache.get(&self.flow_session_id) {
                if let Some(workflow) = &session_data.workflow {
                    self.workflow_definition = Some(workflow.clone());
                    return Ok(());
                }
            }
        }

        // Fetch from DB if not in cache
        let workflow = get_workflow_definition(
            self.state.clone(),
            &self.workflow_id,
            self.version_id.as_ref(),
        )
        .await?;

        // Update cache
        {
            let mut cache = self.state.flow_session_cache.write().await;
            if cache.get(&self.flow_session_id).is_none() {
                let session_data = FlowSessionData {
                    workflow: Some(workflow.clone()),
                    tasks: HashMap::new(),
                    flow_session_id: self.flow_session_id,
                    workflow_id: self.workflow_id,
                    workflow_version_id: self.version_id,
                };
                cache.set(&self.flow_session_id, session_data);
            }
        }

        self.workflow_definition = Some(workflow);
        Ok(())
    }

    fn build_graph(&mut self) {
        if let Some(workflow) = &self.workflow_definition {
            for edge in &workflow.flow_definition.edges {
                self.graph
                    .entry(edge.source.clone())
                    .or_insert_with(Vec::new)
                    .push(edge.target.clone());
            }
        }
    }

    async fn create_initial_task(
        &mut self,
    ) -> Result<Option<Task>, Box<dyn std::error::Error + Send + Sync>> {
        let workflow = self.workflow_definition.as_ref().unwrap();
        let trigger_node = get_trigger_node(&workflow.flow_definition).unwrap();

        let initial_task = if let Some(trigger_task) = self.trigger_task.take() {
            trigger_task
        } else {
            CreateTaskInput {
                account_id: workflow.account_id.to_string(),
                processing_order: 0,
                task_status: TaskStatus::Running.as_str().to_string(),
                flow_id: workflow.flow_id.to_string(),
                flow_version_id: workflow.flow_version_id.to_string(),
                action_label: trigger_node.label.clone(),
                trigger_id: trigger_node.action_id.clone(),
                trigger_session_id: Uuid::new_v4().to_string(),
                trigger_session_status: TriggerSessionStatus::Running.as_str().to_string(),
                flow_session_id: self.flow_session_id.to_string(),
                flow_session_status: FlowSessionStatus::Running.as_str().to_string(),
                action_id: trigger_node.action_id.clone(),
                r#type: ActionType::Trigger,
                plugin_id: trigger_node.plugin_id.clone(),
                stage: if workflow.published {
                    Stage::Production.as_str().to_string()
                } else {
                    Stage::Testing.as_str().to_string()
                },
                config: json!({
                    "variables": serde_json::json!(trigger_node.variables),
                    "input": serde_json::json!(trigger_node.input),
                }),
                result: None,
                started_at: Some(Utc::now()),
                test_config: None,
            }
        };

        let task = create_task(self.state.clone(), &initial_task).await?;

        // Update cache
        {
            let mut cache = self.state.flow_session_cache.write().await;
            if let Some(mut session_data) = cache.get(&self.flow_session_id) {
                session_data
                    .tasks
                    .insert(task.task_id.clone(), task.clone());
                cache.set(&self.flow_session_id, session_data);
            }
        }

        Ok(Some(task))
    }
    async fn process_task(
        &mut self,
        task: &Task,
    ) -> Result<Option<Task>, Box<dyn std::error::Error + Send + Sync>> {
        // Execute current task
        debug!("[PROCESSOR] Executing task: {}", task.task_id);
        let task_result = match execute_task(self.state.clone(), &self.client, task).await {
            Ok(success_value) => {
                debug!("[PROCESSOR] Task {} completed successfully", task.task_id);
                success_value
            }
            Err(error) => {
                debug!("[PROCESSOR] Task {} failed: {:?}", task.task_id, error);
                self.handle_task_failure(task, error.clone()).await?;

                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    error.to_string(),
                )));
            }
        };

        // Update task status and cache
        self.update_task_status(task, task_result.unwrap_or(json!(null)))
            .await?;

        // Find and create next task
        self.create_next_task(task).await
    }

    async fn handle_task_failure(
        &self,
        task: &Task,
        error: Value,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Update task status to failed
        update_task_status(
            self.state.clone(),
            &task.task_id,
            &TaskStatus::Failed,
            Some(error.clone()),
        )
        .await?;

        // Update flow session status
        update_flow_session_status(
            &self.state,
            &self.flow_session_id,
            &FlowSessionStatus::Failed,
            &TriggerSessionStatus::Failed,
        )
        .await?;

        // Update cache
        {
            let mut cache = self.state.flow_session_cache.write().await;
            let mut task_copy = task.clone();
            task_copy.result = Some(error.clone());
            task_copy.task_status = TaskStatus::Failed;
            task_copy.ended_at = Some(Utc::now());
            if let Some(mut session_data) = cache.get(&self.flow_session_id) {
                session_data.tasks.insert(task.task_id.clone(), task_copy);
                cache.set(&self.flow_session_id, session_data);
            }
        }

        // Send error response to webhook if needed
        let mut completions = self.state.flow_completions.lock().await;
        if let Some(completion) = completions.remove(&self.flow_session_id.to_string()) {
            if completion.needs_response {
                debug!("[PROCESSOR] Sending error response through completion channel");
                let _ = completion.sender.send(error.clone());
            }
        }

        Ok(())
    }

    async fn update_task_status(
        &self,
        task: &Task,
        result: Value,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        update_task_status(
            self.state.clone(),
            &task.task_id,
            &TaskStatus::Completed,
            Some(result.clone()),
        )
        .await?;

        // Update cache
        {
            let mut cache = self.state.flow_session_cache.write().await;
            let mut task_copy = task.clone();
            task_copy.result = Some(result);
            task_copy.task_status = TaskStatus::Completed;
            task_copy.ended_at = Some(Utc::now());
            if let Some(mut session_data) = cache.get(&self.flow_session_id) {
                session_data.tasks.insert(task.task_id.clone(), task_copy);
                cache.set(&self.flow_session_id, session_data);
            }
        }

        Ok(())
    }

    async fn create_next_task(
        &mut self,
        current_task: &Task,
    ) -> Result<Option<Task>, Box<dyn std::error::Error + Send + Sync>> {
        let workflow = self.workflow_definition.as_ref().unwrap();

        if let Some(neighbors) = self.graph.get(&current_task.action_id) {
            for neighbor_id in neighbors {
                let next_action = workflow
                    .flow_definition
                    .actions
                    .iter()
                    .find(|action| &action.action_id == neighbor_id);

                if let Some(action) = next_action {
                    // Check if already processed
                    let cache = self.state.flow_session_cache.read().await;
                    if let Some(session_data) = cache.get(&self.flow_session_id) {
                        if !session_data
                            .tasks
                            .values()
                            .any(|t| t.action_id == action.action_id)
                        {
                            let next_task_input = CreateTaskInput {
                                account_id: workflow.account_id.to_string(),
                                processing_order: self.processing_order as i32,
                                task_status: TaskStatus::Running.as_str().to_string(),
                                flow_id: workflow.flow_id.to_string(),
                                flow_version_id: workflow.flow_version_id.to_string(),
                                action_label: action.label.clone(),
                                trigger_id: action.action_id.clone(),
                                trigger_session_id: Uuid::new_v4().to_string(),
                                trigger_session_status: TriggerSessionStatus::Running
                                    .as_str()
                                    .to_string(),
                                flow_session_id: self.flow_session_id.to_string(),
                                flow_session_status: FlowSessionStatus::Running
                                    .as_str()
                                    .to_string(),
                                action_id: action.action_id.clone(),
                                r#type: action.r#type.clone(),
                                plugin_id: action.plugin_id.clone(),
                                stage: if workflow.published {
                                    Stage::Production.as_str().to_string()
                                } else {
                                    Stage::Testing.as_str().to_string()
                                },
                                config: json!({
                                    "variables": serde_json::json!(action.variables),
                                    "input": serde_json::json!(action.input),
                                }),
                                result: None,
                                started_at: Some(Utc::now()),
                                test_config: None,
                            };

                            let new_task =
                                create_task(self.state.clone(), &next_task_input).await?;
                            self.processing_order += 1;

                            // Update cache
                            {
                                let mut cache = self.state.flow_session_cache.write().await;
                                if let Some(mut session_data) = cache.get(&self.flow_session_id) {
                                    session_data
                                        .tasks
                                        .insert(new_task.task_id.clone(), new_task.clone());
                                    cache.set(&self.flow_session_id, session_data);
                                }
                            }

                            return Ok(Some(new_task));
                        }
                    }
                }
            }
        }

        // No more tasks - workflow complete
        update_flow_session_status(
            &self.state,
            &self.flow_session_id,
            &FlowSessionStatus::Completed,
            &TriggerSessionStatus::Completed,
        )
        .await?;

        Ok(None)
    }

    async fn cleanup(&self) {
        // Invalidate cache
        {
            let mut cache = self.state.flow_session_cache.write().await;
            cache.invalidate(&self.flow_session_id);
        }
    }
}

// Main processor function
pub async fn processor(
    state: Arc<AppState>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    debug!("[PROCESSOR] Starting processor");

    let active_flow_sessions = Arc::new(Mutex::new(HashSet::new()));
    let mut rx = state.processor_receiver.lock().await;
    let number_of_processors_semaphore = state.workflow_processor_semaphore.clone();

    while let Some(message) = rx.recv().await {
        let flow_session_id = message.flow_session_id;

        // Check for already processing session
        {
            let mut active_sessions = active_flow_sessions.lock().await;
            if !active_sessions.insert(flow_session_id) {
                continue;
            }
        }

        let state = Arc::clone(&state);
        let permit = number_of_processors_semaphore
            .clone()
            .acquire_owned()
            .await
            .unwrap();
        let active_flow_sessions = Arc::clone(&active_flow_sessions);

        tokio::spawn(async move {
            let result = async {
                let mut processor = WorkflowProcessor::new(state.clone(), message).await?;
                processor.load_workflow_definition().await?;
                processor.build_graph();

                let mut current_task = processor.create_initial_task().await?;

                while let Some(task) = current_task {
                    current_task = processor.process_task(&task).await?;
                }

                processor.cleanup().await;
                Ok::<_, Box<dyn std::error::Error + Send + Sync>>(())
            }
            .await;

            if let Err(e) = result {
                debug!("[PROCESSOR] Error processing workflow: {}", e);
            }

            active_flow_sessions.lock().await.remove(&flow_session_id);
            drop(permit);
        });
    }

    Ok(())
}
