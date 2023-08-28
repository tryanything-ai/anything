use flume::{bounded, Receiver, SendError, Sender};
use std::convert::Infallible;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;
use tracing::info;

pub struct Canceller {
    cancelled: AtomicBool,
    rx: Mutex<Receiver<()>>,
    tx: Sender<()>,
}

impl Canceller {
    #[tracing::instrument(skip(self))]
    pub async fn cancel(&self) -> Result<(), SendError<()>> {
        info!("cancelling inference");
        self.cancelled.store(true, Ordering::Release);
        self.tx.send_async(()).await
    }

    pub fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::Acquire)
    }

    pub fn reset(&self) {
        self.cancelled.store(false, Ordering::Release);
    }

    #[tracing::instrument(skip(self))]
    pub fn inference_feedback(&self) -> Result<llm::InferenceFeedback, Infallible> {
        // When a cancellation occurs, the sender will block until it is received at least
        // once. We want to check and see if that message has been sent, and if so we'll cancel.
        let cancelled = if let Ok(rx) = self.rx.try_lock() {
            rx.try_recv().is_ok()
        } else {
            false
        };
        if cancelled || self.is_cancelled() {
            info!("sending halt");
            Ok(llm::InferenceFeedback::Halt)
        } else {
            Ok(llm::InferenceFeedback::Continue)
        }
    }
}

impl Default for Canceller {
    fn default() -> Self {
        let (tx, rx) = bounded(0);
        Self {
            cancelled: Default::default(),
            rx: Mutex::new(rx),
            tx,
        }
    }
}