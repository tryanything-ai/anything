use anyhow::Result;
use postage::dispatch::{Receiver, Sender};
use tokio::sync::Mutex;

type AnySendMap = anymap::Map<dyn std::any::Any + Send>;

struct Mailbox<T> {
    tx: Sender<T>,
    rx: Receiver<T>,
}

impl<T: Clone> Mailbox<T> {
    fn new() -> Mailbox<T> {
        let (tx, rx) = postage::dispatch::channel(256);
        Mailbox { tx, rx }
    }
}

#[derive(Debug)]
pub struct PostOffice(Mutex<AnySendMap>);

impl PostOffice {
    pub fn open() -> Self {
        Self(Mutex::new(AnySendMap::new()))
    }

    async fn with_mailbox<T: Clone + Send + 'static, F, R>(&self, f: F) -> Result<R>
    where
        F: FnOnce(&mut Mailbox<T>) -> Result<R>,
    {
        let mut postoffice = self.0.lock().await;

        let mailbox = postoffice
            .entry::<Mailbox<T>>()
            .or_insert_with(Mailbox::<T>::new);

        f(mailbox)
    }

    pub async fn receive_mail<T: Clone + Send + 'static>(&self) -> Result<Receiver<T>> {
        self.with_mailbox(|mailbox| Ok(mailbox.rx.clone())).await
    }

    pub async fn post_mail<T: Clone + Send + 'static>(&self) -> Result<Sender<T>> {
        self.with_mailbox(|mailbox| Ok(mailbox.tx.clone())).await
    }
}
