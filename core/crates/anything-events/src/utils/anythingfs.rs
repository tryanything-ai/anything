use std::path::{Path, PathBuf};

use crate::errors::EventsResult;

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
    use super::*;
    use std::io::Write;

    #[test]
    fn filters_files_in_a_directory_starts_with_period() {
        let tempdir = tempfile::tempdir().unwrap();
        let tempdir_str = tempdir.path().as_os_str().to_str().unwrap();

        let testable_filenames = vec![".DS_Store", "all-good.toml", "not.ok"];

        testable_filenames.into_iter().for_each(|name| {
            let file_to_write = format!("{}/{}", tempdir_str, name);
            let mut file = std::fs::File::create(file_to_write).unwrap();
            write!(file, "test").unwrap();
        });

        let files =
            safe_read_directory(PathBuf::from(tempdir_str), vec!["toml".to_string()]).unwrap();

        assert_eq!(
            files,
            vec![PathBuf::from(format!("{}/all-good.toml", tempdir_str))]
        );
    }
}
