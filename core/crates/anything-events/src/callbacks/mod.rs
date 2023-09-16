use std::path::PathBuf;

use anything_engine::executor::Executor;
use anything_graph::flow::{flow::Flow, flowfile::Flowfile};

use crate::{errors::EventsResult, post_office::PostOffice};

// Should we rename this?
pub(crate) mod on_event;

#[derive(Debug)]
pub struct FlowRunner {
    pub flow: Flow,
    pub flow_file: PathBuf,
    pub post_office: PostOffice,
}

impl FlowRunner {
    pub fn new(flow_file: PathBuf) -> Self {
        let flow = Flowfile::from_file(flow_file.clone()).unwrap().flow;
        Self {
            flow,
            flow_file,
            post_office: PostOffice::open(),
        }
    }

    pub async fn run(&self) -> EventsResult<()> {
        let mut executor = Executor::new(&self.flow);
        let _run = executor.run().await;
        let run_context = executor.context.lock().unwrap();
        let latest_output = run_context.latest_output.clone();
        println!("Latest output: {:?}", latest_output);
        Ok(())
    }
}
