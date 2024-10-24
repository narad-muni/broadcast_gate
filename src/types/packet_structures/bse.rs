use crate::{
    constants::*,
    utils::byte_utils::{bytes_to_struct, struct_to_bytes},
};
use twiddler::Twiddle;

#[derive(Debug, Twiddle)]
pub enum BseBroadcastTransactionMapping {
    BcastTimeMessage(BcastTimeMessage),
    BcastSessionChange(BcastSessionChange),
    BcastAuctoinSessionChange(BcastAuctionSessionChange),
    BcastNewsHeadline(BcastNewsHeadline),
    BcastIndex1(BcastIndexChangeMessage),
    BcastIndex2(BcastIndexChangeMessage),
    BcastClosePrice(BcastClosePrice),
    BcastOpenInterestMsg(BcastOpenInterest),
    BcastVarPercentage(BcastVarPercentage),
    BcastAuctionMbp(BcastAuctionMBP),
    BcastMbp(BcastMarketPicture),
    BcastMbpComplexInst(BcastComplexMarketPicture),
    BcastRbiRefRate(BcastRBIReferenceRate),
    BcastOddLotMbp(BcastOddLotMarketPicture),
    BcastImpliedVolatility(BcastImpliedVolatility),
    BcastKeepAlive(BcastKeepAlive),
    BcastDebtMbp(BcastDebtMarketPicture),
    BcastLppRange(BcastLPPRange),
    BcastCallAuctionCxlQtyMsg(BcastCACxlQtyMessage),
}

pub fn build_bse_struct(transaction_id: i16, buf: &[u8]) -> BseBroadcastTransactionMapping {
    match transaction_id {
        2001 => BseBroadcastTransactionMapping::BcastTimeMessage(bytes_to_struct(&buf)),
        2002 => BseBroadcastTransactionMapping::BcastSessionChange(bytes_to_struct(&buf)),
        2003 => BseBroadcastTransactionMapping::BcastAuctoinSessionChange(bytes_to_struct(&buf)),
        2004 => BseBroadcastTransactionMapping::BcastNewsHeadline(bytes_to_struct(&buf)),
        2011 => BseBroadcastTransactionMapping::BcastIndex1(bytes_to_struct(&buf)),
        2012 => BseBroadcastTransactionMapping::BcastIndex2(bytes_to_struct(&buf)),
        2014 => BseBroadcastTransactionMapping::BcastClosePrice(bytes_to_struct(&buf)),
        2015 => BseBroadcastTransactionMapping::BcastOpenInterestMsg(bytes_to_struct(&buf)),
        2016 => BseBroadcastTransactionMapping::BcastVarPercentage(bytes_to_struct(&buf)),
        2017 => BseBroadcastTransactionMapping::BcastAuctionMbp(bytes_to_struct(&buf)),
        2020 => BseBroadcastTransactionMapping::BcastMbp(bytes_to_struct(&buf)),
        2021 => BseBroadcastTransactionMapping::BcastMbpComplexInst(bytes_to_struct(&buf)),
        2022 => BseBroadcastTransactionMapping::BcastRbiRefRate(bytes_to_struct(&buf)),
        2027 => BseBroadcastTransactionMapping::BcastOddLotMbp(bytes_to_struct(&buf)),
        2028 => BseBroadcastTransactionMapping::BcastImpliedVolatility(bytes_to_struct(&buf)),
        2030 => BseBroadcastTransactionMapping::BcastKeepAlive(bytes_to_struct(&buf)),
        2033 => BseBroadcastTransactionMapping::BcastDebtMbp(bytes_to_struct(&buf)),
        2034 => BseBroadcastTransactionMapping::BcastLppRange(bytes_to_struct(&buf)),
        2035 => BseBroadcastTransactionMapping::BcastCallAuctionCxlQtyMsg(bytes_to_struct(&buf)),
        _ => panic!("Invalid bse transaction id: {}", transaction_id),
    }
}

