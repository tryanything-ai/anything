use std::sync::Arc;

use postage::stream::Stream;

use crate::{repositories::flow_repo::FlowRepo, Server, SystemChangeEvent, UpdateFlow};

#[allow(unused)]
pub async fn process_system_change_events(server: Arc<Server>) -> anyhow::Result<()> {
    let mut change_rx = server
        .post_office
        .receive_mail::<SystemChangeEvent>()
        .await?;

    while let Some(msg) = change_rx.recv().await {
        println!("System changed: {:?}", msg);

        match msg {
            SystemChangeEvent::Shutdown(_) => {
                println!("Shutdown event received");
                break;
            }
            SystemChangeEvent::FlowChange(flow) => {
                println!("Flow change event received");
                server
                    .context
                    .repositories
                    .flow_repo
                    .find_or_create_and_update(
                        flow.flow_id,
                        UpdateFlow {
                            flow_name: flow.flow_name,
                            version: Some(flow.latest_version_id),
                        },
                    )
                    .await?;
            }
        }
    }
    Ok(())
}
