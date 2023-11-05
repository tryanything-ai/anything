use std::path::PathBuf;

use anything_graph::{Flow, Flowfile};
use anything_store::FileStore;
use dashmap::DashMap;

use crate::error::{CoordinatorError, CoordinatorResult};

#[derive(Debug, Clone)]
pub struct FlowsModel {
    flows: DashMap<String, Flow>,
    file_store: FileStore,
}

impl FlowsModel {
    pub fn new(file_store: FileStore) -> Self {
        Self {
            flows: DashMap::new(),
            file_store,
        }
    }

    #[allow(unused)]
    pub fn clear(&mut self) {
        self.flows.clear();
    }

    #[allow(unused)]
    pub fn add_flow(&mut self, flow: Flow) {
        self.flows.insert(flow.name.clone(), flow);
    }

    #[allow(unused)]
    pub fn get_flow(&self, name: &str) -> Option<Flow> {
        self.flows.get(name).map(|f| f.value().clone())
    }

    #[allow(unused)]
    pub fn remove_flow(&mut self, name: &str) -> Option<Flow> {
        self.flows.remove(name).map(|(_name, f)| f)
    }

    pub fn get_flows(&self) -> Vec<Flow> {
        self.flows.iter().map(|f| f.clone()).collect()
    }

    pub async fn reload_flows(&mut self) -> CoordinatorResult<()> {
        let root_dir = self.file_store.store_path(&["flows"]);

        let flow_files: Vec<PathBuf> = anything_common::utils::anythingfs::read_flow_directories(
            root_dir,
            vec!["toml".to_string()],
        )
        .map_err(|e| {
            tracing::error!("error when reading flow directories: {:#?}", e);
            CoordinatorError::AnythingError(e)
        })?;

        for flow_file_path in flow_files {
            let flow = match Flowfile::from_file(flow_file_path) {
                Ok(flow) => flow,
                Err(e) => {
                    tracing::error!("error when parsing flow file: {:#?}", e);
                    continue;
                }
            };
            self.add_flow(flow.into());
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{config::AnythingConfig, manager::Manager, test_helper::add_flow_directory};

    use super::*;
    use anything_graph::FlowBuilder;
    use anything_runtime::RuntimeConfig;

    #[test]
    fn test_flows_model_can_be_created() {
        let file_store = FileStore::default();
        let _manager = Manager::default();
        let flows_model = FlowsModel::new(file_store);
        assert!(flows_model.flows.is_empty());
    }

    #[test]
    fn test_flows_model_can_add_a_flow() {
        let file_store = FileStore::default();
        let mut flows_model = FlowsModel::new(file_store);
        let flow = FlowBuilder::default()
            .name("test".to_string())
            .build()
            .unwrap();
        flows_model.add_flow(flow.clone());
        assert_eq!(flows_model.flows.len(), 1);
        assert_eq!(
            flows_model.flows.get(&flow.name).unwrap().name,
            "test".to_string()
        );
    }

    #[test]
    fn test_flows_model_can_remove_a_flow() {
        let file_store = FileStore::default();
        let mut flows_model = FlowsModel::new(file_store);
        let flow = FlowBuilder::default()
            .name("test".to_string())
            .build()
            .unwrap();
        flows_model.add_flow(flow.clone());
        assert_eq!(flows_model.flows.len(), 1);
        flows_model.remove_flow(&flow.name);
        assert_eq!(flows_model.flows.len(), 0);
    }

    #[test]
    fn test_flows_model_can_get_a_flow_by_name() {
        let file_store = FileStore::default();
        let mut flows_model = FlowsModel::new(file_store);
        let flow = FlowBuilder::default()
            .name("test".to_string())
            .build()
            .unwrap();
        flows_model.add_flow(flow.clone());
        assert_eq!(flows_model.flows.len(), 1);
        let flow = flows_model.get_flow(&flow.name).unwrap();
        assert_eq!(flow.name, "test".to_string());
    }

    #[test]
    fn test_flows_model_can_get_all_flows() {
        let file_store = FileStore::default();
        let mut flows_model = FlowsModel::new(file_store);
        let flow = FlowBuilder::default()
            .name("test".to_string())
            .build()
            .unwrap();
        flows_model.add_flow(flow.clone());
        assert_eq!(flows_model.flows.len(), 1);
        let flows = flows_model.get_flows();
        assert_eq!(flows.len(), 1);
        assert_eq!(flows[0].name, "test".to_string());
    }

    #[tokio::test]
    async fn test_flows_model_can_reload_flows_in_a_directory() {
        let mut config = AnythingConfig::default();
        let tmpdir = tempfile::tempdir().unwrap().path().to_path_buf();
        let mut runtime_config = RuntimeConfig::default();
        runtime_config.base_dir = Some(tmpdir.clone());
        config.update_runtime_config(runtime_config);
        // runtime_config.base_dir = temp
        let file_store = FileStore::new(&tmpdir, &["anything"]);

        // is handled in the manager
        let flow_dir = file_store.create_directory(&["flows"]).unwrap();
        add_flow_directory(flow_dir.clone(), "some-sample-flow");

        let mut flows_model = FlowsModel::new(file_store);
        assert_eq!(flows_model.flows.len(), 0);
        let res = flows_model.reload_flows().await;
        assert!(res.is_ok());
        assert_eq!(flows_model.flows.len(), 1);

        add_flow_directory(flow_dir.clone(), "some-sample-flow-2");
        let res = flows_model.reload_flows().await;
        assert!(res.is_ok());
        assert_eq!(flows_model.flows.len(), 2);
    }
}
