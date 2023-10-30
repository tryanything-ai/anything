pub mod errors;

pub static STORE_DIR: &str = ".store";
pub(crate) mod file_store;
pub mod types;
pub(crate) mod watcher;

pub use file_store::FileStore;
