use std::sync::Arc;

use crossbeam::{sync::Unparker, utils::CachePadded};

pub type AnythingResult<T> = Result<T, AnythingError>;

pub(crate) trait Alertable {
    fn alert(&self);
}

impl Alertable for Unparker {
    fn alert(&self) {
        self.unpark()
    }
}

use crate::{
    error::AnythingError,
    event::EventEnvelope,
    serde::{Deserializer, Serializer},
};

pub(crate) type EventWrapper<E: EventType<E>> = CachePadded<Arc<EventEnvelope<E>>>;

pub trait Serde<T>: Serializer<T> + Deserializer<T> {}

impl<K, T> Serde<T> for K where K: Serializer<T> + Deserializer<T> {}

// TODO: Remove temporary
pub trait EventType<T>: Send + Sync + Deserializer<T> {}

impl<K, T> EventType<T> for K where K: Send + Sync + Serde<T> {}
