use crate::constants::*;
use twiddler::Twiddle;

#[repr(C, packed(2))]
#[derive(Debug, Twiddle, Clone, Copy)]
pub struct BcastTimeMessage {
    message_type: i32,
    reserved1: i32,
    reserved2: i32,
    reserved3: u16,
    hour: i16,
    minute: i16,
    econd: i16,
    milli_second: i16,
    reserved4: i16,
    reserved5: i16,
    reserved6: i16,
    reserved7: u8,
    reserved8: u8,
    reserved9: [u8; 2],
}

#[repr(C, packed(2))]
#[derive(Debug, Twiddle, Clone, Copy)]
pub struct BcastKeepAlive {
    message_type: i32,
}

#[repr(C, packed(2))]
#[derive(Debug, Twiddle, Clone, Copy)]
pub struct BcastSessionChange {
    message_type: i32,
    reserved1: i32,
    reserved2: i32,
    reserved3: u16,
    hour: i16,
    minute: i16,
    econd: i16,
    milli_second: i16,
    product_id: i16,
    reserved4: i16,
    filler: i16,
    market_type: i16,
    ession_number: i16,
    reserved5: i32,
    tart_end_flag: u8,
    reserved6: u8,
    reserved7: [u8; 2],
}

#[repr(C, packed(2))]
#[derive(Debug, Twiddle, Clone, Copy)]
pub struct BcastAuctionSessionChange {
    message_type: i32,
    reserved1: i32,
    reserved2: i32,
    reserved3: u16,
    hour: i16,
    minute: i16,
    econd: i16,
    milli_second: i16,
    reserved4: i16,
    reserved5: i16,
    filler: i16,
    reserved6: i16,
    ession_number: i16,
    reserved7: i32,
    reserved8: u8,
    reserved9: u8,
    reserved10: [u8; 2],
}

#[repr(C, packed(2))]
#[derive(Debug, Twiddle, Clone, Copy)]
pub struct BcastNewsHeadline {
    message_type: i32,
    reserved1: i32,
    reserved2: i32,
    reserved3: u16,
    hour: i16,
    minute: i16,
    econd: i16,
    milli_second: i16,
    reserved4: i16,
    reserved5: i16,
    news_category: i16,
    reserved7: i16,
    news_id: i32,
    news_headline: [u8; NEWS_HEADLINE_MSG_LEN],
    reserved8: u8,
    reserved9: u8,
    reserved10: [u8; 2],
}

#[repr(C, packed(2))]
#[derive(Debug, Twiddle, Clone, Copy)]
pub struct BcastMBPData {
    best_bid_rate: i32,
    total_bid_qty: i32,
    no_of_bid_at_price_point: i32,
    implied_buy_qty: i32,
    best_offer_rate: i32,
    total_offer_qty: i32,
    no_of_offer_at_price_point: i32,
    implied_sell_qty: i32,
}

#[repr(C, packed(2))]
#[derive(Debug, Twiddle, Clone, Copy)]
pub struct BcastMBPDetails {
    instrument: i32,
    no_of_trades: i32,
    traded_volume: i32,
    traded_value: i32,
    trade_value_flag: u8,
    reserved6: u8,
    reserved7: u8,
    reserved8: [u8; 1],
    market_type: i16,
    ession_number: i16,
    ltp_hour: u8,
    ltp_minute: u8,
    ltp_second: u8,
    ltp_milli_second: [u8; LTP_MILLI_SEC_LEN],
    reserved9: [u8; 2],
    reserved10: i16,
    no_of_price_points: i16,
    timestamp: i64,
    close_rate: i32,
    ltq: i32,
    ltp: i32,
    open_rate: i32,
    prev_close_rate: i32,
    high_rate: i32,
    low_rate: i32,
    block_deal_ref_rate: i32,
    indicative_equilibrium_price: i32,
    indicative_equilibrium_qty: i32,
    total_bid_qty: i32,
    total_offer_qty: i32,
    lower_price_band: i32,
    upper_price_band: i32,
    weighted_avg_price: i32,
    mbp_data: [BcastMBPData; MAX_BSE_MBP_DATA_IDX],
}

#[repr(C, packed(2))]
#[derive(Debug, Twiddle, Clone, Copy)]
pub struct BcastMarketPicture {
    message_type: i32,
    reserved1: i32,
    reserved2: i32,
    reserved3: u16,
    hour: i16,
    minute: i16,
    econd: i16,
    milli_second: i16,
    reserved4: i16,
    reserved5: i16,
    no_of_records: i16,
    mbp_details: [BcastMBPDetails; MAX_BSE_MBP_DEATIL_IDX],
}

