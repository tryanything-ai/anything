use std::{path::PathBuf, process::Output};

use anything_graph::flow::node::NodeState;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::EngineResult;

pub mod shell;

pub trait Engine {
    fn run(&mut self) -> EngineResult<()>;
    fn validate(&self) -> EngineResult<bool>;
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Default)]
pub struct Process {
    pub uuid: Uuid,
    pub state: ProcessState,
    pub env: ExecEnv,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Default)]
pub struct ProcessState {
    pub status: Option<NodeState>,
    pub stdin: Option<String>,
    pub stdout: Option<String>,
    pub stderr: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Default)]
pub struct ExecEnv {
    pub directory: PathBuf,
    pub attached: bool,
    pub pid: Option<u32>,
}

impl From<&Output> for ProcessState {
    fn from(value: &Output) -> Self {
        let mut stderr = None;
        let mut stdout = None;
        let stdout_str = String::from_utf8(value.clone().stdout).unwrap();
        let stderr_str = String::from_utf8(value.clone().stderr).unwrap();

        if !stderr_str.is_empty() {
            stderr = Some(stderr_str)
        }
        if !stdout_str.is_empty() {
            stdout = Some(stdout_str);
        }

        let status = match value.status.success() {
            true => NodeState::Success,
            false => NodeState::Failed,
        };

        ProcessState {
            status: Some(status),
            stdin: None,
            stdout: stdout,
            stderr: stderr,
        }
    }
}
