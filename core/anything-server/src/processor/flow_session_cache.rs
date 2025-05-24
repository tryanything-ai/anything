use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, SystemTime};
use uuid::Uuid;

use crate::types::task_types::Task;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FlowSessionData {
    pub tasks: HashMap<Uuid, Task>, // task_id -> task
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct CachedSession {
    data: FlowSessionData,
    expires_at: SystemTime,
}

pub struct FlowSessionCache {
    cache: HashMap<Uuid, CachedSession>, // flow_session_id -> session data
    ttl: Duration,
}

impl FlowSessionCache {
    pub fn new(ttl: Duration) -> Self {
        println!(
            "[PROCESSOR] Creating new FlowSessionCache with TTL: {:?}",
            ttl
        );
        Self {
            cache: HashMap::new(),
            ttl,
        }
    }

    pub fn get(&self, flow_session_id: &Uuid) -> Option<FlowSessionData> {
        self.cache.get(flow_session_id).and_then(|entry| {
            let now = SystemTime::now();
            if entry.expires_at > now {
                Some(entry.data.clone())
            } else {
                None
            }
        })
    }

    pub fn set(&mut self, flow_session_id: &Uuid, data: FlowSessionData) {
        println!(
            "[PROCESSOR] Setting flow session cache for session_id: {}",
            flow_session_id
        );
        let expires_at = SystemTime::now() + self.ttl;
        let cached_session = CachedSession { data, expires_at };
        self.cache.insert(*flow_session_id, cached_session);
    }

    pub fn add_task(&mut self, flow_session_id: &Uuid, task: Task) -> bool {
        if let Some(cached_session) = self.cache.get_mut(flow_session_id) {
            if SystemTime::now() > cached_session.expires_at {
                return false;
            }
            cached_session.data.tasks.insert(task.task_id, task);
            true
        } else {
            false
        }
    }

    pub fn update_task(&mut self, flow_session_id: &Uuid, task: Task) -> bool {
        if let Some(cached_session) = self.cache.get_mut(flow_session_id) {
            if SystemTime::now() > cached_session.expires_at {
                return false;
            }
            cached_session.data.tasks.insert(task.task_id, task);
            true
        } else {
            false
        }
    }

    pub fn invalidate(&mut self, flow_session_id: &Uuid) {
        println!(
            "[PROCESSOR] Invalidating flow session cache for session_id: {}",
            flow_session_id
        );
        self.cache.remove(flow_session_id);
    }

    pub fn cleanup_expired(&mut self) -> usize {
        let now = SystemTime::now();
        let before_size = self.cache.len();

        self.cache.retain(|flow_session_id, cached_session| {
            let should_keep = cached_session.expires_at > now;
            if !should_keep {
                println!(
                    "[PROCESSOR] Removing expired flow session from cache: {}",
                    flow_session_id
                );
            }
            should_keep
        });

        let removed = before_size - self.cache.len();
        if removed > 0 {
            println!(
                "[PROCESSOR] Cleaned up {} expired flow sessions, {} remaining",
                removed,
                self.cache.len()
            );
        }
        removed
    }

    pub fn size(&self) -> usize {
        self.cache.len()
    }
}
