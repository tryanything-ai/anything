use std::{
    marker::PhantomData,
    ops::Deref,
    sync::atomic::{AtomicU64, Ordering},
};

use crossbeam::epoch::{pin, Atomic, Guard, Owned};
use lockfree::queue::Queue;
use serde::{Deserialize, Serialize};

use crate::types::{Alertable, EventType, Serde};

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct Event<T: EventType<T>> {
    // pub type_id: TypeId,
    pub data: Box<T>,
}

#[allow(unused)]
impl<T: 'static + Serde<T> + Send + Sync> Event<T> {
    pub fn new(data: T) -> Self {
        Self {
            data: Box::new(data),
        }
    }
}

#[derive(Debug)]
pub struct EventRead<'a, T: 'a + Send + Sync> {
    _guard: Guard,
    raw: *const T,
    _marker: PhantomData<&'a T>,
}

impl<'a, T: 'a + Send + Sync> Deref for EventRead<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.raw }
    }
}

impl<'a, T: 'a + Send + Sync> AsRef<T> for EventRead<'a, T> {
    fn as_ref(&self) -> &T {
        &*self
    }
}

pub(crate) struct EventEnvelope<E: EventType<E>> {
    seq: AtomicU64,
    event: Atomic<Event<E>>,
    num_waiting: AtomicU64,
    subscribers: Queue<Option<Box<dyn Alertable + Send + Sync>>>,
}

impl<E: EventType<E>> EventEnvelope<E> {
    pub fn new() -> Self {
        Self {
            seq: AtomicU64::new(0),
            event: Atomic::null(),
            num_waiting: AtomicU64::new(0),
            subscribers: Queue::new(),
        }
    }

    pub fn sequence(&self) -> u64 {
        self.seq.load(Ordering::Acquire)
    }

    pub fn start_waiting(&self) {
        self.num_waiting.fetch_add(1, Ordering::Acquire);
    }

    pub fn stop_waiting(&self) {
        self.num_waiting.fetch_sub(1, Ordering::Release);
    }

    pub fn add_subscriber(&self, alerter: Box<dyn Alertable + Send + Sync>) {
        self.subscribers.push(Some(alerter))
    }

    pub unsafe fn read(&self) -> Option<EventRead<E>> {
        let guard = pin();

        let event = self.event.load(Ordering::Acquire, &guard).as_raw();

        if !event.is_null() {
            let raw = &*(*event).data;
            return Some(EventRead {
                _guard: guard,
                raw: raw,
                _marker: PhantomData,
            });
            // if let Some(event_data) = event {
            //     return Some(EventRead {
            //         _guard: guard,
            //         raw: (*event_data).data,
            //         _marker: PhantomData,
            //     });
            // }
        }

        return None;
    }

    pub(crate) fn overwrite(&self, sequence: u64, data: E) {
        let mut event = Owned::new(Event {
            data: Box::new(data),
        });

        let guard = pin();

        loop {
            std::hint::spin_loop();

            let current_event = self.event.load(Ordering::Acquire, &guard);

            match self.event.compare_exchange(
                current_event,
                event,
                Ordering::Acquire,
                Ordering::Acquire,
                &guard,
            ) {
                Ok(_) => {
                    self.seq.store(sequence, Ordering::Release);

                    if !current_event.is_null() {
                        unsafe {
                            guard.defer_destroy(current_event);
                        }
                    }

                    loop {
                        match self.num_waiting.compare_exchange(
                            0,
                            0,
                            Ordering::Acquire,
                            Ordering::Relaxed,
                        ) {
                            Ok(_) => break,

                            Err(_) => {
                                let num_waiting = self.num_waiting.load(Ordering::Acquire);

                                for alerter_opt in
                                    self.subscribers.pop_iter().take(num_waiting as usize)
                                {
                                    if let Some(alerter) = alerter_opt {
                                        alerter.alert();
                                    }
                                }
                            }
                        }
                    }

                    break;
                }

                Err(cas_err) => {
                    event = cas_err.new;
                }
            }
        }
    }
}
