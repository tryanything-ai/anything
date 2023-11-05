use crate::{events::StoreChangesPublisher, manager::Manager, models::MODELS};
use anything_mq::new_client;
use anything_store::types::ChangeMessage;
use std::sync::Arc;

pub async fn process_store_events(_manager: Arc<Manager>) -> anyhow::Result<()> {
    let client = new_client::<StoreChangesPublisher>().await.unwrap();
    let sub = client.subscribe("store-events").await.unwrap();

    while let Ok(msg) = sub.recv().await {
        match msg {
            StoreChangesPublisher::ChangeMessage(ChangeMessage { change_type, .. }) => {
                match change_type {
                    anything_store::types::SystemChangeType::Flows => {
                        match MODELS.get().unwrap().try_lock() {
                            Ok(mut models) => {
                                let res = models.flows.reload_flows().await;
                                tracing::debug!("reloaded flows: {:?}", res);
                            }
                            Err(_) => {
                                // Lock is poisoned
                            }
                        }
                    }
                    _ => {
                        // Others not yet handled
                    }
                }
            }
        }
    }

    Ok(())
}
