use std::sync::Arc;

use crate::{ring::RingBuffer, types::EventType};

pub struct Publisher<E: EventType<E>> {
    ring: Arc<RingBuffer<E>>,
}

impl<E: EventType<E>> Publisher<E> {
    pub(crate) fn new(ring: Arc<RingBuffer<E>>) -> Self {
        Self { ring }
    }

    pub fn send(&mut self, event: E) {
        let seq = self.ring.next();

        let envelope = self
            .ring
            .get_envelope(seq)
            .expect("ring buffer not prepopulated with empty event envelope");

        envelope.overwrite(seq, event);
    }
}
