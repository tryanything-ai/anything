use serde::{
    de::{self, MapAccess},
    Deserialize, Deserializer, Serialize,
};
use std::{
    fmt::{self, Display},
    process::{Command, Stdio},
};

use crate::{
    core::config::ExecuteConfig,
    errors::{RuntimeError, RuntimeResult},
    exec::{
        cmd::CommandExt,
        scope::{ExecutionResult, Scope},
        template::render_string,
    },
    utils::system_utils::{create_script_file, set_execute_permission},
};

pub const DEFAULT_SHELL: &str = "sh";
pub const DEFAULT_SHELL_ARG: &str = "-c";

#[derive(Debug, Clone, Serialize, PartialEq, Eq, Hash)]
pub struct SystemShell {
    #[serde(default, alias = "runtime")]
    pub interpreter: String,
    pub args: Vec<String>,
}

impl TryFrom<String> for SystemShell {
    type Error = RuntimeError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::get_from_string(&value).ok_or(RuntimeError::InvalidInterpreter)
    }
}

impl TryFrom<ExecuteConfig> for SystemShell {
    type Error = RuntimeError;

    fn try_from(value: ExecuteConfig) -> Result<Self, Self::Error> {
        match Self::try_from(value.runtime) {
            Ok(mut ss) => {
                value.args.iter().for_each(|v| {
                    ss.args.push(v.to_string());
                });
                Ok(ss)
            }
            Err(e) => Err(e),
        }
    }
}

impl Default for SystemShell {
    fn default() -> Self {
        Self {
            interpreter: DEFAULT_SHELL.to_string(),
            args: vec![DEFAULT_SHELL_ARG.to_string()],
        }
    }
}

impl Display for SystemShell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.args.is_empty() {
            write!(f, "{}", self.interpreter)
        } else {
            write!(f, "{} {}", self.interpreter, self.args.join(" "))
        }
    }
}

impl SystemShell {
    pub fn get_from_string(input: &str) -> Option<Self> {
        let mut parts = input.split(' ');
        let mut args: Vec<String> = Vec::new();

        if let Some(value) = parts.next() {
            while let Some(arg) = parts.next() {
                args.push(arg.to_string());
            }

            Some(Self {
                interpreter: value.to_string(),
                args,
            })
        } else {
            None
        }
    }

    pub fn add_arg(&mut self, arg: &str) {
        self.args.push(arg.to_string());
    }

    pub fn execute(
        &self,
        code: &str,
        scope: &Scope,
        _config: &ExecuteConfig,
    ) -> RuntimeResult<ExecutionResult> {
        let runtime_config = &scope.runtime_config;
        let mut command = Command::new(&self.interpreter);
        command.export_environment(&scope.environment)?;

        if let Some(v) = &runtime_config.current_dir {
            command.current_dir(v);
        }

        for arg in &self.args {
            command.arg(arg);
        }

        let rendered_code = render_string("command_template", code, &scope)?;

        command.arg(rendered_code);

        // handle std(in|out|err)
        let stdout = Stdio::piped();
        let stderr = Stdio::piped();
        command.stdout(stdout);
        command.stderr(stderr);
        command.stdin(Stdio::null());

        match command.output() {
            Ok(output) => {
                let stdout = String::from_utf8_lossy(&output.stdout).to_string();
                let stderr = String::from_utf8_lossy(&output.stderr).to_string();
                let status = output.status.code().unwrap_or(0);

                Ok(ExecutionResult {
                    stdout,
                    stderr,
                    status,
                })
            }
            Err(e) => Err(RuntimeError::IoError(e)),
        }
    }

    pub fn execute_script(
        &self,
        code: &str,
        scope: &Scope,
        config: &ExecuteConfig,
    ) -> RuntimeResult<ExecutionResult> {
        let (script_path, _tmp) = create_script_file(code)?;
        set_execute_permission(&script_path)?;
        self.execute(code, scope, config)
    }
}

impl<'de> Deserialize<'de> for SystemShell {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SystemShellVisitor;

        impl<'de> de::Visitor<'de> for SystemShellVisitor {
            type Value = SystemShell;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("any valid interpreter")
            }

