use crate::{
    global::STATISTICS,
    types::work::{Work, WorkType},
    utils::byte_utils::bytes_to_struct,
    workers::get_bse_processing_fn,
};

use super::Distribute;

pub struct BseDistributor {}

impl BseDistributor {
    pub fn new() -> Self {
        Self {}
    }
}

impl Distribute for BseDistributor {
    fn distribute(&mut self, packet: crate::types::packet::Packet) {
        let mut message_code: i32 = bytes_to_struct(&packet.0);
        // Twiddle
        message_code = message_code.to_be();

        STATISTICS.get().other_packets_count += 1;

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
            atomic_ptr: None,
        };

        super::distribute_to_queue(packet, work);
    }
}
