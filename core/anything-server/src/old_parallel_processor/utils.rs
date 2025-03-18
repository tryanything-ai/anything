use std::collections::HashMap;

use crate::types::{
    action_types::{Action, ActionType},
    workflow_types::WorkflowVersionDefinition,
};

use crate::processor::flow_session_cache::FlowSessionData;
use crate::AppState;

use std::collections::HashSet;
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::processor::db_calls::get_workflow_definition;

use crate::types::{
    task_types::Task,
    workflow_types::DatabaseFlowVersion,
};



pub fn get_trigger_node(workflow: &WorkflowVersionDefinition) -> Option<&Action> {
    workflow
        .actions
        .iter()
        .find(|action| action.r#type == ActionType::Trigger)
}

/// Creates a graph representation of the workflow
pub fn create_workflow_graph(
    workflow_def: &WorkflowVersionDefinition,
) -> HashMap<String, Vec<String>> {
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();
    for edge in &workflow_def.edges {
        graph
            .entry(edge.source.clone())
            .or_insert_with(Vec::new)
            .push(edge.target.clone());
    }
    graph
}

//////////////////////////////////////
////////////// VALIDATION /////////////////
//////////////////////////////////////

/// Checks if a flow session is already being processed and adds it to active sessions if not.
/// Returns true if the session was added (not already active), false otherwise.
pub async fn is_already_processing(
    active_flow_sessions: &Arc<Mutex<HashSet<Uuid>>>,
    flow_session_id: Uuid,
) -> bool {
    // Use a scope block to automatically drop the lock when done
    let mut active_sessions = active_flow_sessions.lock().await;
    if !active_sessions.insert(flow_session_id) {
        println!(
            "[PROCESSOR] Flow session {} is already being processed, skipping",
            flow_session_id
        );
        true
    } else {
        println!(
            "[PROCESSOR] Added flow session {} to active sessions",
            flow_session_id
        );
        false
    }
    // Lock is automatically dropped here at end of scope
}

//////////////////////////////////////
////////////// CACHING /////////////////
//////////////////////////////////////

// Fetches a workflow definition from cache or database and ensures it's cached.
// Returns the workflow definition and cached tasks if found, or an error if the workflow couldn't be retrieved.
// pub async fn get_workflow_and_tasks_from_cache(
//     state: &Arc<AppState>,
//     flow_session_id: Uuid,
//     workflow_id: &Uuid,
//     version_id: &Option<Uuid>,
// ) -> Result<(DatabaseFlowVersion, Option<HashMap<Uuid, Task>>), String> {
//     let mut workflow_definition = None;
//     let mut cached_tasks = None;

//     // Try to get from cache first using a read lock
//     {
//         let cache = state.flow_session_cache.read().await;
//         println!(
//             "[PROCESSOR] Checking cache for flow_session_id: {}",
//             flow_session_id
//         );
//         if let Some(session_data) = cache.get(&flow_session_id) {
//             if let Some(workflow) = &session_data.workflow {
//                 println!(
//                     "[PROCESSOR] Found workflow in cache for flow_session_id: {}",
//                     flow_session_id
//                 );
//                 workflow_definition = Some(workflow.clone());
//             }
//             //When we hydrate old tasks this will have items init from hydrate_processor
//             cached_tasks = Some(session_data.tasks);
//         }
//     }

//     // Only fetch flow definition from DB if we didn't find it in cache
//     if workflow_definition.is_none() {
//         println!(
//             "[PROCESSOR] No workflow found in cache, fetching from DB for flow_session_id: {}",
//             flow_session_id
//         );

//         let workflow =
//             match get_workflow_definition(state.clone(), workflow_id, version_id.as_ref()).await {
//                 Ok(w) => {
//                     println!("[PROCESSOR] Successfully fetched workflow from DB");
//                     w
//                 }
//                 Err(e) => {
//                     let error_msg = format!("[PROCESSOR] Error getting workflow definition: {}", e);
//                     println!("{}", error_msg);
//                     return Err(error_msg);
//                 }
//             };

//         // Only update cache if there isn't already data there
//         {
//             let mut cache = state.flow_session_cache.write().await;
//             if cache.get(&flow_session_id).is_none() {
//                 println!("[PROCESSOR] Creating new session data in cache");
//                 let session_data = FlowSessionData {
//                     workflow: Some(workflow.clone()),
//                     tasks: HashMap::new(),
//                     flow_session_id,
//                     workflow_id: *workflow_id,
//                     workflow_version_id: version_id.clone(),
//                 };
//                 cache.set(&flow_session_id, session_data);
//             }
//         }

//         workflow_definition = Some(workflow);
//     }

//     // Unwrap the workflow definition - we know it's Some at this point
//     match workflow_definition {
//         Some(workflow) => {
//             println!("[PROCESSOR] Workflow definition retrieved successfully");
//             Ok((workflow, cached_tasks))
//         }
//         None => {
//             // This should never happen based on the logic above, but we handle it just in case
//             let error_msg = "[PROCESSOR] No workflow definition found after fetching".to_string();
//             println!("{}", error_msg);
//             Err(error_msg)
//         }
//     }
// }