impl BseBroadcastTransactionMapping {
    pub fn to_bytes(&self, buf: &mut [u8]) {
        match self {
            BseBroadcastTransactionMapping::BcastTimeMessage(s) => struct_to_bytes(&s, buf),
            BseBroadcastTransactionMapping::BcastSessionChange(s) => struct_to_bytes(&s, buf),
            BseBroadcastTransactionMapping::BcastAuctoinSessionChange(s) => {
                struct_to_bytes(&s, buf)
            }
            BseBroadcastTransactionMapping::BcastNewsHeadline(s) => struct_to_bytes(&s, buf),
            BseBroadcastTransactionMapping::BcastIndex1(s) => struct_to_bytes(&s, buf),
            BseBroadcastTransactionMapping::BcastIndex2(s) => struct_to_bytes(&s, buf),
            BseBroadcastTransactionMapping::BcastClosePrice(s) => struct_to_bytes(&s, buf),
            BseBroadcastTransactionMapping::BcastOpenInterestMsg(s) => struct_to_bytes(&s, buf),
            BseBroadcastTransactionMapping::BcastVarPercentage(s) => struct_to_bytes(&s, buf),
            BseBroadcastTransactionMapping::BcastAuctionMbp(s) => struct_to_bytes(&s, buf),
            BseBroadcastTransactionMapping::BcastMbp(s) => struct_to_bytes(&s, buf),
            BseBroadcastTransactionMapping::BcastMbpComplexInst(s) => struct_to_bytes(&s, buf),
            BseBroadcastTransactionMapping::BcastRbiRefRate(s) => struct_to_bytes(&s, buf),
            BseBroadcastTransactionMapping::BcastOddLotMbp(s) => struct_to_bytes(&s, buf),
            BseBroadcastTransactionMapping::BcastImpliedVolatility(s) => struct_to_bytes(&s, buf),
            BseBroadcastTransactionMapping::BcastKeepAlive(s) => struct_to_bytes(&s, buf),
            BseBroadcastTransactionMapping::BcastDebtMbp(s) => struct_to_bytes(&s, buf),
            BseBroadcastTransactionMapping::BcastLppRange(s) => struct_to_bytes(&s, buf),
            BseBroadcastTransactionMapping::BcastCallAuctionCxlQtyMsg(s) => {
                struct_to_bytes(&s, buf)
            }
        }
    }
}

#[repr(C, packed(2))]
#[derive(Debug, Twiddle, Clone, Copy)]
pub struct BcastTimeMessage {
    pub message_type: i32,
    pub reserved1: i32,
    pub reserved2: i32,
    pub reserved3: u16,
    pub hour: i16,
    pub minute: i16,
    pub econd: i16,
    pub milli_second: i16,
    pub reserved4: i16,
    pub reserved5: i16,
    pub reserved6: i16,
    pub reserved7: u8,
    pub reserved8: u8,
    pub reserved9: [u8; 2],
}

#[repr(C, packed(2))]
#[derive(Debug, Twiddle, Clone, Copy)]
pub struct BcastKeepAlive {
    pub message_type: i32,
}

#[repr(C, packed(2))]
#[derive(Debug, Twiddle, Clone, Copy)]
pub struct BcastSessionChange {
    pub message_type: i32,
    pub reserved1: i32,
    pub reserved2: i32,
    pub reserved3: u16,
    pub hour: i16,
    pub minute: i16,
    pub econd: i16,
    pub milli_second: i16,
    pub product_id: i16,
    pub reserved4: i16,
    pub filler: i16,
    pub market_type: i16,
    pub ession_number: i16,
    pub reserved5: i32,
    pub tart_end_flag: u8,
    pub reserved6: u8,
    pub reserved7: [u8; 2],
}

