use crate::{client::Client, MessageProtocol};
use anyhow::Result;

use async_channel::{bounded, Receiver, Sender};
use tokio::sync::Mutex;

type AnySendMap = anymap::Map<dyn std::any::Any + Send>;
// type AnySendMap = HashMap<&'static str, Mailbox<dyn std::any::Any + Send + 'static>>;

lazy_static::lazy_static! {
    pub static ref POST_OFFICE: PostOffice = {
        let post_office = PostOffice::open();
        post_office
    };
}

#[derive(Debug, Clone)]
pub struct Mailbox<T> {
    tx: Sender<T>,
    rx: Receiver<T>,
}

impl<T: Clone + MessageProtocol> Mailbox<T> {
    pub fn new() -> Mailbox<T> {
        let (tx, rx) = bounded(256);
        Mailbox { tx, rx }
    }
}

pub async fn new_client<M: Clone + Send + Sync + 'static + MessageProtocol>() -> Result<Client<M>> {
    let client: Client<M> = Client::new();
    Ok(client)
}

#[derive(Debug)]
pub struct PostOffice(Mutex<AnySendMap>);

impl PostOffice {
    pub fn open() -> Self {
        Self(Mutex::new(AnySendMap::new()))
    }

    async fn with_mailbox<T: MessageProtocol + Clone + Send + 'static, F, R>(
        &self,
        f: F,
    ) -> Result<R>
    where
        F: FnOnce(&mut Mailbox<T>) -> Result<R>,
    {
        let mut postoffice = self.0.lock().await;

        let mailbox = postoffice
            .entry::<Mailbox<T>>()
            .or_insert_with(Mailbox::<T>::new);

        f(mailbox)
    }

    pub async fn receive_mail<T: MessageProtocol + Clone + Send + 'static>(
        &self,
    ) -> Result<Receiver<T>> {
        self.with_mailbox(|mailbox| Ok(mailbox.rx.clone())).await
    }

    pub async fn post_mail<T: MessageProtocol + Clone + Send + 'static>(
        &self,
    ) -> Result<Sender<T>> {
        self.with_mailbox(|mailbox| Ok(mailbox.tx.clone())).await
    }

    pub async fn new_client<M: MessageProtocol + Clone + Send + Sync + 'static>(
        self,
    ) -> Result<Client<M>> {
        let client: Client<M> = Client::new();
        Ok(client)
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use crate::{
        messages::{ClientProtocol, Message, MessagePub, Payload},
        ClientId, MessageId,
    };
    use tokio::time::{sleep, timeout};

    use super::*;

    #[derive(Clone, Debug, PartialEq)]
    enum TestTask {
        Message(ClientId, Message<ClientProtocol>),
    }

    impl MessageProtocol for TestTask {
        fn name() -> &'static str {
            "test-task"
        }
    }

    #[tokio::test]
    async fn test_post_office_can_be_opened() {
        let _po = PostOffice::open();
    }

    #[tokio::test]
    async fn test_postoffice_can_send_and_receive_messages_of_type() {
        let po = PostOffice::open();
        let (stop_tx, mut stop_rx) = tokio::sync::oneshot::channel();

        let tx = po.post_mail::<TestTask>().await.unwrap();

        let listener_task = tokio::spawn(async move {
            let rx = po.receive_mail::<TestTask>().await.unwrap();
            let mut received_msg: Option<TestTask> = None;

            loop {
                tokio::select! {
                    _ = &mut stop_rx => break,
                    Ok(msg) = rx.recv() => {
                        received_msg = Some(msg.clone());
                    }
                }
            }
            received_msg
        });

        tokio::spawn(async move {
            tx.send(TestTask::Message(
                ClientId::default(),
                Message {
                    id: MessageId::default(),
                    content: ClientProtocol::Pub(MessagePub {
                        topic: "just-a-message".to_string(),
                        payload: Payload::from(serde_json::json!({
                            "name": "bob"
                        })),
                    }),
                },
            ))
            .await
            .unwrap();
            sleep(Duration::from_millis(300)).await;
            stop_tx.send(()).unwrap();
        });
        // manager.stop().await.unwrap();
        let res = timeout(Duration::from_secs(10), listener_task)
            .await
            .unwrap();
        assert!(res.is_ok(), "listener task borked");
        let recevied_msg = res.unwrap();
        assert!(recevied_msg.is_some());
        let received_msg = recevied_msg.unwrap();
        assert_eq!(
            received_msg,
            TestTask::Message(
                0,
                Message {
                    id: 0,
                    content: ClientProtocol::Pub(MessagePub {
                        topic: "just-a-message".to_string(),
                        payload: Payload::from(serde_json::json!({
                            "name": "bob"
                        }))
                    })
                }
            )
        );
    }
}