            fn visit_map<A>(self, mut visitor: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut interpreter = SystemShell::default();

                while let Some(key) = visitor.next_key::<String>()? {
                    match key.as_ref() {
                        "runtime" | "interpreter" => {
                            interpreter.interpreter = visitor.next_value()?;
                        }
                        "args" => {
                            interpreter.args = visitor.next_value()?;
                        }
                        _ => {
                            return Err(de::Error::unknown_field(&key, &["runtime"]));
                        }
                    }
                }

                Ok(SystemShell {
                    interpreter: interpreter.interpreter,
                    args: interpreter.args,
                })
            }
        }
        deserializer.deserialize_map(SystemShellVisitor)
    }
}

#[cfg(test)]
mod tests {
    use crate::{core::config::ExecuteConfigBuilder, RuntimeConfig};

    use super::*;

    #[test]
    fn test_get_from_string() {
        let shell = SystemShell::get_from_string("bash -c").unwrap();
        assert_eq!(shell.interpreter, "bash");
        assert_eq!(shell.args, vec!["-c".to_string()]);
    }

    #[test]
    fn test_get_from_string_with_multiple_args() {
        let shell = SystemShell::get_from_string("bash -c --").unwrap();
        assert_eq!(shell.interpreter, "bash");
        assert_eq!(shell.args, vec!["-c".to_string(), "--".to_string()]);
    }

    #[test]
    fn test_get_from_string_with_no_args() {
        let shell = SystemShell::get_from_string("bash").unwrap();
        assert_eq!(shell.interpreter, "bash");
        let expected_args: Vec<String> = vec![];
        assert_eq!(shell.args, expected_args);
    }

    #[test]
    fn test_get_from_string_with_no_interpreter() {
        let shell = SystemShell::get_from_string("").unwrap();
        assert_eq!(shell.interpreter, "");
        let expected_args: Vec<String> = vec![];
        assert_eq!(shell.args, expected_args);
    }

    #[test]
    fn test_from_execution_config() {
        let config = ExecuteConfigBuilder::default()
            .runtime("bash -c --".to_string())
            .args(vec!["--arg1".to_string(), "--arg2".to_string()])
            .build()
            .unwrap();

        let shell = SystemShell::try_from(config).unwrap();
        assert_eq!(shell.interpreter, "bash");
        assert_eq!(
            shell.args,
            vec![
                "-c".to_string(),
                "--".to_string(),
                "--arg1".to_string(),
                "--arg2".to_string()
            ]
        );
    }

    #[test]
    fn test_execute_simple_command() {
        let shell = SystemShell::get_from_string("bash -c").unwrap();
        let mut config = RuntimeConfig::default();
        let tmpdir = tempfile::tempdir().unwrap();
        config.current_dir = Some(tmpdir.path().to_path_buf());
        let mut scope = Scope::default();
        scope.set_runtime_config(&config);

        let res = shell.execute("echo hello", &scope, &ExecuteConfig::default());
        assert!(res.is_ok());
        let res = res.unwrap();
        assert_eq!(res.status, 0);
    }

    #[test]
    fn test_execute_simple_command_with_curr_directory() {
        let shell = SystemShell::get_from_string("bash -c").unwrap();

        let tmpdir = tempfile::tempdir().unwrap();
        let tmpdir_str = tmpdir.path().to_string_lossy().to_string();
        let mut config = RuntimeConfig::default();
        config.current_dir = Some(tmpdir.path().to_path_buf());
        let mut scope = Scope::default();
        scope.set_runtime_config(&config);

        let res = shell.execute("pwd", &scope, &ExecuteConfig::default());
        assert!(res.is_ok());
        let res = res.unwrap();
        assert_eq!(res.status, 0);
        assert!(res.stdout.contains(tmpdir_str.as_str()));
    }

    #[test]
    fn test_execute_simple_command_with_variables() {
        let shell = SystemShell::get_from_string("bash -c").unwrap();

        let tmpdir = tempfile::tempdir().unwrap();
        let mut config = RuntimeConfig::default();
        config.current_dir = Some(tmpdir.path().to_path_buf());
        let mut scope = Scope::default();
        scope.insert_binding("name", "pablo", None).unwrap();
        scope.set_runtime_config(&config);

        let res = shell.execute("echo {{name}}", &scope, &ExecuteConfig::default());
        assert!(res.is_ok());
        let res = res.unwrap();
        assert_eq!(res.status, 0);
        assert_eq!(res.stdout, "pablo\n");
    }
}
