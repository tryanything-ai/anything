use std::time::Duration;

use anything_common::tracing::*;
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use tokio::sync::mpsc::{channel, Receiver, Sender};

use crate::{errors::Result, file_store::FileStore, types::ChangeMessage}; 

pub async fn store_watcher(notifier: Sender<ChangeMessage>, store: &FileStore) -> Result<()> {
    let (mut watcher, mut rx) = async_watcher()?;

    watcher.watch(store.base_dir.as_path(), RecursiveMode::Recursive)?;
    while let Some(res) = rx.recv().await {
        match res {
            Ok(evt) => {
                notifier.send(ChangeMessage::from(evt)).await?;
            }
            Err(e) => {
                println!("error sending to notifier: {:?}", e);
                error!("watch error: {:?}", e);
            }
        }
    }

    Ok(())
}

fn async_watcher() -> notify::Result<(RecommendedWatcher, Receiver<notify::Result<notify::Event>>)>
{
    let (tx, rx) = channel(128);

    let notify_config = notify::Config::default()
        .with_compare_contents(true)
        .with_poll_interval(Duration::from_millis(200));

    let watcher = RecommendedWatcher::new(
        move |res| {
            futures::executor::block_on(async {
                match tx.send(res).await {
                    Ok(_) => {
                        trace!("watcher event sent");
                    }
                    Err(e) => {
                        error!("watch error: {:?}", e);
                    }
                }
            })
        },
        notify_config,
    )?;

    Ok((watcher, rx))
}

#[cfg(test)]
mod tests {
    use std::{fs::OpenOptions, path::PathBuf, thread::sleep, time::Duration};

    use anything_common::copy_recursively;
    use std::io::Write;
    use tempfile::tempdir;
    use tokio::sync::mpsc::channel;

    use crate::types::{ChangeMessage, DirectoryChangeKind, SystemChangeType};

    use super::*;

    #[tokio::test]
    async fn creating_a_new_file_triggers_notice() -> Result<()> {
        let (change_tx, mut change_rx) = channel::<ChangeMessage>(1);

        let (store, _dir) = setup().await?;
        let flows_dir = store.create_directory(&["flows"])?;
        // create a new file
        let change = async move {
            let path = flows_dir.join("flow1.json");
            std::fs::File::create(&path).unwrap();

            if let Some(msg) = change_rx.recv().await {
                let msg = msg.clone() as ChangeMessage;
                assert_eq!(msg.kind, DirectoryChangeKind::Create);
                assert_eq!(msg.change_type, SystemChangeType::Flows);
            }
        };
        watch_tempdir(change_tx, store).await;
        tokio::spawn(change).await?;
        Ok(())
    }

    #[tokio::test]
    async fn creating_a_file_modification_triggers_notice() -> Result<()> {
        let (change_tx, mut change_rx) = channel::<ChangeMessage>(1);

        let (store, _dir) = setup().await?;
        let flows_dir = store.base_dir.join("flows");
        let path = flows_dir.join("simple.yaml");
        // modify file
        let change = async move {
            let mut file = OpenOptions::new().append(true).open(path.clone()).unwrap();
            let appended_contents = "hello world".to_string().clone();
            file.write_all(appended_contents.as_bytes()).unwrap();

            while let Some(msg) = change_rx.recv().await {
                let msg = msg.clone() as ChangeMessage;

                assert_eq!(msg.kind, DirectoryChangeKind::Modify);
                assert_eq!(msg.change_type, SystemChangeType::Flows);
                break;
            }
        };
        watch_tempdir(change_tx, store).await;
        tokio::spawn(change).await?;
        Ok(())
    }

    #[tokio::test]
    async fn deleting_a_file_modification_triggers_notice() -> Result<()> {
        let (change_tx, mut change_rx) = channel::<ChangeMessage>(1);

        let (store, _dir) = setup().await?;
        let flows_dir = store.base_dir.join("flows");
        let path = flows_dir.join("simple.yaml");
        // modify file
        let change = async move {
            std::fs::remove_file(path.clone()).unwrap();

            while let Some(msg) = change_rx.recv().await {
                let msg = msg.clone() as ChangeMessage;

                assert_eq!(msg.kind, DirectoryChangeKind::Remove);
                assert_eq!(msg.change_type, SystemChangeType::Flows);
                break;
            }
        };
        watch_tempdir(change_tx, store).await;
        tokio::spawn(change).await?;
        Ok(())
    }

    // ----------------- helper functions -----------------
    async fn watch_tempdir(notifier: Sender<ChangeMessage>, store: FileStore) {
        tokio::spawn(async move { store_watcher(notifier, &store).await.unwrap_or_default() });
    }

    pub async fn setup() -> Result<(FileStore, PathBuf)> {
        let store = setup_test_directory().await?;

        let base_dir = store.base_dir.clone();
        Ok((store, base_dir))
    }

    pub async fn setup_test_directory() -> Result<FileStore> {
        let simple_fixture_dir = get_fixtures_dir().join("simple");
        let temp_dir = setup_temp_dir()?;
        let store = setup_store(simple_fixture_dir, temp_dir).await.unwrap();
        Ok(store)
    }

    async fn setup_store(fixture_dir_path: PathBuf, temp_dir: PathBuf) -> Result<FileStore> {
        let store = FileStore::create(temp_dir.as_path(), &[""])?;
        let base_dir = store.base_dir.clone();

        // Ensure the fixture directory exists
        assert!(fixture_dir_path.exists(), "Fixture directory not found");

        // Copy the contents of the fixture directory to the temporary directory
        copy_recursively(fixture_dir_path, base_dir).expect("Failed to copy directory");
        Ok(store)
    }

    pub fn setup_temp_dir() -> Result<PathBuf> {
        // Create a temporary directory
        let temp_dir = tempdir().expect("Failed to create temporary directory");

        let temp_dir_pathbuf = temp_dir.into_path();
        let mut is_created = false;
        // Prevent race condition
        while !is_created {
            is_created = temp_dir_pathbuf.exists();
            sleep(Duration::from_millis(100));
        }
        // Return the temporary directory path
        Ok(temp_dir_pathbuf)
    }

    pub fn get_fixtures_dir() -> PathBuf {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("tests");
        d.push("fixtures");
        d
    }
}
