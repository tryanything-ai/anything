use std::path::PathBuf;

use crate::{config::AnythingEventsConfig, errors::EventsResult};

#[allow(unused)]
pub fn load_flows(config: &AnythingEventsConfig) -> EventsResult<Vec<PathBuf>> {
    let root_dir = config.root_dir.clone();
    let flows_dir = root_dir.join("flows");

    let mut flows = vec![];

    if flows_dir.exists() {
        let paths = std::fs::read_dir(flows_dir)?;
        for path in paths {
            let path = path?.path();
            if path.is_file() {
                flows.push(path);
            }
        }
    }

    Ok(flows)
}