#[repr(C, packed(2))]
#[derive(Debug, Twiddle, Clone, Copy)]
pub struct BcastAuctionSessionChange {
    pub message_type: i32,
    pub reserved1: i32,
    pub reserved2: i32,
    pub reserved3: u16,
    pub hour: i16,
    pub minute: i16,
    pub econd: i16,
    pub milli_second: i16,
    pub reserved4: i16,
    pub reserved5: i16,
    pub filler: i16,
    pub reserved6: i16,
    pub ession_number: i16,
    pub reserved7: i32,
    pub reserved8: u8,
    pub reserved9: u8,
    pub reserved10: [u8; 2],
}

#[repr(C, packed(2))]
#[derive(Debug, Twiddle, Clone, Copy)]
pub struct BcastNewsHeadline {
    pub message_type: i32,
    pub reserved1: i32,
    pub reserved2: i32,
    pub reserved3: u16,
    pub hour: i16,
    pub minute: i16,
    pub econd: i16,
    pub milli_second: i16,
    pub reserved4: i16,
    pub reserved5: i16,
    pub news_category: i16,
    pub reserved7: i16,
    pub news_id: i32,
    pub news_headline: [u8; NEWS_HEADLINE_MSG_LEN],
    pub reserved8: u8,
    pub reserved9: u8,
    pub reserved10: [u8; 2],
}

#[repr(C, packed(2))]
#[derive(Debug, Twiddle, Clone, Copy, Default)]
pub struct BcastMBPData {
    pub best_bid_rate: i32,
    pub total_bid_qty: i32,
    pub no_of_bid_at_price_point: i32,
    pub implied_buy_qty: i32,
    pub best_offer_rate: i32,
    pub total_offer_qty: i32,
    pub no_of_offer_at_price_point: i32,
    pub implied_sell_qty: i32,
}

#[repr(C, packed(2))]
#[derive(Debug, Twiddle, Clone, Copy, Default)]
pub struct BcastMBPDetails {
    pub instrument: i32,
    pub no_of_trades: i32,
    pub traded_volume: i32,
    pub traded_value: i32,
    pub trade_value_flag: u8,
    pub reserved6: u8,
    pub reserved7: u8,
    pub reserved8: [u8; 1],
    pub market_type: i16,
    pub ession_number: i16,
    pub ltp_hour: u8,
    pub ltp_minute: u8,
    pub ltp_second: u8,
    pub ltp_milli_second: [u8; LTP_MILLI_SEC_LEN],
    pub reserved9: [u8; 2],
    pub reserved10: i16,
    pub no_of_price_points: i16,
    pub timestamp: i64,
    pub close_rate: i32,
    pub ltq: i32,
    pub ltp: i32,
    pub open_rate: i32,
    pub prev_close_rate: i32,
    pub high_rate: i32,
    pub low_rate: i32,
    pub block_deal_ref_rate: i32,
    pub indicative_equilibrium_price: i32,
    pub indicative_equilibrium_qty: i32,
    pub total_bid_qty: i32,
    pub total_offer_qty: i32,
    pub lower_price_band: i32,
    pub upper_price_band: i32,
    pub weighted_avg_price: i32,
    pub mbp_data: [BcastMBPData; MAX_BSE_MBP_DATA_IDX],
}

#[repr(C, packed(2))]
#[derive(Debug, Twiddle, Clone, Copy, Default)]
pub struct BcastMarketPicture {
    pub message_type: i32,
    pub reserved1: i32,
    pub reserved2: i32,
    pub reserved3: u16,
    pub hour: i16,
    pub minute: i16,
    pub econd: i16,
    pub milli_second: i16,
    pub reserved4: i16,
    pub reserved5: i16,
    pub no_of_records: i16,
    pub mbp_details: [BcastMBPDetails; MAX_BSE_MBP_DEATIL_IDX],
}

