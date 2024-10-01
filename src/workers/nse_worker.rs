use crate::{
    constants::SKIP_BYTES,
    types::{
        packet::Packet,
        packet_structures::{
            ncd::build_ncd_struct,
            neq::{build_neq_struct, BcastHeaders},
            nfo::build_nfo_struct,
        },
    },
    utils::byte_utils::{bytes_to_struct, struct_to_bytes},
};

pub fn cast_and_twiddle_nfo(packet: &mut Packet) {
    let mut header: BcastHeaders = bytes_to_struct(&packet.0[SKIP_BYTES..]);
    header.twiddle();

    let mut nfo_struct = build_nfo_struct(header.trans_code, &packet.0[SKIP_BYTES..]);
    nfo_struct.twiddle();

    struct_to_bytes(&nfo_struct, &mut packet.0);
}

pub fn cast_and_twiddle_neq(packet: &mut Packet) {
    let mut header: BcastHeaders = bytes_to_struct(&packet.0[SKIP_BYTES..]);
    header.twiddle();

    let mut neq_struct = build_neq_struct(header.trans_code, &packet.0[SKIP_BYTES..]);
    neq_struct.twiddle();

    struct_to_bytes(&neq_struct, &mut packet.0);
}

pub fn cast_and_twiddle_ncd(packet: &mut Packet) {
    let mut header: BcastHeaders = bytes_to_struct(&packet.0[SKIP_BYTES..]);
    header.twiddle();

    let mut ncd_struct = build_ncd_struct(header.trans_code, &packet.0[SKIP_BYTES..]);
    ncd_struct.twiddle();

    struct_to_bytes(&ncd_struct, &mut packet.0);
}
