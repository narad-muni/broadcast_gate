use crate::{
    constants::{BSE_BCAST_COMPLEX, BSE_BCAST_DEBT, BSE_BCAST_MBP},
    global::STATISTICS,
    types::{
        packet::Packet,
        work::{Work, WorkType},
    },
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
    fn distribute(&mut self, packet: Packet) {
        let mut message_code: i32 = bytes_to_struct(&packet.0);
        // Twiddle
        message_code = message_code.to_be();

        STATISTICS.get().other_packets_count += 1;

        // Create work
        let work_type = match message_code {
            // BcastMbp, BcastMbpComplexInst, BcastDebtMbp are of type compressed
            BSE_BCAST_MBP | BSE_BCAST_COMPLEX | BSE_BCAST_DEBT => WorkType::BseCompressed,
            _ => WorkType::BseUncompressed,
        };

        let processing_fn = get_bse_processing_fn(&work_type);
        let work = Work {
            work_type,
            processing_fn,
            atomic_ptr: None,
            mcx_state: None,
            seq_no: 0,
        };

        super::distribute_to_queue(packet, work);
    }
}
