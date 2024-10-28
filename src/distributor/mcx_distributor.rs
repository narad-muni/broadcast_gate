use std::{alloc::{dealloc, Layout}, fs, sync::atomic::Ordering};

use bytes::Bytes;
use fastlib::{Decoder, ModelFactory};
use serde::Deserialize;

use crate::{constants::BUF_SIZE, global::{MCX_TOKEN_WISE_MAP, TPOOL_QUEUE}, settings, types::{packet::Packet, packet_structures::mcx::{DepthIncremental, DepthSnapshot, Message}, state::McxTokenState, work::{Work, WorkType}}, utils::byte_utils::struct_to_bytes, workers::{get_mcx_processing_fn, mcx_workers::process_mcx_depth}};

use super::Distribute;

pub struct McxDistributor {
    decoder: Decoder,
}

// Required for Decoder, safe because is used by only single thread
unsafe impl Send for McxDistributor {}

impl McxDistributor {
    pub fn new() -> Self {
        let settings = settings::get().clone();
        let template = fs::read_to_string(
            &settings
                .fast_template
                .expect("Fast template path required for mcx"),
        )
        .unwrap();

        let decoder = Decoder::new_from_xml(&template).unwrap();

        Self { decoder }
    }
}

impl Distribute for McxDistributor {
    fn distribute(&mut self, packet: Packet) {
        let mut raw = Bytes::from(packet.0[packet.1..].to_owned());

        loop {
            // decode_reader consumes raw.
            // If raw is consumed, it will be empty
            if raw.is_empty() {
                break;
            }

            let mut msg = ModelFactory::new();
            let err = self.decoder.decode_reader(&mut raw, &mut msg);

            if err.is_err() {
                println!("Error: {:?}", err);
                break;
            }

            let message = Message::deserialize(msg.data.unwrap().clone());

            // Get result or error
            let message = if let Err(error) = message {
                println!("Error: {:?}", error);
                break;
            } else {
                message.unwrap()
            };

            // Reset decoder and skip these messages
            if let Message::FastReset(_) = message {
                self.decoder.reset();
                continue;
            } else if let Message::MDPacketHeader(_) = message {
                continue;
            }

            match message {
                Message::DepthSnapshot(depth_snapshot) => self.distribute_snapshot(depth_snapshot),
                Message::DepthIncremental(depth_incremental) => self.distribute_incremental(depth_incremental),
                _ => self.distribute_others(message),
            }

            
        }
    }
}

impl McxDistributor {
    pub fn distribute_snapshot(&self, depth_snapshot: DepthSnapshot) {
        // Get token and mcx state
        let token = depth_snapshot.SecurityID as usize;
        let mcx_state = MCX_TOKEN_WISE_MAP.entry(token).or_insert(McxTokenState::new());

        // Do not process if packet's seq no is older than current
        let current_seq_no = mcx_state.seq_no.load(Ordering::SeqCst);
        if depth_snapshot.MsgSeqNum.unwrap_or(0) <= current_seq_no {
            return;
        }

        // Create packet
        let mut packet = Packet([0; BUF_SIZE], BUF_SIZE);
        packet.1 = struct_to_bytes(&depth_snapshot, &mut packet.0);

        // Create work
        let work = Work {
            work_type: WorkType::Mcx,
            processing_fn: get_mcx_processing_fn(&WorkType::Mcx),
            atomic_ptr: None,
            mcx_state: Some(mcx_state.clone()),
        };

        // Swap new packet in atomic ptr
        let new_packet_ptr = Box::into_raw(Box::new(packet));
        let old_packet_ptr = mcx_state.ptr.swap(new_packet_ptr, Ordering::SeqCst);

        // Free old packet
        if old_packet_ptr.is_null() {
            unsafe {
                dealloc(old_packet_ptr as *mut u8, Layout::new::<Packet>());
            }
        }

        // Only add work if work queue is empty
        if mcx_state.packet_queue.len() == 0 {
            let mut empty_packet = Packet([0; BUF_SIZE], BUF_SIZE);
            empty_packet.1 = struct_to_bytes(&Message::DepthSnapshotEmpty, &mut empty_packet.0);

            mcx_state.packet_queue.push(empty_packet);

            if mcx_state.work_lock.compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst).is_ok() {
                TPOOL_QUEUE.push(work);
            }
        }
    }

    pub fn distribute_incremental(&self, depth_incremental: DepthIncremental) {
        let messages = depth_incremental.MDIncGrp;

        for message in messages {
            // Get token and mcx state
            let token = message.SecurityID as usize;
            let mcx_state = MCX_TOKEN_WISE_MAP.entry(token).or_insert(McxTokenState::new());

            // Do not process if packet's seq no is older than current
            let current_seq_no = mcx_state.seq_no.load(Ordering::SeqCst);
            if depth_incremental.MsgSeqNum <= current_seq_no {
                continue;
            }

            // Create packet
            let mut packet = Packet([0; BUF_SIZE], BUF_SIZE);
            packet.1 = struct_to_bytes(&message, &mut packet.0);

            let work = Work {
                work_type: WorkType::Mcx,
                processing_fn: get_mcx_processing_fn(&WorkType::Mcx),
                atomic_ptr: None,
                mcx_state: Some(mcx_state.clone()),
            };

            mcx_state.packet_queue.push(packet);

            if mcx_state.work_lock.compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst).is_ok() {
                TPOOL_QUEUE.push(work);
            }
        }
    }

    pub fn distribute_others(&self, message: Message) {
        todo!("Handle other messages")
    }
}
