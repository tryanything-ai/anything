use crate::error::DenoPluginResult;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt::Debug, path::Path, rc::Rc, sync::Arc};

use deno_runtime::{
    deno_core::{v8, ResolutionKind},
    permissions::PermissionsContainer,
};

use crate::{error::DenoPluginError, utils::create_script_file};

pub struct FunctionRuntime {
    runtime: deno_runtime::worker::MainWorker,
    main_module_id: usize,
}

impl Debug for FunctionRuntime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FunctionRuntime")
            .field("main_module_id", &self.main_module_id)
            .finish()
    }
}

impl FunctionRuntime {
    pub fn new(code: &str) -> DenoPluginResult<Self> {
        let mut module_loader = ModuleLoader::new();

        let (script_file, _tmpdir) =
            create_script_file("main.js", code).expect("unable to create script file");
        let parent_dir = script_file.parent().expect("unable to get parent dir");

        module_loader.files.insert(
            "main.js".to_string(),
            std::fs::read_to_string(&script_file).expect("unable to read script file"),
        );

        let module_loader = Rc::new(module_loader);
        let options = deno_runtime::worker::WorkerOptions {
            module_loader,
            broadcast_channel:
                deno_runtime::deno_broadcast_channel::InMemoryBroadcastChannel::default(),
            fs: Arc::new(deno_runtime::deno_fs::RealFs),
            should_wait_for_inspector_session: false,
            ..Default::default()
        };

        let js_path = Path::new(&script_file);
        let main_module =
            deno_runtime::deno_core::resolve_path(&js_path.to_string_lossy(), parent_dir)?;
        let permissions = deno_runtime::permissions::Permissions::allow_all();

        let mut worker = deno_runtime::worker::MainWorker::bootstrap_from_options(
            main_module.clone(),
            PermissionsContainer::new(permissions),
            options,
        );

        let mut rt = tokio::runtime::Runtime::new().unwrap();
        let local = tokio::task::LocalSet::new();

        let res: anyhow::Result<i32> = local.block_on(&mut rt, async {
            let module_id = worker.preload_main_module(&main_module).await?;
            worker.evaluate_module(module_id).await?;
            worker.run_event_loop(false).await?;
            Ok(module_id as _)
        });
        let result = res?;

        Ok(Self {
            runtime: worker,
            main_module_id: result as _,
        })
    }

    #[allow(unused)]
    pub fn get_params(&mut self) -> DenoPluginResult<FunctionParams> {
        let module_ns = self
            .runtime
            .js_runtime
            .get_module_namespace(self.main_module_id)?;
        let scope = &mut self.runtime.js_runtime.handle_scope();
        let module_namespace = v8::Local::<v8::Object>::new(scope, module_ns);
        let export_name = v8::String::new(scope, "params").unwrap();
        let binding = module_namespace.get(scope, export_name.into());
        match binding {
            Some(value) => {
                if !value.is_object() {
                    return Err(DenoPluginError::ParamsIsNotAnObject);
                }
                let params: FunctionParams =
                    deno_runtime::deno_core::serde_v8::from_v8(scope, value)
                        .map_err(|_| anyhow::anyhow!("unable to deserialize params"))?;
                Ok(params)
            }
            None => Err(DenoPluginError::NoDefaultExportFound),
        }
    }

