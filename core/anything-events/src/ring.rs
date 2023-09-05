use std::sync::Arc;

use crossbeam::utils::CachePadded;

use crate::{
    error::AnythingError,
    event::EventEnvelope,
    seq::sequencer::Sequencer,
    types::{AnythingResult, EventType, EventWrapper},
    wait_strategy::WaitStrategy,
};

pub struct RingBuffer<E: EventType<E>> {
    capacity: u64,
    buffer: Vec<EventWrapper<E>>,
    seq: Sequencer,
    wait_strategy: WaitStrategy,
}

impl<E: EventType<E>> RingBuffer<E> {
    pub fn new(capacity: u64, wait_strategy: WaitStrategy) -> AnythingResult<Self> {
        if capacity > 1 && capacity.is_power_of_two() {
            let seq = Sequencer::new(capacity, wait_strategy.clone());
            let usize_cap = capacity as usize;
            let mut buffer = Vec::with_capacity(usize_cap);

            for i in 0..usize_cap {
                buffer.insert(i, CachePadded::new(Arc::new(EventEnvelope::new())))
            }

            Ok(Self {
                capacity,
                buffer,
                seq,
                wait_strategy,
            })
        } else {
            Err(AnythingError::InvalidRingBufferCapacityError)
        }
    }

    pub(crate) fn sequencer(&self) -> &Sequencer {
        &self.seq
    }

    pub(crate) fn wait_strategy(&self) -> WaitStrategy {
        self.wait_strategy
    }

    pub(crate) fn next(&self) -> u64 {
        self.seq.next()
    }

    pub(crate) fn index_from_sequence(&self, sequence: u64) -> usize {
        (sequence & (self.capacity - 1)) as usize
    }

    pub(crate) fn get_envelope(&self, sequence: u64) -> Option<EventWrapper<E>> {
        let index = self.index_from_sequence(sequence);

        if let Some(envelope) = self.buffer.get(index).clone() {
            Some(envelope.clone())
        } else {
            None
        }
    }
}

impl<'a, E: 'a + EventType<E>> Default for RingBuffer<E> {
    fn default() -> Self {
        Self::new(256, WaitStrategy::AllSubscribers).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::{ring::RingBuffer, wait_strategy::WaitStrategy};

    #[test]
    fn error_if_not_power_of_two() {
        assert!(RingBuffer::<String>::new(3, WaitStrategy::AllSubscribers).is_err());
    }

    #[test]
    fn success_if_power_of_two() {
        assert!(RingBuffer::<String>::new(16, WaitStrategy::AllSubscribers).is_ok());
    }
}
