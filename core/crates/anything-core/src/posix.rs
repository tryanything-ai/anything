use std::{
    fs,
    path::{Path, PathBuf},
};

pub fn copy_recursively(
    source: impl AsRef<Path>,
    destination: impl AsRef<Path>,
) -> std::io::Result<()> {
    fs::create_dir_all(&destination)?;
    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let filetype = entry.file_type()?;
        if filetype.is_dir() {
            copy_recursively(entry.path(), destination.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), destination.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}

pub fn path_contains_directory(path: &PathBuf, directory: &str) -> bool {
    for component in path.components() {
        if let Some(name) = component.as_os_str().to_str() {
            if name == directory {
                return true;
            }
        }
    }
    false
}