    pub fn run(&mut self, params: &String, payload: &String) -> DenoPluginResult<String> {
        let mut rt = tokio::runtime::Runtime::new().unwrap();
        let local = tokio::task::LocalSet::new();
        let result = local.block_on(&mut rt, async {
            let promise: v8::Global<v8::Value>;
            {
                let module_namespace = self
                    .runtime
                    .js_runtime
                    .get_module_namespace(self.main_module_id)?;
                let scope = &mut self.runtime.js_runtime.handle_scope();
                let module_namespace = v8::Local::<v8::Object>::new(scope, module_namespace);
                let export_name = v8::String::new(scope, "default").unwrap();
                let binding = module_namespace.get(scope, export_name.into());
                promise = match binding {
                    Some(value) => {
                        if !value.is_function() {
                            return Err(anyhow::anyhow!("No default export function found."));
                        }
                        let function: v8::Local<v8::Function> = value.try_into()?;
                        let recv = v8::undefined(scope).into();

                        // JSON decode params
                        let json =
                            deno_runtime::deno_core::JsRuntime::eval::<v8::Object>(scope, "JSON")
                                .unwrap();
                        let json = v8::Global::new(scope, json);

                        let parse = v8::String::new(scope, "parse").unwrap().into();
                        let parse = json.open(scope).get(scope, parse).unwrap();
                        let parse: v8::Local<v8::Function> = parse.try_into()?;

                        let arg = v8::String::new(scope, params.as_str()).unwrap().into();
                        let this = v8::undefined(scope).into();
                        let params = match parse.call(scope, this, &[arg]) {
                            Some(r) => Ok(r),
                            None => Err(anyhow::anyhow!("Error decoding params.")),
                        }?;

                        let arg = v8::String::new(scope, payload.as_str()).unwrap().into();
                        let this = v8::undefined(scope).into();
                        let payload = match parse.call(scope, this, &[arg]) {
                            Some(r) => Ok(r),
                            None => Err(anyhow::anyhow!("Error decoding payload.")),
                        }?;

                        let try_scope = &mut v8::TryCatch::new(scope);
                        let value = function.call(try_scope, recv, &[params, payload]).unwrap();
                        if try_scope.has_caught() || try_scope.has_terminated() {
                            dbg!("caught terminated");
                            try_scope.rethrow();
                            return Ok("".to_owned());
                        };

                        let promise_global = v8::Global::new(try_scope, value);
                        Ok(promise_global)
                    }
                    None => Err(anyhow::Error::msg("No default export found.")),
                }?;
            }

            self.runtime.js_runtime.run_event_loop(false).await?;
            let result = self.runtime.js_runtime.resolve_value(promise).await?;

            let scope = &mut self.runtime.js_runtime.handle_scope();
            let json =
                deno_runtime::deno_core::JsRuntime::eval::<v8::Object>(scope, "JSON").unwrap();
            let json = v8::Global::new(scope, json);

            let stringify = v8::String::new(scope, "stringify").unwrap().into();
            let stringify = json.open(scope).get(scope, stringify).unwrap();
            let stringify: v8::Local<v8::Function> = stringify.try_into()?;
            let result = v8::Local::<v8::Value>::new(scope, &result);
            let this = v8::undefined(scope).into();
            let result = match stringify.call(scope, this, &[result]) {
                Some(r) => Ok(r),
                None => Err(anyhow::anyhow!("Error stringifying result.")),
            }?;

            Ok(result.to_rust_string_lossy(scope))
        })?;
        return Ok(result);
    }
}

#[derive(Deserialize, Debug, Serialize)]
#[allow(dead_code)]
pub struct FunctionParam {
    name: String,
    r#type: String,
}

pub type FunctionParams = HashMap<String, FunctionParam>;

pub struct ModuleLoader {
    pub files: HashMap<String, String>,
}

impl ModuleLoader {
    pub fn new() -> Self {
        return Self {
            files: HashMap::new(),
        };
    }
}

impl deno_runtime::deno_core::ModuleLoader for ModuleLoader {
    fn resolve(
        &self,
        _specifier: &str,
        _referrer: &str,
        _kind: ResolutionKind,
    ) -> Result<deno_runtime::deno_core::ModuleSpecifier, anyhow::Error> {
        Ok(deno_runtime::deno_core::ModuleSpecifier::parse(_specifier)?)
    }

