use postage::prelude::Stream;
use std::sync::Arc;

use postage::prelude::*;

use crate::{
    flow_notification::FlowFileNotification, models::event::Event, server::server::Server,
};

pub async fn process_on_events(server: Arc<Server>) -> anyhow::Result<()> {
    let mut events_rx = server.post_office.receive_mail::<Event>().await?;
    // let handler_rx = server
    //     .post_office
    //     .receive_mail::<FlowFileNotification>()
    //     .await?;

    while let Some(evt) = events_rx.recv().await {
        // Do something with this new event
        // Iterate through "registered" flows that are "listening" for events
        // based upon the event `name` and `source`
        //

        // TODO: update this

        println!("in event handler: {:?}", evt);

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
