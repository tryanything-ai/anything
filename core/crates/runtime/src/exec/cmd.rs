use std::process::Command;

use crate::errors::RuntimeResult;

pub trait CommandExt {
    fn export_environment(
        &mut self,
        environment: &indexmap::IndexMap<String, String>,
    ) -> RuntimeResult<()>;
}

impl CommandExt for Command {
    fn export_environment(
        &mut self,
        environment: &indexmap::IndexMap<String, String>,
    ) -> RuntimeResult<()> {
        for (key, value) in environment.into_iter() {
            self.env(key, value);
        }

        Ok(())
    }
}