    fn load(
        &self,
        module_specifier: &deno_runtime::deno_core::ModuleSpecifier,
        _maybe_referrer: Option<&deno_runtime::deno_core::ModuleSpecifier>,
        _is_dyn_import: bool,
    ) -> std::pin::Pin<Box<deno_runtime::deno_core::ModuleSourceFuture>> {
        use deno_ast::MediaType;
        use deno_runtime::deno_core::*;
        use futures::future::FutureExt;

        let module_specifier = module_specifier.clone();
        let files = self.files.clone();
        // let media_type = MediaType::from(&path);
        async move {
            let module_specifier = module_specifier.clone();
            let path = module_specifier
                .to_file_path()
                .map_err(|_| anyhow::anyhow!("Only file: URLs are supported."))?;
            let module_file = module_specifier.path().rsplitn(2, '/').next();
            let media_type = MediaType::from_path(&path);

            let code = match module_file {
                Some(filename) => files.get(filename),
                None => None,
            }
            .ok_or(anyhow::anyhow!("File not found."))?;

            let (module_type, should_transpile) = match MediaType::from_path(&path) {
                MediaType::JavaScript | MediaType::Mjs | MediaType::Cjs => {
                    (ModuleType::JavaScript, false)
                }
                MediaType::Jsx => (ModuleType::JavaScript, true),
                MediaType::TypeScript
                | MediaType::Mts
                | MediaType::Cts
                | MediaType::Dts
                | MediaType::Dmts
                | MediaType::Dcts
                | MediaType::Tsx => (ModuleType::JavaScript, true),
                MediaType::Json => (ModuleType::Json, false),
                _ => anyhow::bail!("Unknown extension {:?}", path.extension()),
            };

            let code = if should_transpile {
                let parsed = deno_ast::parse_module(deno_ast::ParseParams {
                    specifier: module_specifier.to_string(),
                    text_info: deno_ast::SourceTextInfo::from_string(code.clone()),
                    media_type,
                    capture_tokens: false,
                    scope_analysis: false,
                    maybe_syntax: None,
                })?;
                parsed.transpile(&Default::default())?.text
            } else {
                code.clone()
            };

            let code = FastString::Owned(code.into_boxed_str());
            let module = ModuleSource::new(module_type, code, &module_specifier);

            Ok(module)
        }
        .boxed_local()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_non_async_function() {
        let code = r#"
        export default function() {
            return {"msg":"yes"};
        }
        "#;
        let mut runtime = FunctionRuntime::new(code).unwrap();
        let params = r#"{}"#.to_string();
        let payload = r#"{}"#.to_string();
        let result = runtime.run(&params, &payload);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), r#"{"msg":"yes"}"#);
    }

    #[test]
    fn test_function_runtime_http_post() {
        let code = r#"
            export default async function(params, payload) {
                let result = await fetch("https://httpbin.org/post", {
                    method: "POST",
                    body: JSON.stringify({ params, payload }),
                    headers: { "Content-Type": "application/json" },
                });
                return { params, payload };
            }
        "#;
        let mut runtime = FunctionRuntime::new(code).unwrap();
        let params = r#"{ "foo": "bar" }"#.to_string();
        let payload = r#"{ "baz": "qux" }"#.to_string();
        let result = runtime.run(&params, &payload).unwrap();
        assert_eq!(
            result,
            r#"{"params":{"foo":"bar"},"payload":{"baz":"qux"}}"#
        );
    }

    #[test]
    fn test_function_runtime_async() {
        let code = r#"
            export default async function(params, payload) {
                return new Promise((resolve, reject) => {
                    setTimeout(() => {
                        resolve({ name: "bobby" });
                    }, 1000);
                });
            }
        "#;
        let mut runtime = FunctionRuntime::new(code).unwrap();
        let params = r#"{ "foo": "bar" }"#.to_string();
        let payload = r#"{ "baz": "qux" }"#.to_string();
        let result = runtime.run(&params, &payload).unwrap();
        assert_eq!(result, r#"{"name":"bobby"}"#);
    }
}
