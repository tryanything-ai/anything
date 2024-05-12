extern crate anything_runtime;

use std::process::Command;

use anything_common::tracing;
use anything_runtime::prelude::*;

use serde_json::Value;

pub const DEFAULT_SHELL: &str = "sh";
pub const DEFAULT_SHELL_ARGS: &[&str] = &["-c"];

#[derive(Debug, Default)]
pub struct SystemShellPlugin {
    config: RuntimeConfig,
}

impl Extension for SystemShellPlugin {
    fn name(&self) -> &'static str {
        "system-shell"
    }

    fn on_load(&mut self, config: RuntimeConfig) {
        self.config = config;
    }

    fn on_unload(&self) {
        // Nothing to do here
    }

    fn register_action(&self) -> &'static str {
        static JSON_DATA: &str = r#"{
            "trigger": false,
            "node_name": "cli_action",
            "node_label": "CLI Action",
            "icon": "<svg width=\"24\" height=\"24\" viewBox=\"0 0 24 24\" xmlns=\"http://www.w3.org/2000/svg\" fill=\"currentColor\"><path fill-rule=\"evenodd\" clip-rule=\"evenodd\" d=\"M1.5 3L3 1.5H21L22.5 3V21L21 22.5H3L1.5 21V3ZM3 3V21H21V3H3Z\"/><path d=\"M7.06078 7.49988L6.00012 8.56054L10.2427 12.8032L6 17.0459L7.06066 18.1066L12 13.1673V12.4391L7.06078 7.49988Z\"/><rect x=\"12\" y=\"16.5\" width=\"6\" height=\"1.5\"/></svg>",
            "description": "CLI Action",
            "handles": [
                {
                    "id": "a",
                    "position": "top",
                    "type": "target"
                },
                {
                    "id": "b",
                    "position": "bottom",
                    "type": "source"
                }
            ],
            "variables": [],
            "config": {
                "command": "",
                "run_folder": ""
            },
            "extension_id": "system-shell"
        }"#;

        JSON_DATA
    }
}

impl SystemShellPlugin {}

impl ExecutionPlugin for SystemShellPlugin {
    fn execute(
        &self,
        scope: &Scope,
        config: &ExecuteConfig,
    ) -> Result<ExecutionResult, Box<PluginError>> {
        let shell = match config.options.get("shell") {
            Some(PluginOption::String(shell)) => (*shell).clone(),
            _ => DEFAULT_SHELL.to_string(),
        };

        let mut command = Command::new(&shell);

        //Make the CLI execute in the folder of the flow
        if let Some(value) = &self.config.current_dir {
            command.current_dir(value);
        }

        // TODO: decide if we always want this or not
        command.arg("-c");

        let cli_command = match config.context.get("command") {
            Some(serde_json::Value::String(value)) => value.clone(),
            _ => {
                return Err(Box::new(PluginError::Custom(
                    "unable to find cli command in context".to_string(),
                )))
            }
        };

        command.arg(cli_command.clone());

        tracing::debug!("system shell config: {:#?}", config);
        println!("system-shell plugin command: {:?}", cli_command.clone());

        match command.output() {
            Ok(output) => {
                let stdout_raw = String::from_utf8_lossy(&output.stdout).to_string();
                let stdout_clean = strip_newline_suffix(stdout_raw);

                // Attempt to parse stdout as JSON. If this fails, use stdout as is.
                let stdout_json: Value = serde_json::from_str(&stdout_clean)
                    .unwrap_or_else(|_| serde_json::json!({ "output": stdout_clean }));

                let stderr =
                    strip_newline_suffix(String::from_utf8_lossy(&output.stderr).to_string());

                Ok(ExecutionResult {
                    stdout: stdout_clean, // Keep this as the cleaned-up string representation
                    stderr,
                    status: output.status.code().unwrap_or_default(),
                    result: stdout_json,
                })
            }
            Err(error) => Err(Box::new(error.into())),
        }
    }
}

fn strip_newline_suffix(s: String) -> String {
    match s.strip_suffix("\n") {
        Some(value) => value.to_string(),
        None => s,
    }
}

declare_plugin!(SystemShellPlugin, SystemShellPlugin::default);
