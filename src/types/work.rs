use super::packet::Packet;

pub type ProcessingFn = fn(&mut Packet);

#[derive(Debug, Clone, Copy)]
pub struct Work {
    pub work_type: WorkType,
    pub processing_fn: ProcessingFn,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum WorkType {
    Compressed,
    Uncompressed,
    SegmentWise(u16),
    TokenWise(usize),
}

impl WorkType {
    pub fn get_id(&self) -> usize {
        match self {
            Self::Compressed => 0,
            Self::Uncompressed => 1,
            Self::SegmentWise(i) => 2 + *i as usize,
            Self::TokenWise(i) => 3 + u16::MAX as usize + *i as usize, // Shouldn't be used
        }
    }
}
