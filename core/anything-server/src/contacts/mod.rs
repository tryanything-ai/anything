pub mod create;
pub mod delete;
pub mod get;
pub mod update;

pub use create::create_contact;
pub use delete::delete_contact;
pub use get::{get_contact, get_contacts};
pub use update::update_contact;
