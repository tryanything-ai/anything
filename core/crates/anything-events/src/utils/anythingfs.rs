use std::path::{Path, PathBuf};

use crate::errors::EventsResult;

/// Flows are setup as a directory with configuration files
/// in the directory.  This function will find all the flow directories
/// and return a vector of paths to the directories.
///
/// # Arguments
///
/// * `root_dir` - The root directory to search for flow directories
/// * `safe_extensions` - A vector of file extensions to include
///
/// # Returns
///
/// * `Vec<PathBuf>` - A vector of paths to the flow directories
pub fn read_flow_directories(
    root_dir: PathBuf,
    safe_extensions: Vec<String>,
) -> EventsResult<Vec<PathBuf>> {
    let mut buf = vec![];
    let entries = std::fs::read_dir(root_dir)?;

    for entry in entries {
        let entry = entry?;
        let meta = entry.metadata()?;

        if meta.is_dir() {
            // let mut subdir = read_flow_directories(entry.path(), safe_extensions.clone())?;
            // buf.append(&mut subdir);
            let mut files = safe_read_directory(entry.path(), safe_extensions.clone())?;
            buf.append(&mut files);
        }

        // if meta.is_file() {
        //     buf.push(entry.path());
        // }
    }

    Ok(buf)
}

pub fn safe_read_directory(
    root_dir: PathBuf,
    safe_extensions: Vec<String>,
) -> EventsResult<Vec<PathBuf>> {
    let paths = std::fs::read_dir(root_dir.clone())?;
    let files = paths
        .into_iter()
        .filter_map(|entry| {
            entry.ok().and_then(|d| {
                d.path()
                    .file_name()
                    .and_then(|n| n.to_str().map(|s| String::from(s)))
            })
        })
        .filter(|f| filter_file(f, safe_extensions.clone()))
        .map(|f| root_dir.join(f))
        .collect::<Vec<PathBuf>>();

    Ok(files)
}

pub fn filter_file(file: &String, safe_extensions: Vec<String>) -> bool {
    let path = Path::new(file);
    if file.starts_with(".") || file.ends_with(".DS_Store") {
        return false;
    }
    match path.extension() {
        Some(ext) => {
            tracing::debug!(
                "filter file => {:?}.contains({:?})",
                safe_extensions,
                safe_extensions.contains(&ext.to_string_lossy().to_string())
            );
            safe_extensions.contains(&ext.to_string_lossy().to_string())
        }
        None => return false,
    }
}

#[cfg(test)]
mod tests {
    use tempfile::TempDir;

    use super::*;
    use std::io::Write;

    #[test]
    fn finds_all_folders_in_a_directory() {
        let tempdir = setup_temp_dir(vec![]);
        write_files_to_directory(
            tempdir.path().to_path_buf(),
            "Charlie's Flow".to_string(),
            vec!["flow.toml"],
        );
        write_files_to_directory(
            tempdir.path().to_path_buf(),
            "Mariannas's Flow".to_string(),
            vec!["flow.toml"],
        );

        let files =
            read_flow_directories(tempdir.path().to_path_buf(), vec!["toml".to_string()]).unwrap();

        assert_eq!(files.len(), 2);
        assert_eq!(
            files[0].as_path().file_name().unwrap().to_str().unwrap(),
            "flow.toml"
        );
        assert_eq!(
            files[1].as_path().file_name().unwrap().to_str().unwrap(),
            "flow.toml"
        );
    }

    #[test]
    fn filters_files_in_a_directory_starts_with_period() {
        let testable_filenames = vec![".DS_Store", "all-good.toml", "not.ok"];
        let tempdir = setup_temp_dir(testable_filenames);
        let tempdir_str = tempdir.path().as_os_str().to_str().unwrap();

        let files =
            safe_read_directory(PathBuf::from(tempdir_str), vec!["toml".to_string()]).unwrap();

        assert_eq!(
            files,
            vec![PathBuf::from(format!("{}/all-good.toml", tempdir_str))]
        );
    }

    fn setup_temp_dir(filenames: Vec<&str>) -> TempDir {
        let tempdir = tempfile::tempdir().unwrap();
        let tempdir_str = tempdir.path().as_os_str().to_str().unwrap();

        filenames.into_iter().for_each(|name| {
            let file_to_write = format!("{}/{}", tempdir_str, name);
            let mut file = std::fs::File::create(file_to_write).unwrap();
            write!(file, "test").unwrap();
        });
        tempdir
    }

    fn write_files_to_directory(tempdir: PathBuf, dir_name: String, filenames: Vec<&str>) {
        let dir = tempdir.join(dir_name);

        if !dir.exists() {
            std::fs::create_dir_all(dir.clone()).unwrap();
        }

        let dir = dir.clone();
        filenames.into_iter().for_each(|name| {
            let path_to_file = dir.clone().join(Path::new(name));
            let mut file = std::fs::File::create(path_to_file).unwrap();
            write!(file, "test").unwrap();
        });
    }
}
