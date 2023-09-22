use anything_engine::{executor::Executor, types::ProcessState};
use anything_graph::flow::node::NodeState;
use postage::prelude::Stream;
use std::sync::Arc;

// use postage::prelude::*;

use crate::{server::server::Server, system_handler::SystemHandler, Trigger};

pub async fn process_triggers(server: Arc<Server>) -> anyhow::Result<()> {
    let mut trigger_rx = server.post_office.receive_mail::<Trigger>().await?;
    let system_handler = SystemHandler::global();
    // let mut system_handler = server.system_handler.clone();
    // let handler_rx = server
    //     .post_office
    //     .receive_mail::<FlowFileNotification>()
    //     .await?;

    while let Some(evt) = trigger_rx.recv().await {
        // Do something with this new event
        // Iterate through "registered" flows that are "listening" for events
        // based upon the event `name` and `source`

        // TODO: update this

        println!("in event handler: {:?}", evt);

        let all_triggered_flows = system_handler
            .lock()
            .await
            .get_all_flows_for_trigger(evt.clone());
        let payload = evt.payload.clone();

        // Trigger parallel runs
        let tasks =
            anything_core::spawning::join_parallel(all_triggered_flows.into_iter().map(|flow| {
                let flow = flow.clone();
                let payload = payload.clone();
                async move {
                    let mut executor = Executor::new(&flow);
                    match executor.run(&payload.clone()).await {
                        Ok(_) => {
                            let run_context = executor.context.lock().unwrap();
                            let latest_output = run_context.latest_output.clone();
                            latest_output
                        }
                        Err(e) => {
                            tracing::error!("run failed: {:?}", e);
                            ProcessState {
                                status: Some(NodeState::Failed),
                                ..Default::default()
                            }
                        }
                    }
                }
            }))
            .await;

        for t in &tasks {
            println!("task: {:?}", t);
        }
        // all_triggered_flows.into_iter().for_each(f)

        // let flows = get_flows_interested_in_event(evt)
        // Hardcoded for now
        // let flow = Flowfile::from_file(PathBuf::from("examples/simple_flow.toml"))
        //     .unwrap()
        //     .flow;

        // let mut executor = Executor::new(&flow);
        // let _run = executor.run().await;
        // let run_context = executor.context.lock().unwrap();
        // let latest_output = run_context.latest_output.clone();
        // println!("Latest output: {:?}", latest_output);
    }

    Ok(())
}
