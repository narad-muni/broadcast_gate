use rdkafka::message::ToBytes;

use crate::{constants::BUF_SIZE, workers::get_processing_fn};

use super::work::{Work, WorkType};

#[derive(Debug, Clone, Copy)]
pub struct Packet(pub [u8; BUF_SIZE]);

/// Required for RdKafka's `ToBytes` trait to allow sending `Packet` to kafka
impl ToBytes for Packet {
    fn to_bytes(&self) -> &[u8] {
        &self.0
    }
}

impl Packet {
    pub fn create_work(&self) -> Work {
        let work_type = self.identify_work_type();
        let processing_fn = get_processing_fn(&work_type);

        Work {
            processing_fn,
            work_type,
        }
    }

    pub fn identify_work_type(&self) -> WorkType {
        WorkType::TokenWise(2)
        // todo!()
    }
}
