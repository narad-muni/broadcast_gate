use crate::{
    constants::*,
    utils::byte_utils::{bytes_to_struct, struct_to_bytes},
};
use twiddler::Twiddle;

use super::nfo::{
    BcastCMAssestOI, BcastContMsg, BcastIndustryIndices, BcastInquiryResponse,
    BcastInstrumentUpdateInfo, BcastJournalMessage, BcastLimitExceeded, BcastMBOMBP, BcastOnlyMBP,
    BcastParticipantUpdateInfo, BcastSecurityOpenMessage, BcastSecurityStatusUpdateInfo,
    BcastSecurityUpdateInfo, BcastSpreadMarketInfo, BcastSpreadUpdateInfo, BcastSystemInfoData,
    BcastTickerTradeData, BcastVCTMessages,
};

pub enum NcdBroadcastTransactionMapping {
    BcastContMsg(BcastContMsg),
    BcastSecurityOpenPrice(BcastSecurityOpenMessage),
    BcastJrnlVctMsg(BcastJournalMessage),
    BcastAssetUpdtIntRateChg(BcastAssestUpdateInterestRateInfo),
    BcastOpenMessage(BcastVCTMessages),
    BcastCloseMessage(BcastVCTMessages),
    BcastPostcloseMsg(BcastVCTMessages),
    BcastPreopenShutdownMsg(BcastVCTMessages),
    BcastNormalMktPreopenEnded(BcastVCTMessages),
    BcastCircuitCheck(BcastHeaders),
    BcastMktMvmtCmOiIn(BcastCMAssestOI),
    BcastMboMbpUpdate(BcastMBOMBP),
    BcastMwRoundRobin(BcastInquiryResponse),
    BcastTickerAndMktIndex(BcastTickerTradeData),
    BcastIndustryIndexUpdate(BcastIndustryIndices),
    BcastSystemInformationOut(BcastSystemInfoData),
    BcastOnlyMbp(BcastOnlyMBP),
    BcastSpdMbpDelta(BcastSpreadMarketInfo),
    BcastCurrencyAssets(BcastAssetData),
    BcastInterestAssets(BcastAssetsMBPInfo),
    BcastQtyMbaDelta(BcastQtyMBADelta),
    BcastPriceMbaDelta(BcastPriceMBADelta),
    BcastTradeExecutionRange(BcastTradeExecRange),
    BcastSecurityMstrChg(BcastSecurityUpdateInfo),
    BcastSecMstrChngPeriodic(BcastSecurityUpdateInfo),
    BcastPartMstrChg(BcastParticipantUpdateInfo),
    BcastSecurityStatusChgPreopen(BcastSecurityStatusUpdateInfo),
    BcastSecurityStatusChg(BcastSecurityStatusUpdateInfo),
    BcastInstrMstrChg(BcastInstrumentUpdateInfo),
    BcastSpdMstrChgPeriodic(BcastSpreadUpdateInfo),
    BcastTurnoverExceeded(BcastLimitExceeded),
    BcastBrokerReactivated(BcastLimitExceeded),
}

