pub use crossbeam::channel;
pub use crossbeam_channel::{Receiver, Sender};

pub type NodeReceiver<T> = Option<Receiver<T>>;
pub type NodeSender<T> = Vec<(Sender<T>, Option<T>)>;

#[derive(Clone, Debug)]
pub enum NodeError {
    DataError,
    PermanentError,
    DataEnd,
    CommError,
}

/// The trait that all nodes in the library implement.
pub trait Node: Send {
    fn start(&mut self);
    fn call(&mut self) -> Result<(), NodeError>;
    fn is_connected(&self) -> bool;
}
