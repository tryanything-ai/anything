use anything_engine::executor::Executor;
use futures::Future;
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

        let all_triggered_flows = system_handler.lock().await.get_all_flows_for_trigger(evt);

        // Trigger parallel runs
        let tasks = join_parallel(all_triggered_flows.into_iter().map(|flow| {
            let flow = flow.clone();
            async move {
                let mut executor = Executor::new(&flow);
                let _run = executor.run().await;
                let run_context = executor.context.lock().unwrap();
                let latest_output = run_context.latest_output.clone();
                latest_output
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

async fn join_parallel<T: Send + 'static>(
    futs: impl IntoIterator<Item = impl Future<Output = T> + Send + 'static>,
) -> Vec<T> {
    let tasks: Vec<_> = futs.into_iter().map(tokio::spawn).collect();
    // unwrap the Result because it is introduced by tokio::spawn()
    // and isn't something our caller can handle
    futures::future::join_all(tasks)
        .await
        .into_iter()
        .map(Result::unwrap)
        .collect()
}
