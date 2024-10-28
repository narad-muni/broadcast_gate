use std::{ptr, sync::atomic::Ordering};

use crate::{constants::BUF_SIZE, types::{
        packet::Packet,
        packet_structures::mcx::{DepthSnapshot, Message},
        work::Work}, utils::byte_utils::{bytes_to_struct_bincode, struct_to_bytes_bincode}};

pub fn process_mcx_depth(packet: &mut Packet, work: &Work) {
    // Swap atomic ptr with null, and add atomic ptr to work
    let mcx_state = work.mcx_state.clone().unwrap();

    // Swap atomic ptr with null
    let ptr = mcx_state.ptr.swap(ptr::null_mut(), Ordering::SeqCst);
    let mut ptr = unsafe { Box::from_raw(ptr) };
    let mut snapshot: DepthSnapshot = bytes_to_struct_bincode(&ptr.0[..]);

    let message: Message = bytes_to_struct_bincode(&packet.0[..]);

    if let Message::DepthSnapshotEmpty(()) = message {
        mcx_state.seq_no.store(work.seq_no as u32, Ordering::SeqCst);

        snapshot.MsgSeqNum = Some(work.seq_no as u32);
    } else if let Message::MDIncGrp(md_incr_grp) = message {
        let seq_no_update = mcx_state.seq_no.compare_exchange(
            (work.seq_no - 1) as u32,
            work.seq_no as u32,
            Ordering::SeqCst,
            Ordering::SeqCst,
        );

        println!("{:?}", md_incr_grp);

        if seq_no_update.is_err() {
            // Put ptr back into atomic ptr
            mcx_state.ptr.swap(Box::into_raw(ptr), Ordering::SeqCst);
            return;
        }

        snapshot.MsgSeqNum = Some(work.seq_no as u32);
    } else {
        mcx_state.ptr.swap(Box::into_raw(ptr), Ordering::SeqCst);
        todo!();
    }

    // Put modified data back into atomic ptr
    let mut packet = Packet([0; BUF_SIZE], BUF_SIZE);

    packet.1 = struct_to_bytes_bincode(&snapshot, &mut packet.0);

    *ptr = packet;

    mcx_state.ptr.swap(Box::into_raw(ptr), Ordering::SeqCst);
}