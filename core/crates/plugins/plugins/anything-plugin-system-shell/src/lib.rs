extern crate anything_runtime;

use std::process::Command;

use anything_common::tracing;
use anything_runtime::prelude::*;

pub const DEFAULT_SHELL: &str = "sh";
pub const DEFAULT_SHELL_ARGS: &[&str] = &["-c"];

#[derive(Debug, Default)]
pub struct SystemShellPlugin {
    config: RuntimeConfig,
}

impl Plugin for SystemShellPlugin {
    fn name(&self) -> &'static str {
        "system-shell"
    }

    fn on_load(&mut self, config: RuntimeConfig) {
        self.config = config;
    }

    fn on_unload(&self) {
        // Nothing to do here
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
        command
            .export_environment(&scope.environment)
            .expect("unable to export environment");

        for (k, v) in &scope.environment {
            command.env(k, v);
        }

        if let Some(value) = &self.config.current_dir {
            command.current_dir(value);
        }

        // TODO: decide if we always want this or not
        command.arg("-c");

        //FIXME: this is like context building which should be done outside the plugin in the main system
        //TODO: make context bunding work in CORE
        // for (idx, arg) in config.args.clone().into_iter().enumerate() {
        //     let rendered_arg = match render_string(&format!("arg-{}", idx), &arg, &scope) {
        //         Ok(value) => value,
        //         Err(error) => {
        //             eprintln!("unable to render arg: {}", error);
        //             return Err(Box::new(
        //                 std::io::Error::new(std::io::ErrorKind::Other, error).into(),
        //             ));
        //         }
        //     };
        //     command.arg(rendered_arg);
        // }

        let cli_command = match config.context.get("command") {
            Some(serde_json::Value::String(value)) => value.clone(),
            _ => {
                return Err(Box::new(PluginError::Custom(
                    "unable to find cli command in context".to_string(),
                )))
            }
        };

        command.arg(cli_command.clone());

        // let cli_command = match config.context.get("command") {
        //     Some(value) => value.to_string(),
        //     None => {
        //         return Err(Box::new(PluginError::Custom(
        //             "unable to find cli command in context".to_string(),
        //         )))
        //     }
        // };

        // command.arg(cli_command.clone());
        // command.arg(command);

        // for arg in &config.config {
        //     println!("Arg: {:?}", arg);
        //     command.arg(arg);
        // }
        // for arg in &config.args {
        //     println!("Arg: {:?}", arg);
        //     command.arg(arg);
        // }

        tracing::debug!("system shell config: {:#?}", config);
        println!("system-shell plugin command: {:?}", cli_command.clone());

        match command.output() {
            Ok(output) => {
                let stdout =
                    strip_newline_suffix(String::from_utf8_lossy(&output.stdout).to_string());
                let stderr =
                    strip_newline_suffix(String::from_utf8_lossy(&output.stderr).to_string());

                Ok(ExecutionResult {
                    stdout,
                    stderr,
                    status: output.status.code().unwrap_or(0),
                })
            }
            Err(error) => Err(Box::new(PluginError::RuntimeError(error))),
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

#[cfg(test)]
mod tests {
    use super::*;
    use anything_runtime::ExecuteConfigBuilder;

    // #[test]
    // fn test_execute_success() {
    //     let plugin = SystemShellPlugin::default();
    //     let scope = Scope::default();
    //     let config = ExecuteConfigBuilder::default()
    //         .args(vec!["-c".to_string(), "echo 'hello'".to_string()])
    //         .build()
    //         .unwrap();

    //     let result = plugin.execute(&scope, &config).unwrap();

    //     assert_eq!(result.status, 0);
    //     assert_eq!(result.stdout, "hello");
    //     assert_eq!(result.stderr, "");
    // }

    // #[test]
    // fn test_execute_templated_code() {
    //     let plugin = SystemShellPlugin::default();
    //     let config = ExecuteConfigBuilder::default()
    //         .args(vec!["-c".to_string(), "echo {{ name }}".to_string()])
    //         .build()
    //         .unwrap();
    //     let mut scope = Scope::default();
    //     let _ = scope.insert_binding("name", "bobby", None);

    //     let result = plugin.execute(&scope, &config).unwrap();

    //     assert_eq!(result.status, 0);
    //     assert_eq!(result.stdout, "bobby");
    //     assert_eq!(result.stderr, "");
    // }

    // #[test]
    // fn test_execute_with_err() {
    //     let plugin = SystemShellPlugin::default();
    //     let scope = Scope::default();
    //     let config = ExecuteConfigBuilder::default()
    //         .args(vec!["-c".to_string(), "echos 'hello'".to_string()])
    //         .build()
    //         .unwrap();

    //     let result = plugin.execute(&scope, &config).unwrap();

    //     assert_eq!(result.status, 127);
    //     assert_eq!(result.stderr, "sh: echos: command not found");
    //     assert_eq!(result.stdout, "");
    // }
}
