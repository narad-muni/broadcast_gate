use std::{
    sync::atomic::{AtomicPtr, Ordering},
    thread::{self, JoinHandle},
};

use crate::{
    global::{INPUT_QUEUE, TOKEN_WISE_MAP, TPOOL_QUEUE, WORK_LOCKS, WORK_QUEUES},
    types::{
        packet::Packet,
        work::{Work, WorkType},
    },
};

pub struct Distributor {}

impl Distributor {
    pub fn new() -> Self {
        Self {}
    }

    pub fn start_distributor(self) -> JoinHandle<()> {
        thread::spawn(move || loop {
            if let Some(packet) = INPUT_QUEUE.pop() {
                // println!("Received in ditributor: {:?}", packet);

                let work = packet.create_work();

                if let WorkType::TokenWise(_) = work.work_type {
                    Distributor::distribute_to_map(packet, work);
                } else {
                    Distributor::distribute_to_queue(packet, work);
                }
            }
        })
    }

    pub fn distribute_to_queue(packet: Packet, work: Work) {
        let work_queue = &WORK_QUEUES[work.work_type.get_id()];
        let work_lock = &WORK_LOCKS[work.work_type.get_id()];

        work_queue.push(packet);

        // If no work of current type in threadpool
        if work_lock
            .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
            .is_ok()
        {
            TPOOL_QUEUE.push(work);
        }
    }

    pub fn distribute_to_map(packet: Packet, work: Work) {
        let boxed = Box::new(packet);

        let atomic_ptr = TOKEN_WISE_MAP.get(&work.work_type.get_id());

        if let Some(atomic_ptr) = atomic_ptr {
            // If value exists
            // retreive old packet by swaping with new value
            let old_packet = atomic_ptr.swap(Box::into_raw(boxed), Ordering::SeqCst);

            // If old packet ptr was set to null
            // create new work
            if old_packet.is_null() {
                TPOOL_QUEUE.push(work);
            } else {
                // If old packet was not null
                // means it is still allocated in heap
                // manually create box from it and drop
                unsafe {
                    let _ = Box::from_raw(old_packet);
                }
            }
        } else {
            let atomic_ptr = AtomicPtr::new(Box::into_raw(boxed));

            TOKEN_WISE_MAP.insert(work.work_type.get_id(), atomic_ptr);

            TPOOL_QUEUE.push(work);
        }
    }
}
