use lazy_static::lazy_static;
use std::collections::HashMap;
use std::path::PathBuf;

lazy_static! {
    static ref LOCAL_PATH_DIRECTORY: PathBuf = {
        crate::utils::project_utils::workspace_dir()
            .join("crates")
            .join("plugins")
            .join("artifacts")
    };
    pub static ref BUILT_IN_PLUGINS: HashMap<&'static str, PathBuf> = {
        HashMap::from([
            (
                "system-shell",
                LOCAL_PATH_DIRECTORY.join(format!("libanything_plugin_system_shell.dylib")),
            ),
            (
                "deno",
                LOCAL_PATH_DIRECTORY.join(format!("libanything_plugin_deno.dylib")),
            ),
        ])
    };
}
