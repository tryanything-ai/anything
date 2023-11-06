use std::path::{Path, PathBuf};

use tokio::sync::mpsc::Sender;

use crate::{
    errors::{Result, StoreError},
    types::ChangeMessage,
    watcher::store_watcher,
    STORE_DIR,
};

#[derive(Debug, Clone)]
pub struct FileStore {
    pub base_dir: PathBuf,
}

impl Default for FileStore {
    fn default() -> Self {
        let tempdir = tempfile::tempdir().unwrap().path().to_path_buf();
        Self { base_dir: tempdir }
    }
}

impl FileStore {
    pub fn new(root_dir: &Path, base_dir: &[&str]) -> Self {
        let base_dir = Self::build_base_dir(root_dir, base_dir);
        Self { base_dir }
    }

    pub fn create(root_dir: &Path, base_dir: &[&str]) -> Result<Self> {
        let base_dir = Self::build_base_dir(root_dir, base_dir);

        std::fs::create_dir_all(&base_dir).map_err(|err| StoreError::UnableToCreateDirectory {
            path: base_dir.clone(),
            err,
        })?;
        Ok(Self { base_dir })
    }

    pub async fn notify_changes(&mut self, tx: Sender<ChangeMessage>) -> Result<()> {
        store_watcher(tx, &self).await?;
        Ok(())
    }

    pub async fn get_files_in_dir(
        &mut self,
        dir_path: &[&str],
        extensions: &[&str],
    ) -> Result<Vec<PathBuf>> {
        let path = self.store_path(dir_path);
        let mut files = vec![];
        let paths = std::fs::read_dir(path.clone())
            .map_err(|err| StoreError::UnableToReadDirectory {
                path: path.clone(),
                err,
            })
            .expect("unable to read directory");

        for path in paths {
            let path = path.expect("unable to get path");
            let file_path = path.path();
            let extension = file_path.extension();
            // Skip empty extensions
            if extension.is_none() {
                continue;
            }
            let extension = extension.unwrap();
            if file_path.is_file() && extensions.contains(&extension.to_str().unwrap()) {
                files.push(path.path());
            }
        }
        Ok(files)
    }

    pub fn create_directory(&self, dir_path: &[&str]) -> Result<PathBuf> {
        let path = self.store_path(dir_path);
        std::fs::create_dir_all(&path)
            .map_err(|err| StoreError::UnableToCreateDirectory {
                path: path.clone(),
                err,
            })
            .map(|_| path)
    }

    pub fn delete_directory(&self, dir_path: &[&str]) -> Result<()> {
        let path = self.store_path(dir_path);
        std::fs::remove_dir_all(&path).map_err(|err| StoreError::UnableToDeleteDirectory {
            path: path.clone(),
            err,
        })
    }

    pub fn create_base_dir(&self) -> Result<()> {
        std::fs::create_dir_all(&self.base_dir).map_err(|err| StoreError::UnableToCreateDirectory {
            path: self.base_dir.clone(),
            err,
        })
    }

    pub fn cleanup_base_dir(&self) -> Result<()> {
        if self.base_dir.exists() {
            std::fs::remove_dir_all(&self.base_dir).map_err(|err| {
                StoreError::UnableToDeleteDirectory {
                    path: self.base_dir.clone(),
                    err,
                }
            })
        } else {
            Ok(())
        }
    }

    pub fn file_exists(&self, file_path: &[&str]) -> bool {
        self.store_path(file_path).exists()
    }

    pub fn write_file(&self, path: &[&str], content: &[u8]) -> Result<()> {
        let path = self.store_path(path);
        std::fs::write(&path, content).map_err(|err| StoreError::UnableToWriteFile {
            path: path.clone(),
            err,
        })
    }

    pub fn read_file(&self, path: &[&str]) -> Result<Vec<u8>> {
        let path = self.store_path(path);
        std::fs::read(&path).map_err(|err| StoreError::UnableToReadFile {
            path: path.clone(),
            err,
        })
    }

    pub fn copy_file(&self, source: &Path, dest: &[&str]) -> Result<()> {
        let dest = self.store_path(dest);
        std::fs::copy(source, &dest).map_err(|err| StoreError::UnableToWriteFile {
            path: dest.clone(),
            err,
        })?;
        Ok(())
    }

    pub fn file_hash(path: &Path) -> Result<String> {
        let content = std::fs::read(path).map_err(|err| StoreError::UnableToReadFile {
            path: path.to_path_buf(),
            err,
        })?;

        Ok(blake3::hash(&content).to_string())
    }

    pub fn store_path(&self, file_path: &[&str]) -> PathBuf {
        file_path
            .iter()
            .fold(self.base_dir.clone(), |acc, x| acc.join(x))
    }

