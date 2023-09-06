use std::{borrow::BorrowMut, collections::HashMap};

use crate::{types::AnythingResult, Observable};

use self::device::Device;

mod device;

pub struct EventBus<'a, T> {
    pub devices: HashMap<&'a str, Device>,
    // pub devices: HashMap<&'a str, Box<Device>>,
    pub observables: HashMap<String, Observable<'a, T>>,
}

impl<'a, T> EventBus<'a, T> {
    pub fn new() -> Self {
        Self {
            devices: HashMap::default(),
            observables: HashMap::default(),
        }
    }

    pub fn get_device(&mut self, addr: &'a str) -> AnythingResult<&mut Device> {
        let device = self.devices.entry(addr).or_insert_with(|| Device::new());
        Ok(device)
    }

    pub fn device_connect(
        &mut self,
        addr: &'a str,
        socket_type: zmq::SocketType,
    ) -> AnythingResult<()> {
        let device = self.devices.entry(addr).or_insert_with(|| Device::new());
        device.connect(addr, socket_type)?;

        Ok(())
    }

    /// Get access to the listening device
    // pub fn get_device(&self, addr: &'a str) -> AnythingResult<&Device> {
    //     match &self.devices.get(addr) {
    //         Some(v) => {
    //             let d = *v;
    //             Ok(d)
    //         }
    //         None => {
    //             return Err(AnythingError::DeviceError(
    //                 DeviceError::DeviceNotAvailableError,
    //             ))
    //         }
    //     }
    // }

    // pub fn subscribe_to_device(
    //     &mut self,
    //     addr: &'a str,
    //     topic: Option<&'a str>,
    // ) -> AnythingResult<()> {
    //     self.with_device(addr, |d: &mut Device| {
    //         d.subscribe(addr, topic)?;
    //         Ok(())
    //     })
    // }

    // pub fn listen_for_device(&mut self, addr: &'a str) -> AnythingResult<()> {
    //     self.with_device(addr, |d: &mut Device| {
    //         d.publish(addr)?;
    //         Ok(())
    //     })
    // }

    pub fn with_device<F>(&mut self, addr: &'a str, func: F) -> AnythingResult<()>
    where
        F: FnOnce(&mut Device) -> AnythingResult<()>,
    {
        match self.devices.get_mut(addr) {
            Some(v) => {
                func(v)?;
                Ok(())
            }
            None => {
                let mut device = Device::new();
                //         // let _d = device.publish(addr)?;
                match func(&mut device) {
                    Ok(_) => {
                        self.devices.borrow_mut().insert(addr, device);
                        Ok(())
                    }
                    Err(e) => Err(e),
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use tmq::publish;

    use super::*;

    #[tokio::test]
    async fn test_publish_on_socket_works() {
        let mut bus = EventBus::<String>::new();
        let addr = "inproc://anything-events";
        let data = "hello world".as_bytes();
        {
            let res = bus.device_connect(addr, zmq::PUSH);
            assert!(res.is_ok());
            let res = bus.with_device(addr, |d| d.send(data));
            assert!(res.is_ok());
        }
    }

    #[tokio::test]
    async fn test_subscribe_on_socket_works() {
        let mut bus = EventBus::<String>::new();
        let addr = "inproc://anything-events";
        let data = "hello world".as_bytes();
        {
            let res = bus.device_connect(addr, zmq::SUB);
            assert!(res.is_ok());
            let res = bus.with_device(addr, |d| d.subscribe("".as_bytes()));
            assert!(res.is_ok());

            let mut publisher = Device::new();
            let _ = publisher.connect(addr, zmq::REQ);

            let device = bus.get_device(addr).unwrap();
            let sock = device.socket.as_ref().unwrap();
            let poll_item = sock.as_poll_item(zmq::POLLIN);
            let mut items = vec![poll_item];

            loop {
                if zmq::poll(&mut items, 1000).is_err() {
                    break;
                }
                println!("is readable: {:?}", items[0].is_readable());
                if items[0].is_readable() {
                    let topic = sock.recv_msg(0).unwrap();
                    println!("message: {:?}", topic);
                    break;
                }
            }
        }
    }
}
