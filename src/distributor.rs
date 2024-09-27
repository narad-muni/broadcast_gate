use std::{
    ptr,
    sync::atomic::{AtomicPtr, Ordering},
    thread::{self, JoinHandle},
};

use crate::{
    global::{INPUT_QUEUE, TOKEN_WISE_MAP, TPOOL_QUEUE, WORK_LOCKS, WORK_QUEUES},
    types::{packet::Packet, work::{Work, WorkType}},
};

pub struct Distributor {}

impl Distributor {
    pub fn new() -> Self {
        Self {}
    }

    pub fn start_distributor(self) -> JoinHandle<()> {
        thread::spawn(move || {
            loop {
                if let Some(packet) = INPUT_QUEUE.pop() {
                    // println!("Received in ditributor: {:?}", data);

                    let work = packet.create_work();

                    if let WorkType::TokenWise(_) = work.work_type {
                        Distributor::distribute_to_map(packet, work);
                    } else {
                        Distributor::distribute_to_queue(packet, work);
                    }
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
            .compare_exchange(false, true, Ordering::Acquire, Ordering::Acquire)
            .is_ok()
        {
            TPOOL_QUEUE.push(work);
        }
    }

    pub fn distribute_to_map(packet: Packet, work: Work) {
        let mut boxed = packet;

        let old_packet = TOKEN_WISE_MAP.get(&work.work_type.get_id());

        if let Some(data) = old_packet {
            if data.swap(&mut boxed, Ordering::SeqCst) == ptr::null_mut() {
                TPOOL_QUEUE.push(work);
            }
        } else {
            let atomic_ptr = AtomicPtr::new(&mut boxed);

            TOKEN_WISE_MAP.insert(work.work_type.get_id(), atomic_ptr);

            TPOOL_QUEUE.push(work);
        }
    }

}