#[repr(C, packed(2))]
#[derive(Debug, Twiddle, Clone, Copy)]
pub struct BcastComplexMBPDetails {
    pub contract_code: i64,
    pub no_of_trades: i32,
    pub traded_volume: i32,
    pub traded_value: i32,
    pub trade_value_flag: u8,
    pub reserved6: u8,
    pub reserved7: u8,
    pub reserved8: [u8; 1],
    pub market_type: i16,
    pub ession_number: i16,
    pub ltp_hour: u8,
    pub ltp_minute: u8,
    pub ltp_second: u8,
    pub ltp_milli_second: [u8; LTP_MILLI_SEC_LEN],
    pub reserved9: [u8; 2],
    pub reserved10: i16,
    pub no_of_price_points: i16,
    pub timestamp: i64,
    pub close_rate: i32,
    pub ltq: i32,
    pub ltp: i32,
    pub open_rate: i32,
    pub prev_close_rate: i32,
    pub high_rate: i32,
    pub low_rate: i32,
    pub block_deal_ref_rate: i32,
    pub indicative_equilibrium_price: i32,
    pub indicative_equilibrium_qty: i32,
    pub total_bid_qty: i32,
    pub total_offer_qty: i32,
    pub lower_price_band: i32,
    pub upper_price_band: i32,
    pub weighted_avg_price: i32,
    pub mbp_data: [BcastMBPData; MAX_BSE_MBP_DATA_IDX],
}

#[repr(C, packed(2))]
#[derive(Debug, Twiddle, Clone, Copy)]
pub struct BcastComplexMarketPicture {
    pub message_type: i32,
    pub reserved1: i32,
    pub reserved2: i32,
    pub reserved3: u16,
    pub hour: i16,
    pub minute: i16,
    pub econd: i16,
    pub milli_second: i16,
    pub reserved4: i16,
    pub reserved5: i16,
    pub no_of_records: i16,
    pub mbp_details: [BcastComplexMBPDetails; MAX_BSE_MBP_DEATIL_IDX],
}

#[repr(C, packed(2))]
#[derive(Debug, Twiddle, Clone, Copy)]
pub struct BcastAuctionMBPData {
    pub likely_cut_off_rate: i32,
    pub offer_qty: i32,
}

#[repr(C, packed(2))]
#[derive(Debug, Twiddle, Clone, Copy)]
pub struct BcastAuctionMBPDetails {
    pub instrument: i32,
    pub reserved5: i32,
    pub auction_qty: i32,
    pub ceiling_price: i32,
    pub floor_price: i32,
    pub cut_off_rate: i32,
    pub lowest_offered_rate: i32,
    pub cumulative_qty: i32,
    pub reserved6: i32,
    pub reserved7: i16,
    pub reserved8: i16,
    pub reserved9: u8,
    pub reserved10: u8,
    pub reserved11: [u8; 1],
    pub reserved12: [u8; 1],
    pub mbp_data: [BcastAuctionMBPData; MAX_AUCTION_MBP_DATA_IDX],
}

#[repr(C, packed(2))]
#[derive(Debug, Twiddle, Clone, Copy)]
pub struct BcastAuctionMBP {
    pub message_type: i32,
    pub reserved1: i32,
    pub reserved2: i32,
    pub reserved3: u16,
    pub hour: i16,
    pub minute: i16,
    pub econd: i16,
    pub milli_second: i16,
    pub auction_number: i16,
    pub auction_trading_session: i16,
    pub no_of_records: i16,
    pub notice_number: [u8; NOTICE_NUMBER_LEN],
    pub reserved4: u8,
    pub mbp_details: [BcastAuctionMBPDetails; MAX_AUCTION_MBP_DETAILS_IDX],
}

#[repr(C, packed(2))]
#[derive(Debug, Twiddle, Clone, Copy)]
pub struct BcastOddLotMBPDetails {
    pub instrument: i32,
    pub open_rate: i32,
    pub prev_close_rate: i32,
    pub high_rate: i32,
    pub low_rate: i32,
    pub no_of_trades: i32,
    pub traded_volume: i64,
    pub traded_value: i32,
    pub ltq: i64,
    pub ltp: i32,
    pub close_rate: i32,
    pub trade_value_flag: u8,
    pub reserved7: u8,
    pub reserved8: u8,
    pub reserved9: u8,
    pub lower_price_band: i32,
    pub upper_price_band: i32,
    pub weighted_avg_price: i32,
    pub market_type: i16,
    pub ession_number: i16,
    pub ltp_hour: u8,
    pub ltp_minute: u8,
    pub ltp_second: u8,
    pub ltp_milli_second: [u8; LTP_MILLI_SEC_LEN],
    pub reserved10: [u8; 2],
}