#[repr(C, packed(2))]
#[derive(Debug, Twiddle, Clone, Copy)]
pub struct BcastComplexMBPDetails {
    contract_code: i64,
    no_of_trades: i32,
    traded_volume: i32,
    traded_value: i32,
    trade_value_flag: u8,
    reserved6: u8,
    reserved7: u8,
    reserved8: [u8; 1],
    market_type: i16,
    ession_number: i16,
    ltp_hour: u8,
    ltp_minute: u8,
    ltp_second: u8,
    ltp_milli_second: [u8; LTP_MILLI_SEC_LEN],
    reserved9: [u8; 2],
    reserved10: i16,
    no_of_price_points: i16,
    timestamp: i64,
    close_rate: i32,
    ltq: i32,
    ltp: i32,
    open_rate: i32,
    prev_close_rate: i32,
    high_rate: i32,
    low_rate: i32,
    block_deal_ref_rate: i32,
    indicative_equilibrium_price: i32,
    indicative_equilibrium_qty: i32,
    total_bid_qty: i32,
    total_offer_qty: i32,
    lower_price_band: i32,
    upper_price_band: i32,
    weighted_avg_price: i32,
    mbp_data: [BcastMBPData; MAX_BSE_MBP_DATA_IDX],
}

#[repr(C, packed(2))]
#[derive(Debug, Twiddle, Clone, Copy)]
pub struct BcastComplexMarketPicture {
    message_type: i32,
    reserved1: i32,
    reserved2: i32,
    reserved3: u16,
    hour: i16,
    minute: i16,
    econd: i16,
    milli_second: i16,
    reserved4: i16,
    reserved5: i16,
    no_of_records: i16,
    mbp_details: [BcastComplexMBPDetails; MAX_BSE_MBP_DEATIL_IDX],
}

#[repr(C, packed(2))]
#[derive(Debug, Twiddle, Clone, Copy)]
pub struct BcastAuctionMBPData {
    likely_cut_off_rate: i32,
    offer_qty: i32,
}

#[repr(C, packed(2))]
#[derive(Debug, Twiddle, Clone, Copy)]
pub struct BcastAuctionMBPDetails {
    instrument: i32,
    reserved5: i32,
    auction_qty: i32,
    ceiling_price: i32,
    floor_price: i32,
    cut_off_rate: i32,
    lowest_offered_rate: i32,
    cumulative_qty: i32,
    reserved6: i32,
    reserved7: i16,
    reserved8: i16,
    reserved9: u8,
    reserved10: u8,
    reserved11: [u8; 1],
    reserved12: [u8; 1],
    mbp_data: [BcastAuctionMBPData; MAX_AUCTION_MBP_DATA_IDX],
}

#[repr(C, packed(2))]
#[derive(Debug, Twiddle, Clone, Copy)]
pub struct BcastAuctionMBP {
    message_type: i32,
    reserved1: i32,
    reserved2: i32,
    reserved3: u16,
    hour: i16,
    minute: i16,
    econd: i16,
    milli_second: i16,
    auction_number: i16,
    auction_trading_session: i16,
    no_of_records: i16,
    notice_number: [u8; NOTICE_NUMBER_LEN],
    reserved4: u8,
    mbp_details: [BcastAuctionMBPDetails; MAX_AUCTION_MBP_DETAILS_IDX],
}

#[repr(C, packed(2))]
#[derive(Debug, Twiddle, Clone, Copy)]
pub struct BcastOddLotMBPDetails {
    instrument: i32,
    open_rate: i32,
    prev_close_rate: i32,
    high_rate: i32,
    low_rate: i32,
    no_of_trades: i32,
    traded_volume: i64,
    traded_value: i32,
    ltq: i64,
    ltp: i32,
    close_rate: i32,
    trade_value_flag: u8,
    reserved7: u8,
    reserved8: u8,
    reserved9: u8,
    lower_price_band: i32,
    upper_price_band: i32,
    weighted_avg_price: i32,
    market_type: i16,
    ession_number: i16,
    ltp_hour: u8,
    ltp_minute: u8,
    ltp_second: u8,
    ltp_milli_second: [u8; LTP_MILLI_SEC_LEN],
    reserved10: [u8; 2],
}

#[repr(C, packed(2))]
#[derive(Debug, Twiddle, Clone, Copy)]
pub struct BcastOddLotMarketPicture {
    message_type: i32,
    reserved1: i32,
    reserved2: i32,
    reserved3: u16,
    hour: i16,
    minute: i16,
    econd: i16,
    milli_second: i16,
    reserved4: i16,
    reserved5: i16,
    no_of_records: i16,
    mbp_details: [BcastOddLotMBPDetails; MAX_BSE_MBP_DEATIL_IDX],
}

