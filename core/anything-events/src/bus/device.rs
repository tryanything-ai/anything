use std::{borrow::BorrowMut, fmt::Debug};

use tmq::AsZmqSocket;
use zmq::{Context, Socket};

use crate::{
    error::{AnythingError, DeviceError},
    types::AnythingResult,
};

pub struct Device {
    pub context: Context,
    pub socket: Option<zmq::Socket>,
    // pub publisher: Option<tmq::publish::Publish>,
    // pub subscriber: Option<tmq::subscribe::Subscribe>,
}

impl Debug for Device {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Device")
            .field(
                "socket",
                &format_args!("connected: {}", self.is_initialized()),
            )
            .finish()
    }
}

impl Device {
    pub fn new() -> Self {
        let ctx = Context::new();
        Self {
            context: ctx,
            socket: None,
        }
    }

    pub fn as_subscriber(addr: &str, topic: &[u8]) -> AnythingResult<Self> {
        let mut inst = Self::new();
        inst.connect(addr, zmq::SUB)?;
        inst.subscribe(topic)?;
        Ok(inst)
    }

    pub fn as_publisher(addr: &str) -> AnythingResult<Self> {
        let mut inst = Self::new();
        inst.connect(addr, zmq::PUB)?;
        Ok(inst)
    }

    pub fn socket(&self) -> AnythingResult<&Socket> {
        if !self.is_initialized() {
            return Err(AnythingError::DeviceError(
                DeviceError::DeviceNotAvailableError,
            ));
        }
        Ok(self.socket.as_ref().unwrap())
    }

    pub fn send(&mut self, data: &[u8]) -> AnythingResult<()> {
        if !self.is_initialized() {
            return Err(AnythingError::DeviceError(
                DeviceError::DeviceNotAvailableError,
            ));
        }
        let msg = zmq::Message::from(data);
        let sock = self.socket.as_ref().unwrap();
        sock.send(msg, 0)
            .map_err(|e| AnythingError::DeviceError(DeviceError::SocketError(e)))?;

        Ok(())
    }

    pub fn subscribe(&mut self, topic: &[u8]) -> AnythingResult<()> {
        if !self.is_initialized() {
            return Err(AnythingError::DeviceError(
                DeviceError::DeviceNotAvailableError,
            ));
        }

        let sock = self.socket.as_ref().unwrap();
        let _res = sock
            .set_subscribe(topic)
            .map_err(|e| AnythingError::DeviceError(DeviceError::SocketError(e)))?;

        Ok(())
    }

    pub fn unsubscribe(&mut self, topic: &[u8]) -> AnythingResult<()> {
        if !self.is_initialized() {
            return Err(AnythingError::DeviceError(
                DeviceError::DeviceNotAvailableError,
            ));
        }

        let sock = self.socket.as_ref().unwrap();
        sock.set_unsubscribe(topic)
            .map_err(|e| AnythingError::DeviceError(DeviceError::SocketError(e)))?;

        Ok(())
    }

    pub fn reconnect(&mut self, addr: &str, socket_type: zmq::SocketType) -> AnythingResult<()> {
        self.socket = None;
        self.connect(addr, socket_type)
    }

    pub fn connect(&mut self, addr: &str, socket_type: zmq::SocketType) -> AnythingResult<()> {
        if !self.is_initialized() {
            let socket = self
                .context
                .socket(socket_type)
                .map_err(|e| AnythingError::DeviceError(DeviceError::SocketError(e)))?;

            socket
                .bind(addr)
                .map_err(|e| AnythingError::DeviceError(DeviceError::SocketError(e)))?;

            self.socket = Some(socket);
        }
        Ok(())
    }

    fn is_initialized(&self) -> bool {
        self.socket.is_some()
    }

    // pub fn get_publisher(&self) -> AnythingResult<&tmq::publish::Publish> {
    //     match &self.publisher {
    //         None => Err(AnythingError::DeviceError(
    //             DeviceError::DeviceNotAvailableError,
    //         )),
    //         Some(v) => Ok(v),
    //     }
    // }

    // pub fn subscribe(&mut self, addr: &str, topic: Option<&str>) -> AnythingResult<()> {
    //     if self.is_subscribing() {
    //         return Err(AnythingError::DeviceError(
    //             DeviceError::AlreadySubscribingError,
    //         ));
    //     }

    //     let topic = match topic {
    //         None => b"",
    //         Some(v) => v.as_bytes(),
    //     };
    //     let subscribe_result = match tmq::subscribe(&self.context).connect(addr) {
    //         Ok(s) => s.subscribe(topic),
    //         Err(_e) => {
    //             return Err(AnythingError::DeviceError(
    //                 DeviceError::UnableToSubscribeError,
    //             ))
    //         }
    //     };

    //     let subscribe = match subscribe_result {
    //         Ok(s) => s,
    //         Err(_e) => {
    //             return Err(AnythingError::DeviceError(
    //                 DeviceError::UnableToSubscribeError,
    //             ))
    //         }
    //     };

    //     self.subscriber = Some(subscribe);

    //     Ok(())
    // }

    // pub fn publish(&mut self, addr: &str) -> AnythingResult<()> {
    //     if self.is_publishing() {
    //         return Err(AnythingError::DeviceError(
    //             DeviceError::AlreadyPublishingError,
    //         ));
    //     }
    //     let publish = match tmq::publish(&self.context).bind(addr) {
    //         Ok(p) => p,
    //         Err(e) => return Err(AnythingError::DeviceError(DeviceError::SocketError(e))),
    //     };

    //     self.publisher = Some(publish);

    //     Ok(())
    // }

    // pub fn is_publishing(&self) -> bool {
    //     self.publisher.is_some()
    // }

    // pub fn is_subscribing(&self) -> bool {
    //     self.subscriber.is_some()
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_subscriber() {
        let addr = "inproc://server-pull";
        let data = "hello world".as_bytes();
        let topic = "".as_bytes();
        {
            let sub = Device::as_subscriber(addr, topic);
            assert!(sub.is_ok());
            let sub = sub.unwrap();
            let publisher = Device::as_publisher(addr);
            assert!(publisher.is_ok());
            let mut publisher = publisher.unwrap();

            let sub_socket = sub.socket().unwrap();
            let poll_item = sub_socket.as_poll_item(zmq::POLLIN);
            let mut items = vec![poll_item];

            let mut msg = zmq::Message::new();

            loop {
                if zmq::poll(&mut items, 1000).is_err() {
                    break;
                }
                println!("----");
                // let res = publisher.send(data);
                // println!("HERE: {:?}", res);
                let msg = sub_socket.recv(&mut msg, 0);
                println!("msg: {:?}", msg);
            }
        }
    }
}
