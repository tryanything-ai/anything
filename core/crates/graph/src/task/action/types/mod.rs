use serde::{Deserialize, Serialize};
mod shell_action;

use shell_action::ShellAction;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum ActionType {
    Shell(ShellAction),
}
