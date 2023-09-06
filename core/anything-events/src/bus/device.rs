use zmq::Context;

use crate::{
    error::{AnythingError, DeviceError},
    types::AnythingResult,
};
use std::{marker::PhantomData, ops::Deref};

pub struct Device {
    pub context: Context,
    pub publisher: Option<Box<tmq::publish::Publish>>,
    pub subscriber: Option<Box<tmq::subscribe::Subscribe>>,
}

impl Device {
    pub fn new() -> Self {
        let ctx = Context::new();
        Self {
            context: ctx,
            publisher: None,
            subscriber: None,
        }
    }

    pub fn subscribe(&mut self, addr: &str, topic: Option<&str>) -> AnythingResult<()> {
        if self.is_subscribing() {
            return Err(AnythingError::DeviceError(
                DeviceError::AlreadySubscribingError,
            ));
        }

        let topic = match topic {
            None => b"",
            Some(v) => v.as_bytes(),
        };
        let subscribe_result = match tmq::subscribe(&self.context).connect(addr) {
            Ok(s) => s.subscribe(topic),
            Err(_e) => {
                return Err(AnythingError::DeviceError(
                    DeviceError::UnableToSubscribeError,
                ))
            }
        };

        let subscribe = match subscribe_result {
            Ok(s) => s,
            Err(_e) => {
                return Err(AnythingError::DeviceError(
                    DeviceError::UnableToSubscribeError,
                ))
            }
        };

        self.subscriber = Some(Box::new(subscribe));

        Ok(())
    }

    pub fn publish(&mut self, addr: &str) -> AnythingResult<()> {
        if self.is_publishing() {
            return Err(AnythingError::DeviceError(
                DeviceError::AlreadyPublishingError,
            ));
        }
        let publish = match tmq::publish(&self.context).bind(addr) {
            Ok(p) => p,
            Err(e) => return Err(AnythingError::DeviceError(DeviceError::SocketError(e))),
        };

        self.publisher = Some(Box::new(publish));

        Ok(())
    }

    pub fn is_publishing(&self) -> bool {
        self.publisher.is_some()
    }

    pub fn is_subscribing(&self) -> bool {
        self.subscriber.is_some()
    }
}
