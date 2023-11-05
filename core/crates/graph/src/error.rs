#![allow(unused)]
use petgraph::graph;
use std::{fmt, io::Error as IoError};
use thiserror::Error;

#[cfg(feature = "tracing")]
use tracing::error;

use tera::Error as TeraError;

use crate::graph::node::{NodeType, Task};

pub type BoxedError = Box<dyn std::error::Error + Send + Sync + 'static>;
pub type GraphResult<T> = anyhow::Result<T, GraphError>;

#[derive(Debug, Error)]
pub enum GraphError {
    #[error("Panic: {0}")]
    Panic(String),

    #[error("file does not exist: {0}")]
    FileDoesNotExist(String),

    #[error("File not found: {0}")]
    FileNotFound(String),

    #[error(transparent)]
    FileReadError(#[from] std::io::Error),

    #[error("flow file parsing error")]
    FlowFileParsingError(#[from] toml::de::Error),

    #[error("Runtime error: {0}")]
    RuntimeError(String),

    #[error(transparent)]
    IOError(IoError),

    #[error("Flow error: {0}")]
    FlowError(String),

    #[error("Flow node error: {0}")]
    FlowNodeError(String),

    #[error("node \"{node}\" does not belong to \"{current_group}\"")]
    NodeNotInGroup { node: String, current_group: String },

    #[error("group \"{dependency}\" of group \"{group}\" is not defined")]
    UndefinedGroup { group: String, dependency: String },

    #[error("node \"{node}\" not found in group \"{group}\"")]
    NodeNotFound { group: String, node: String },

    #[error("error: {0}")]
    GeneralError(String),

    #[error("graph cycle error")]
    CycleError(Cycle),
}

#[derive(Debug, Error)]
pub enum RunError {
    #[error("Task run error: {0}")]
    TaskRunError(String),
}

impl From<anyhow::Error> for GraphError {
    fn from(value: anyhow::Error) -> Self {
        GraphError::GeneralError(value.to_string())
    }
}

/// The graph of groups and tasks is not acyclic
#[derive(Debug, Error)]
pub struct Cycle {
    /// A list of pre-formatted strongly connected components
    pub sccs: Vec<Vec<NodeType>>,
}

impl fmt::Display for Cycle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let to_string = |node: &NodeType| match node {
            NodeType::Task(x) => node_prefix(&x),
            NodeType::GroupStart(_) | NodeType::GroupEnd(_) => "group-start".to_string(),
        };

        write!(
            f,
            "a dependency cycle is created by the edges between the node \
             set{} ",
            if self.sccs.len() == 1 { "" } else { "s" }
        )?;

        for (is_last, scc) in self
            .sccs
            .iter()
            .enumerate()
            .map(|(i, x)| (i + 1 == self.sccs.len(), x))
        {
            let at_least_two = scc.len() >= 2;
            let exactly_two = scc.len() == 2;

            for (is_last, node) in scc.iter().enumerate().map(|(i, x)| (i + 1 == scc.len(), x)) {
                if is_last && at_least_two {
                    write!(f, r#"and "{}""#, to_string(node))?;
                } else if is_last {
                    write!(f, r#""{}""#, to_string(node))?;
                } else if exactly_two {
                    write!(f, r#""{}" "#, to_string(node))?;
                } else {
                    write!(f, r#""{}", "#, to_string(node))?;
                }
            }
            if !is_last {
                write!(f, "; ")?;
            }
        }

        Ok(())
    }
}

fn node_prefix(node: &Task) -> String {
    let to_str = &node.name;

    format!("node-{}", to_str)
}
