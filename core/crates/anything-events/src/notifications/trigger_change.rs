use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum TriggerChange {
    AddFlow,
    RemoveFlow,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum DirectoryChangeKind {
    Any,
    Access,
    Create,
    Modify,
    Remove,
    Other,
}

impl From<notify::EventKind> for DirectoryChangeKind {
    fn from(value: notify::EventKind) -> Self {
        match value {
            notify::EventKind::Access(_) => Self::Access,
            notify::EventKind::Create(_) => Self::Create,
            notify::EventKind::Modify(_) => Self::Modify,
            notify::EventKind::Remove(_) => Self::Remove,
            _ => Self::Other,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum SystemChangeType {
    Database,
    Logs,
    Flows,
    Nodes,
    Unknown,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ChangeMessage {
    pub kind: DirectoryChangeKind,
    pub path: PathBuf,
    pub change_type: SystemChangeType,
}

impl ChangeMessage {
    pub fn new(kind: DirectoryChangeKind, path: PathBuf, change_type: SystemChangeType) -> Self {
        Self {
            kind,
            path,
            change_type,
        }
    }
}

impl From<notify::Event> for ChangeMessage {
    fn from(value: notify::Event) -> Self {
        let kind = value.kind.into();
        let path = value.paths.first().unwrap().clone();

        let change_type = if path.to_str().unwrap().contains("nodes") {
            SystemChangeType::Nodes
        } else if path.to_str().unwrap().contains("flows") {
            SystemChangeType::Flows
        } else if path.to_str().unwrap().contains("logs") {
            SystemChangeType::Logs
        } else if path.to_str().unwrap().contains("database") {
            SystemChangeType::Database
        } else {
            SystemChangeType::Unknown
        };
        // let paths = value.paths.iter().map(|p| p.clone()).collect();
        // Grab the first path and use that to determine the change type
        Self::new(kind, path, change_type)
    }
}
