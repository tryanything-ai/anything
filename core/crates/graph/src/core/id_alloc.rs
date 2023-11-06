use std::sync::atomic::AtomicUsize;

/// Allocator of ids
pub struct IDAllocator {
    id: AtomicUsize,
}

impl IDAllocator {
    fn alloc(&self) -> usize {
        let origin = self.id.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        if origin > self.id.load(std::sync::atomic::Ordering::Relaxed) {
            panic!("ID overflow")
        }
        origin
    }
}

pub static ID_ALLOCATOR: IDAllocator = IDAllocator {
    id: AtomicUsize::new(1),
};

pub fn alloc_id() -> usize {
    ID_ALLOCATOR.alloc()
}

pub fn alloc_uuid_string() -> String {
    uuid::Uuid::new_v4().to_string()
}
