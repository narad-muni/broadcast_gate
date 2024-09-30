use rdkafka::message::ToBytes;

use crate::{
    constants::{BUF_SIZE, SKIP_BYTES},
    global::NSE_HEADER_SIZE,
    utils::byte_utils::bytes_to_struct,
};

use super::{
    packet_structures::{neq::BcastHeaders, CompressionData, PackData},
    work::WorkType,
};

#[derive(Debug, Clone, Copy)]
pub struct Packet(pub [u8; BUF_SIZE]);

/// Required for RdKafka's `ToBytes` trait to allow sending `Packet` to kafka
impl ToBytes for Packet {
    fn to_bytes(&self) -> &[u8] {
        &self.0
    }
}

impl Packet {
    pub fn get_nse_packets(&self) -> Vec<(Packet, WorkType)> {
        let mut packets = vec![];

        let mut packet: PackData = bytes_to_struct(&self.0);
        packet.twiddle();

        let mut offset = 0;

        for _ in 0..packet.no_of_packets {
            let mut compression_data: CompressionData =
                bytes_to_struct(&packet.pack_data[offset..]);
            compression_data.twiddle();

            // Packet is not compressed
            if compression_data.compression_len == 0 {
                let packet = Packet(compression_data.broadcast_data);

                let mut header: BcastHeaders = bytes_to_struct(&packet.0[SKIP_BYTES..]);
                header.twiddle();

                offset += header.message_length as usize + SKIP_BYTES + size_of::<u16>();

                let work_type = WorkType::Uncompressed;

                packets.push((packet, work_type));
            } else {
                // Packet is compressed

                offset += compression_data.compression_len as usize;

                let mut compressed_packet =
                    &compression_data.broadcast_data[..compression_data.compression_len as usize];
                let mut decompressed_packet = [0u8; BUF_SIZE];

                mylzo::decompress(&mut compressed_packet, &mut decompressed_packet)
                    .expect("Error decompressing packet");

                let packet = Packet(decompressed_packet);

                let mut bcast_header: BcastHeaders = bytes_to_struct(&packet.0[SKIP_BYTES..]);
                bcast_header.twiddle();

                // Fetch worktype for compressed packet
                let work_type = if bcast_header.trans_code == 7208 {
                    let token_start = NSE_HEADER_SIZE + SKIP_BYTES + size_of::<i16>();
                    let token_end = token_start + size_of::<i32>();

                    let token =
                        i32::from_be_bytes(packet.0[token_start..token_end].try_into().unwrap());

                    WorkType::TokenWise(token)
                } else {
                    // get segment id
                    let segment = bcast_header.filler2[0];

                    WorkType::SegmentWise(segment)
                };

                packets.push((packet, work_type));
            }

            offset += size_of::<u16>();
        }

        packets
    }
}