#[repr(C, packed(2))]
#[derive(Debug, Twiddle, Clone, Copy)]
pub struct BcastOddLotMarketPicture {
    pub message_type: i32,
    pub reserved1: i32,
    pub reserved2: i32,
    pub reserved3: u16,
    pub hour: i16,
    pub minute: i16,
    pub econd: i16,
    pub milli_second: i16,
    pub reserved4: i16,
    pub reserved5: i16,
    pub no_of_records: i16,
    pub mbp_details: [BcastOddLotMBPDetails; MAX_BSE_MBP_DEATIL_IDX],
}

#[repr(C, packed(2))]
#[derive(Debug, Twiddle, Clone, Copy)]
pub struct BcastDebtMBPData {
    pub best_bid_rate: i32,
    pub total_bid_qty: i32,
    pub buy_ytm: i32,
    pub buy_ytp: i32,
    pub buy_ytc: i32,
    pub no_of_bid_at_price_point: i32,
    pub filler1: i32,
    pub best_offer_rate: i32,
    pub total_offer_qty: i32,
    pub sell_ytm: i32,
    pub sell_ytp: i32,
    pub sell_ytc: i32,
    pub no_of_offer_at_price_point: i32,
    pub filler2: i32,
}

#[repr(C, packed(2))]
#[derive(Debug, Twiddle, Clone, Copy)]
pub struct BcastDebtMBPDetails {
    pub instrument: i32,
    pub no_of_trades: i32,
    pub traded_volume: i32,
    pub traded_value: i32,
    pub trade_value_flag: u8,
    pub reserved6: u8,
    pub reserved7: u8,
    pub reserved8: [u8; 1],
    pub market_type: i16,
    pub ession_number: i16,
    pub ltp_hour: u8,
    pub ltp_minute: u8,
    pub ltp_second: u8,
    pub ltp_milli_second: [u8; LTP_MILLI_SEC_LEN],
    pub reserved9: [u8; 2],
    pub reserved10: i16,
    pub no_of_price_points: i16,
    pub timestamp: i64,
    pub close_rate: i32,
    pub ytm: i32,
    pub ytp: i32,
    pub ytc: i32,
    pub ltq: i32,
    pub ltp: i32,
    pub open_rate: i32,
    pub prev_close_rate: i32,
    pub high_rate: i32,
    pub low_rate: i32,
    pub reserved11: i32,
    pub indicative_equilibrium_price: i32,
    pub indicative_equilibrium_qty: i32,
    pub total_bid_qty: i32,
    pub total_offer_qty: i32,
    pub lower_price_band: i32,
    pub upper_price_band: i32,
    pub weighted_avg_price: i32,
    pub mbp_data: [BcastDebtMBPData; MAX_BSE_MBP_DATA_IDX],
}

#[repr(C, packed(2))]
#[derive(Debug, Twiddle, Clone, Copy)]
pub struct BcastDebtMarketPicture {
    pub message_type: i32,
    pub reserved1: i32,
    pub reserved2: i32,
    pub reserved3: u16,
    pub hour: i16,
    pub minute: i16,
    pub econd: i16,
    pub milli_second: i16,
    pub reserved4: i16,
    pub reserved5: i16,
    pub no_of_records: i16,
    pub mbp_details: [BcastDebtMBPDetails; MAX_BSE_MBP_DEATIL_IDX],
}

