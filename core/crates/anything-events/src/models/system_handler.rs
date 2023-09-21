use std::collections::HashMap;

use anything_graph::flow::{flow::Flow, flowfile::Flowfile, trigger::Trigger};
use futures::lock::Mutex;
use once_cell::sync::OnceCell;

use crate::{config::AnythingEventsConfig, errors::EventsResult};

pub static SYSTEM_HANDLER: OnceCell<Mutex<SystemHandler>> = OnceCell::new();

// TODO: Make this a bit more abstract
#[derive(Debug, Clone)]
pub struct SystemHandler {
    flows: HashMap<String, Flow>,
    config: AnythingEventsConfig,
}

impl SystemHandler {
    pub async fn setup<'a>(config: &'a AnythingEventsConfig) -> EventsResult<()> {
        let instance = SystemHandler::new(config.clone());
        SYSTEM_HANDLER
            .set(Mutex::new(instance.clone()))
            .expect("unable to set global flow handler");
        Ok(())
    }

    pub fn global() -> &'static Mutex<SystemHandler> {
        if SYSTEM_HANDLER.get().is_none() {
            let instance = SystemHandler::new(AnythingEventsConfig::default());
            SYSTEM_HANDLER
                .set(Mutex::new(instance.clone()))
                .expect("unable to set global flow handler");
        }
        SYSTEM_HANDLER.get().expect("flow handler not initialized")
    }

    pub fn new(config: AnythingEventsConfig) -> Self {
        Self {
            flows: HashMap::new(),
            config,
        }
    }

    // pub fn get_config(&self) -> AnythingEventsConfig {
    //     self.config.clone()
    // }

    // pub fn get_flow_path(&self) -> PathBuf {
    //     self.config.root_dir.join(std::path::Path::new("flows"))
    // }

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
        let mut root_dir = self.config.root_dir.clone();
        root_dir.push("flows");
        // READ DIRECTORY AND RELOAD FLOWS
        let flow_files = std::fs::read_dir(root_dir)?;
        for flow_file in flow_files {
            let flow_file = flow_file?;
            let flow_file_path = flow_file.path();
            let flow = Flowfile::from_file(flow_file_path)
                .expect("unable to load flow")
                .flow;
            self.add_flow(flow);
        }
        Ok(())
    }

    pub async fn get_all_flow_triggers(&mut self) -> EventsResult<Vec<Trigger>> {
        let mut triggers = vec![];
        let flows = self.get_all_flows();
        for flow in flows.iter() {
            triggers.push(flow.trigger.clone());
        }
        Ok(triggers)
    }

    pub fn get_all_flows(&self) -> Vec<Flow> {
        let mut flows = vec![];
        for (_, flow) in self.flows.iter() {
            flows.push(flow.clone());
        }
        flows
    }

    // pub fn get_flow_nodes(&self, flow_name: String) -> Vec<Node> {
    // let flow = self.flows.get(&flow_name).expect("unable to find flow");
    // flow.nodes.clone()
    // }
}

#[cfg(test)]
mod tests {
    use crate::internal::test_helper::setup_test_directory;

    use super::*;

    #[tokio::test]
    async fn test_system_handler_loads_flows() -> anyhow::Result<()> {
        let config = setup_test_directory()?;
        let mut system_handler = SystemHandler::new(config);
        system_handler
            .reload_flows()
            .await
            .expect("unable to reload flows");
        let flows = system_handler.get_all_flows();
        assert_eq!(flows.len(), 1);
        let mut flow_names = vec![];
        for flow in flows {
            flow_names.push(flow.name);
        }
        assert_eq!(flow_names, vec!["SimpleFlow"]);
        Ok(())
    }

    #[tokio::test]
    async fn test_system_handler_loads_flow_triggers() -> anyhow::Result<()> {
        let config = setup_test_directory()?;
        let mut system_handler = SystemHandler::new(config);
        system_handler
            .reload_flows()
            .await
            .expect("unable to reload flows");
        let flows_triggers = system_handler.get_all_flow_triggers().await?;
        assert_eq!(flows_triggers.len(), 1);
        // assert_eq!(flow_names, vec!["SimpleFlow"]);
        Ok(())
    }
}
