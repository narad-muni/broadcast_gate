
use crate::{global::OUTPUT_QUEUE, types::packet::Packet};

pub fn process_segment_wise(packet: Packet) {
    OUTPUT_QUEUE.push(packet);
}