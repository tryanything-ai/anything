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
    static ref CORE_PATH_DIRECTORY: PathBuf = {
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

        // let mut path = dirs::home_dir().unwrap();
        // path.push("anything");
        // path.push("core");
        // path.push("crates");
        // path.push("plugins");
        // path.push("artifacts");
        // println!("CORE_PATH_DIR: {}", CORE_PATH_DIRECTORY.display());
        path
    };
    pub static ref BUILT_IN_PLUGINS: HashMap<&'static str, PathBuf> = {
        HashMap::from([
            (
                "system-shell",
                CORE_PATH_DIRECTORY.join(format!("libanything_plugin_system_shell.dylib")),
            ),
            (
                "deno",
                CORE_PATH_DIRECTORY.join(format!("libanything_plugin_deno.dylib")),
            ),
        ])
    };
}
