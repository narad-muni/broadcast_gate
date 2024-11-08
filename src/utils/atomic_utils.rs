use std::sync::atomic::{AtomicU32, Ordering};

pub fn compare_and_swap_gte(atomic: &AtomicU32, new: u32) -> Result<u32, u32> {
    let current = atomic.load(Ordering::Relaxed);

    if new >= current {
        atomic.compare_exchange_weak(current, new, Ordering::SeqCst, Ordering::Relaxed)
    } else {
        Err(current)
    }
}
