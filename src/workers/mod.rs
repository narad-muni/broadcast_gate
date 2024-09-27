use crate::types::work::{ProcessingFn, WorkType};

pub mod compressed;
pub mod segment_wise;
pub mod token_wise;
pub mod uncompressed;

pub fn get_processing_fn(work_type: &WorkType) -> ProcessingFn {
    match work_type {
        WorkType::Compressed => compressed::process_compressed,
        WorkType::Uncompressed => uncompressed::process_uncompressed,
        WorkType::SegmentWise(_) => segment_wise::process_segment_wise,
        WorkType::TokenWise(_) => token_wise::process_token_wise,
    }
}