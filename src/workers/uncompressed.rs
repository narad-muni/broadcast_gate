use crate::{global::OUTPUT_QUEUE, types::packet::Packet};

pub fn process_uncompressed(packet: Packet) {
    OUTPUT_QUEUE.push(packet);
}
