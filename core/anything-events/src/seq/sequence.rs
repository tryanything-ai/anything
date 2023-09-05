use std::{
    hash::{Hash, Hasher},
    sync::atomic::AtomicU64,
};

pub struct Sequence {
    value: AtomicU64,
}

impl Sequence {
    pub fn with_value(initial: u64) -> Self {
        Self {
            value: AtomicU64::new(initial),
        }
    }

    pub fn get(&self) -> u64 {
        self.value.load(std::sync::atomic::Ordering::Acquire)
    }

    pub fn set(&self, new_val: u64) -> u64 {
        self.value
            .swap(new_val, std::sync::atomic::Ordering::Release)
    }

    pub fn compare_exchange(&self, expected: u64, new_val: u64) -> bool {
        self.value
            .compare_exchange(
                expected,
                new_val,
                std::sync::atomic::Ordering::Acquire,
                std::sync::atomic::Ordering::Relaxed,
            )
            .is_ok()
    }

    pub fn increment(&self) -> u64 {
        self.value
            .fetch_add(1, std::sync::atomic::Ordering::Release)
    }
}

impl Hash for Sequence {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.get().hash(state)
    }
}

impl PartialEq for Sequence {
    fn eq(&self, other: &Self) -> bool {
        self.get() == other.get()
    }
}

impl Eq for Sequence {}

impl PartialOrd for Sequence {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.get().cmp(&other.get()))
    }
}

impl Ord for Sequence {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.get().cmp(&other.get())
    }
}
