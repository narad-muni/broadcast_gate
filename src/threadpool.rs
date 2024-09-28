use std::{
    ptr,
    sync::atomic::Ordering,
    thread::{self, JoinHandle},
};

use threadpool::ThreadPool;

use crate::{
    global::{OUTPUT_QUEUE, TOKEN_WISE_MAP, TPOOL_QUEUE, WORK_LOCKS, WORK_QUEUES},
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
                        // Work on queue for other types
                        _ => work_on_queue(work),
                    }
                });
            }
        })
    }
}

pub fn work_on_map(data: Work) {
    let value = TOKEN_WISE_MAP.get(&data.work_type.get_id()).unwrap();

    let packet_ptr = value.swap(ptr::null_mut(), Ordering::SeqCst);
    unsafe {
        // Creating box from raw ptr is unsafe, because it could be null
        // however, we only ensure that this value is not null
        let mut packet = Box::from_raw(packet_ptr);

        // Call associated function
        (data.processing_fn)(&mut *packet);

        OUTPUT_QUEUE.push(*packet);
    }
}

pub fn work_on_queue(data: Work) {
    let work_queue = &WORK_QUEUES[data.work_type.get_id()];
    let work_lock = &WORK_LOCKS[data.work_type.get_id()];

    while let Some(mut packet) = work_queue.pop() {
        // println!("Data received in work_on_queue: {:?}", packet);

        (data.processing_fn)(&mut packet);

        OUTPUT_QUEUE.push(packet);

        if !work_queue.is_empty() {
            if TPOOL_QUEUE.is_empty() {
                // If no other work in tpool, continue current work
                continue;
            } else {
                // If some work in tpool, push current work to tpool and exit
                TPOOL_QUEUE.push(data);
                return;
            }
        }
    }

    // No more work of same type
    work_lock.store(false, Ordering::SeqCst);

    // Check if queue has more work and we can still acquire lock
    if !work_queue.is_empty()
        && work_lock
            .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
            .is_ok()
    {
        // push to tpool
        TPOOL_QUEUE.push(data);
    }
}
