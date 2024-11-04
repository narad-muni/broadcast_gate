use crate::{
    constants::*,
    utils::byte_utils::{bytes_to_struct, struct_to_bytes},
};
use twiddler::Twiddle;

#[derive(Debug, Twiddle)]
pub enum NfoBroadcastTransactionMapping {
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
    BcastLimitPriceProtectionRange(BcastLPPRange),
}

pub fn build_nfo_struct(transaction_id: i16, buf: &[u8]) -> Option<NfoBroadcastTransactionMapping> {
    match transaction_id {
        5294 => Some(NfoBroadcastTransactionMapping::BcastContMsg(
            bytes_to_struct(buf),
        )),
        6013 => Some(NfoBroadcastTransactionMapping::BcastSecurityOpenPrice(
            bytes_to_struct(buf),
        )),
        6501 => Some(NfoBroadcastTransactionMapping::BcastJrnlVctMsg(
            bytes_to_struct(buf),
        )),
        6511 => Some(NfoBroadcastTransactionMapping::BcastOpenMessage(
            bytes_to_struct(buf),
        )),
        6521 => Some(NfoBroadcastTransactionMapping::BcastCloseMessage(
            bytes_to_struct(buf),
        )),
        6531 => Some(NfoBroadcastTransactionMapping::BcastPreopenShutdownMsg(
            bytes_to_struct(buf),
        )),
        6541 => Some(NfoBroadcastTransactionMapping::BcastCircuitCheck(
            bytes_to_struct(buf),
        )),
        6571 => Some(NfoBroadcastTransactionMapping::BcastNormalMktPreopenEnded(
            bytes_to_struct(buf),
        )),
        7130 => Some(NfoBroadcastTransactionMapping::BcastMktMvmtCmOiIn(
            bytes_to_struct(buf),
        )),
        7200 => Some(NfoBroadcastTransactionMapping::BcastMboMbpUpdate(
            bytes_to_struct(buf),
        )),
        7201 => Some(NfoBroadcastTransactionMapping::BcastMwRoundRobin(
            bytes_to_struct(buf),
        )),
        7202 => Some(NfoBroadcastTransactionMapping::BcastTickerAndMktIndex(
            bytes_to_struct(buf),
        )),
        7203 => Some(NfoBroadcastTransactionMapping::BcastIndustryIndexUpdate(
            bytes_to_struct(buf),
        )),
        7206 => Some(NfoBroadcastTransactionMapping::BcastSystemInformationOut(
            bytes_to_struct(buf),
        )),
        7208 => Some(NfoBroadcastTransactionMapping::BcastOnlyMbp(
            bytes_to_struct(buf),
        )),
        7210 => Some(
            NfoBroadcastTransactionMapping::BcastSecurityStatusChgPreopen(bytes_to_struct(buf)),
        ),
        7211 => Some(NfoBroadcastTransactionMapping::BcastSpdMbpDelta(
            bytes_to_struct(buf),
        )),
        7220 => Some({
            NfoBroadcastTransactionMapping::BcastLimitPriceProtectionRange(bytes_to_struct(buf))
        }),
        7305 => Some(NfoBroadcastTransactionMapping::BcastSecurityMstrChg(
            bytes_to_struct(buf),
        )),
        7306 => Some(NfoBroadcastTransactionMapping::BcastPartMstrChg(
            bytes_to_struct(buf),
        )),
        7320 => Some(NfoBroadcastTransactionMapping::BcastSecurityStatusChg(
            bytes_to_struct(buf),
        )),
        7324 => Some(NfoBroadcastTransactionMapping::BcastInstrMstrChg(
            bytes_to_struct(buf),
        )),
        7340 => Some(NfoBroadcastTransactionMapping::BcastSecMstrChngPeriodic(
            bytes_to_struct(buf),
        )),
        7341 => Some(NfoBroadcastTransactionMapping::BcastSpdMstrChgPeriodic(
            bytes_to_struct(buf),
        )),
        9010 => Some(NfoBroadcastTransactionMapping::BcastTurnoverExceeded(
            bytes_to_struct(buf),
        )),
        9011 => Some(NfoBroadcastTransactionMapping::BcastBrokerReactivated(
            bytes_to_struct(buf),
        )),
        _ => {
            println!("Invalid fao transaction id: {}", transaction_id);
            None
        }
    }
}