pub fn build_ncd_struct(transaction_id: i16, buf: &[u8]) -> NcdBroadcastTransactionMapping {
    match transaction_id {
        5294 => NcdBroadcastTransactionMapping::BcastContMsg(bytes_to_struct(&buf)),
        6013 => NcdBroadcastTransactionMapping::BcastSecurityOpenPrice(bytes_to_struct(&buf)),
        6501 => NcdBroadcastTransactionMapping::BcastJrnlVctMsg(bytes_to_struct(&buf)),
        6503 => NcdBroadcastTransactionMapping::BcastAssetUpdtIntRateChg(bytes_to_struct(&buf)),
        6511 => NcdBroadcastTransactionMapping::BcastOpenMessage(bytes_to_struct(&buf)),
        6521 => NcdBroadcastTransactionMapping::BcastCloseMessage(bytes_to_struct(&buf)),
        6522 => NcdBroadcastTransactionMapping::BcastPostcloseMsg(bytes_to_struct(&buf)),
        6531 => NcdBroadcastTransactionMapping::BcastPreopenShutdownMsg(bytes_to_struct(&buf)),
        6541 => NcdBroadcastTransactionMapping::BcastCircuitCheck(bytes_to_struct(&buf)),
        6571 => NcdBroadcastTransactionMapping::BcastNormalMktPreopenEnded(bytes_to_struct(&buf)),
        7130 => NcdBroadcastTransactionMapping::BcastMktMvmtCmOiIn(bytes_to_struct(&buf)),
        7200 => NcdBroadcastTransactionMapping::BcastMboMbpUpdate(bytes_to_struct(&buf)),
        7201 => NcdBroadcastTransactionMapping::BcastMwRoundRobin(bytes_to_struct(&buf)),
        7202 => NcdBroadcastTransactionMapping::BcastTickerAndMktIndex(bytes_to_struct(&buf)),
        7203 => NcdBroadcastTransactionMapping::BcastIndustryIndexUpdate(bytes_to_struct(&buf)),
        7206 => NcdBroadcastTransactionMapping::BcastSystemInformationOut(bytes_to_struct(&buf)),
        7208 => NcdBroadcastTransactionMapping::BcastOnlyMbp(bytes_to_struct(&buf)),
        7210 => {
            NcdBroadcastTransactionMapping::BcastSecurityStatusChgPreopen(bytes_to_struct(&buf))
        }
        7211 => NcdBroadcastTransactionMapping::BcastSpdMbpDelta(bytes_to_struct(&buf)),
        7213 => NcdBroadcastTransactionMapping::BcastCurrencyAssets(bytes_to_struct(&buf)),
        7214 => NcdBroadcastTransactionMapping::BcastInterestAssets(bytes_to_struct(&buf)),
        7215 => NcdBroadcastTransactionMapping::BcastQtyMbaDelta(bytes_to_struct(&buf)),
        7216 => NcdBroadcastTransactionMapping::BcastPriceMbaDelta(bytes_to_struct(&buf)),
        7220 => NcdBroadcastTransactionMapping::BcastTradeExecutionRange(bytes_to_struct(&buf)),
        7305 => NcdBroadcastTransactionMapping::BcastSecurityMstrChg(bytes_to_struct(&buf)),
        7306 => NcdBroadcastTransactionMapping::BcastPartMstrChg(bytes_to_struct(&buf)),
        7320 => NcdBroadcastTransactionMapping::BcastSecurityStatusChg(bytes_to_struct(&buf)),
        7324 => NcdBroadcastTransactionMapping::BcastInstrMstrChg(bytes_to_struct(&buf)),
        7340 => NcdBroadcastTransactionMapping::BcastSecMstrChngPeriodic(bytes_to_struct(&buf)),
        7341 => NcdBroadcastTransactionMapping::BcastSpdMstrChgPeriodic(bytes_to_struct(&buf)),
        9010 => NcdBroadcastTransactionMapping::BcastTurnoverExceeded(bytes_to_struct(&buf)),
        9011 => NcdBroadcastTransactionMapping::BcastBrokerReactivated(bytes_to_struct(&buf)),
        _ => panic!("Invalid ncd transaction id: {}", transaction_id),
    }
}