#[repr(C, packed(2))]
#[derive(Debug, Twiddle, Clone, Copy)]
pub struct BcastIndexDetails {
    pub index_code: i32,
    pub index_high: i32,
    pub index_low: i32,
    pub index_open: i32,
    pub prev_index_close: i32,
    pub index_value: i32,
    pub index_id: [u8; INDEX_ID_LEN],
    pub reserved6: u8,
    pub reserved7: u8,
    pub reserved8: u8,
    pub reserved9: [u8; 2],
    pub index_close_value_indicator: i16,
    pub reserved10: i16,
}

#[repr(C, packed(2))]
#[derive(Debug, Twiddle, Clone, Copy)]
pub struct BcastIndexChangeMessage {
    pub message_type: i32,
    pub reserved1: i32,
    pub reserved2: i32,
    pub reserved3: u16,
    pub hour: i16,
    pub minute: i16,
    pub econd: i16,
    pub milli_second: i16,
    pub reserved4: i16,
    pub reserved5: i16,
    pub no_of_records: i16,
    pub index_details: [BcastIndexDetails; MAX_BSE_INDEX_DEATIL_IDX],
}

#[repr(C, packed(2))]
#[derive(Debug, Twiddle, Clone, Copy)]
pub struct BcastLPPRangeDetails {
    pub instrument_code: i32,
    pub upper_limit_exec_price: i32,
    pub lower_limit_exec_price: i32,
    pub reserved6: i32,
    pub reserved7: i32,
}

#[repr(C, packed(2))]
#[derive(Debug, Twiddle, Clone, Copy)]
pub struct BcastLPPRange {
    pub message_type: i32,
    pub reserved1: i32,
    pub reserved2: i32,
    pub reserved3: u16,
    pub hour: i16,
    pub minute: i16,
    pub econd: i16,
    pub milli_second: i16,
    pub reserved4: i16,
    pub reserved5: i16,
    pub no_of_records: i16,
    pub lpp_range_details: [BcastLPPRangeDetails; MAX_BSE_LPP_RANGE_IDX],
}

#[repr(C, packed(2))]
#[derive(Debug, Twiddle, Clone, Copy)]
pub struct BcastCACxlQtyDetails {
    pub instrument_code: i32,
    pub cancelled_buy_qty: i32,
    pub cancelled_buy_orders: i32,
    pub cancelled_sell_qty: i32,
    pub cancelled_sell_orders: i32,
    pub reserved6: i32,
    pub reserved7: i32,
}

#[repr(C, packed(2))]
#[derive(Debug, Twiddle, Clone, Copy)]
pub struct BcastCACxlQtyMessage {
    pub message_type: i32,
    pub reserved1: i32,
    pub reserved2: i32,
    pub reserved3: u16,
    pub hour: i16,
    pub minute: i16,
    pub econd: i16,
    pub milli_second: i16,
    pub reserved4: i16,
    pub reserved5: i16,
    pub no_of_records: i16,
    pub ca_cxl_qty_details: [BcastCACxlQtyDetails; MAX_BSE_CA_CXL_QTY_IDX],
}

#[repr(C, packed(2))]
#[derive(Debug, Twiddle, Clone, Copy)]
pub struct BcastClosePriceDetails {
    pub instrument_code: i32,
    pub price: i32,
    pub reserved6: u8,
    pub traded_flag: u8,
    pub reserved7: u8,
    pub reserved8: u8,
}

#[repr(C, packed(2))]
#[derive(Debug, Twiddle, Clone, Copy)]
pub struct BcastClosePrice {
    pub message_type: i32,
    pub reserved1: i32,
    pub reserved2: i32,
    pub reserved3: u16,
    pub hour: i16,
    pub minute: i16,
    pub econd: i16,
    pub milli_second: i16,
    pub reserved4: i16,
    pub reserved5: i16,
    pub no_of_records: i16,
    pub close_price_details: [BcastClosePriceDetails; MAX_BSE_CLOSE_PRICE_IDX],
}

#[repr(C, packed(2))]
#[derive(Debug, Twiddle, Clone, Copy)]
pub struct BcastOpenInterestDetails {
    pub instrument_id: i32,
    pub open_interest_qty: i32,
    pub open_interest_value: i32,
    pub open_interest_change: i32,
    pub reserved6: [u8; 4],
    pub reserved7: i32,
    pub reserved8: i16,
    pub reserved9: i16,
    pub reserved10: u8,
    pub reserved11: u8,
    pub reserved12: [u8; 2],
}

