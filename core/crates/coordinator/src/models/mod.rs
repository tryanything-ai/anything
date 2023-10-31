use anything_graph::{Flow, FlowBuilder};
use anything_store::FileStore;
use once_cell::sync::OnceCell;
use tokio::sync::Mutex;

use crate::error::CoordinatorResult;

use self::flows::FlowsModel;

pub(crate) mod flows;

pub static MODELS: OnceCell<Mutex<Models>> = OnceCell::new();

#[derive(Debug, Clone)]
pub struct Models {
    pub flows: flows::FlowsModel,
}

impl Models {
    pub fn new(file_store: FileStore) -> Self {
        let flows = FlowsModel::new(file_store);
        Self { flows }
    }

    pub async fn setup<'a>(file_store: FileStore) -> CoordinatorResult<()> {
        let models = Models::new(file_store);
        match MODELS.get() {
            Some(_e) => Ok(()),
            None => match MODELS.set(Mutex::new(models)) {
                Ok(_) => Ok(()),
                Err(e) => {
                    println!("Error setting up models: {:?}", e);
                    return Err(crate::error::CoordinatorError::RuntimeError);
                }
            },
        }
    }

    pub async fn reload_flows(&mut self) {
        self.flows.reload_flows().await.unwrap();
    }

    pub fn get_flows(&self) -> Vec<Flow> {
        self.flows.get_flows()
    }

    pub fn get_flow(&self, name: &str) -> Option<Flow> {
        self.flows.get_flow(name)
    }

    pub fn create_flow(&mut self, flow_name: String, flow_id: String) -> CoordinatorResult<Flow> {
        let new_flow = FlowBuilder::default()
            .name(flow_name)
            .version(flow_id)
            .build()
            .unwrap();
        self.flows.add_flow(new_flow.clone());
        // TODO: serialize flow into a file
        Ok(new_flow)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::manager::Manager;

    #[test]
    fn test_models_can_be_created() {
        let manager = Manager::default();
        let file_store = manager.file_store.clone();
        let models = Models::new(file_store);
        assert!(models.flows.get_flows().is_empty());
    }
}
