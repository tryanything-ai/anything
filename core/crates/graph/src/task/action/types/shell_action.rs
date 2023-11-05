use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{
    error::RunError,
    task::{
        action::IAction,
        state::{Input, Output},
    },
};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct ShellAction {
    pub executor: Option<String>,
    pub command: String,
    pub args: Option<HashMap<String, String>>,
    pub cwd: Option<String>,
}

impl IAction for ShellAction {
    fn run(&self, input: Input) -> Result<Output, RunError> {
        todo!()
    }

    fn before_run(&self, input: Input) -> Result<Output, RunError> {
        todo!()
    }

    fn after_run(&self, input: Input) -> Result<Output, RunError> {
        todo!()
    }
}
