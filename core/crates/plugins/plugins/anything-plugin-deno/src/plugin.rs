use std::path::PathBuf;

use crate::function_runtime::FunctionRuntime;
use anything_runtime::prelude::*;
use serde_json::Value;

static PATH_OPTION_KEY: &'static str = "directory";
static CODE_KEY: &'static str = "code";

#[derive(Debug, Default)]
pub struct JsRuntime {
    config: RuntimeConfig,
}

impl Plugin for JsRuntime {
    fn name(&self) -> &'static str {
        "deno"
    }

    fn on_load(&mut self, config: RuntimeConfig) {
        self.config = config;
    }

    fn on_unload(&self) {
        // println!("js runtime unloaded");
    }

    fn register_action(&self) -> Value {
        let config = serde_json::json!({
            "trigger": "false",
            "node_name": "deno",
            "node_label": "CLI Action",
            "icon": "icon_placeholder",
            "description": "This plugin does XYZ",
            "variables": ["var1", "var2"],
            "config": {
                "command": "your_command",
                "run_folder": "path/to/folder"
            },
            "extension_id": "deno",
        });
        println!("Config being returned: {:?}", config);
        config
    }
}

impl JsRuntime {
    pub fn resolve_code(&self, config: &ExecuteConfig) -> PluginResult<String> {
        // First check if the code is in the current working directory
        let code = match config.args.len() {
            1 => config.args[0].clone(),
            _ => match config.options.contains_key(CODE_KEY) {
                true => match config.options.get(CODE_KEY) {
                    Some(PluginOption::String(d)) => d.clone(),
                    _ => {
                        return Err(PluginError::Custom(format!(
                            "{} not found in config",
                            CODE_KEY
                        )))
                    }
                },
                _ => "".to_string(),
            },
        };

        let code_path = PathBuf::from(code.clone());

        // If it's absolute, no need to continue searching
        // if code_path.exists() {
        //     let contents = match std::fs::read_to_string(code_path) {
        //         Ok(contents) => contents,
        //         Err(e) => return Err(PluginError::StdError(e)),
        //     };

        //     return Ok(contents);
        // }

        // Otherwise keep looking
        let cwd = match self.pathbuf_from_execute_config(config) {
            Ok(p) => p,
            Err(_e) => match self.pathbuf_from_runtime_config() {
                Ok(v) => v,
                Err(e) => return Err(e),
            },
        };

        // Is the `code` a file in the current working directory?
        let potential_script = cwd.join(code_path);
        if potential_script.exists() {
            let contents = match std::fs::read_to_string(potential_script) {
                Ok(contents) => contents,
                Err(e) => return Err(e.into()),
            };

            return Ok(contents);
        } else {
            Ok(code)
        }
    }

    pub fn pathbuf_from_execute_config(&self, config: &ExecuteConfig) -> PluginResult<PathBuf> {
        match config.options.contains_key(PATH_OPTION_KEY) {
            true => match config.options.get(PATH_OPTION_KEY) {
                Some(PluginOption::String(d)) => Ok(PathBuf::from(d)),
                _ => Err(PluginError::Custom(format!(
                    "{} not found in config",
                    PATH_OPTION_KEY
                ))),
            },
            _ => Err(PluginError::Custom(format!(
                "{} not found in config",
                PATH_OPTION_KEY
            ))),
        }
    }

    pub fn pathbuf_from_runtime_config(&self) -> PluginResult<PathBuf> {
        match &self.config.current_dir {
            Some(v) => Ok(v.clone()),
            None => match std::env::current_dir() {
                Ok(v) => Ok(v),
                _ => Err(PluginError::Custom(
                    "unable to get current working directory".to_string(),
                )),
            },
        }
    }
}

impl ExecutionPlugin for JsRuntime {
    fn execute(
        &self,
        scope: &Scope,
        config: &ExecuteConfig,
    ) -> Result<ExecutionResult, Box<PluginError>> {
        // Find the code to execute
        // If `code` points to a path in the current directory
        // If it resolves to a file local to the current working directory
        // or is the code itself, that's the code the plugin will execute
        let resolved_code = match self.resolve_code(config) {
            Ok(s) => s.clone(),
            Err(e) => return Err(Box::new(e)),
        };

        let rendered_template = render_string("deno-runtime", &resolved_code, &scope).unwrap();

        let mut runtime = match FunctionRuntime::new(rendered_template.as_str()) {
            Ok(runtime) => runtime,
            Err(e) => return Err(Box::new(e.into())),
        };

        let params = serde_json::to_string(&scope.environment).unwrap();
        let payload = r#"{}"#.to_string();
        match runtime.run(&params, &payload) {
            Ok(result) => {
                return Ok(ExecutionResult {
                    stdout: strip_newline_suffix(result),
                    stderr: "".to_string(),
                    status: 0,
                    result: serde_json::Value::Object(serde_json::Map::new()),
                })
            }
            Err(e) => return Err(Box::new(e.into())),
        }
    }
}

fn strip_newline_suffix(s: String) -> String {
    match s.strip_suffix("\n") {
        Some(value) => value.to_string(),
        None => s,
    }
}

declare_plugin!(JsRuntime, JsRuntime::default);

// #[cfg(test)]
// mod tests {

//     use super::*;

//     #[test]
//     fn test_resolve_code_absolute_directory_from_args() {
//         let fixtures_directory = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
//             .join("tests")
//             .join("fixtures");
//         let mut runtime = JsRuntime::default();
//         let runtime_config = RuntimeConfig::default();
//         runtime.on_load(runtime_config);
//         let exec_config = ExecuteConfigBuilder::default()
//             .args(vec![fixtures_directory
//                 .join("simple_script.js")
//                 .into_os_string()
//                 .into_string()
//                 .unwrap()])
//             .build()
//             .unwrap();

