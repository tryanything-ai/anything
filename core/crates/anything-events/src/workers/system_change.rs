use anything_core::posix::path_contains_directory;
use futures::{
    channel::mpsc::{channel, Receiver},
    SinkExt, StreamExt,
};
use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use postage::prelude::*;
use std::{path::PathBuf, sync::Arc};
use tracing::info;

use crate::{
    models::system_handler::SystemHandler,
    trigger_change::{ChangeMessage, DirectoryChangeKind, SystemChangeType},
    Server,
};

// For now, just watches the flow directory change
pub async fn file_watcher(server: Arc<Server>) -> anyhow::Result<()> {
    let mut directory_change_tx = server.post_office.post_mail::<ChangeMessage>().await?;

    let config = server.context.config.clone();
    // watcher.watch(config.root_dir.as_path(), RecursiveMode::Recursive)?;
    let (mut watcher, mut rx) = async_watcher()?;

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher.watch(config.root_dir.as_path(), RecursiveMode::Recursive)?;

    while let Some(res) = rx.next().await {
        match res {
            Ok(event) => {
                let evt = setup_change_message(event)?;
                directory_change_tx.send(evt).await?;
            }
            Err(e) => tracing::error!("watch error: {:?}", e),
        }
    }

    Ok(())
}

fn setup_change_message(evt: notify::Event) -> anyhow::Result<ChangeMessage> {
    let path = evt.paths[0].clone();
    let mut change_type = SystemChangeType::Flows;
    let mut kind = DirectoryChangeKind::Create;

    if path_contains_directory(&path, "flows") {
        change_type = SystemChangeType::Flows;
    }

    if evt.kind.is_modify() {
        kind = DirectoryChangeKind::Modify;
    } else if evt.kind.is_create() {
        kind = DirectoryChangeKind::Create;
    } else if evt.kind.is_access() {
        kind = DirectoryChangeKind::Access;
    } else if evt.kind.is_remove() {
        kind = DirectoryChangeKind::Remove;
    }

    let change_message = ChangeMessage {
        path,
        change_type,
        kind,
    };
    Ok(change_message)
}

// A worker to handle any changes in flow
pub async fn handle_system_change(server: Arc<Server>) -> anyhow::Result<()> {
    let mut directory_change_rx = server.post_office.receive_mail::<ChangeMessage>().await?;

    while let Some(msg) = directory_change_rx.recv().await {
        // while let msg = directory_change_rx.recv().await {
        println!("msg: {:?}", msg);
        match msg.change_type {
            SystemChangeType::Flows => {
                info!("Flows change ({:?}) at {:?}", msg.kind, msg.path);
                // TODO: Reload the flows
                let mut fh = SystemHandler::global().lock().await;
                fh.reload_flows().await?;
            }
            _ => {}
        }
    }

    Ok(())
}

pub fn async_watcher(
) -> notify::Result<(RecommendedWatcher, Receiver<notify::Result<notify::Event>>)> {
    let (mut tx, rx) = channel(128);

    let watcher = RecommendedWatcher::new(
        move |res| {
            futures::executor::block_on(async {
                tx.send(res).await.unwrap();
            })
        },
        Config::default(),
    )?;

    Ok((watcher, rx))
}

#[cfg(test)]
mod tests {
    use crate::workers::test_helper::TestHarness;

    use super::*;

    #[tokio::test]
    async fn create_a_new_flow_triggers_reload_of_flows() -> anyhow::Result<()> {
        let mut test: TestHarness<ChangeMessage> = TestHarness::setup().await?;
        let mut test_clone = test.clone();

        let change = async move {
            test.create_flow_file("some_new_flow.toml".to_string());

            if let Some(msg) = test.change_receiver.recv().await {
                let msg = msg.clone() as ChangeMessage;
                let file_path = msg.path.clone();
                assert!(path_contains_directory(&file_path, "flows"));
                assert_eq!(msg.kind, DirectoryChangeKind::Create);
                assert_eq!(msg.change_type, SystemChangeType::Flows);
            }
        };
        test_clone.watch_tempdir().await;
        tokio::spawn(change).await?;
        Ok(())
    }

    #[tokio::test]
    async fn change_of_flow_in_root_dir_triggers_reload() -> anyhow::Result<()> {
        let mut test: TestHarness<ChangeMessage> = TestHarness::setup().await?;
        let mut test_clone = test.clone();

        let change = async move {
            test.modify_flow_file("simple_flow.toml".to_string(), None);

            if let Some(msg) = test.change_receiver.recv().await {
                let msg = msg.clone() as ChangeMessage;
                let file_path = msg.path.clone();
                assert!(path_contains_directory(&file_path, "flows"));
                assert_eq!(msg.kind, DirectoryChangeKind::Modify);
                assert_eq!(msg.change_type, SystemChangeType::Flows);
            }
        };
        test_clone.watch_tempdir().await;
        tokio::spawn(change).await?;
        Ok(())
    }

    #[tokio::test]
    async fn delete_a_flow_triggers_reload_of_flows() -> anyhow::Result<()> {
        let mut test: TestHarness<ChangeMessage> = TestHarness::setup().await?;
        let mut test_clone = test.clone();

        let change = async move {
            test.remove_flow_file("simple_flow.toml".to_string())
                .unwrap();

            if let Some(msg) = test.change_receiver.recv().await {
                let msg = msg.clone() as ChangeMessage;
                let file_path = msg.path.clone();
                assert!(path_contains_directory(&file_path, "simple_flow.toml"));
                assert_eq!(msg.kind, DirectoryChangeKind::Remove);
                assert_eq!(msg.change_type, SystemChangeType::Flows);
            }
        };
        test_clone.watch_tempdir().await;
        tokio::spawn(change).await?;
        Ok(())
    }
}
