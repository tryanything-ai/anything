use std::{marker::PhantomData, sync::Arc};

use crossbeam::sync::Parker;

use crate::{
    event::EventRead,
    ring::RingBuffer,
    seq::sequence::Sequence,
    types::{EventType, EventWrapper},
};

pub struct Subscriber<E: EventType<E>> {
    ring: Arc<RingBuffer<E>>,
    seq: Arc<Sequence>,
    _marker: PhantomData<E>,
}

impl<E: EventType<E>> Subscriber<E>
where
    E: EventType<E>,
{
    pub(crate) fn new(ring: Arc<RingBuffer<E>>, sequence: Arc<Sequence>) -> Self {
        Self {
            ring,
            seq: sequence,
            _marker: PhantomData,
        }
    }

    pub fn sequence(&self) -> u64 {
        self.seq.get()
    }

    pub fn recv(&self) -> EventRead<'_, E> {
        loop {
            let seq = self.seq.get();

            let env = self
                .ring
                .get_envelope(seq)
                .expect("ring buffer invalid prepopulation")
                .clone();

            env.start_waiting();
            let env_seq = env.sequence();
            if seq == env_seq {
                if let Some(event) = self.read_event(env) {
                    return event;
                }
            } else if seq > env_seq {
                let parker = Parker::new();
                env.add_subscriber(Box::new(parker.unparker().clone()));
                parker.park();

                if let Some(event) = self.read_event(env) {
                    return event;
                }
            } else {
                match self.ring.wait_strategy() {
                    crate::wait_strategy::WaitStrategy::AllSubscribers => unreachable!(),
                    _ => {
                        self.seq.set(env_seq);
                    }
                }
            }
        }
    }

    pub(crate) fn read_event(&self, envelope: EventWrapper<E>) -> Option<EventRead<E>> {
        let event_opt = unsafe { envelope.read() };

        envelope.stop_waiting();

        self.seq.increment();
        return event_opt;
    }
}
