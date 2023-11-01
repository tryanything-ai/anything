use std::path::{Path, PathBuf};

pub fn workspace_dir() -> PathBuf {
    let output = std::process::Command::new(env!("CARGO"))
        .arg("locate-project")
        .arg("--workspace")
        .arg("--message-format=plain")
        .output()
        .unwrap()
        .stdout;
    let cargo_path = Path::new(std::str::from_utf8(&output).unwrap().trim());
    cargo_path.parent().unwrap().to_path_buf()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_find_an_existing_built_in_plugin() {
        let root_dir = workspace_dir();
        assert_eq!(root_dir.ends_with("core"), true);
    }
}
