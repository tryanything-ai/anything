use self::{event_repo::EventRepoImpl, flow_repo::FlowRepoImpl};

pub(crate) mod event_repo;
pub(crate) mod flow_repo;

#[derive(Debug, Clone)]
pub struct Repositories {
    pub event_repo: EventRepoImpl,
    pub flow_repo: FlowRepoImpl,
}
