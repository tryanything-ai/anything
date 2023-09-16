use futures::{
    channel::mpsc::{channel, Receiver},
    SinkExt, StreamExt,
};
use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use postage::prelude::*;
use std::sync::Arc;
use tracing::info;

use crate::{
    trigger_change::{ChangeMessage, SystemChangeType},
    Server,
};

// For now, just watches the flow directory change
pub async fn file_watcher(server: Arc<Server>) -> anyhow::Result<()> {
    let mut directory_change_tx = server.post_office.post_mail::<ChangeMessage>().await?;
    // let mut execute_tx = server.post_office.post_mail::<ChangeMessage>().await?;
    // let mut token_tx = server.post_office.post_mail::<ChangeMessage>().await?;
    // let (mut tx, rx) = std::sync::mpsc::Sender<Result<notify::Event, notify::Error>>

    // let (mut tx, mut rx) = postage::mpsc::channel::<notify::Event>(128);
    // let (mut tx, rx) = postage::mpsc::channel::<notify::Event>(128);
    // let (tx, rx) = tokio::sync::broadcast::channel::<notify::Event>(128);
    // // let (tx, rx) = std::sync::mpsc::channel();
    // let mut watcher = RecommendedWatcher::new(
    //     move |res| {
    //         futures::executor::block_on(tx.send(res)).unwrap();
    //     },
    //     Config::default(),
    // )?;
    let config = server.context.config.clone();
    // watcher.watch(config.root_dir.as_path(), RecursiveMode::Recursive)?;
    let (mut watcher, mut rx) = async_watcher()?;

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher.watch(config.root_dir.as_path(), RecursiveMode::Recursive)?;

    while let Some(res) = rx.next().await {
        match res {
            Ok(event) => {
                // info!("changed: {:?}", event);
                directory_change_tx.send(event.into()).await?;
            }
            Err(e) => tracing::error!("watch error: {:?}", e),
        }
    }

    // while let Ok(change_evt) = rx.recv() {
    //     match change_evt {
    //         Ok(event) => {
    //             info!("event: {:?}", event);
    //             directory_change_tx
    //                 .send(ChangeMessage {
    //                     kind: event.kind.into(),
    //                     paths: event.paths,
    //                 })
    //                 .await?;
    //         }
    //         Err(e) => info!("watch error: {:?}", e),
    //     }
    // }

    Ok(())
}

// A worker to handle any changes in flow
pub async fn handle_system_change(server: Arc<Server>) -> anyhow::Result<()> {
    // let directory_change_rx = server.post_office.receive_mail::<ChangeMessage>().await?;
    let mut directory_change_rx = server.post_office.receive_mail::<ChangeMessage>().await?;

    // let flow_handler: FlowHandler = FlowHandler::new();
    // let current_flows: HashSet<Flow> = HashSet::new();
    // println!("msg => {:?}", msg);
    while let Some(msg) = directory_change_rx.recv().await {
        // while let msg = directory_change_rx.recv().await {
        match msg.change_type {
            SystemChangeType::Flows => {
                info!("Flows change ({:?}) at {:?}", msg.kind, msg.path);
                // TODO: Reload the flows
            }
            _ => {}
        }
    }

    Ok(())
}

fn async_watcher() -> notify::Result<(RecommendedWatcher, Receiver<notify::Result<notify::Event>>)>
{
    let (mut tx, rx) = channel(1);

    // Automatically select the best implementation for your platform.
    // You can also access each implementation directly e.g. INotifyWatcher.
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