#[repr(C, packed(2))]
#[derive(Debug, Twiddle, Clone, Copy)]
pub struct BcastOpenInterest {
    pub message_type: i32,
    pub reserved1: i32,
    pub reserved2: i32,
    pub reserved3: u16,
    pub hour: i16,
    pub minute: i16,
    pub econd: i16,
    pub milli_second: i16,
    pub reserved4: i16,
    pub reserved5: i16,
    pub no_of_records: i16,
    pub close_price_details: [BcastOpenInterestDetails; MAX_BSE_OPEN_INTEREST_IDX],
}

#[repr(C, packed(2))]
#[derive(Debug, Twiddle, Clone, Copy)]
pub struct BcastVarDetails {
    pub instrument_code: i32,
    pub varim_percene: i32,
    pub elmvar_percene: i32,
    pub reserved6: i32,
    pub reserved7: i16,
    pub reserved: i16,
    pub reserved8: u8,
    pub identifier: u8,
    pub reserved9: [u8; 2],
}

#[repr(C, packed(2))]
#[derive(Debug, Twiddle, Clone, Copy)]
pub struct BcastVarPercentage {
    pub message_type: i32,
    pub reserved1: i32,
    pub reserved2: i32,
    pub reserved3: u16,
    pub hour: i16,
    pub minute: i16,
    pub econd: i16,
    pub milli_second: i16,
    pub reserved4: i16,
    pub reserved5: i16,
    pub no_of_records: i16,
    pub var_details: [BcastVarDetails; MAX_BSE_VAR_IDX],
}

#[repr(C, packed(2))]
#[derive(Debug, Twiddle, Clone, Copy)]
pub struct BcastRBIReferenceRateDetails {
    pub underlying_assetid: i32,
    pub rbi_rate: i32,
    pub elmvar_percene: i32,
    pub reserved6: i32,
    pub reserved7: i16,
    pub date: [u8; DATE_LEN],
    pub filler: u8,
}

#[repr(C, packed(2))]
#[derive(Debug, Twiddle, Clone, Copy)]
pub struct BcastRBIReferenceRate {
    pub message_type: i32,
    pub reserved1: i32,
    pub reserved2: i32,
    pub reserved3: u16,
    pub hour: i16,
    pub minute: i16,
    pub econd: i16,
    pub milli_second: i16,
    pub reserved4: i16,
    pub reserved5: i16,
    pub no_of_records: i16,
    pub rbi_ref_rate_details: [BcastRBIReferenceRateDetails; MAX_BSE_VAR_IDX],
}

#[repr(C, packed(2))]
#[derive(Debug, Twiddle, Clone, Copy)]
pub struct BcastImpliedVolatilityDetails {
    pub intrument_id: i32,
    pub implied_volatility: i64,
    pub reserved6: i64,
    pub reserved7: i64,
    pub reserved8: i64,
    pub reserved9: i64,
    pub reserved10: i64,
    pub reserved11: i64,
    pub reserved12: i32,
    pub reserved13: i16,
    pub reserved14: i16,
    pub reserved15: u8,
    pub reserved16: u8,
    pub reserved17: [u8; 2],
}

#[repr(C, packed(2))]
#[derive(Debug, Twiddle, Clone, Copy)]
pub struct BcastImpliedVolatility {
    pub message_type: i32,
    pub reserved1: i32,
    pub reserved2: i32,
    pub reserved3: u16,
    pub hour: i16,
    pub minute: i16,
    pub econd: i16,
    pub milli_second: i16,
    pub reserved4: i16,
    pub reserved5: i16,
    pub no_of_records: i16,
    pub implied_volatility: [BcastImpliedVolatilityDetails; MAX_IMPLIED_VOLATILITY_IDX],
}
