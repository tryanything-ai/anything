use std::{cell::RefCell, collections::HashMap, rc::Rc};

use deno_ast::MediaType;
use deno_ast::ModuleSpecifier;
use deno_ast::ParseParams;
use deno_ast::SourceMapOption;
use deno_core::ModuleSourceCode;
use deno_core::ModuleType;
use deno_core::{
    error::AnyError, resolve_import, ModuleLoadResponse, ModuleLoader, ModuleSource,
    RequestedModuleType, ResolutionKind,
};

use anyhow::anyhow;
use anyhow::bail;
use anyhow::Error;

type SourceMapStore = Rc<RefCell<HashMap<String, Vec<u8>>>>;

pub struct TypescriptModuleLoader {
    pub source_maps: SourceMapStore,
}

impl ModuleLoader for TypescriptModuleLoader {
    fn resolve(
        &self,
        specifier: &str,
        referrer: &str,
        _kind: ResolutionKind,
    ) -> Result<ModuleSpecifier, Error> {
        Ok(resolve_import(specifier, referrer)?)
    }

    fn load(
        &self,
        module_specifier: &ModuleSpecifier,
        _maybe_referrer: Option<&ModuleSpecifier>,
        _is_dyn_import: bool,
        _requested_module_type: RequestedModuleType,
    ) -> ModuleLoadResponse {
        let source_maps = self.source_maps.clone();
        fn load(
            source_maps: SourceMapStore,
            module_specifier: &ModuleSpecifier,
        ) -> Result<ModuleSource, AnyError> {
            println!("👀 load: {}", module_specifier);

            if module_specifier.as_str() == "file:///anon.js" {
                return Ok(ModuleSource::new(
                    ModuleType::JavaScript,
                    ModuleSourceCode::String(String::new().into()),
                    module_specifier,
                    None,
                ));
            }

            let (code, should_transpile, media_type, module_type) =
                if module_specifier.scheme() == "file" {
                    let path = module_specifier.to_file_path().map_err(|_| {
                        anyhow!("There was an error converting the module specifier to a file path")
                    })?;

                    let media_type = MediaType::from_path(&path);
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
                        _ => bail!("Unknown extension {:?}", path.extension()),
                    };

                    (
                        std::fs::read_to_string(&path)?,
                        should_transpile,
                        media_type,
                        module_type,
                    )
                } else if module_specifier.scheme() == "https" {
                    let url = module_specifier.to_string();
                    let response = ureq::get(&url).call()?.into_string()?;

                    (
                        response,
                        false,
                        MediaType::JavaScript,
                        ModuleType::JavaScript,
                    )
                } else {
                    println!("👀 unknown scheme {:?}", module_specifier.scheme());
                    bail!("Unknown scheme {:?}", module_specifier.scheme())
                };

            let code = if should_transpile {
                let parsed = deno_ast::parse_module(ParseParams {
                    specifier: module_specifier.clone(),
                    text: code.into(),
                    media_type,
                    capture_tokens: false,
                    scope_analysis: false,
                    maybe_syntax: None,
                })?;
                let res = parsed.transpile(
                    &deno_ast::TranspileOptions {
                        imports_not_used_as_values: deno_ast::ImportsNotUsedAsValues::Remove,
                        use_decorators_proposal: true,
                        ..Default::default()
                    },
                    &deno_ast::TranspileModuleOptions::default(),
                    &deno_ast::EmitOptions {
                        source_map: SourceMapOption::Separate,
                        inline_sources: true,
                        ..Default::default()
                    },
                )?;
                let res = res.into_source();
                let source_map = res.source_map.unwrap();
                source_maps
                    .borrow_mut()
                    .insert(module_specifier.to_string(), source_map.into_bytes());
                String::from_utf8(res.text.into_bytes()).unwrap()
            } else {
                code
            };
            Ok(ModuleSource::new(
                module_type,
                ModuleSourceCode::String(code.into()),
                module_specifier,
                None,
            ))
        }

        ModuleLoadResponse::Sync(load(source_maps, module_specifier))
    }

    fn get_source_map(&self, specifier: &str) -> Option<Vec<u8>> {
        self.source_maps.borrow().get(specifier).cloned()
    }
}
