use std::{borrow::BorrowMut, collections::HashMap, sync::Mutex};

use crate::{types::AnythingResult, Observable};

use self::device::Device;

mod device;

pub struct EventBus<'a, T> {
    pub devices: HashMap<&'a str, Mutex<Box<Device>>>,
    pub observables: HashMap<String, Observable<'a, T>>,
}

impl<'a, T> EventBus<'a, T> {
    pub fn new() -> Self {
        Self {
            devices: HashMap::default(),
            observables: HashMap::default(),
        }
    }

    pub fn subscribe_to_device(
        &mut self,
        addr: &'a str,
        topic: Option<&'a str>,
    ) -> AnythingResult<()> {
        self.with_device(addr, |d: &mut Device| {
            d.subscribe(addr, topic)?;
            Ok(())
        })
    }

    pub fn listen_for_device(&mut self, addr: &'a str) -> AnythingResult<()> {
        self.with_device(addr, |d: &mut Device| {
            d.publish(addr)?;
            Ok(())
        })
    }

    pub fn with_device<F>(&mut self, addr: &'a str, func: F) -> AnythingResult<()>
    where
        F: FnOnce(&mut Device) -> AnythingResult<()>,
    {
        match self.devices.get(addr) {
            Some(v) => {
                func(v.lock().borrow_mut().as_mut().unwrap())?;
                Ok(())
            }
            None => {
                let mut device = Device::new();
                //         // let _d = device.publish(addr)?;
                match func(&mut device) {
                    Ok(_) => {
                        let mutex_device = Mutex::new(Box::new(device));
                        self.devices.borrow_mut().insert(addr, mutex_device);
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
    use super::*;

    #[test]
    fn test_publish_on_socket() {}
}