impl NfoBroadcastTransactionMapping {
    pub fn to_bytes(&self, buffer: &mut [u8]) {
        match self {
            NfoBroadcastTransactionMapping::BcastContMsg(s) => struct_to_bytes(s, buffer),
            NfoBroadcastTransactionMapping::BcastSecurityOpenPrice(s) => struct_to_bytes(s, buffer),
            NfoBroadcastTransactionMapping::BcastJrnlVctMsg(s) => struct_to_bytes(s, buffer),
            NfoBroadcastTransactionMapping::BcastAssetUpdtIntRateChg(s) => {
                struct_to_bytes(s, buffer)
            }
            NfoBroadcastTransactionMapping::BcastOpenMessage(s) => struct_to_bytes(s, buffer),
            NfoBroadcastTransactionMapping::BcastCloseMessage(s) => struct_to_bytes(s, buffer),
            NfoBroadcastTransactionMapping::BcastPostcloseMsg(s) => struct_to_bytes(s, buffer),
            NfoBroadcastTransactionMapping::BcastPreopenShutdownMsg(s) => {
                struct_to_bytes(s, buffer)
            }
            NfoBroadcastTransactionMapping::BcastNormalMktPreopenEnded(s) => {
                struct_to_bytes(s, buffer)
            }
            NfoBroadcastTransactionMapping::BcastCircuitCheck(s) => struct_to_bytes(s, buffer),
            NfoBroadcastTransactionMapping::BcastMktMvmtCmOiIn(s) => struct_to_bytes(s, buffer),
            NfoBroadcastTransactionMapping::BcastMboMbpUpdate(s) => struct_to_bytes(s, buffer),
            NfoBroadcastTransactionMapping::BcastMwRoundRobin(s) => struct_to_bytes(s, buffer),
            NfoBroadcastTransactionMapping::BcastTickerAndMktIndex(s) => struct_to_bytes(s, buffer),
            NfoBroadcastTransactionMapping::BcastIndustryIndexUpdate(s) => {
                struct_to_bytes(s, buffer)
            }
            NfoBroadcastTransactionMapping::BcastSystemInformationOut(s) => {
                struct_to_bytes(s, buffer)
            }
            NfoBroadcastTransactionMapping::BcastOnlyMbp(s) => struct_to_bytes(s, buffer),
            NfoBroadcastTransactionMapping::BcastSpdMbpDelta(s) => struct_to_bytes(s, buffer),
            NfoBroadcastTransactionMapping::BcastCurrencyAssets(s) => struct_to_bytes(s, buffer),
            NfoBroadcastTransactionMapping::BcastInterestAssets(s) => struct_to_bytes(s, buffer),
            NfoBroadcastTransactionMapping::BcastQtyMbaDelta(s) => struct_to_bytes(s, buffer),
            NfoBroadcastTransactionMapping::BcastPriceMbaDelta(s) => struct_to_bytes(s, buffer),
            NfoBroadcastTransactionMapping::BcastTradeExecutionRange(s) => {
                struct_to_bytes(s, buffer)
            }
            NfoBroadcastTransactionMapping::BcastSecurityMstrChg(s) => struct_to_bytes(s, buffer),
            NfoBroadcastTransactionMapping::BcastSecMstrChngPeriodic(s) => {
                struct_to_bytes(s, buffer)
            }
            NfoBroadcastTransactionMapping::BcastPartMstrChg(s) => struct_to_bytes(s, buffer),
            NfoBroadcastTransactionMapping::BcastSecurityStatusChgPreopen(s) => {
                struct_to_bytes(s, buffer)
            }
            NfoBroadcastTransactionMapping::BcastSecurityStatusChg(s) => struct_to_bytes(s, buffer),
            NfoBroadcastTransactionMapping::BcastInstrMstrChg(s) => struct_to_bytes(s, buffer),
            NfoBroadcastTransactionMapping::BcastSpdMstrChgPeriodic(s) => {
                struct_to_bytes(s, buffer)
            }
            NfoBroadcastTransactionMapping::BcastTurnoverExceeded(s) => struct_to_bytes(s, buffer),
            NfoBroadcastTransactionMapping::BcastBrokerReactivated(s) => struct_to_bytes(s, buffer),
            NfoBroadcastTransactionMapping::BcastLimitPriceProtectionRange(s) => {
                struct_to_bytes(s, buffer)
            }
        };
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

#[repr(C, packed(2))]
#[derive(Debug, Twiddle, Clone, Copy)]
pub struct BcastMBAData {
    mba_buy: BcastMBABuy,
    mba_sell: BcastMBASell,
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
pub struct BcastAssestUpdateInterestRateInfo {
    bcast_header: BcastHeaders,
    token: i32,
    foreign_interest_rate: i16,
    volatility: i32,
    domestii32erest_rate: i16,
}

#[repr(C, packed(2))]
#[derive(Debug, Twiddle, Clone, Copy)]
pub struct BcastContMsg {
    bcast_header: BcastHeaders,
    tream_number: i16,
    tatus: i16,
    reserved: [u8; 200],
}

#[repr(C, packed(2))]
#[derive(Debug, Twiddle, Clone, Copy)]
pub struct BcastSecurityEligibilityIndicators {
    aon: u8,
    minimum_fill: u8,
    books_merged: u8,
    reserved1: u8,
    reserved2: u8,
}

#[repr(C, packed(2))]
#[derive(Debug, Twiddle, Clone, Copy)]
pub struct BcastStockEligibilityIndicators {
    // aon: u8, 1 bit
    // minimum_fill: u8, 1 bit
    // books_merged: u8, 1 bit
    reserved1: u8,
    reserved2: u8,
}

#[repr(C, packed(2))]
#[derive(Debug, Twiddle, Clone, Copy)]
pub struct BcastMktStatus {
    normal: i16,
    oddlot: i16,
    pot: i16,
    auction: i16,
}

#[repr(C, packed(2))]
#[derive(Debug, Twiddle, Clone, Copy)]
pub struct BcastEXMktStatus {
    normal: i16,
    oddlot: i16,
    pot: i16,
    auction: i16,
}

#[repr(C, packed(2))]
#[derive(Debug, Twiddle, Clone, Copy)]
pub struct BcastPLMktStatus {
    normal: i16,
    oddlot: i16,
    pot: i16,
    auction: i16,
}

#[repr(C, packed(2))]
#[derive(Debug, Twiddle, Clone, Copy)]
pub struct BcastSystemInfoData {
    bcast_header: BcastHeaders,
    market_status: BcastMktStatus,
    ex_market_status: BcastEXMktStatus,
    pl_market_status: BcastPLMktStatus,
    update_portfolio: u8,
    market_index: i32,
    default_period_normal: i16,
    default_period_spot: i16,
    default_period_auction: i16,
    competitor_period: i16,
    olicitor_period: i16,
    warning_percent: i16,
    volume_freeze_percent: i16,
    nap_quote_time: i16,
    reserved: [u8; 2],
    board_lot_quantity: i32,
    tick_size: i32,
    maximum_gtc_days: i16,
    tock_eligible_indicators: BcastStockEligibilityIndicators,
    disclosed_quantity_percent_allowed: i16,
    risk_free_interest_rate: i32,
}

#[repr(C, packed(2))]
#[derive(Debug, Twiddle, Clone, Copy)]
pub struct BcastDestination {
    // trader_ws: u8, 1 bit
    // control_ws: u8, 1 bit
    // tandem: u8, 1 bit
    // journalling_required: u8, 1 bit
    reserved1: u8,
    reserved2: u8,
}

#[repr(C, packed(2))]
#[derive(Debug, Twiddle, Clone, Copy)]
pub struct BcastJournalMessage {
    bcast_header: BcastHeaders,
    branch_number: i16,
    broker_number: [u8; BROKERNUMBER_LEN],
    action_code: [u8; ACTIONCODE_LEN],
    bcast_destination: BcastDestination,
    reserved1: [u8; 26],
    bcast_msg_length: i16,
    bcast_message: [u8; BCAST_MSG_LEN - 1],
}

#[repr(C, packed(2))]
#[derive(Debug, Twiddle, Clone, Copy)]
pub struct BcastSECInfo {
    instrument_name: [u8; 6],
    symbol: [u8; SYMBOL_LEN],
    series: [u8; SERIES_LEN],
    expiry_date: i32,
    trike_price: i32,
    option_type: [u8; 2],
    ca_level: u8,
}

#[repr(C, packed(2))]
#[derive(Debug, Twiddle, Clone, Copy)]
pub struct BcastSecurityEligibilityPerMarket {
    // eligibility: u8, 1 bit
    reserved: u8,
    tatus: i16,
}

#[repr(C, packed(2))]
#[derive(Debug, Twiddle, Clone, Copy)]
pub struct BcastEligibilityIndicators {
    // participate_in_market_index: u8, 1 bit
    // aon: u8, 1 bit
    // minimum_fill: u8, 1 bit
    reserved1: u8,
    reserved2: u8,
}

#[repr(C, packed(2))]
#[derive(Debug, Twiddle, Clone, Copy)]
pub struct BcastPurpose {
    // dividend: u8, 1 bit
    // rights: u8, 1 bit
    // bonus: u8, 1 bit
    // interest: u8, 1 bit
    // agm: u8, 1 bit
    // egm: u8, 1 bit
    // reserved1: u8, 1 bit
    // exercise_style: u8, 1 bit
    // ex_allowed: u8, 1 bit
    // ex_rejection_allowed: u8, 1 bit
    // pl_allowed: u8, 1 bit
    // is_this_asset: u8, 1 bit
    // is_corporate_adjusted: u8, 1 bit
    reserved1: u8,
    reserved2: u8,
}

#[repr(C, packed(2))]
#[derive(Debug, Twiddle, Clone, Copy)]
pub struct BcastSecurityUpdateInfo {
    bcast_header: BcastHeaders,
    token: i32,
    ec_info: BcastSECInfo,
    permitted_to_trade: i16,
    lf_issued_capital: f64,
    warning_qty: i32,
    freeze_qty: i32,
    credit_rating: [u8; CREDITRATING_LEN_12],
    eligibility_per_market: [BcastSecurityEligibilityPerMarket; 4],
    issue_rate: i16,
    issue_start_date: i32,
    interest_payment_date: i32,
    issue_maturity_date: i32,
    margin_percene: i32,
    minimum_lot_quantity: i32,
    board_lot_quantity: i32,
    tick_size: i32,
    name: [u8; REMARKS_LEN],
    reserved2: u8,
    listing_date: i32,
    expulsion_date: i32,
    re_admission_date: i32,
    record_date: i32,
    low_price_range: i32,
    high_price_range: i32,
    expiry_date: i32,
    no_delivery_start_date: i32,
    no_delivery_end_date: i32,
    eligibility_indicators: BcastEligibilityIndicators,
    book_closure_start_date: i32,
    book_closure_end_date: i32,
    exercise_start_date: i32,
    exercise_end_date: i32,
    old_token: i32,
    asset_instrument: [u8; ASSET_INSTRUMENT_LEN],
    asset_name: [u8; ASSET_NAME_LEN],
    asset_token: i32,
    intrinsic_value: i32,
    extrinsic_value: i32,
    purpose: BcastPurpose,
    local_update_date_time: i32,
    delete_flag: u8,
    remark: [u8; REMARKS_LEN],
    base_price: i32,
}

#[repr(C, packed(2))]
#[derive(Debug, Twiddle, Clone, Copy)]
pub struct BcastInstrumentUpdateInfo {
    bcast_header: BcastHeaders,
    instrument_id: i16,
    instrument_name: [u8; INSTRUMENT_NAME_LEN],
    instrument_description: [u8; INSTRUMENT_DESC_LEN],
    instrument_update_time: i32,
    delete_flag: u8,
}

#[repr(C, packed(2))]
#[derive(Debug, Twiddle, Clone, Copy)]
pub struct BcastParticipantUpdateInfo {
    bcast_header: BcastHeaders,
    participant_id: [u8; PARTICIPANT_ID_LEN],
    participant_name: [u8; PARTICIPANT_NAME_LEN],
    participant_status: u8,
    participant_update_date_time: i32,
    delete_flag: u8,
}

#[repr(C, packed(2))]
#[derive(Debug, Twiddle, Clone, Copy)]
pub struct BcastSecurityStatusPerMarket {
    tatus: i16,
}

#[repr(C, packed(2))]
#[derive(Debug, Twiddle, Clone, Copy)]
pub struct BcastTokenAndEligibility {
    token: i32,
    tatus_per_market: [BcastSecurityStatusPerMarket; MAX_SEC_STATUS_PERMARKET_IDX - 2],
}

#[repr(C, packed(2))]
#[derive(Debug, Twiddle, Clone, Copy)]
pub struct BcastSecurityStatusUpdateInfo {
    bcast_header: BcastHeaders,
    number_of_records: i16,
    eligibility: [BcastTokenAndEligibility; MAX_TOKEN_ELIGIBILITY_IDX + 10],
}

#[repr(C, packed(2))]
#[derive(Debug, Twiddle, Clone, Copy)]
pub struct BcastLimitExceeded {
    bcast_header: BcastHeaders,
    broker_code: [u8; BROKERCODE_LEN],
    counter_broker_code: [u8; COUNTER_BROKERCODE_LEN],
    warning_type: i16,
    token: i32,
    instrument_name: [u8; 6],
    symbol: [u8; 10],
    expiry_date: i32,
    trike_price: i32,
    option_type: [u8; 2],
    ca_level: [u8; 2],
    trade_number: i32,
    trade_price: i32,
    trade_volume: i32,
    _final: u8,
    filler: u8,
}

#[repr(C, packed(2))]
#[derive(Debug, Twiddle, Clone, Copy)]
pub struct BcastVCTMessages {
    bcast_header: BcastHeaders,
    token: i32,
    ec_info: BcastSECInfo,
    market_type: i16,
    bcast_dest: BcastDestination,
    broadcast_message_length: i16,
    bcast_message: [u8; BCAST_MSG_LEN - 1],
}

#[repr(C, packed(2))]
#[derive(Debug, Twiddle, Clone, Copy)]
pub struct BcastTickerIndexInfo {
    token: i32,
    market_type: i16,
    fill_price: i32,
    fill_volume: i32,
    open_interest: u32,
    day_hi_oi: u32,
    day_lo_oi: u32,
}

#[repr(C, packed(2))]
#[derive(Debug, Twiddle, Clone, Copy)]
pub struct BcastTickerTradeData {
    bcast_header: BcastHeaders,
    number_of_records: i16,
    ticker_index_info: [BcastTickerIndexInfo; MAX_TICKER_INDEX_INFO_IDX - 11],
}

#[repr(C, packed(2))]
#[derive(Debug, Twiddle, Clone, Copy)]
pub struct BcastMBOMBPTerms {
    // mf: u8, 1 bit
    // aon: u8, 1 bit
    reserved1: u8,
    reserved: u8,
}

#[repr(C, packed(2))]
#[derive(Debug, Twiddle, Clone, Copy)]
pub struct BcastMBOInfo {
    trader_id: i32,
    qty: i32,
    price: i32,
    mbombp_terms: BcastMBOMBPTerms,
    min_fill_qty: i32,
}

#[repr(C, packed(2))]
#[derive(Debug, Twiddle, Clone, Copy)]
pub struct BcastInteractiveMBOData {
    pub token: i32,
    pub book_type: i16,
    pub trading_status: i16,
    pub volume_traded_today: u32,
    pub last_traded_price: i32,
    pub net_change_indicator: u8,
    pub net_price_change_from_closing_price: i32,
    pub last_trade_quantity: i32,
    pub last_trade_time: i64,
    pub average_trade_price: i32,
    pub auction_number: i16,
    pub auction_status: i16,
    pub initiator_type: i16,
    pub initiator_price: i32,
    pub initiator_quantity: i32,
    pub auction_price: i32,
    pub auction_quantity: i32,
    pub mbo_info: [BcastMBOInfo; MAX_MBOINFO_IDX], // 10
}

#[repr(C, packed(2))]
#[derive(Debug, Twiddle, Clone, Copy)]
pub struct BcastMBPInfo {
    pub qty: i32,
    pub price: i32,
    pub number_of_orders: i16,
    pub bb_buy_sell_flag: i16,
}

#[repr(C, packed(2))]
#[derive(Debug, Twiddle, Clone, Copy)]
pub struct BcastMBPIndicator {
    // buy: u8, 1 bit
    // sell: u8, 1 bit
    // last_trade_more: u8, 1 bit
    // last_trade_less: u8, 1 bit
    reserved1: u8,
    reserved2: u8,
}

#[repr(C, packed(2))]
#[derive(Debug, Twiddle, Clone, Copy)]
pub struct BcastInteractiveMBPData {
    pub token: i32,
    pub book_type: i16,
    pub trading_status: i16,
    pub volume_traded_today: u32,
    pub last_traded_price: i32,
    pub net_change_indicator: u8,
    pub net_price_change_from_closing_price: i32,
    pub last_trade_quantity: i32,
    pub last_trade_time: i64,
    pub average_trade_price: i32,
    pub auction_number: i16,
    pub auction_status: i16,
    pub initiator_type: i16,
    pub initiator_price: i32,
    pub initiator_quantity: i32,
    pub auction_price: i32,
    pub auction_quantity: i32,
    pub mbp_info: [BcastMBPInfo; MAX_MBPINFO_IDX],
    pub bb_total_buy_flag: i16,
    pub bb_total_sell_flag: i16,
    pub total_buy_quantity: f64,
    pub total_sell_quantity: f64,
    pub mbp_indicator: BcastMBPIndicator,
    pub closing_price: i32,
    pub open_price: i32,
    pub high_price: i32,
    pub low_price: i32,
}

#[repr(C, packed(2))]
#[derive(Debug, Twiddle, Clone, Copy)]
pub struct BcastOnlyMBP {
    pub bcast_header: BcastHeaders,
    pub no_of_records: i16,
    pub mbp_data: [BcastInteractiveMBPData; MAX_MBP_DATA_IDX], // 2
}

#[repr(C, packed(2))]
#[derive(Debug, Twiddle, Clone, Copy)]
pub struct BcastMBOMBPIndicator {
    // last_trade_more: u8,
    // last_trade_less: u8,
    // buy: u8,
    // sell: u8,
    reserved1: u8,
    reserved2: u8,
}

#[repr(C, packed(2))]
#[derive(Debug, Twiddle, Clone, Copy)]
pub struct BcastMBOMBP {
    pub bcast_header: BcastHeaders,
    pub mbo_data: BcastInteractiveMBOData,
    pub mbp_info: [BcastMBPInfo; MAX_MBPINFO_IDX], // 10
    pub lf_total_buy_quantity: f64,
    pub lf_total_sell_quantity: f64,
    pub mbombp_indicator: BcastMBOMBPIndicator,
    pub closing_price: i32,
    pub open_price: i32,
    pub high_price: i32,
    pub low_price: i32,
}

#[repr(C, packed(2))]
#[derive(Debug, Twiddle, Clone, Copy)]
pub struct BcastMarketWiseInfo {
    mbombp_indicator: BcastMBOMBPIndicator,
    buy_volume: i32,
    buy_price: i32,
    ell_volume: i32,
    ell_price: i32,
    last_trade_price: i32,
    last_trade_time: i32,
}

#[repr(C, packed(2))]
#[derive(Debug, Twiddle, Clone, Copy)]
pub struct BcastMarketWatch {
    token: i32,
    market_wise_info: [BcastMarketWiseInfo; MAX_MARKET_WISE_INFO_IDX], // 3
    open_interest: u32,
}

#[repr(C, packed(2))]
#[derive(Debug, Twiddle, Clone, Copy)]
pub struct BcastInquiryResponse {
    bcast_header: BcastHeaders,
    number_of_records: i16,
    market_watch: [BcastMarketWatch; MAX_MARKET_WATCH_IDX], // 5
}

#[repr(C, packed(2))]
#[derive(Debug, Twiddle, Clone, Copy)]
pub struct BcastSecurityOpenMessage {
    bcast_header: BcastHeaders,
    symbol: [u8; SYMBOL_LEN], // 10
    series: [u8; SERIES_LEN],
    token: i32,
    opening_price: i32,
    reserved: u8,
}

#[repr(C, packed(2))]
#[derive(Debug, Twiddle, Clone, Copy)]
pub struct NFOIndices {
    index_name: [u8; 21],
    index_value: i32,
    high_index_value: i32,
    low_index_value: i32,
    opening_index: i32,
    closing_index: i32,
    percent_change: i32,
    yearly_high: i32,
    yearly_low: i32,
    no_of_upmoves: i32,
    no_of_downmoves: i32,
    lf_market_capitalisation: f64,
    net_change_indicator: u8,
    reserved: u8,
}

#[repr(C, packed(2))]
#[derive(Debug, Twiddle, Clone, Copy)]
pub struct BcastIndices {
    bcast_header: BcastHeaders,
    number_of_records: i16,
    nfo_indices: [NFOIndices; MAX_INDICES_IDX],
}

#[repr(C, packed(2))]
#[derive(Debug, Twiddle, Clone, Copy)]
pub struct IndustryIndices {
    industry_name: [u8; 15],
    index_value: i32,
}

#[repr(C, packed(2))]
#[derive(Debug, Twiddle, Clone, Copy)]
pub struct BcastIndustryIndices {
    bcast_header: BcastHeaders,
    number_of_records: i16,
    indices: [IndustryIndices; MAX_INDUSTRY_INDICES_IDX],
}

#[repr(C, packed(2))]
#[derive(Debug, Twiddle, Clone, Copy)]
pub struct MBPBuys {
    no_orders: i16,
    volume: i32,
    price: i32,
}

#[repr(C, packed(2))]
#[derive(Debug, Twiddle, Clone, Copy)]
pub struct MBPSells {
    no_orders: i16,
    volume: i32,
    price: i32,
}

#[repr(C, packed(2))]
#[derive(Debug, Twiddle, Clone, Copy)]
pub struct TotalOrderVolume {
    lf_buy: f64,
    lf_sell: f64,
}

#[repr(C, packed(2))]
#[derive(Debug, Twiddle, Clone, Copy)]
pub struct BcastSpreadMarketInfo {
    bcast_header: BcastHeaders,
    token1: i32,
    token2: i32,
    mbp_buy: i16,
    mbp_sell: i16,
    last_active_time: i32,
    traded_volume: u32,
    lf_total_traded_value: f64,
    mbp_buys: [MBPBuys; MAX_MARKET_WATCH_IDX],
    mbp_sells: [MBPSells; MAX_MARKET_WATCH_IDX],
    total_order_volume: TotalOrderVolume,
    open_price_difference: i32,
    day_high_price_difference: i32,
    day_low_price_difference: i32,
    last_traded_price_difference: i32,
    last_update_time: i32,
}

#[repr(C, packed(2))]
#[derive(Debug, Twiddle, Clone, Copy)]
pub struct BcastSpreadEligibility {
    reserved: u8,
    // eligibility: u8, 1 bit
}

#[repr(C, packed(2))]
#[derive(Debug, Twiddle, Clone, Copy)]
pub struct BcastSpreadUpdateInfo {
    bcast_header: BcastHeaders,
    token1: i32,
    token2: i32,
    ec_info1: BcastSECInfo,
    ec_info2: BcastSECInfo,
    reference_price: i32,
    day_low_price_diff_range: i32,
    day_high_price_diff_range: i32,
    op_low_price_diff_range: i32,
    op_high_price_diff_range: i32,
    eligibility: BcastSpreadEligibility,
    reserved1: u8,
    delete_flag: u8,
    reserved2: u8,
}

#[repr(C, packed(2))]
#[derive(Debug, Twiddle, Clone, Copy)]
pub struct OpenInterest {
    token: i32,
    current_io: u32,
}

#[repr(C, packed(2))]
#[derive(Debug, Twiddle, Clone, Copy)]
pub struct BcastCMAssestOI {
    reserved1: [u8; 4],
    log_time: i32,
    market_type: [u8; 2],
    transaction_code: i16,
    no_of_records: i16,
    reserved2: [u8; 8],
    time_stamp: i64,
    reserved3: [u8; 8],
    message_length: i16,
    open_interest: [OpenInterest; MAX_ASSET_OI_IDX],
}

#[repr(C, packed(2))]
#[derive(Debug, Twiddle, Clone, Copy)]
pub struct BcastLPPRangeDetails {
    token: i32,
    high_exec_band: i32,
    low_exec_band: i32,
}

#[repr(C, packed(2))]
#[derive(Debug, Twiddle, Clone, Copy)]
pub struct BcastLPPRangeData {
    msg_count: i32,
    lpp_range_details: [BcastLPPRangeDetails; MAX_LPP_RANGE_INDX],
}

#[repr(C, packed(2))]
#[derive(Debug, Twiddle, Clone, Copy)]
pub struct BcastLPPRange {
    bcast_header: BcastHeaders,
    lpp_range_data: BcastLPPRangeData,
}
