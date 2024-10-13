use std::{
    alloc::{dealloc, Layout}, sync::atomic::{AtomicPtr, Ordering}, thread::{self, JoinHandle}, time::Duration
};

use crate::{
    global::{INPUT_QUEUE, TOKEN_WISE_MAP, TPOOL_QUEUE, WORK_LOCKS, WORK_QUEUES},
    settings,
    types::{
        packet::Packet,
        settings::Exchange,
        work::{Work, WorkType},
    },
    utils::byte_utils::bytes_to_struct,
    workers::{
        get_bse_processing_fn, get_ncd_processing_fn, get_neq_processing_fn, get_nfo_processing_fn,
    },
};

pub struct Distributor {}

impl Distributor {
    pub fn new() -> Self {
        Self {}
    }

    pub fn start_distributor(self) -> JoinHandle<()> {
        let settings = settings::get();

        // NSE or BSE processing function
        let processing_fn = match settings.exchange {
            Exchange::BSE => Distributor::process_bse_packet,
            Exchange::NEQ => Distributor::process_neq_packet,
            Exchange::NFO => Distributor::process_nfo_packet,
            Exchange::NCD => Distributor::process_ncd_packet,
        };

        thread::spawn(move || loop {
            if let Some(packet) = INPUT_QUEUE.pop() {
                processing_fn(packet);
            }
        })
    }

    pub fn process_neq_packet(packet: Packet) {
        // If nse, extract packets received in single packet
        let (packets, no_of_packets) = packet.get_nse_packets();

        for i in 0..no_of_packets {

            let (packet, work_type) = packets[i];
            // Create work
            let processing_fn = get_neq_processing_fn(&work_type);
            let work = Work {
                work_type,
                processing_fn,
            };

            if let WorkType::TokenWise(_) = work_type {
                Distributor::distribute_to_map(packet, work);
            } else {
                // Distributor::distribute_to_queue(packet, work);
            }
        }
    }

    pub fn process_nfo_packet(packet: Packet) {
        // If nse, extract packets received in single packet
        let (packets, no_of_packets) = packet.get_nse_packets();

        for i in 0..no_of_packets {

            let (packet, work_type) = packets[i];
            // Create work
            let processing_fn = get_nfo_processing_fn(&work_type);
            let work = Work {
                work_type,
                processing_fn,
            };

            if let WorkType::TokenWise(_) = work_type {
                Distributor::distribute_to_map(packet, work);
            } else {
                Distributor::distribute_to_queue(packet, work);
            }
        }
    }

    pub fn process_ncd_packet(packet: Packet) {
        // If nse, extract packets received in single packet
        let (packets, no_of_packets) = packet.get_nse_packets();

        for i in 0..no_of_packets {

            let (packet, work_type) = packets[i];
            // Create work
            let processing_fn = get_ncd_processing_fn(&work_type);
            let work = Work {
                work_type,
                processing_fn,
            };

            if let WorkType::TokenWise(_) = work_type {
                Distributor::distribute_to_map(packet, work);
            } else {
                Distributor::distribute_to_queue(packet, work);
            }
        }
    }

    pub fn process_bse_packet(packet: Packet) {
        let mut message_code: i32 = bytes_to_struct(&packet.0);
        // Twiddle
        message_code = message_code.to_be();

        // Create work
        let work_type = match message_code {
            // BcastMbp, BcastMbpComplexInst, BcastDebtMbp are of type compressed
            2020 | 2021 | 2033 => WorkType::BseCompressed,
            _ => WorkType::BseUncompressed,
        };

        let processing_fn = get_bse_processing_fn(&work_type);
        let work = Work {
            work_type,
            processing_fn,
        };

        Distributor::distribute_to_queue(packet, work);
    }

    pub fn distribute_to_queue(packet: Packet, work: Work) {
        let work_id = work.work_type.get_id();

        let work_queue = &WORK_QUEUES[work_id];
        let work_lock = &WORK_LOCKS[work_id];

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
        let boxed = Box::into_raw(Box::new(packet));

        let value = TOKEN_WISE_MAP.get(&work.work_type.get_id());

        if let Some(packet_ptr) = value {
            // If value exists
            // retreive old packet by swaping with new value
            let old_packet = packet_ptr.swap(boxed, Ordering::SeqCst);

            // If old packet ptr was set to null
            // create new work
            if old_packet.is_null() {
                TPOOL_QUEUE.push(work);
            } else if !old_packet.is_null() {
                // If old packet was not null
                // means it is still allocated in heap
                // manually create box from it and drop
                unsafe {
                    dealloc(old_packet as *mut u8, Layout::new::<Packet>());
                }
            }
        } else {
            let packet_ptr = AtomicPtr::new(boxed);

            TOKEN_WISE_MAP.insert(work.work_type.get_id(), packet_ptr);

            TPOOL_QUEUE.push(work);
        }
    }
}
