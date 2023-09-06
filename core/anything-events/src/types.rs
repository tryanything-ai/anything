use crossbeam::channel::{Receiver, Sender};

use crate::error::AnythingError;

pub type AnythingResult<T> = Result<T, AnythingError>;

pub type NodeReceiver<T> = Option<Receiver<T>>;
pub type NodeSender<T> = Vec<(Sender<T>, Option<T>)>;
