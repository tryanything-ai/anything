use std::collections::HashMap;

use anything_graph::flow::flow::Flow;

// TODO: Make this a bit more abstract
pub struct FlowHandler {
    flows: HashMap<String, Flow>,
}

impl FlowHandler {
    pub fn new() -> Self {
        FlowHandler {
            flows: HashMap::new(),
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

    pub fn get_all_flows(&self) -> Vec<Flow> {
        let mut flows = vec![];
        for (_, flow) in self.flows.iter() {
            flows.push(flow.clone());
        }
        flows
    }
}
