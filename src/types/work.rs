use std::sync::{atomic::AtomicPtr, Arc};

use super::{packet::Packet, state::McxTokenState};

pub type ProcessingFn = fn(&mut Packet, &Work) -> bool;

#[derive(Debug, Clone)]
pub struct Work {
    pub work_type: WorkType,
    pub processing_fn: ProcessingFn,
    pub seq_no: usize,
    pub atomic_ptr: Option<Arc<AtomicPtr<Packet>>>,
    pub mcx_state: Option<McxTokenState>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum WorkType {
    BseCompressed,
    BseUncompressed,
    NseUncompressed,
    SegmentWise(u8),
    TokenWise(i32),
    McxDepthSnapshot,
    McxDepthIncr,
    McxOther,
}

impl WorkType {
    pub fn get_id(&self) -> usize {
        match self {
            Self::BseCompressed => 0,
            Self::BseUncompressed => 1,
            Self::NseUncompressed => 2, // First element of queue is for uncompressed
            Self::SegmentWise(i) => 3 + *i as usize, // each segment has its own queue
            Self::TokenWise(i) => *i as usize, // Shouldn't be used on queue, only on map
            Self::McxDepthSnapshot => 0,
            Self::McxDepthIncr => 0,
            Self::McxOther => 0,
        }
    }
}