    fn build_base_dir(root_dir: &Path, base_dir: &[&str]) -> PathBuf {
        base_dir
            .iter()
            .fold(root_dir.join(STORE_DIR), |acc, x| acc.join(x))
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use tokio::{
        sync::mpsc,
        time::{sleep, timeout},
    };

    use crate::types::DirectoryChangeKind;

    use super::*;

    // To ensure we're not in a race with the OS, we need to wait some time
    // between fs operations
    const SLEEP_TIME: u64 = 100;

    #[test]
    fn test_store_path() {
        let tempdir = tempfile::tempdir().unwrap().path().to_path_buf();
        let store = FileStore::new(&tempdir.as_path(), &["foo", "bar"]);
        let path = store.store_path(&["baz", "qux"]);
        assert_eq!(path, tempdir.join(".store/foo/bar/baz/qux"));
    }

    #[test]
    fn test_create_store() {
        let tempdir = tempfile::tempdir().unwrap().path().to_path_buf();
        let store = FileStore::create(&tempdir.as_path(), &["foo", "bar"]).unwrap();
        assert!(store.base_dir.exists());
    }

    #[test]
    fn test_cleanup_store() {
        let tempdir = tempfile::tempdir().unwrap().path().to_path_buf();
        let store = FileStore::create(&tempdir.as_path(), &["foo", "bar"]).unwrap();
        assert!(store.base_dir.exists());
        store.cleanup_base_dir().unwrap();
        assert!(!store.base_dir.exists());
    }

    #[tokio::test]
    async fn test_read_and_write_file_in_store() {
        let tempdir = tempfile::tempdir().unwrap().path().to_path_buf();
        let store = FileStore::create(&tempdir.as_path(), &["skipper"]).unwrap();
        assert!(store.base_dir.exists());
        store
            .write_file(&["motd.txt"], "Hello, world!".as_bytes())
            .unwrap();
        assert!(store.file_exists(&["motd.txt"]));
        sleep(Duration::from_millis(SLEEP_TIME)).await;
        let content = store.read_file(&["motd.txt"]).unwrap();
        assert_eq!(content, "Hello, world!".as_bytes());
    }

    #[tokio::test]
    async fn test_copy_to_store() {
        let tempdir = tempfile::tempdir().unwrap().path().to_path_buf();
        let store = FileStore::create(&tempdir.as_path(), &["skipper"]).unwrap();
        assert!(store.base_dir.exists());
        let source = tempdir.join("source.txt");
        std::fs::write(&source, "Hello, world!").unwrap();
        store.copy_file(&source, &["motd.txt"]).unwrap();
        sleep(Duration::from_millis(SLEEP_TIME)).await;
        let content = store.read_file(&["motd.txt"]).unwrap();
        assert_eq!(content, "Hello, world!".as_bytes());
    }

    #[tokio::test]
    async fn test_can_list_all_files_in_directory_with_extension() {
        let tmpdir = tempfile::tempdir().unwrap().path().to_path_buf();
        let mut store = FileStore::create(&tmpdir.as_path(), &["skipper"]).unwrap();
        assert!(store.base_dir.exists());

        // Write a few files
        store
            .write_file(&["motd.txt"], "Hello, world!".as_bytes())
            .unwrap();

        store
            .write_file(&["other.toml"], "name = \"bob\"".as_bytes())
            .unwrap();

        store
            .write_file(&["flag.txt"], "Capture the flag".as_bytes())
            .unwrap();

        let files = store.get_files_in_dir(&[""], &["txt"]).await.unwrap();
        assert_eq!(files.len(), 2);
        assert!(files.contains(&store.store_path(&["motd.txt"])));
        assert!(files.contains(&store.store_path(&["flag.txt"])));
    }

    #[tokio::test]
    async fn test_notify_changes() {
        let tmpdir = tempfile::tempdir().unwrap().path().to_path_buf();
        let store = FileStore::create(&tmpdir.as_path(), &["skipper"]).unwrap();
        assert!(store.base_dir.exists());

        let (tx, mut rx) = mpsc::channel(1);

        let mut s = store.clone();
        tokio::spawn(async move {
            let _res = s.notify_changes(tx.clone()).await;
        });

        let s2 = store.clone();
        // let new_tmp_dir = tempfile::tempdir().unwrap().path().to_path_buf();
        let server_task = tokio::spawn(async move {
            let source = tmpdir.join("source.txt");
            std::fs::write(&source, "Hello, world!").unwrap();
            s2.copy_file(&source, &["other.txt"]).unwrap();
            let content = s2.read_file(&["other.txt"]).unwrap();
            assert_eq!(content, "Hello, world!".as_bytes());
            let msg = rx.recv().await;
            assert!(msg.is_some());
            let msg = msg.unwrap();
            assert_eq!(msg.kind, DirectoryChangeKind::Create);
        });

        let res = timeout(Duration::from_secs(10), server_task).await;
        assert!(res.is_ok(), "Server task did not quit");
    }
}
