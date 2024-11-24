use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, SystemTime};
use tracing::debug;

use crate::task_types::Task;
use crate::workflow_types::Workflow;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FlowSessionData {
    pub workflow: Workflow,
    pub tasks: HashMap<String, Task>, // task_id -> task
    pub metadata: HashMap<String, String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TaskInfo {
    pub task_id: String,
    pub status: TaskStatus,
    pub result: Option<serde_json::Value>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum TaskStatus {
    Pending,
    Running,
    Completed,
    Failed,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct CachedSession {
    data: FlowSessionData,
    expires_at: SystemTime,
}

pub struct FlowSessionCache {
    cache: HashMap<String, CachedSession>, // flow_session_id -> session data
    ttl: Duration,
}

impl FlowSessionCache {
    pub fn new(ttl: Duration) -> Self {
        debug!("[PROCESSOR] Creating new FlowSessionCache with TTL: {:?}", ttl);
        Self {
            cache: HashMap::new(),
            ttl,
        }
    }

    pub fn get(&self, flow_session_id: &str) -> Option<FlowSessionData> {
        self.cache.get(flow_session_id).and_then(|entry| {
            let now = SystemTime::now();
            if entry.expires_at > now {
                Some(entry.data.clone())
            } else {
                None
            }
        })
    }

    pub fn set(&mut self, flow_session_id: &str, data: FlowSessionData) {
        debug!(
            "[PROCESSOR] Setting flow session cache for session_id: {}",
            flow_session_id
        );
        let expires_at = SystemTime::now() + self.ttl;
        let cached_session = CachedSession { data, expires_at };
        self.cache.insert(flow_session_id.to_string(), cached_session);
    }

    pub fn update_task(&mut self, flow_session_id: &str, task: Task) -> bool {
        if let Some(cached_session) = self.cache.get_mut(flow_session_id) {
            if SystemTime::now() > cached_session.expires_at {
                return false;
            }
            cached_session.data.tasks.insert(task.task_id.to_string(), task);
            true
        } else {
            false
        }
    }

    pub fn remove_task(&mut self, flow_session_id: &str, task_id: &str) -> bool {
        if let Some(cached_session) = self.cache.get_mut(flow_session_id) {
            if SystemTime::now() > cached_session.expires_at {
                return false;
            }
            cached_session.data.tasks.remove(task_id).is_some()
        } else {
            false
        }
    }

    pub fn invalidate(&mut self, flow_session_id: &str) {
        debug!(
            "[PROCESSOR] Invalidating flow session cache for session_id: {}",
            flow_session_id
        );
        self.cache.remove(flow_session_id);
    }

    pub fn cleanup(&mut self) {
        debug!("[PROCESSOR] Starting flow session cache cleanup");
        let now = SystemTime::now();
        self.cache.retain(|_, session| session.expires_at > now);
    }
}
