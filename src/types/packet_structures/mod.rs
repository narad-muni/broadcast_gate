use twiddler::Twiddle;

use crate::constants::BUF_SIZE;

pub mod bse;
pub mod mcx;
pub mod ncd; // NSE Commodities
pub mod neq; // NSE Equity
pub mod nfo; // NSE FAO

#[derive(Debug, Twiddle)]
#[repr(C, packed(2))]
pub struct PackData {
    pub net_id: u8,
    pub reserved: u8,
    pub no_of_packets: u16,
    pub pack_data: [u8; 512],
}

#[derive(Debug, Clone, Copy, Twiddle)]
#[repr(C, packed(2))]
pub struct CompressionData {
    pub compression_len: u16,
    pub broadcast_data: [u8; BUF_SIZE],
}