#[repr(C, packed(2))]
#[derive(Debug, Twiddle, Clone, Copy)]
pub struct BcastDebtMBPData {
    best_bid_rate: i32,
    total_bid_qty: i32,
    buy_ytm: i32,
    buy_ytp: i32,
    buy_ytc: i32,
    no_of_bid_at_price_point: i32,
    filler1: i32,
    best_offer_rate: i32,
    total_offer_qty: i32,
    ell_ytm: i32,
    ell_ytp: i32,
    ell_ytc: i32,
    no_of_offer_at_price_point: i32,
    filler2: i32,
}

#[repr(C, packed(2))]
#[derive(Debug, Twiddle, Clone, Copy)]
pub struct BcastDebtMBPDetails {
    instrument: i32,
    no_of_trades: i32,
    traded_volume: i32,
    traded_value: i32,
    trade_value_flag: u8,
    reserved6: u8,
    reserved7: u8,
    reserved8: [u8; 1],
    market_type: i16,
    ession_number: i16,
    ltp_hour: u8,
    ltp_minute: u8,
    ltp_second: u8,
    ltp_milli_second: [u8; LTP_MILLI_SEC_LEN],
    reserved9: [u8; 2],
    reserved10: i16,
    no_of_price_points: i16,
    timestamp: i64,
    close_rate: i32,
    ytm: i32,
    ytp: i32,
    ytc: i32,
    ltq: i32,
    ltp: i32,
    open_rate: i32,
    prev_close_rate: i32,
    high_rate: i32,
    low_rate: i32,
    reserved11: i32,
    indicative_equilibrium_price: i32,
    indicative_equilibrium_qty: i32,
    total_bid_qty: i32,
    total_offer_qty: i32,
    lower_price_band: i32,
    upper_price_band: i32,
    weighted_avg_price: i32,
    mbp_data: [BcastDebtMBPData; MAX_BSE_MBP_DATA_IDX],
}

#[repr(C, packed(2))]
#[derive(Debug, Twiddle, Clone, Copy)]
pub struct BcastDebtMarketPicture {
    message_type: i32,
    reserved1: i32,
    reserved2: i32,
    reserved3: u16,
    hour: i16,
    minute: i16,
    econd: i16,
    milli_second: i16,
    reserved4: i16,
    reserved5: i16,
    no_of_records: i16,
    mbp_details: [BcastDebtMBPDetails; MAX_BSE_MBP_DEATIL_IDX],
}

#[repr(C, packed(2))]
#[derive(Debug, Twiddle, Clone, Copy)]
pub struct BcastIndexDetails {
    index_code: i32,
    index_high: i32,
    index_low: i32,
    index_open: i32,
    prev_index_close: i32,
    index_value: i32,
    index_id: [u8; INDEX_ID_LEN],
    reserved6: u8,
    reserved7: u8,
    reserved8: u8,
    reserved9: [u8; 2],
    index_close_value_indicator: i16,
    reserved10: i16,
}

#[repr(C, packed(2))]
#[derive(Debug, Twiddle, Clone, Copy)]
pub struct BcastIndexChangeMessage {
    message_type: i32,
    reserved1: i32,
    reserved2: i32,
    reserved3: u16,
    hour: i16,
    minute: i16,
    econd: i16,
    milli_second: i16,
    reserved4: i16,
    reserved5: i16,
    no_of_records: i16,
    index_details: [BcastIndexDetails; MAX_BSE_INDEX_DEATIL_IDX],
}

#[repr(C, packed(2))]
#[derive(Debug, Twiddle, Clone, Copy)]
pub struct BcastLPPRangeDetails {
    instrument_code: i32,
    upper_limit_exec_price: i32,
    lower_limit_exec_price: i32,
    reserved6: i32,
    reserved7: i32,
}

#[repr(C, packed(2))]
#[derive(Debug, Twiddle, Clone, Copy)]
pub struct BcastLPPRange {
    message_type: i32,
    reserved1: i32,
    reserved2: i32,
    reserved3: u16,
    hour: i16,
    minute: i16,
    econd: i16,
    milli_second: i16,
    reserved4: i16,
    reserved5: i16,
    no_of_records: i16,
    lpp_range_details: [BcastLPPRangeDetails; MAX_BSE_LPP_RANGE_IDX],
}

#[repr(C, packed(2))]
#[derive(Debug, Twiddle, Clone, Copy)]
pub struct BcastCACxlQtyDetails {
    instrument_code: i32,
    cancelled_buy_qty: i32,
    cancelled_buy_orders: i32,
    cancelled_sell_qty: i32,
    cancelled_sell_orders: i32,
    reserved6: i32,
    reserved7: i32,
}

#[repr(C, packed(2))]
#[derive(Debug, Twiddle, Clone, Copy)]
pub struct BcastCACxlQtyMessage {
    message_type: i32,
    reserved1: i32,
    reserved2: i32,
    reserved3: u16,
    hour: i16,
    minute: i16,
    econd: i16,
    milli_second: i16,
    reserved4: i16,
    reserved5: i16,
    no_of_records: i16,
    ca_cxl_qty_details: [BcastCACxlQtyDetails; MAX_BSE_CA_CXL_QTY_IDX],
}