impl NcdBroadcastTransactionMapping {
    pub fn twiddle(&mut self) {
        match self {
            NcdBroadcastTransactionMapping::BcastContMsg(s) => s.twiddle(),
            NcdBroadcastTransactionMapping::BcastSecurityOpenPrice(s) => s.twiddle(),
            NcdBroadcastTransactionMapping::BcastJrnlVctMsg(s) => s.twiddle(),
            NcdBroadcastTransactionMapping::BcastAssetUpdtIntRateChg(s) => s.twiddle(),
            NcdBroadcastTransactionMapping::BcastOpenMessage(s) => s.twiddle(),
            NcdBroadcastTransactionMapping::BcastCloseMessage(s) => s.twiddle(),
            NcdBroadcastTransactionMapping::BcastPostcloseMsg(s) => s.twiddle(),
            NcdBroadcastTransactionMapping::BcastPreopenShutdownMsg(s) => s.twiddle(),
            NcdBroadcastTransactionMapping::BcastNormalMktPreopenEnded(s) => s.twiddle(),
            NcdBroadcastTransactionMapping::BcastCircuitCheck(s) => s.twiddle(),
            NcdBroadcastTransactionMapping::BcastMktMvmtCmOiIn(s) => s.twiddle(),
            NcdBroadcastTransactionMapping::BcastMboMbpUpdate(s) => s.twiddle(),
            NcdBroadcastTransactionMapping::BcastMwRoundRobin(s) => s.twiddle(),
            NcdBroadcastTransactionMapping::BcastTickerAndMktIndex(s) => s.twiddle(),
            NcdBroadcastTransactionMapping::BcastIndustryIndexUpdate(s) => s.twiddle(),
            NcdBroadcastTransactionMapping::BcastSystemInformationOut(s) => s.twiddle(),
            NcdBroadcastTransactionMapping::BcastOnlyMbp(s) => s.twiddle(),
            NcdBroadcastTransactionMapping::BcastSpdMbpDelta(s) => s.twiddle(),
            NcdBroadcastTransactionMapping::BcastCurrencyAssets(s) => s.twiddle(),
            NcdBroadcastTransactionMapping::BcastInterestAssets(s) => s.twiddle(),
            NcdBroadcastTransactionMapping::BcastQtyMbaDelta(s) => s.twiddle(),
            NcdBroadcastTransactionMapping::BcastPriceMbaDelta(s) => s.twiddle(),
            NcdBroadcastTransactionMapping::BcastTradeExecutionRange(s) => s.twiddle(),
            NcdBroadcastTransactionMapping::BcastSecurityMstrChg(s) => s.twiddle(),
            NcdBroadcastTransactionMapping::BcastSecMstrChngPeriodic(s) => s.twiddle(),
            NcdBroadcastTransactionMapping::BcastPartMstrChg(s) => s.twiddle(),
            NcdBroadcastTransactionMapping::BcastSecurityStatusChgPreopen(s) => s.twiddle(),
            NcdBroadcastTransactionMapping::BcastSecurityStatusChg(s) => s.twiddle(),
            NcdBroadcastTransactionMapping::BcastInstrMstrChg(s) => s.twiddle(),
            NcdBroadcastTransactionMapping::BcastSpdMstrChgPeriodic(s) => s.twiddle(),
            NcdBroadcastTransactionMapping::BcastTurnoverExceeded(s) => s.twiddle(),
            NcdBroadcastTransactionMapping::BcastBrokerReactivated(s) => s.twiddle(),
        };
    }

