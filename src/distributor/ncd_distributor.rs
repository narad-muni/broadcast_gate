use crate::{
    types::{
        packet::Packet,
        work::{Work, WorkType},
    },
    workers::get_ncd_processing_fn,
};

use super::Distribute;

pub struct NcdDistributor {}

impl NcdDistributor {
    pub fn new() -> Self {
        Self {}
    }
}

impl Distribute for NcdDistributor {
    fn distribute(&mut self, packet: Packet) {
        // If nse, extract packets received in single packet
        let (packets, no_of_packets) = packet.get_nse_packets();

        for i in 0..no_of_packets {
            let (packet, work_type) = packets[i];
            // Create work
            let processing_fn = get_ncd_processing_fn(&work_type);
            let work = Work {
                work_type,
                processing_fn,
                atomic_ptr: None,
                mcx_state: None,
                seq_no: 0,
            };

            if let WorkType::TokenWise(_) = work_type {
                super::distribute_to_map(packet, work);
            } else {
                super::distribute_to_queue(packet, work);
            }
        }
    }
}
