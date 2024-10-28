use std::{
    ptr,
    sync::atomic::Ordering,
    thread::{self, JoinHandle},
};

use threadpool::ThreadPool;

use crate::{
    global::{OUTPUT, PACKET_QUEUES, TPOOL_QUEUE, WORK_LOCKS},
    types::work::{Work, WorkType},
};

pub struct ThreadPoolMaster {
    pool: ThreadPool,
}

impl ThreadPoolMaster {
    pub fn new(num_threads: usize) -> Self {
        let pool = ThreadPool::new(num_threads);

        Self { pool }
    }

    pub fn start_tpool(self) -> JoinHandle<()> {
        thread::spawn(move || loop {
            if let Some(work) = TPOOL_QUEUE.pop() {
                // println!("Received in tpool {work:?}");

                // Run in threadpool
                self.pool.execute(move || {
                    match work.work_type {
                        // Work on map for token wise
                        WorkType::TokenWise(_) => work_on_map(work),
                        WorkType::McxDepth => work_on_mcx(work),
                        // Work on queue for other types
                        _ => work_on_queue(work),
                    }
                });
            }
        })
    }
}

pub fn work_on_map(work: Work) {
    // We assume that work.atomic_ptr is not null
    let atomic_ptr = unsafe { work.atomic_ptr.clone().unwrap_unchecked() };

    let old_packet_ptr = atomic_ptr.swap(ptr::null_mut(), Ordering::SeqCst);

    // Creating box from raw ptr is unsafe, because it could be null
    // however, we only ensure that this value is not null
    let mut old_packet = unsafe { Box::from_raw(old_packet_ptr) };

    // Call associated function
    (work.processing_fn)(&mut *old_packet, &work);

    OUTPUT.write(&old_packet);
}

pub fn work_on_mcx(work: Work) {
    let mcx_state = work
        .mcx_state
        .clone()
        .expect("MCX state required for processing mcx work");
    let packet_queue = mcx_state.packet_queue;
    let work_lock = mcx_state.work_lock;

    while let Some(mut packet) = packet_queue.pop() {
        (work.processing_fn)(&mut packet, &work);

        OUTPUT.write(&packet);

        if !packet_queue.is_empty() {
            if TPOOL_QUEUE.is_empty() {
                // If no other work in tpool, continue current work
                continue;
            } else {
                // If some work in tpool, push current work to tpool and exit
                TPOOL_QUEUE.push(work);
                return;
            }
        }
    }

    // No more work of same type
    work_lock.store(false, Ordering::SeqCst);

    // Check if queue has more work and we can still acquire lock
    if !packet_queue.is_empty()
        && work_lock
            .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
            .is_ok()
    {
        // push to tpool
        TPOOL_QUEUE.push(work);
    }
}

pub fn work_on_queue(work: Work) {
    let packet_queue = &PACKET_QUEUES[work.work_type.get_id()];
    let work_lock = &WORK_LOCKS[work.work_type.get_id()];

    while let Some(mut packet) = packet_queue.pop() {
        (work.processing_fn)(&mut packet, &work);

        OUTPUT.write(&packet);

        if !packet_queue.is_empty() {
            if TPOOL_QUEUE.is_empty() {
                // If no other work in tpool, continue current work
                continue;
            } else {
                // If some work in tpool, push current work to tpool and exit
                TPOOL_QUEUE.push(work);
                return;
            }
        }
    }

    // No more work of same type
    work_lock.store(false, Ordering::SeqCst);

    // Check if queue has more work and we can still acquire lock
    if !packet_queue.is_empty()
        && work_lock
            .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
            .is_ok()
    {
        // push to tpool
        TPOOL_QUEUE.push(work);
    }
}
