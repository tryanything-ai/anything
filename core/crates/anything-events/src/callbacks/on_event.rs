use std::{path::PathBuf, sync::Arc};

use anything_engine::executor::Executor;
use anything_graph::flow::flowfile::Flowfile;

use crate::{models::event::Event, server::server::Server};

pub async fn process_on_events(server: Arc<Server>) -> anyhow::Result<()> {
    let events_rx = server.post_office.receive_mail::<Event>().await?;

    while let Ok(_evt) = events_rx.recv() {
        // Do something with this new event
        // Iterate through "registered" flows that are "listening" for events
        // based upon the event `name` and `source`
        //
        // let flows = get_flows_interested_in_event(evt)
        // Hardcoded for now
        let flow = Flowfile::from_file(PathBuf::from("examples/simple_flow.toml"))
            .unwrap()
            .flow;

        let mut executor = Executor::new(&flow);
        let _run = executor.run().await;
        let run_context = executor.context.lock().unwrap();
        let latest_output = run_context.latest_output.clone();
        println!("Latest output: {:?}", latest_output);
    }

    Ok(())
}
