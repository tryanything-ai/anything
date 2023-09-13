use std::{
    collections::HashMap,
    env::temp_dir,
    fs::{remove_file, File},
    io::BufReader,
    process::{Command, Stdio},
};

extern crate fs_extra;

use std::io::Read;

use anything_core::utils::trim_newline;
use anything_graph::flow::{
    action::{ActionType, ShellAction},
    node::Node,
};
use fs_extra::{copy_items, dir};
use miette::IntoDiagnostic;
use serde::{Deserialize, Serialize};

use crate::{
    context::{ExecutionContext, NodeExecutionContext},
    error::{EngineError, EngineResult},
    types::{ExecEnv, Process, ProcessState},
};

use super::Engine;

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct ShellEngine {
    pub config: ShellAction,
    pub process: Option<Process>,
}

impl ShellEngine {
    pub fn new(config: ShellAction) -> Self {
        Self {
            config,
            process: None,
        }
    }
}

impl ShellEngine {
    pub fn validate(&self) -> EngineResult<bool> {
        // Validate command is greater than ""
        if self.config.command.is_empty() {
            return Err(EngineError::ShellError("command is empty".to_string()));
        }

        Ok(true)
    }

    pub fn clean(&self) -> EngineResult<()> {
        if self.process.is_none() {
            tracing::error!("Cannot clean a process that has not run");
            return Err(EngineError::ShellProcessHasNotRunError);
        }
        let process = self.process.as_ref().unwrap();
        let base_dir = process.env.directory.clone();
        let uuid = process.uuid.clone();

        let stdout_path = format!("{}/{}_stdout", base_dir.to_string_lossy(), uuid);
        let stderr_path = format!("{}/{}_stderr", base_dir.to_string_lossy(), uuid);

        remove_file(stdout_path).into_diagnostic()?;
        remove_file(stderr_path).into_diagnostic()?;
        Ok(())
    }

    fn read_status(&mut self) -> EngineResult<()> {
        if self.process.is_none() {
            tracing::error!("Cannot clean a process that has not run");
            return Err(EngineError::ShellProcessHasNotRunError);
        }

        let process = self.process.as_mut().unwrap();
        let stdout_path = format!(
            "{}/{}_stdout",
            process.env.directory.to_string_lossy(),
            process.uuid
        );
        let stderr_path = format!(
            "{}/{}_stderr",
            process.env.directory.to_string_lossy(),
            process.uuid
        );

        let f = File::open(stdout_path).into_diagnostic()?;
        let mut buf_reader = BufReader::new(f);
        let mut stdout = String::new();
        buf_reader.read_to_string(&mut stdout).into_diagnostic()?;

        let f = File::open(stderr_path).into_diagnostic()?;
        let mut buf_reader = BufReader::new(f);
        let mut stderr = String::new();
        buf_reader.read_to_string(&mut stderr).into_diagnostic()?;

        let state = ProcessState {
            status: process.state.status.clone(),
            stdin: process.state.stdin.clone(),
            stderr: Some(trim_newline(&stderr)),
            stdout: Some(trim_newline(&stdout)),
        };

        // process.state = state;
        let process = process.clone();
        self.process = Some(Process {
            state: state,
            ..process
        });

        Ok(())
    }
}

impl Engine for ShellEngine {
    /// Render a shell engine command from a Node's `ShellAction` configuration
    ///
    /// For now, this "render" function accepts a mutable reference to the engine, but
    /// in future iterations, this will be immutable.
    fn render(
        &mut self,
        node: &Node,
        global_context: &ExecutionContext,
    ) -> EngineResult<NodeExecutionContext> {
        let mut exec_context = NodeExecutionContext {
            node: node.clone(),
            status: None,
            process: None,
        };

        let command = self.config.command.clone();
        let evaluated_command = global_context.render_string(&exec_context, command);

        self.config.command = evaluated_command.clone();

        let mut evaluated_args: HashMap<String, String> = HashMap::new();
        if let Some(args) = &self.config.args {
            for (key, val) in args.into_iter() {
                let evaluated_val = global_context.render_string(&exec_context, val.clone());
                evaluated_args.insert(key.clone(), evaluated_val);
            }
        }

        exec_context.node.node_action.action_type = ActionType::Shell(ShellAction {
            command: evaluated_command.clone(),
            args: Some(evaluated_args.clone()),
            ..self.config.clone()
        });

        Ok(exec_context)
    }
    /// Run a shell engine command from a Node's `ShellAction` configuration
    fn run(&mut self, _context: &NodeExecutionContext) -> EngineResult<()> {
        self.validate()?;

        let uuid = uuid::Uuid::new_v4();

        let config = self.config.clone();

        let executor = match config.executor {
            None => "/bin/sh".to_string(),
            Some(v) => v,
        }
        .clone();

        // TODO: COW this, possibly
        let base_dir = temp_dir();
        let base_dir_str = base_dir.to_string_lossy();
        match config.cwd {
            None => {}
            Some(v) => {
                // Copy files from the cwd
                let from_paths = vec![v];
                let options = dir::CopyOptions::new();
                copy_items(&from_paths, base_dir.clone(), &options)?;
            }
        };

        let stdout_path = format!("{}/{}_stdout", &base_dir_str, uuid);
        let stderr_path = format!("{}/{}_stderr", &base_dir_str, uuid);
        let stdout = Stdio::from(File::create(&stdout_path).into_diagnostic()?);
        let stderr = Stdio::from(File::create(&stderr_path).into_diagnostic()?);

        let args: HashMap<String, String> = match config.args {
            None => HashMap::default(),
            Some(v) => v,
        };

        let mut child = Command::new(executor);
        let child = child.arg("-c");
        let mut child = child.arg(config.command);

        for (key, val) in args {
            child = child.arg(format!("{}={}", key, val));
        }

        let child = child
            .current_dir(&base_dir)
            .stdin(Stdio::null())
            .stdout(stdout)
            .stderr(stderr)
            .spawn()
            .into_diagnostic()?;

        let child_pid = child.id();

        let output = child.wait_with_output().into_diagnostic()?;
        let status = ProcessState::from(&output).status;

        let state = ProcessState {
            // status:
            status,
            stdin: None, // TODO: fill this in
            stdout: Some(stdout_path),
            stderr: Some(stderr_path),
        };

        let env = ExecEnv {
            directory: base_dir,
            attached: false,
            pid: Some(child_pid),
        };

        let process = Process { uuid, env, state };
        self.process = Some(process);

        let _ = self.read_status();
        let _ = self.clean();

        Ok(())
    }

    fn process(&self) -> Option<Process> {
        self.process.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_shell_executes_with_simple() -> anyhow::Result<()> {
        let config = ShellAction {
            executor: None,
            command: "echo 'ducks'".to_string(),
            args: None,
            cwd: None,
        };

        let mut action = ShellEngine {
            config,
            process: None,
        };

        let node_context = NodeExecutionContext::default();

        action.run(&node_context)?;
        assert!(action.process.is_some());
        let process = action.process.unwrap();
        assert!(process.state.stdout.is_some());
        let stdout = process.state.stdout.unwrap();
        assert_eq!(stdout, "ducks\n");

        Ok(())
    }

    #[tokio::test]
    async fn test_shell_errors_with_no_command() -> anyhow::Result<()> {
        let mut action = ShellEngine {
            config: ShellAction {
                executor: None,
                command: "".to_string(),
                args: None,
                cwd: None,
            },
            process: None,
        };

        let node_context = NodeExecutionContext::default();
        let res = action.run(&node_context);
        assert!(res.is_err());
        Ok(())
    }
}