    pub fn to_bytes(&self, buffer: &mut [u8]) {
        match self {
            NcdBroadcastTransactionMapping::BcastContMsg(s) => struct_to_bytes(s, buffer),
            NcdBroadcastTransactionMapping::BcastSecurityOpenPrice(s) => struct_to_bytes(s, buffer),
            NcdBroadcastTransactionMapping::BcastJrnlVctMsg(s) => struct_to_bytes(s, buffer),
            NcdBroadcastTransactionMapping::BcastAssetUpdtIntRateChg(s) => {
                struct_to_bytes(s, buffer)
            }
            NcdBroadcastTransactionMapping::BcastOpenMessage(s) => struct_to_bytes(s, buffer),
            NcdBroadcastTransactionMapping::BcastCloseMessage(s) => struct_to_bytes(s, buffer),
            NcdBroadcastTransactionMapping::BcastPostcloseMsg(s) => struct_to_bytes(s, buffer),
            NcdBroadcastTransactionMapping::BcastPreopenShutdownMsg(s) => {
                struct_to_bytes(s, buffer)
            }
            NcdBroadcastTransactionMapping::BcastNormalMktPreopenEnded(s) => {
                struct_to_bytes(s, buffer)
            }
            NcdBroadcastTransactionMapping::BcastCircuitCheck(s) => struct_to_bytes(s, buffer),
            NcdBroadcastTransactionMapping::BcastMktMvmtCmOiIn(s) => struct_to_bytes(s, buffer),
            NcdBroadcastTransactionMapping::BcastMboMbpUpdate(s) => struct_to_bytes(s, buffer),
            NcdBroadcastTransactionMapping::BcastMwRoundRobin(s) => struct_to_bytes(s, buffer),
            NcdBroadcastTransactionMapping::BcastTickerAndMktIndex(s) => struct_to_bytes(s, buffer),
            NcdBroadcastTransactionMapping::BcastIndustryIndexUpdate(s) => {
                struct_to_bytes(s, buffer)
            }
            NcdBroadcastTransactionMapping::BcastSystemInformationOut(s) => {
                struct_to_bytes(s, buffer)
            }
            NcdBroadcastTransactionMapping::BcastOnlyMbp(s) => struct_to_bytes(s, buffer),
            NcdBroadcastTransactionMapping::BcastSpdMbpDelta(s) => struct_to_bytes(s, buffer),
            NcdBroadcastTransactionMapping::BcastCurrencyAssets(s) => struct_to_bytes(s, buffer),
            NcdBroadcastTransactionMapping::BcastInterestAssets(s) => struct_to_bytes(s, buffer),
            NcdBroadcastTransactionMapping::BcastQtyMbaDelta(s) => struct_to_bytes(s, buffer),
            NcdBroadcastTransactionMapping::BcastPriceMbaDelta(s) => struct_to_bytes(s, buffer),
            NcdBroadcastTransactionMapping::BcastTradeExecutionRange(s) => {
                struct_to_bytes(s, buffer)
            }
            NcdBroadcastTransactionMapping::BcastSecurityMstrChg(s) => struct_to_bytes(s, buffer),
            NcdBroadcastTransactionMapping::BcastSecMstrChngPeriodic(s) => {
                struct_to_bytes(s, buffer)
            }
            NcdBroadcastTransactionMapping::BcastPartMstrChg(s) => struct_to_bytes(s, buffer),
            NcdBroadcastTransactionMapping::BcastSecurityStatusChgPreopen(s) => {
                struct_to_bytes(s, buffer)
            }
            NcdBroadcastTransactionMapping::BcastSecurityStatusChg(s) => struct_to_bytes(s, buffer),
            NcdBroadcastTransactionMapping::BcastInstrMstrChg(s) => struct_to_bytes(s, buffer),
            NcdBroadcastTransactionMapping::BcastSpdMstrChgPeriodic(s) => {
                struct_to_bytes(s, buffer)
            }
            NcdBroadcastTransactionMapping::BcastTurnoverExceeded(s) => struct_to_bytes(s, buffer),
            NcdBroadcastTransactionMapping::BcastBrokerReactivated(s) => struct_to_bytes(s, buffer),
        }
    }
}

#[repr(C, packed(2))]
#[derive(Debug, Twiddle, Clone, Copy)]
pub struct BcastHeaders {
    pub reserved1: [u8; 4],
    pub log_time: i32,
    pub alpha_char: [u8; 2],
    pub trans_code: i16,
    pub error_code: i16,
    pub bc_seq_no: i32,
    pub reserved2: [u8; 4],
    pub time_stamp2: [u8; 8],
    pub filler2: [u8; 8],
    pub message_length: i16,
}

#[repr(C, packed(2))]
#[derive(Debug, Twiddle, Clone, Copy)]
pub struct BcastAssestUpdateInterestRateInfo {
    bcast_header: BcastHeaders,
    token: i32,
    foreign_interest_rate: i16,
    volatility: i32,
    domestii32erest_rate: i16,
}

