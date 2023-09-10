use self::event_repo::EventRepoImpl;

pub(crate) mod event_repo;
pub(crate) mod tag_repo;

#[derive(Debug, Clone)]
pub struct Repositories {
    pub event_repo: EventRepoImpl,
}
