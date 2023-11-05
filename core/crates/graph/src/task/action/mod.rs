use serde::{Deserialize, Serialize};
use std::{fmt::Debug, process::Output};

use crate::error::RunError;

use super::state::Input;
pub mod types;

/// Action trait
/// [`Action`] defines how a specific action should be run
pub trait IAction: Send + Sync {
    fn run(&self, input: Input) -> Result<Output, RunError>;
    // Hooks
    /// Before the execution of an action, the `before_run`
    /// function executes
    fn before_run(&self, input: Input) -> Result<Output, RunError>;
    /// After the execution of an action, `after_run` will
    /// execute
    fn after_run(&self, input: Input) -> Result<Output, RunError>;
}

impl Debug for dyn IAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Action")
    }
}

#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, Eq, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct Action {
    id: usize,
    pub name: String,
    pub action_type: ActionType,
}

// Action types
