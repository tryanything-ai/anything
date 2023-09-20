use self::{event_repo::EventRepoImpl, flow_repo::FlowRepoImpl, trigger_repo::TriggerRepoImpl};

pub(crate) mod event_repo;
pub(crate) mod flow_repo;
pub(crate) mod trigger_repo;

#[derive(Debug, Clone)]
pub struct Repositories {
    pub event_repo: EventRepoImpl,
    pub flow_repo: FlowRepoImpl,
    pub trigger_repo: TriggerRepoImpl,
}
