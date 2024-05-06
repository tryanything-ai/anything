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

impl Extension for JsRuntime {
    fn name(&self) -> &'static str {
        "deno"
    }

    fn on_load(&mut self, config: RuntimeConfig) {
        self.config = config;
    }

    fn on_unload(&self) {
        // println!("js runtime unloaded");
    }
    // include two ## for svg cause it uses it
    //https://rahul-thakoor.github.io/rust-raw-string-literals/
    fn register_action(&self) -> &'static str {
        static JSON_DATA: &str = r##"{
            "trigger": false,
            "node_name": "deno_action",
            "node_label": "JS Action",
            "icon":  "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"30\" height=\"30\" fill=\"none\" viewBox=\"0 0 30 30\">\n<path fill=\"#fff\" d=\"M14.664 22.34a.733.733 0 0 0-.893.498l-.006.018-.996 3.691-.004.018a.732.732 0 0 0 1.414.381l.005-.017.996-3.691.004-.018a.735.735 0 0 0 .016-.084l.003-.028-.024-.12-.034-.171-.022-.108a.732.732 0 0 0-.46-.37Zm-6.942-3.802a.738.738 0 0 0-.045.114l-.007.024-.996 3.692-.005.018a.732.732 0 0 0 1.414.381l.005-.018.903-3.347a6.622 6.622 0 0 1-1.269-.864Zm-2.375-4.245a.732.732 0 0 0-.893.498l-.005.018-.996 3.692-.005.017a.732.732 0 0 0 1.414.382l.005-.018.996-3.691.005-.018a.732.732 0 0 0-.52-.88Zm22.335-.838a.732.732 0 0 0-.893.498l-.005.018-.996 3.691-.005.018a.732.732 0 0 0 1.414.382l.005-.018.996-3.692.005-.017a.732.732 0 0 0-.521-.88ZM3.178 8.525a13.383 13.383 0 0 0-1.564 4.908.732.732 0 0 0 1.252-.275l.005-.018.996-3.691.005-.018a.732.732 0 0 0-.694-.906Zm21.981.026a.732.732 0 0 0-.893.498l-.005.018-.996 3.691-.005.018a.732.732 0 0 0 1.414.382l.005-.018.996-3.691.005-.018a.732.732 0 0 0-.521-.88ZM7.513 5.04a.732.732 0 0 0-.893.5l-.005.017-.996 3.691-.005.018a.732.732 0 0 0 1.414.382l.005-.018.996-3.691.005-.018a.732.732 0 0 0-.521-.88Zm12.799.698a.732.732 0 0 0-.893.5l-.005.017-.67 2.48c.434.214.848.466 1.237.753l.064.048.783-2.9.004-.017a.732.732 0 0 0-.52-.88Zm-6.515-4.162a13.47 13.47 0 0 0-1.393.197l-.097.02-.929 3.441-.004.018a.732.732 0 0 0 1.413.381l.005-.017.997-3.692.004-.017a.728.728 0 0 0 .004-.33Zm9.166 2.55-.196.726-.005.017a.732.732 0 0 0 1.414.382l.005-.018.021-.078a13.548 13.548 0 0 0-1.153-.965l-.086-.064Zm-5.796-2.43-.353 1.31-.005.018a.732.732 0 0 0 1.414.381l.005-.017.372-1.378c-.438-.121-.881-.22-1.329-.296l-.104-.017ZM9.69 24.625a.733.733 0 0 1 1.415.382l-.005.018-.713 2.641-.1-.036a13.38 13.38 0 0 1-1.26-.546l.659-2.441.005-.018Z\"/>\n<path fill=\"#fff\" fill-rule=\"evenodd\" d=\"M6.66 14.652c0-3.415 3.392-6.161 7.754-6.161 2.098 0 3.898.58 5.314 1.688 1.761 1.377 2.426 3.273 2.996 5.33l2.284 8.517c-2.142 2.355-5.064 3.916-8.225 4.334-.19-1.289-.435-2.573-.68-3.852l-.037-.196-.142-.75c-.254-1.34-.526-2.776-.638-3.089-.134-.372-.253-.649-.665-.64-4.84.103-7.962-1.956-7.962-5.181Zm9.63-2.757a.937.937 0 1 0-1.876 0 .937.937 0 0 0 1.875 0Z\" clip-rule=\"evenodd\"/></svg>",
            "description": "JS Action",
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
                "code": ""
            },
            "extension_id": "deno"
        }"##;

        JSON_DATA
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