#[repr(C, packed(2))]
#[derive(Debug, Twiddle, Clone, Copy)]
pub struct BcastClosePriceDetails {
    instrument_code: i32,
    price: i32,
    reserved6: u8,
    traded_flag: u8,
    reserved7: u8,
    reserved8: u8,
}

#[repr(C, packed(2))]
#[derive(Debug, Twiddle, Clone, Copy)]
pub struct BcastClosePrice {
    message_type: i32,
    reserved1: i32,
    reserved2: i32,
    reserved3: u16,
    hour: i16,
    minute: i16,
    econd: i16,
    milli_second: i16,
    reserved4: i16,
    reserved5: i16,
    no_of_records: i16,
    close_price_details: [BcastClosePriceDetails; MAX_BSE_CLOSE_PRICE_IDX],
}

#[repr(C, packed(2))]
#[derive(Debug, Twiddle, Clone, Copy)]
pub struct BcastOpenInterestDetails {
    instrument_id: i32,
    open_interest_qty: i32,
    open_interest_value: i32,
    open_interest_change: i32,
    reserved6: [u8; 4],
    reserved7: i32,
    reserved8: i16,
    reserved9: i16,
    reserved10: u8,
    reserved11: u8,
    reserved12: [u8; 2],
}

#[repr(C, packed(2))]
#[derive(Debug, Twiddle, Clone, Copy)]
pub struct BcastOpenInterest {
    message_type: i32,
    reserved1: i32,
    reserved2: i32,
    reserved3: u16,
    hour: i16,
    minute: i16,
    econd: i16,
    milli_second: i16,
    reserved4: i16,
    reserved5: i16,
    no_of_records: i16,
    close_price_details: [BcastOpenInterestDetails; MAX_BSE_OPEN_INTEREST_IDX],
}

#[repr(C, packed(2))]
#[derive(Debug, Twiddle, Clone, Copy)]
pub struct BcastVarDetails {
    instrument_code: i32,
    varim_percene: i32,
    elmvar_percene: i32,
    reserved6: i32,
    reserved7: i16,
    reserved: i16,
    reserved8: u8,
    identifier: u8,
    reserved9: [u8; 2],
}

#[repr(C, packed(2))]
#[derive(Debug, Twiddle, Clone, Copy)]
pub struct BcastVarPercene {
    message_type: i32,
    reserved1: i32,
    reserved2: i32,
    reserved3: u16,
    hour: i16,
    minute: i16,
    econd: i16,
    milli_second: i16,
    reserved4: i16,
    reserved5: i16,
    no_of_records: i16,
    var_details: [BcastVarDetails; MAX_BSE_VAR_IDX],
}

#[repr(C, packed(2))]
#[derive(Debug, Twiddle, Clone, Copy)]
pub struct BcastRBIReferenceRateDetails {
    underlying_assetid: i32,
    rbi_rate: i32,
    elmvar_percene: i32,
    reserved6: i32,
    reserved7: i16,
    date: [u8; DATE_LEN],
    filler: u8,
}

#[repr(C, packed(2))]
#[derive(Debug, Twiddle, Clone, Copy)]
pub struct BcastRBIReferenceRate {
    message_type: i32,
    reserved1: i32,
    reserved2: i32,
    reserved3: u16,
    hour: i16,
    minute: i16,
    econd: i16,
    milli_second: i16,
    reserved4: i16,
    reserved5: i16,
    no_of_records: i16,
    rbi_ref_rate_details: [BcastRBIReferenceRateDetails; MAX_BSE_VAR_IDX],
}

#[repr(C, packed(2))]
#[derive(Debug, Twiddle, Clone, Copy)]
pub struct BcastImpliedVolatilityDetails {
    intrument_id: i32,
    implied_volatility: i64,
    reserved6: i64,
    reserved7: i64,
    reserved8: i64,
    reserved9: i64,
    reserved10: i64,
    reserved11: i64,
    reserved12: i32,
    reserved13: i16,
    reserved14: i16,
    reserved15: u8,
    reserved16: u8,
    reserved17: [u8; 2],
}

#[repr(C, packed(2))]
#[derive(Debug, Twiddle, Clone, Copy)]
pub struct BcastImpliedVolatility {
    message_type: i32,
    reserved1: i32,
    reserved2: i32,
    reserved3: u16,
    hour: i16,
    minute: i16,
    econd: i16,
    milli_second: i16,
    reserved4: i16,
    reserved5: i16,
    no_of_records: i16,
    implied_volatility: [BcastImpliedVolatilityDetails; MAX_IMPLIED_VOLATILITY_IDX],
}
