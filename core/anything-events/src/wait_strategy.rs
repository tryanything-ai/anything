use std::time::Duration;

#[derive(Debug, Clone, Copy)]
pub enum WaitStrategy {
    AllSubscribers,
    NoWait,
    WaitForDuration(Duration),
}

#[cfg(test)]
mod tests {
    use std::{
        thread::{sleep, spawn},
        time::Duration,
    };

    use crate::{bus::EventBus, event::Event, serde::json::Json};

    #[test]
    fn test_wait_for_all_subs() {
        let bus = EventBus::new(4).unwrap();

        let sub = bus.subscribe();

        let _thread = std::thread::spawn(move || {
            for i in 0..4 {
                let i = Event::new(String::from("junk"));
                bus.publish(i);
            }
        });

        std::thread::sleep(std::time::Duration::from_millis(200));
        let msg = sub.recv();
        assert_eq!(0, *msg);
    }

    #[test]
    fn test_no_wait_for_subs() {
        let bus = EventBus::with_strategy(2, super::WaitStrategy::NoWait).unwrap();
        let sub = bus.subscribe();
        let _thread = std::thread::spawn(move || {
            for i in 0..3 {
                bus.publish(i as usize);
            }
        });

        sleep(Duration::from_millis(200));
        let msg = sub.recv();
        assert_eq!(2, *msg);
    }

    #[test]
    fn test_wait_for_duration() {
        let bus = EventBus::with_strategy(
            2,
            super::WaitStrategy::WaitForDuration(Duration::from_millis(50)),
        )
        .unwrap();

        let sub1 = bus.subscribe();
        let sub2 = bus.subscribe();

        let _thread = spawn(move || {
            for i in 0..4 {
                bus.publish(i as usize);
            }
        });

        let msg = sub1.recv();
        assert_eq!(0, *msg);

        sleep(Duration::from_millis(200));
        let msg = sub2.recv();
        assert_eq!(2, *msg);
    }
}
