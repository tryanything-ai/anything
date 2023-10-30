use std::marker::PhantomData;

// use postage::sink::Sink;

use async_channel::{Receiver, Sender};

use crate::{error::MqResult, post_office::POST_OFFICE, MessageProtocol};

#[derive(Debug)]
pub struct Client<M> {
    // mailbox: Mailbox<M>,
    _phantom_data: PhantomData<M>,
}

impl<M> Client<M>
where
    M: Clone + Send + Sync + 'static + MessageProtocol,
{
    pub fn new() -> Self {
        // let mailbox = Mailbox::<M>::new();
        Self {
            // mailbox,
            _phantom_data: PhantomData::default(),
        }
    }

    pub async fn subscribe(&self, _topic: &str) -> MqResult<Receiver<M>> {
        let sub = POST_OFFICE.receive_mail::<M>().await?;
        // let sub = self.post_office.receive_mail::<M>().await?;
        Ok(sub)
    }

    pub async fn publisher(&self) -> MqResult<Sender<M>> {
        Ok(POST_OFFICE.post_mail::<M>().await?)
    }

    pub async fn publish(&self, _topic: &str, message: M) -> MqResult<()> {
        let publisher = self.publisher().await?;
        Ok(publisher.send(message).await?)
    }
}
