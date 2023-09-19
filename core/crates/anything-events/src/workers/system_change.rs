use clap::error::KindFormatter;
use futures::{
    channel::mpsc::{channel, Receiver},
    SinkExt, StreamExt,
};
use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use postage::prelude::*;
use std::{path::PathBuf, sync::Arc};
use tracing::info;

use crate::{
    events::Event,
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

fn path_contains_directory(path: &PathBuf, directory: &str) -> bool {
    for component in path.components() {
        if let Some(name) = component.as_os_str().to_str() {
            if name == directory {
                return true;
            }
        }
    }
    false
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

fn async_watcher() -> notify::Result<(RecommendedWatcher, Receiver<notify::Result<notify::Event>>)>
{
    let (mut tx, rx) = channel(1);

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
    use std::fs::OpenOptions;
    use std::io::Write;
    use std::path::PathBuf;

    use crate::internal::test_helper::{get_test_context_with_config, setup_test_directory};

    use super::*;

    #[tokio::test]
    async fn change_of_file_in_root_dir_triggers_reload() -> anyhow::Result<()> {
        let config = setup_test_directory()?;
        let context = get_test_context_with_config(config.clone()).await;
        let server = Server::new(context).await?;

        let cloned_server = server.clone();

        let change_file = async move {
            let mut directory_change_rx = server
                .post_office
                .receive_mail::<ChangeMessage>()
                .await
                .unwrap();

            let mut root_dir = config.root_dir.clone();
            root_dir.push("flows");
            let flow_path = PathBuf::new().join(root_dir.clone().join("simple_flow.toml"));

            let mut file = OpenOptions::new()
                .write(true)
                .open(flow_path.clone())
                .unwrap();

            let appended_contents = "trigger = \"on_event\"";

            file.write_all(appended_contents.as_bytes()).unwrap();
            if let Some(msg) = directory_change_rx.recv().await {
                assert!(msg.path == flow_path);
                assert_eq!(msg.kind, DirectoryChangeKind::Modify);
                assert_eq!(msg.change_type, SystemChangeType::Flows);
            } else {
                assert!(false, "no message received");
            }
        };

        launch_file_watcher(cloned_server).await;
        tokio::spawn(change_file).await?;

        Ok(())
    }

    async fn launch_file_watcher(server: Arc<Server>) {
        tokio::spawn(async move { file_watcher(server).await.unwrap() });
    }
}
