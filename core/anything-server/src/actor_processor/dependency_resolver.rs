use crate::types::{
    action_types::Action, react_flow_types::Edge, task_types::Task,
    workflow_types::WorkflowVersionDefinition,
};
use std::collections::{HashMap, HashSet, VecDeque};
use tracing::{info, warn};
use uuid::Uuid;

/// Represents a task dependency graph and execution order
#[derive(Debug, Clone)]
pub struct DependencyGraph {
    /// Map from action_id to list of dependent action_ids
    pub dependencies: HashMap<String, Vec<String>>,
    /// Map from action_id to list of action_ids that depend on it
    pub dependents: HashMap<String, Vec<String>>,
    /// Topologically sorted execution order
    pub execution_order: Vec<String>,
}

impl DependencyGraph {
    /// Creates a new dependency graph from workflow definition
    pub fn new(workflow_def: &WorkflowVersionDefinition) -> Self {
        let mut dependencies: HashMap<String, Vec<String>> = HashMap::new();
        let mut dependents: HashMap<String, Vec<String>> = HashMap::new();

        // Initialize empty dependency lists for all actions
        for action in &workflow_def.actions {
            dependencies.insert(action.action_id.clone(), Vec::new());
            dependents.insert(action.action_id.clone(), Vec::new());
        }

        // Build dependency graph from edges
        for edge in &workflow_def.edges {
            // target depends on source
            dependencies
                .entry(edge.target.clone())
                .or_insert_with(Vec::new)
                .push(edge.source.clone());

            // source has target as dependent
            dependents
                .entry(edge.source.clone())
                .or_insert_with(Vec::new)
                .push(edge.target.clone());
        }

        // Calculate topological order
        let execution_order = Self::topological_sort(&dependencies, &workflow_def.actions);

        info!(
            "[DEPENDENCY_RESOLVER] Created dependency graph with {} actions, execution order: {:?}",
            workflow_def.actions.len(),
            execution_order
        );

        Self {
            dependencies,
            dependents,
            execution_order,
        }
    }

    /// Performs topological sort to determine execution order
    fn topological_sort(
        dependencies: &HashMap<String, Vec<String>>,
        actions: &[Action],
    ) -> Vec<String> {
        let mut in_degree: HashMap<String, usize> = HashMap::new();
        let mut queue = VecDeque::new();
        let mut result = Vec::new();

        // Initialize in-degree count
        for action in actions {
            let empty_vec = Vec::new();
            let deps = dependencies.get(&action.action_id).unwrap_or(&empty_vec);
            in_degree.insert(action.action_id.clone(), deps.len());

            // If no dependencies, add to queue
            if deps.is_empty() {
                queue.push_back(action.action_id.clone());
            }
        }

        // Process queue
        while let Some(action_id) = queue.pop_front() {
            result.push(action_id.clone());

            // Find all actions that depend on this one
            for action in actions {
                if let Some(deps) = dependencies.get(&action.action_id) {
                    if deps.contains(&action_id) {
                        // Decrease in-degree
                        if let Some(degree) = in_degree.get_mut(&action.action_id) {
                            *degree -= 1;
                            if *degree == 0 {
                                queue.push_back(action.action_id.clone());
                            }
                        }
                    }
                }
            }
        }

        // Check for cycles
        if result.len() != actions.len() {
            warn!(
                "[DEPENDENCY_RESOLVER] Detected cycle in dependency graph. Expected {} actions, got {}",
                actions.len(),
                result.len()
            );
            // Return all actions in original order as fallback
            return actions.iter().map(|a| a.action_id.clone()).collect();
        }

        result
    }

    /// Gets the dependencies for a given action
    pub fn get_dependencies(&self, action_id: &str) -> Vec<String> {
        self.dependencies
            .get(action_id)
            .cloned()
            .unwrap_or_default()
    }

    /// Gets the dependents for a given action
    pub fn get_dependents(&self, action_id: &str) -> Vec<String> {
        self.dependents.get(action_id).cloned().unwrap_or_default()
    }

    /// Checks if all dependencies for an action are satisfied
    pub fn are_dependencies_satisfied(
        &self,
        action_id: &str,
        completed_tasks: &HashMap<Uuid, Task>,
    ) -> bool {
        let dependencies = self.get_dependencies(action_id);

        for dep_action_id in dependencies {
            // Check if any completed task has this action_id
            let is_satisfied = completed_tasks
                .values()
                .any(|task| task.action_id == dep_action_id);

            if !is_satisfied {
                return false;
            }
        }

        true
    }

    /// Gets the next ready actions that can be executed
    pub fn get_ready_actions(
        &self,
        actions: &[Action],
        completed_tasks: &HashMap<Uuid, Task>,
        running_tasks: &HashSet<String>,
    ) -> Vec<Action> {
        let mut ready_actions = Vec::new();

        for action in actions {
            // Skip if already running or completed
            if running_tasks.contains(&action.action_id) {
                continue;
            }

            if completed_tasks
                .values()
                .any(|task| task.action_id == action.action_id)
            {
                continue;
            }

            // Check if dependencies are satisfied
            if self.are_dependencies_satisfied(&action.action_id, completed_tasks) {
                ready_actions.push(action.clone());
            }
        }

        info!(
            "[DEPENDENCY_RESOLVER] Found {} ready actions out of {} total",
            ready_actions.len(),
            actions.len()
        );

        ready_actions
    }
}
