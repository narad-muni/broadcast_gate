use crate::types::work::{ProcessingFn, WorkType};

pub mod bse_worker;
pub mod nse_worker;

pub fn get_neq_processing_fn(work_type: &WorkType) -> ProcessingFn {
    match work_type {
        WorkType::Compressed => panic!("Invalid work type for NEQ processing function"),
        _ => nse_worker::cast_and_twiddle_neq,
    }
}

pub fn get_nfo_processing_fn(work_type: &WorkType) -> ProcessingFn {
    match work_type {
        WorkType::Compressed => panic!("Invalid work type for Nfo processing function"),
        _ => nse_worker::cast_and_twiddle_nfo,
    }
}

pub fn get_ncd_processing_fn(work_type: &WorkType) -> ProcessingFn {
    match work_type {
        WorkType::Compressed => panic!("Invalid work type for Ncd processing function"),
        _ => nse_worker::cast_and_twiddle_ncd,
    }
}

pub fn get_bse_processing_fn(work_type: &WorkType) -> ProcessingFn {
    match work_type {
        WorkType::Uncompressed => bse_worker::process_bse_uncompressed,
        WorkType::Compressed => bse_worker::process_bse_compressed,
        _ => panic!("Invalid work type for BSE processing function"),
    }
}
