use std::sync::Arc;

use crate::{
    pubsub::{publisher::Publisher, subscriber::Subscriber},
    ring::RingBuffer,
    seq::sequence::Sequence,
    types::{AnythingResult, EventType},
    wait_strategy::WaitStrategy,
};

#[derive(Clone)]
pub struct EventBus<E: EventType<E>> {
    ring: Arc<RingBuffer<E>>,
}

impl<'a, E: EventType<E>> EventBus<E> {
    pub fn new(capacity: u64) -> AnythingResult<Self> {
        Ok(Self {
            ring: Arc::new(RingBuffer::new(capacity, WaitStrategy::AllSubscribers)?),
        })
    }

    pub fn with_strategy(capacity: u64, wait_strategy: WaitStrategy) -> AnythingResult<Self> {
        Ok(Self {
            ring: Arc::new(RingBuffer::new(capacity, wait_strategy)?),
        })
    }

    pub fn publisher(&self) -> Publisher<E> {
        Publisher::new(self.ring.clone())
    }

    pub fn publish(&self, message: E) {
        let seq = self.ring.next();

        if let Some(event_store) = self.ring.get_envelope(seq).clone() {
            event_store.overwrite(seq, message);
        }
    }

    pub fn subscribe(&self) -> Subscriber<E> {
        let seq = Arc::new(Sequence::with_value(self.ring.sequencer().get() + 1));

        self.ring.sequencer().register_gating_sequence(seq.clone());

        Subscriber::new(self.ring.clone(), seq)
    }
}

impl<E: EventType<E>> From<RingBuffer<E>> for EventBus<E> {
    fn from(value: RingBuffer<E>) -> Self {
        Self {
            ring: Arc::new(value),
        }
    }
}

impl<E: EventType<E>> From<Arc<RingBuffer<E>>> for EventBus<E> {
    fn from(ring: Arc<RingBuffer<E>>) -> Self {
        Self { ring }
    }
}

#[cfg(test)]
mod tests {
    use std::{
        thread::{sleep, spawn},
        time::Duration,
    };

    use crate::event::Event;

    use super::EventBus;

    #[test]
    fn test_publish_and_subscribe() {
        let bus = EventBus::new(2);
        assert!(bus.is_ok());
        let bus = bus.unwrap();
        let sub = bus.subscribe();
        assert_eq!(1, sub.sequence());

        let msg = "Hello".to_string();
        bus.publish(msg);

        let mut m1 = sub.recv();
        assert_eq!("Hello".to_string(), *m1);

        let bus2 = bus.clone();

        spawn(move || {
            sleep(Duration::from_millis(100));
            bus2.publish("World".to_string());
        });

        m1 = sub.recv();
        assert_eq!(*m1, "World");
    }

    #[test]
    fn test_bus_with_multiple_subscriptions() {
        let bus: EventBus<'_, String> = EventBus::new(4).unwrap();
        let sub1 = bus.subscribe();
        let sub2 = bus.subscribe();

        let msg = Event::new(String::from("Lady"));
    }
}
