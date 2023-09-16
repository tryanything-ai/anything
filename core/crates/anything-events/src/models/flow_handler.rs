use std::collections::HashMap;

use anything_graph::flow::flow::Flow;
use futures::lock::Mutex;
use once_cell::sync::OnceCell;

use crate::{config::AnythingEventsConfig, errors::EventsResult};

pub static FLOW_HANDLER: OnceCell<Mutex<FlowHandler>> = OnceCell::new();

// TODO: Make this a bit more abstract
#[derive(Debug, Clone)]
pub struct FlowHandler {
    flows: HashMap<String, Flow>,
    config: AnythingEventsConfig,
}

impl FlowHandler {
    pub fn global() -> &'static Mutex<FlowHandler> {
        FLOW_HANDLER.get().expect("flow handler not initialized")
    }

    pub fn new(config: AnythingEventsConfig) -> Self {
        FlowHandler {
            flows: HashMap::new(),
            config,
        }
    }

    pub fn clear(&mut self) {
        self.flows.clear();
    }

    pub fn add_flow(&mut self, flow: Flow) {
        self.flows.insert(flow.name.clone(), flow);
    }

    pub fn remove_flow(&mut self, flow_name: String) {
        self.flows.remove(&flow_name);
    }

    pub async fn reload_flows(&mut self) -> EventsResult<()> {
        Ok(())
    }

    pub fn get_all_flows(&self) -> Vec<Flow> {
        let mut flows = vec![];
        for (_, flow) in self.flows.iter() {
            flows.push(flow.clone());
        }
        flows
    }
}
