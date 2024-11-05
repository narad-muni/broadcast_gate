use serde::Serialize;
use serde_big_array::BigArray;
use twiddler::Twiddle;

use crate::constants::{ALPHA_CHAR_LEN, MAX_MARKET_DEPTH_IDX, TIMESTAMP_LEN};

#[derive(Debug, Twiddle, Clone, Copy, Serialize)]
#[repr(C, packed(2))]
pub struct TagMarketDepthInfo {
    pub qty: i64,
    pub price: i32,
    pub number_of_orders: i16,
}

#[derive(Debug, Twiddle, Clone, Copy, Serialize)]
#[repr(C, packed(2))]
pub struct TagMessageHeader {
    pub message_code: i32,
    pub transaction_type: i16,
    pub log_time: i32,
    pub alpha_char: [u8; ALPHA_CHAR_LEN],
    pub trader_id: i32,
    pub error_code: i16,
    pub timestamp: u64,
    pub timestamp1: [u8; TIMESTAMP_LEN],
    pub timestamp2: [u8; TIMESTAMP_LEN],
    pub message_length: i16,
}

#[derive(Debug, Twiddle, Clone, Copy, Serialize)]
#[repr(C, packed(2))]
pub struct TagMarketPictureBroadcast {
    pub msg_header: TagMessageHeader,
    pub token: i64,
    pub total_buy_qty: i64,
    pub total_sell_qty: i64,
    pub volume_traded_today: i64,
    pub open_price: i32,
    pub close_price: i32,
    pub high_price: i32,
    pub low_price: i32,
    pub ltp: i32,
    pub ltq: i32,
    pub ltt: i32,
    pub atp: i32,
    pub indicative_close_price: i32,
    pub lut: i64,
    pub buy_depth_count: i32,
    pub sell_depth_count: i32,
    pub trading_status: i16,
    #[serde(with = "BigArray")]
    pub market_depth_info: [TagMarketDepthInfo; MAX_MARKET_DEPTH_IDX],
}