//         let result = runtime.execute(&Scope::default(), &exec_config).unwrap();
//         assert_eq!(result.stdout, "{\"success\":true}".to_string());
//         assert_eq!(result.stderr, "");
//         assert_eq!(result.status, 0);
//     }

//     #[test]
//     fn test_resolve_code_in_runtime_current_directory_from_args() {
//         let mut runtime = JsRuntime::default();
//         let fixtures_directory = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
//             .join("tests")
//             .join("fixtures");
//         let mut runtime_config = RuntimeConfig::default();
//         runtime_config.current_dir = Some(fixtures_directory);

//         let exec_config = ExecuteConfigBuilder::default()
//             .args(vec!["simple_script.js".to_string()])
//             .build()
//             .unwrap();

//         runtime.on_load(runtime_config);
//         let result = runtime.execute(&Scope::default(), &exec_config).unwrap();
//         assert_eq!(result.stdout, "{\"success\":true}".to_string());
//         assert_eq!(result.stderr, "");
//         assert_eq!(result.status, 0);
//     }

//     #[test]
//     fn test_resolve_code_in_exec_config_current_directory_from_args() {
//         let mut runtime = JsRuntime::default();
//         let fixtures_directory = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
//             .join("tests")
//             .join("fixtures");
//         let fixtures_directory_str = fixtures_directory.into_os_string().into_string().unwrap();

//         let runtime_config = RuntimeConfig::default();
//         let exec_config = ExecuteConfigBuilder::default()
//             .args(vec!["simple_script.js".to_string()])
//             .options(indexmap::indexmap! {
//                 "directory".to_string() => PluginOption::String(fixtures_directory_str.clone().to_string())
//             })
//             .build()
//             .unwrap();

//         runtime.on_load(runtime_config);
//         let result = runtime.execute(&Scope::default(), &exec_config).unwrap();
//         assert_eq!(result.stdout, "{\"success\":true}".to_string());
//         assert_eq!(result.stderr, "");
//         assert_eq!(result.status, 0);
//     }

//     #[test]
//     fn test_js_runtime_simple_from_args() {
//         let mut runtime = JsRuntime::default();
//         runtime.on_load(RuntimeConfig::default());

//         let exec_config = ExecuteConfigBuilder::default()
//             .args(vec![r#"export default function() {
//                 return 'js runtime stdout';
//             }"#
//             .to_string()])
//             .build()
//             .unwrap();

//         let result = runtime.execute(&Scope::default(), &exec_config).unwrap();
//         assert_eq!(result.stdout, "\"js runtime stdout\"");
//         assert_eq!(result.stderr, "");
//         assert_eq!(result.status, 0);
//     }

//     #[test]
//     fn test_js_runtime_simple_from_options() {
//         let mut runtime = JsRuntime::default();
//         runtime.on_load(RuntimeConfig::default());

//         let exec_config = ExecuteConfigBuilder::default()
//             .args(vec![])
//             .options(indexmap::indexmap! {
//             "code".to_string() => PluginOption::String(r#"export default function() {
//                     return 'js runtime stdout';
//                 }"#.to_string())
//                                                 })
//             .build()
//             .unwrap();

//         let result = runtime.execute(&Scope::default(), &exec_config).unwrap();
//         assert_eq!(result.stdout, "\"js runtime stdout\"");
//         assert_eq!(result.stderr, "");
//         assert_eq!(result.status, 0);
//     }

//     #[test]
//     fn test_js_runtime_with_templated_code() {
//         let mut runtime = JsRuntime::default();
//         runtime.on_load(RuntimeConfig::default());
//         let mut scope = Scope::default();
//         let runtime_config = RuntimeConfig::default();
//         scope.set_runtime_config(&runtime_config);
//         let _ = scope.insert_binding("name", "bobby", None);

//         let exec_config = ExecuteConfigBuilder::default()
//             .args(vec![])
//             .options(indexmap::indexmap! {
//             "code".to_string() => PluginOption::String(r#"export default function(params) {
//                     return 'js runtime stdout for {{ name }}';
//                 }"#.to_string())
//                                                 })
//             .build()
//             .unwrap();

//         let result = runtime.execute(&scope, &exec_config).unwrap();
//         assert_eq!(result.stdout, "\"js runtime stdout for bobby\"");
//         assert_eq!(result.stderr, "");
//         assert_eq!(result.status, 0);
//     }

//     #[test]
//     fn test_js_runtime_with_err() {
//         let mut runtime = JsRuntime::default();
//         let runtime_config = RuntimeConfig::default();
//         runtime.on_load(runtime_config);

//         let exec_config = ExecuteConfigBuilder::default()
//         .args(vec![])
//             .options(indexmap::indexmap! {
//             "code".to_string() => PluginOption::String("'barbie' return 'js runtime stdout';".to_string())
//         })
//             .build()
//             .unwrap();

//         let result = runtime.execute(&Scope::default(), &exec_config);
//         assert!(result.is_err());
//         let err = result.unwrap_err();
//         assert!(err
//             .to_string()
//             .contains("execution error: Uncaught SyntaxError: Unexpected token 'return'"));
//         // assert_eq!(result.stdout, "");
//         // assert!(result
//         //     .stderr
//         //     .contains("Uncaught SyntaxError: Unexpected token 'return'"));
//         // assert_eq!(result.status, 1);
//     }
// }
