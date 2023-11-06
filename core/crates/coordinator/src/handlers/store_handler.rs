use crate::{events::StoreChangesPublisher, manager::Manager, models::MODELS, CoordinatorResult};
use anything_mq::new_client;
use anything_store::types::ChangeMessage;
use std::{path::PathBuf, sync::Arc};

pub async fn process_store_events(manager: Arc<Manager>) -> anyhow::Result<()> {
    let client = new_client::<StoreChangesPublisher>().await.unwrap();
    let sub = client.subscribe("store-events").await.unwrap();
    let manager_clone = manager.clone();

    while let Ok(msg) = sub.recv().await {
        match msg {
            StoreChangesPublisher::ChangeMessage(ChangeMessage { change_type, .. }) => {
                match change_type {
                    anything_store::types::SystemChangeType::Flows => {
                        // reload_flows(manager_clone).await?
                    }
                    _ => {}
                }
            }
        }
    }

    Ok(())
}

// pub async fn reload_flows(manager: Arc<Manager>) -> CoordinatorResult<()> {
//     let root_dir = manager.file_store.store_path(&["flows"]);

//     let flow_files: Vec<PathBuf> = anything_common::utils::anythingfs::read_flow_directories(
//         root_dir,
//         vec!["toml".to_string()],
//     )
//     .map_err(|e| {
//         tracing::error!("error when reading flow directories: {:#?}", e);
//         CoordinatorError::AnythingError(e)
//     })?;

//     for flow_file_path in flow_files {
//         let flow = match Flowfile::from_file(flow_file_path) {
//             Ok(flow) => flow,
//             Err(e) => {
//                 tracing::error!("error when parsing flow file: {:#?}", e);
//                 continue;
//             }
//         };
//         self.add_flow(flow.into());
//     }

//     Ok(())
// }
