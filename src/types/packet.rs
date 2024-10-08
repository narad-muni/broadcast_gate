use std::mem::offset_of;

use rdkafka::message::ToBytes;

use crate::{
    constants::{BCAST_ONLY_MBP, BUF_SIZE, MAX_SUB_PACKETS, SKIP_BYTES},
    global::NSE_HEADER_SIZE,
    utils::byte_utils::{bytes_to_struct, bytes_to_struct_mut, create_empty},
};

use super::{
    packet_structures::{neq::BcastHeaders, nfo::BcastOnlyMBP, CompressionData, PackData},
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
    pub fn get_nse_packets(&self) -> ([(Packet, WorkType); MAX_SUB_PACKETS], usize) {
        let mut packets: [(Packet, WorkType); MAX_SUB_PACKETS] = create_empty();
        let mut packet_idx = 0;

        let mut packet: PackData = bytes_to_struct(&self.0);
        packet.twiddle();

        let mut offset = 0;

        for _ in 0..packet.no_of_packets {
            let mut compression_data: CompressionData =
                bytes_to_struct(&packet.pack_data[offset..]);
            compression_data.twiddle();

            // Increment for compression length field, which is u16, 2 bytes
            offset += size_of::<u16>();

            // Packet is not compressed
            if compression_data.compression_len == 0 {
                let packet = Packet(compression_data.broadcast_data);

                // Extract message length to increase offset
                let start = SKIP_BYTES + offset_of!(BcastHeaders, message_length);
                let end = start + size_of::<i16>();

                let mut message_length: i16 = bytes_to_struct(&packet.0[start..end]);
                message_length = message_length.to_be();

                // Increment offset by each message
                offset += message_length as usize + SKIP_BYTES + size_of::<u16>();

                let work_type = WorkType::NseUncompressed;

                packets[packet_idx] = (packet, work_type);
                packet_idx += 1;
            } else {
                // Packet is compressed

                offset += compression_data.compression_len as usize;

                let mut compressed_packet =
                    &compression_data.broadcast_data[..compression_data.compression_len as usize];
                let mut decompressed_packet = [0u8; BUF_SIZE];

                mylzo::decompress(&mut compressed_packet, &mut decompressed_packet)
                    .expect("Error decompressing packet");

                let mut packet = Packet(decompressed_packet);

                let trans_code = BcastHeaders::get_trans_code(&packet.0);

                // Fetch worktype for compressed packet
                let work_type = if trans_code == BCAST_ONLY_MBP {
                    let token_start = NSE_HEADER_SIZE + SKIP_BYTES + size_of::<i16>();
                    let token_end = token_start + size_of::<i32>();

                    let token =
                        i32::from_be_bytes(packet.0[token_start..token_end].try_into().unwrap());

                    WorkType::TokenWise(token)
                } else {
                    // get segment id
                    let segment = BcastHeaders::get_segment(&packet.0);

                    WorkType::SegmentWise(segment)
                };

                // For 7208, has multiple records
                // Seperate each token for parallel processing
                if let WorkType::TokenWise(_) = work_type {
                    let start = SKIP_BYTES + offset_of!(BcastOnlyMBP, no_of_records);
                    let end = start + size_of::<i16>();

                    let mut no_of_records: i16 = bytes_to_struct(&packet.0[start..end]);
                    no_of_records = no_of_records.to_be();

                    // Set no of records to 0, for original packet
                    *bytes_to_struct_mut::<i16>(&mut packet.0[start..end]) = 0;

                    // If more than one record
                    // Push first record into packet by setting no of records as 0
                    // increment it and re add same packet
                    if no_of_records > 1 {
                        // Clone packet
                        // Set no of records to zero and push
                        let mut packet = packet.clone();

                        // Mutable ref to slice
                        let no_of_records = bytes_to_struct_mut::<i16>(&mut packet.0[start..end]);
                        *no_of_records = 1;
                        // Twiddle
                        *no_of_records = no_of_records.to_be();

                        packets[packet_idx] = (packet, work_type);
                        packet_idx += 1;
                    }
                }

                packets[packet_idx] = (packet, work_type);
                packet_idx += 1;
            }
        }

        (packets, packet_idx)
    }
}
