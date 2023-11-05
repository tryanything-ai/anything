#![allow(unused)]
use std::path::PathBuf;

pub(crate) fn add_flow_directory(path: PathBuf, name: &str) {
    let mut flow_path = path.clone();
    flow_path.push(name);
    std::fs::create_dir_all(flow_path.clone()).unwrap();

    add_flow_file_into_directory(flow_path, name);
}

pub(crate) fn add_flow_file_into_directory(path: PathBuf, name: &str) {
    let mut flow_path = path.clone();
    // flow_path.push("flows");
    std::fs::create_dir_all(flow_path.clone()).unwrap();
    flow_path.push("flow.toml");
    let toml = format!(
        r#"
    name = "{}"
    version = "0.0.1"
    description = "test flow"

    [[nodes]]
    name = "echo"

    [nodes.engine]
    engine = "bash"
    args = ["echo", "hello world"]

    "#,
        name
    );
    std::fs::write(flow_path, toml).unwrap();
}
