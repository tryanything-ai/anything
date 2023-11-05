pub(crate) mod bus;
pub(crate) mod client;
pub mod error;
pub(crate) mod messages;
pub(crate) mod post_office;

const PAYLOAD_COMPRESSION_THRESHOLD: usize = 4096;
pub(crate) type ClientId = usize;
pub(crate) type MessageId = usize;

pub use crate::client::Client;
pub use crate::post_office::{new_client, Mailbox, PostOffice};

pub trait MessageProtocol: Send + Sync + 'static {
    fn name() -> &'static str;
}