#[repr(C, packed(2))]
#[derive(Debug, Twiddle, Clone, Copy)]
pub struct BcastTradeExecRangeDetails {
    token: i32,
    high_exec_band: i32,
    low_exec_band: i32,
}

#[repr(C, packed(2))]
#[derive(Debug, Twiddle, Clone, Copy)]
pub struct BcastTradeExecRangeData {
    msg_count: i32,
    trade_exec_range_details: [BcastTradeExecRangeDetails; MAX_TRADE_EXEC_RANGE_DETAILS_IDX],
}

#[repr(C, packed(2))]
#[derive(Debug, Twiddle, Clone, Copy)]
pub struct BcastTradeExecRange {
    bcast_header: BcastHeaders,
    trade_exec_range_data: BcastTradeExecRangeData,
}

#[repr(C, packed(2))]
#[derive(Debug, Twiddle, Clone, Copy)]
pub struct BcastAssetData {
    bcast_header: BcastHeaders,
    token: i32,
    bid_price: i32,
    ask_price: i32,
    deal_price: i32,
    symbol: [u8; SYMBOL_LEN],
    instrument: [u8; INSTRUMENT_NAME_LEN],
    closing_price: i32,
}

#[repr(C, packed(2))]
#[derive(Debug, Twiddle, Clone, Copy)]
pub struct BcastMBPBuy {
    order_count: i32,
    order_price: i32,
    yield_rate: i32,
    lf_order_qunatity: f64,
}

#[repr(C, packed(2))]
#[derive(Debug, Twiddle, Clone, Copy)]
pub struct BcastMBPSell {
    order_count: i32,
    order_price: i32,
    yield_rate: i32,
    lf_order_qunatity: f64,
}

#[repr(C, packed(2))]
#[derive(Debug, Twiddle, Clone, Copy)]
pub struct BcastAssetsMBPInfo {
    bcast_header: BcastHeaders,
    token: i32,
    symbol: [u8; SYMBOL_LEN],
    mbp_buy: [BcastMBPBuy; MAX_ASSEST_MBP_INFO_IDX],
    mbp_sell: [BcastMBPSell; MAX_ASSEST_MBP_INFO_IDX],
    book_type: i16,
    intrument_type: [u8; INSTRUMENT_NAME_LEN],
}

#[repr(C, packed(2))]
#[derive(Debug, Twiddle, Clone, Copy)]
pub struct BcastMBABuy {
    cum_qty_of_orders: i32,
    wap: i32,
    lf_total_value: f64,
    impact_cost: i32,
    order_price: i32,
}

#[repr(C, packed(2))]
#[derive(Debug, Twiddle, Clone, Copy)]
pub struct BcastMBASell {
    cum_qty_of_orders: i32,
    wap: i32,
    lf_total_value: f64,
    impact_cost: i32,
    order_price: i32,
}

#[repr(C, packed(2))]
#[derive(Debug, Twiddle, Clone, Copy)]
pub struct BcastMBAData {
    mba_buy: BcastMBABuy,
    mba_sell: BcastMBASell,
}

#[repr(C, packed(2))]
#[derive(Debug, Twiddle, Clone, Copy)]
pub struct BcastQtyMBADelta {
    bcast_header: BcastHeaders,
    token: i32,
    noof_records: i32,
    market_buy_price: i32,
    market_sell_price: i32,
    ideal_price: i32,
    mba_data: [BcastMBAData; MAX_MBA_DATA_IDX],
}

#[repr(C, packed(2))]
#[derive(Debug, Twiddle, Clone, Copy)]
pub struct BcastPriceMBADelta {
    bcast_header: BcastHeaders,
    token: i32,
    noof_records: i32,
    market_buy_price: i32,
    market_sell_price: i32,
    ideal_price: i32,
    mba_data: [BcastMBAData; MAX_MBA_DATA_IDX],
}
