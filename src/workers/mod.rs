use crate::types::work::{ProcessingFn, WorkType};

pub mod bse_worker;
pub mod mcx_workers;
pub mod nse_worker;

pub fn get_neq_processing_fn(work_type: &WorkType) -> ProcessingFn {
    match work_type {
        WorkType::NseUncompressed | WorkType::SegmentWise(_) | WorkType::TokenWise(_) => {
            nse_worker::cast_and_twiddle_neq
        }
        _ => panic!("Invalid work type for NEQ processing function"),
    }
}

pub fn get_nfo_processing_fn(work_type: &WorkType) -> ProcessingFn {
    match work_type {
        WorkType::NseUncompressed | WorkType::SegmentWise(_) | WorkType::TokenWise(_) => {
            nse_worker::cast_and_twiddle_nfo
        }
        _ => panic!("Invalid work type for Nfo processing function"),
    }
}

pub fn get_ncd_processing_fn(work_type: &WorkType) -> ProcessingFn {
    match work_type {
        WorkType::NseUncompressed | WorkType::SegmentWise(_) | WorkType::TokenWise(_) => {
            nse_worker::cast_and_twiddle_ncd
        }
        _ => panic!("Invalid work type for Ncd processing function"),
    }
}

pub fn get_bse_processing_fn(work_type: &WorkType) -> ProcessingFn {
    match work_type {
        WorkType::BseUncompressed => bse_worker::process_bse_uncompressed,
        WorkType::BseCompressed => bse_worker::process_bse_compressed,
        _ => panic!("Invalid work type for BSE processing function"),
    }
}

pub fn get_mcx_processing_fn(work_type: &WorkType) -> ProcessingFn {
    match work_type {
        WorkType::McxDepthSnapshot => mcx_workers::process_mcx_depth_snapshot,
        WorkType::McxDepthIncr => mcx_workers::process_mcx_depth_incremental,
        WorkType::McxOther => mcx_workers::process_mcx_depth_others,
        _ => panic!("Invalid work type for MCX processing function"),
    }
}
