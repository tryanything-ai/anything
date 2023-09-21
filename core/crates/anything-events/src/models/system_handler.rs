use anything_graph::flow::trigger::TriggerType;
use std::collections::HashMap;

use anything_graph::flow::{flow::Flow, flowfile::Flowfile};
use futures::lock::Mutex;
use once_cell::sync::OnceCell;

use crate::{config::AnythingEventsConfig, errors::EventsResult, Trigger};

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
            let flow = match Flowfile::from_file(flow_file_path) {
                Ok(flowfile) => flowfile.flow,
                Err(e) => {
                    tracing::error!("Failed to load flow: {}", e.to_string());
                    continue;
                }
            };
            self.add_flow(flow);
        }
        Ok(())
    }

    pub fn get_all_flows_for_trigger(&self, event_trigger: Trigger) -> Vec<Flow> {
        let mut flows = vec![];
        for (_, flow) in self.flows.iter() {
            let flow_trigger = flow.trigger.clone();
            if is_trigger_match(&event_trigger, flow_trigger) {
                flows.push(flow.clone());
            }
        }
        flows
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

fn is_trigger_match(
    event_trigger: &Trigger,
    flow_trigger: anything_graph::flow::trigger::Trigger,
) -> bool {
    let event_name = &event_trigger.event_name;
    flow_trigger
        .is_match(&event_name, &event_trigger.payload)
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use anything_graph::flow::flow::FlowBuilder;
    use anything_graph::flow::trigger as graph_trigger;
    use anything_graph::flow::trigger::Trigger as GraphTrigger;
    use serde_json::json;

    use crate::{internal::test_helper::setup_test_directory, TriggerBuilder};

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
    async fn test_system_handler_loads_flow_triggers_matches_empty() -> anyhow::Result<()> {
        let system_handler = setup_system_handler().await;

        // Event
        let evt = Trigger::new(
            "empty".to_string(),
            json!({
                "name": "bob",
            }),
            None,
        );

        let flows_triggers = system_handler.get_all_flows_for_trigger(evt);
        assert_eq!(flows_triggers.len(), 1);
        Ok(())
    }

    #[tokio::test]
    async fn test_system_handler_loads_flow_triggers_matches_webhook() -> anyhow::Result<()> {
        let system_handler = setup_system_handler().await;

        // Event
        let evt = Trigger::new(
            "webhook/".to_string(),
            json!({
                "match_url": "http://localhost:3030/anything/events",
            }),
            None,
        );

        let flows_triggers = system_handler.get_all_flows_for_trigger(evt);
        assert_eq!(flows_triggers.len(), 1);
        Ok(())
    }

    // -------------------------------
    // Helpers
    // -------------------------------
    async fn setup_system_handler() -> SystemHandler {
        let config = setup_test_directory().unwrap();
        let mut system_handler = SystemHandler::new(config);
        build_test_flows()
            .iter()
            .for_each(|flow| system_handler.add_flow(flow.clone()));
        // system_handler
        //     .reload_flows()
        //     .await
        //     .expect("unable to reload flows");
        system_handler
    }

    fn build_test_flows() -> Vec<Flow> {
        let empty_trigger = graph_trigger::EmptyTriggerBuilder::default()
            .settings(json!(
            {
                "name": "empty",
                "source": "gray",
            }))
            .build()
            .unwrap();
        let manual_trigger = graph_trigger::ManualTriggerBuilder::default()
            .settings(json!(
            {
                "name": "manual",
            }))
            .build()
            .unwrap();

        let webhook_trigger = graph_trigger::WebhookTriggerBuilder::default()
            .settings(json!(
            {
                "name": "webhook",
                "from_url": "http://localhost:8080/anything/events",
            }))
            .build()
            .unwrap();
        let schedule_trigger = graph_trigger::ScheduleTriggerBuilder::default()
            .settings(json!(
            {
                "name": "schedule",
                "cron": "0 0 0 * * *",
            }))
            .build()
            .unwrap();
        let file_change_trigger = graph_trigger::FileChangeTriggerBuilder::default()
            .settings(json!(
            {
                "name": "file_change",
                "path": "/tmp/anything/other-file.txt",
            }))
            .build()
            .unwrap();
        vec![
            FlowBuilder::default()
                .name("EmptyFlow".to_string())
                .trigger(GraphTrigger::Empty(empty_trigger))
                .build()
                .unwrap(),
            FlowBuilder::default()
                .name("ManualFlow".to_string())
                .trigger(GraphTrigger::Manual(manual_trigger))
                .build()
                .unwrap(),
            FlowBuilder::default()
                .name("WebhookFlow".to_string())
                .trigger(GraphTrigger::Webhook(webhook_trigger))
                .build()
                .unwrap(),
            FlowBuilder::default()
                .name("ScheduleFlow".to_string())
                .trigger(GraphTrigger::Schedule(schedule_trigger))
                .build()
                .unwrap(),
            FlowBuilder::default()
                .name("FileChangeFlow".to_string())
                .trigger(GraphTrigger::FileChange(file_change_trigger))
                .build()
                .unwrap(),
        ]
    }

    fn build_test_triggers() -> Vec<Trigger> {
        vec![
            // Empty
            TriggerBuilder::default()
                .payload(serde_json::Value::default())
                .build()
                .unwrap(),
            // Manual
            TriggerBuilder::default()
                .payload(json!(
                    {
                        "name": "bob",
                    }
                ))
                .build()
                .unwrap(),
            // Webhook
            TriggerBuilder::default()
                .payload(json!(
                    {
                        "from_url": "http://localhost:8080/anything/events",
                    }
                ))
                .build()
                .unwrap(),
            // Schedule
            TriggerBuilder::default()
                .payload(json!(
                    {
                        "cron": "0 0 0 * * *",
                    }
                ))
                .build()
                .unwrap(),
            // File change
            TriggerBuilder::default()
                .payload(json!(
                    {
                        "path": "/tmp/anything/other-file.txt",
                    }
                ))
                .build()
                .unwrap(),
        ]
    }
}
