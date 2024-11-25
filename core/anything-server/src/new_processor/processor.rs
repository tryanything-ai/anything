use crate::new_processor::execute_task::execute_task;
use crate::new_processor::parsing_utils::get_trigger_node;
use crate::workflow_types::{CreateTaskInput, WorkflowVersionDefinition};
use crate::AppState;
use serde_json::json;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::debug;
use uuid::Uuid;

use crate::new_processor::db_calls::{create_task, get_workflow_definition, update_task_status};
use crate::task_types::{
    ActionType, FlowSessionStatus, Stage, Task, TaskStatus, TriggerSessionStatus,
};

// Add this near your other type definitions
#[derive(Debug, Clone)]
pub struct ProcessorMessage {
    pub workflow_id: Uuid,
    pub version_id: Option<Uuid>, //When we are calling a workflow from a webhook, we don't have a version id
    pub flow_session_id: Uuid,
    pub trigger_task: Option<Task>,
}

pub async fn processor(
    state: Arc<AppState>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    debug!("[PROCESSOR] Starting processor");

    // Create a shared set to track active flow sessions
    let active_flow_sessions = Arc::new(Mutex::new(HashSet::new()));

    // Get the receiver from the state
    let mut rx = state.processor_receiver.lock().await;
    // Guard againts too many workflows running at once
    let number_of_processors_semaphore = state.workflow_processor_semaphore.clone();

    while let Some(message) = rx.recv().await {
        let workflow_id = message.workflow_id;
        let version_id = message.version_id;
        let flow_session_id = message.flow_session_id;
        debug!("[PROCESSOR] Received workflow_id: {}", flow_session_id);

        // Check if this flow session is already being processed
        let mut active_sessions = active_flow_sessions.lock().await;

        if !active_sessions.insert(flow_session_id) {
            debug!(
                "[PROCESSOR] Flow session {} is already being processed, skipping",
                flow_session_id
            );
            continue;
        }
        drop(active_sessions);

        // Clone what we need for the new task
        let state = Arc::clone(&state);
        let permit = number_of_processors_semaphore
            .clone()
            .acquire_owned()
            .await
            .unwrap();
        let client = state.anything_client.clone();
        let active_flow_sessions = Arc::clone(&active_flow_sessions);

        // Spawn a new task for this workflow
        //SPAWN NEW PROCESSOR FOR EACH WORKFLOW
        tokio::spawn(async move {
            debug!(
                "[PROCESSOR] Starting workflow processing for {}",
                flow_session_id
            );

            let mut workflow_definition = None;

            // Try to get from cache first using a read lock
            {
                let cache = state.flow_session_cache.read().await;
                if let Some(session_data) = cache.get(&flow_session_id) {
                    if let Some(workflow) = &session_data.workflow {
                        debug!(
                            "[PROCESSOR] Using cached workflow for flow_session_id: {}",
                            flow_session_id
                        );
                        workflow_definition = Some(workflow.clone());
                    }
                }
            }

            // If not in cache, fetch from DB and update cache
            if workflow_definition.is_none() {
                debug!(
                    "[PROCESSOR] Cache miss for workflow, fetching from DB for flow_session_id: {}",
                    flow_session_id
                );

                let workflow =
                    get_workflow_definition(state.clone(), &workflow_id, version_id.as_ref())
                        .await
                        .map_err(|e| {
                            debug!("[PROCESSOR] Error getting workflow definition: {}", e);
                            e
                        })
                        .unwrap();

                // Update cache with a write lock
                {
                    let mut cache = state.flow_session_cache.write().await;
                    if let Some(mut session_data) = cache.get(&flow_session_id) {
                        session_data.workflow = Some(workflow.clone());
                        cache.set(&flow_session_id, session_data);
                        debug!(
                            "[PROCESSOR] Updated workflow cache for flow_session_id: {}",
                            flow_session_id
                        );
                    }
                }

                // let workflow_clone = workflow.clone();
                workflow_definition = Some(workflow.clone());

                let trigger_node = get_trigger_node(&workflow.flow_definition);

                //TODO: we need to find out how to deal with a workflow that say only gets half processed and we shut down.
                //How do we recover from that? -> Proper shutdown signal is probably needed.
                //WE ARE ASSUMING THAT THE WORKFLOW WILL BE COMPLETED IN THIS SINGLE PROCESSOR CALL.
                // ... existing code ...

                if let Some(workflow) = &workflow_definition {
                    debug!("[PROCESSOR] Starting workflow execution");

                    // Create initial trigger task
                    let trigger_node = get_trigger_node(&workflow.flow_definition).unwrap();
                    let initial_task = CreateTaskInput {
                        account_id: workflow.account_id.to_string(),
                        processing_order: 0,
                        task_status: TaskStatus::Running.as_str().to_string(),
                        flow_id: workflow_id.to_string(),
                        flow_version_id: workflow.flow_version_id.to_string(),
                        action_label: trigger_node.label.clone(),
                        trigger_id: trigger_node.action_id.clone(),
                        trigger_session_id: Uuid::new_v4().to_string(),
                        trigger_session_status: TriggerSessionStatus::Running.as_str().to_string(),
                        flow_session_id: flow_session_id.to_string(),
                        flow_session_status: FlowSessionStatus::Running.as_str().to_string(),
                        action_id: trigger_node.action_id.clone(),
                        r#type: ActionType::Trigger,
                        plugin_id: trigger_node.plugin_id.clone(),
                        stage: if workflow.published {
                            Stage::Production.as_str().to_string()
                        } else {
                            Stage::Testing.as_str().to_string()
                        },
                        config: json!({}),
                        result: None,
                        test_config: None,
                    };

                    // Start with trigger task
                    let mut current_task = match create_task(state.clone(), &initial_task).await {
                        Ok(task) => {
                            // Update cache with new task
                            let mut cache = state.flow_session_cache.write().await;
                            if let Some(mut session_data) = cache.get(&flow_session_id) {
                                session_data
                                    .tasks
                                    .insert(task.task_id.clone(), task.clone());
                                cache.set(&flow_session_id, session_data);
                            }
                            Some(task)
                        }
                        Err(e) => {
                            debug!("[PROCESSOR] Error creating initial task: {}", e);
                            None
                        }
                    };

                    // Create graph for BFS traversal
                    let workflow_def: WorkflowVersionDefinition = workflow.flow_definition.clone();

                    let mut graph: HashMap<String, Vec<String>> = HashMap::new();
                    for edge in &workflow_def.edges {
                        graph
                            .entry(edge.source.clone())
                            .or_insert_with(Vec::new)
                            .push(edge.target.clone());
                    }

                    // Process tasks until workflow completion
                    while let Some(task) = current_task {
                        // Execute current task
                        debug!("[PROCESSOR] Executing task: {}", task.task_id);

                        let result = execute_task(state, &client, &task).await;
                        let result_value = match result {
                            Ok(value) => Some(value),
                            Err(err) => Some(err),
                        };

                        // Update task status and result in cache and DB
                        update_task_status(state.clone(), &task.task_id, &TaskStatus::Completed, result_value).await.map_err(|e| e.to_string())?;
                      
                      
                        // Find next task using BFS
                        let next_action = if let Some(neighbors) = graph.get(&task.action_id) {
                            // Get the first unprocessed neighbor
                            for neighbor_id in neighbors {
                                let neighbor = workflow_def
                                    .actions
                                    .iter()
                                    .find(|action| &action.action_id == neighbor_id);

                                if let Some(action) = neighbor {
                                    // Check if this task has already been processed
                                    let cache = state.flow_session_cache.read().await;
                                    if let Some(session_data) = cache.get(&flow_session_id) {
                                        if !session_data
                                            .tasks
                                            .iter()
                                            .any(|(_, t)| t.action_id == action.action_id)
                                        {
                                            Some(action.clone())
                                        } else {
                                            None
                                        }
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                }
                            }
                            None
                        } else {
                            None
                        };

                        // Create next task if available
                        current_task = if let Some(next_action) = next_action {
                            let next_task_input = CreateTaskInput {
                                account_id: workflow.account_id.to_string(),
                                processing_order: task.processing_order + 1,
                                task_status: TaskStatus::Pending.as_str().to_string(),
                                flow_id: workflow_id.to_string(),
                                flow_version_id: workflow.flow_version_id.to_string(),
                                action_label: next_action.label.clone(),
                                trigger_id: next_action.action_id.clone(),
                                trigger_session_id: Uuid::new_v4().to_string(),
                                trigger_session_status: TriggerSessionStatus::Pending
                                    .as_str()
                                    .to_string(),
                                flow_session_id: flow_session_id.to_string(),
                                flow_session_status: FlowSessionStatus::Pending
                                    .as_str()
                                    .to_string(),
                                action_id: next_action.action_id,
                                r#type: next_action.r#type,
                                plugin_id: next_action.plugin_id.clone(),
                                stage: if workflow.published {
                                    Stage::Production.as_str().to_string()
                                } else {
                                    Stage::Testing.as_str().to_string()
                                },
                                config: json!({}),
                                result: None,
                                test_config: None,
                            };

                            match create_task(state.clone(), &next_task_input).await {
                                Ok(new_task) => {
                                    // Update cache
                                    let mut cache = state.flow_session_cache.write().await;
                                    if let Some(mut session_data) = cache.get(&flow_session_id) {
                                        session_data
                                            .tasks
                                            .insert(new_task.task_id.clone(), new_task.clone());
                                        cache.set(&flow_session_id, session_data);
                                    }
                                    Some(new_task)
                                }
                                Err(e) => {
                                    debug!("[PROCESSOR] Error creating next task: {}", e);
                                    None
                                }
                            }
                        } else {
                            // No more tasks - workflow is complete
                            debug!("[PROCESSOR] Workflow completed: {}", flow_session_id);
                            None
                        };
                    }
                }

                // ... rest of the code ...
            }

            // debug!(
            //     "[PROCESSOR] Completed workflow processing for {}",
            //     flow_session_id
            // );

            // // Remove the flow session from active sessions when done
            // active_flow_sessions.lock().await.remove(&flow_session_id);
            // drop(permit);
        });
        //END SPAWNED PROCESSOR
    }

    Ok(())
}

//TODO:
//Traverse the worfklow definition to get next task
//Update task status in cache and db
//Bundle the task
//Run task
//Update status and result in cache and db
//Determine if workflow is complete
//If complete, update flow session status in cache and db
//If not complete, update flow session with next task in line
//Send signal to webhook engine if response is needed
