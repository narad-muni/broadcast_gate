use crate::{
    types::work::{Work, WorkType},
    workers::get_nfo_processing_fn,
};

use super::Distribute;

pub struct NfoDistributor {}

impl NfoDistributor {
    pub fn new() -> Self {
        Self {}
    }
}

impl Distribute for NfoDistributor {
    fn distribute(&mut self, packet: crate::types::packet::Packet) {
        // If nse, extract packets received in single packet
        let (packets, no_of_packets) = packet.get_nse_packets();

        for i in 0..no_of_packets {
            let (packet, work_type) = packets[i];
            // Create work
            let processing_fn = get_nfo_processing_fn(&work_type);
            let work = Work {
                work_type,
                processing_fn,
                atomic_ptr: None,
            };

            if let WorkType::TokenWise(_) = work_type {
                super::distribute_to_map(packet, work);
            } else {
                super::distribute_to_queue(packet, work);
            }
        }
    }
}