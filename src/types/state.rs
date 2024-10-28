use std::sync::{atomic::{AtomicBool, AtomicPtr, AtomicU32}, Arc};

use crossbeam::queue::SegQueue;

use super::packet::Packet;

#[derive(Clone)]
pub struct NseTokenState {
    pub ptr: Arc<AtomicPtr<Packet>>,
}

#[derive(Debug, Clone)]
pub struct McxTokenState {
    pub packet_queue: Arc<SegQueue<Packet>>,
    pub work_lock: Arc<AtomicBool>,
    pub ptr: Arc<AtomicPtr<Packet>>,
    pub seq_no: Arc<AtomicU32>,
}

impl McxTokenState {
    pub fn new() -> Self {
        Self {
            packet_queue: Arc::new(SegQueue::new()),
            ptr: Arc::new(AtomicPtr::new(std::ptr::null_mut())),
            seq_no: Arc::new(AtomicU32::new(0)),
            work_lock: Arc::new(AtomicBool::new(false)),
        }
    }
}