use std::mem::{offset_of, size_of};

use crate::{
    constants::{
        BCAST_MBO_MBP, BCAST_ONLY_MBP, BCAST_ONLY_MBP_EQ, BUF_SIZE, MAX_SUB_PACKETS, SKIP_BYTES,
    },
    global::STATISTICS,
    utils::byte_utils::{bytes_to_struct, bytes_to_struct_mut, create_empty},
    workers::nse_worker::get_token,
};

use super::{
    packet_structures::{neq::BcastHeaders, nfo, CompressionData, PackData},
    work::WorkType,
};

#[derive(Debug, Clone, Copy)]
pub struct Packet(pub [u8; BUF_SIZE], pub usize);

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
                let packet = Packet(compression_data.broadcast_data, BUF_SIZE);

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
                STATISTICS.get().other_packets_count += 1;
            } else {
                // Packet is compressed

                offset += compression_data.compression_len as usize;

                let mut compressed_packet =
                    &compression_data.broadcast_data[..compression_data.compression_len as usize];
                let mut decompressed_packet = [0u8; BUF_SIZE];

                mylzo::decompress(&mut compressed_packet, &mut decompressed_packet)
                    .expect("Error decompressing packet");

                let mut packet = Packet(decompressed_packet, BUF_SIZE);

                let trans_code = BcastHeaders::get_trans_code(&packet.0);

                // Fetch worktype for compressed packet
                let work_type = if trans_code == BCAST_ONLY_MBP
                    || trans_code == BCAST_ONLY_MBP_EQ
                    || trans_code == BCAST_MBO_MBP
                {
                    let token = get_token(trans_code, &packet.0, 0);

                    WorkType::TokenWise(token)
                } else {
                    // get segment id
                    let segment = BcastHeaders::get_segment(&packet.0);

                    WorkType::SegmentWise(segment)
                };

                if trans_code == BCAST_MBO_MBP {
                    // Used because cannot use another condition with below if let
                    // Below code shouldn't be executed for BCAST_MBO_MBP 7200
                } else if let WorkType::TokenWise(_) = work_type {
                    // For 7208, 18705, has multiple records
                    // Seperate each token for parallel processing

                    // common for eq, fao, cd for no_of_records offset
                    let start = SKIP_BYTES + offset_of!(nfo::BcastOnlyMBP, no_of_records);
                    let end = start + size_of::<i16>();

                    // Get no of packets
                    let mut no_of_records: i16 = bytes_to_struct(&packet.0[start..end]);
                    no_of_records = no_of_records.to_be();

                    // Set no of records to 0, for original packet
                    // No need to twiddle 0
                    *bytes_to_struct_mut::<i16>(&mut packet.0[start..end]) = 0;

                    // If more than one record
                    // Push first record into packet by setting no of records as 0
                    // increment it and re add same packet
                    if no_of_records > 1 {
                        // Clone packet
                        // Set no of records to zero and push
                        let mut packet = packet.clone();

                        // Get token for second packet and update work type
                        let token = get_token(trans_code, &packet.0, 1);
                        let work_type = WorkType::TokenWise(token);

                        // Mutable ref to slice
                        let no_of_records = bytes_to_struct_mut::<i16>(&mut packet.0[start..end]);
                        *no_of_records = 1;
                        // Twiddle, because this will be twiddeled again while processing
                        *no_of_records = no_of_records.to_be();

                        // Add packet and increase packet idx
                        packets[packet_idx] = (packet, work_type);
                        packet_idx += 1;
                        STATISTICS.get().depth_packets_count += 1;
                    }
                }

                packets[packet_idx] = (packet, work_type);
                packet_idx += 1;
                STATISTICS.get().depth_packets_count += 1;
            }
        }

        (packets, packet_idx)
    }
}
