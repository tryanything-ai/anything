use lazy_static::lazy_static;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

lazy_static! {
    static ref LOCAL_PATH_DIRECTORY: PathBuf = {    
        //gets /Users/uesrname/anything/apps/tauri/src-tauri/Cargo.toml
        crate::utils::project_utils::workspace_dir()
            .join("crates")
            .join("plugins")
            .join("artifacts")
    };
    static ref PLUGINS_DIRECTORY: PathBuf = {
        let current_file_path = Path::new(file!());
        //crates/plugins/artifacts
        let path = current_file_path
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("plugins")
        .join("artifacts");

        path
    };

    pub static ref BUILT_IN_PLUGINS: HashMap<&'static str, PathBuf> = {
        HashMap::from([
            (
                "system-shell",
                PLUGINS_DIRECTORY.join(format!("libanything_plugin_system_shell.dylib")),
            ),
            (
                "deno",
                PLUGINS_DIRECTORY.join(format!("libanything_plugin_deno.dylib")),
            ),
        ])
    };
}
