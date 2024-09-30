use crate::{constants::*, utils::byte_utils::bytes_to_struct};
use twiddler::Twiddle;

#[derive(Debug)]
pub enum NeqBroadcastTransactionMapping {
    BcastContMsg(BcastContMsg),
    BcastJrnlVctMsg(BcastJournalMessage),
    BcastOpenMessage(BcastVCTMessages),
    BcastCloseMessage(BcastVCTMessages),
    BcastPreopenShutdownMsg(BcastVCTMessages),
    BcastNormalMktPreopenEnded(BcastVCTMessages),
    BcastAuctionStatusChange(BcastAuctionStatusChange),
    BcastMboMbpCedtc(BcastMBOMBP),
    BcastMwRoundRobinCedtc(BcastInquiryResponse),
    BcastTickerAndMktIndex(BcastTickerTradeData),
    BcastSystemInformationOut(BcastSystemInfoData),
    BcastOnlyMbpCedtc(BcastOnlyMBPCEDTC),
    BcastCallAuctionOrdCxlUpdate(BcastCAOrdCxUpdate),
    BcastCallAuctionMbpCedtc(BcastCallAuctionMBP),
    BcastCaMwCedtc(BcastCallAuctionMW),
    BcastIndices(BcastIndices),
    BcastIndicesVix(BcastIndices),
    BcastPartMstrChg(BcastParticipantMasterUpdateInfo),
    BcastSymbolStatusChangeAction(BcastSymbolStatusChangeAction),
    BcastIndicativeIndices(BcastIndices),
    BcastTurnoverExceeded(BcastLimitExceeded),
    BcastBrokerReactivated(BcastLimitExceeded),
    BcastMarketStatsReportDataCedtcH(BcastReportHdr),
    BcastMarketStatsReportDataCedtcR(BcastReportMktStatsData),
    BcastAuctionInquiryOut(BcastAuctionINQData),
    BcastSecurityStatusChg(BcastSecurityStatusUpdateInfo),
    BcastSecurityStatusChgPreopen(BcastSecurityStatusUpdateInfo),
    BcastOnlyMbp(BcastOnlyMBP),
    BcastBuyBack(BcastBuyBack),
    BcastSecurityMstrChg(BcastSecurityMasterUpdateInfo),
}

pub fn build_neq_struct(transaction_id: i16, buf: &[u8]) -> NeqBroadcastTransactionMapping {
    match transaction_id {
        5294 => NeqBroadcastTransactionMapping::BcastContMsg(bytes_to_struct(buf)),
        6501 => NeqBroadcastTransactionMapping::BcastJrnlVctMsg(bytes_to_struct(buf)),
        6511 => NeqBroadcastTransactionMapping::BcastOpenMessage(bytes_to_struct(buf)),
        6521 => NeqBroadcastTransactionMapping::BcastCloseMessage(bytes_to_struct(buf)),
        6531 => NeqBroadcastTransactionMapping::BcastPreopenShutdownMsg(bytes_to_struct(buf)),
        6571 => NeqBroadcastTransactionMapping::BcastNormalMktPreopenEnded(bytes_to_struct(buf)),
        6581 => NeqBroadcastTransactionMapping::BcastAuctionStatusChange(bytes_to_struct(buf)),
        7200 => NeqBroadcastTransactionMapping::BcastMboMbpCedtc(bytes_to_struct(buf)),
        7201 => NeqBroadcastTransactionMapping::BcastMwRoundRobinCedtc(bytes_to_struct(buf)),
        7206 => NeqBroadcastTransactionMapping::BcastSystemInformationOut(bytes_to_struct(buf)),
        7207 => NeqBroadcastTransactionMapping::BcastIndices(bytes_to_struct(buf)),
        7208 => NeqBroadcastTransactionMapping::BcastOnlyMbpCedtc(bytes_to_struct(buf)),
        7210 => NeqBroadcastTransactionMapping::BcastCallAuctionOrdCxlUpdate(bytes_to_struct(buf)),
        7214 => NeqBroadcastTransactionMapping::BcastCallAuctionMbpCedtc(bytes_to_struct(buf)),
        7215 => NeqBroadcastTransactionMapping::BcastCaMwCedtc(bytes_to_struct(buf)),
        7216 => NeqBroadcastTransactionMapping::BcastIndicesVix(bytes_to_struct(buf)),
        7306 => NeqBroadcastTransactionMapping::BcastPartMstrChg(bytes_to_struct(buf)),
        7764 => NeqBroadcastTransactionMapping::BcastSymbolStatusChangeAction(bytes_to_struct(buf)),
        8207 => NeqBroadcastTransactionMapping::BcastIndicativeIndices(bytes_to_struct(buf)),
        9010 => NeqBroadcastTransactionMapping::BcastTurnoverExceeded(bytes_to_struct(buf)),
        9011 => NeqBroadcastTransactionMapping::BcastBrokerReactivated(bytes_to_struct(buf)),
        18130 => NeqBroadcastTransactionMapping::BcastSecurityStatusChg(bytes_to_struct(buf)),
        18201 => {
            NeqBroadcastTransactionMapping::BcastMarketStatsReportDataCedtcR(bytes_to_struct(buf))
        }
        18700 => NeqBroadcastTransactionMapping::BcastAuctionInquiryOut(bytes_to_struct(buf)),
        18703 => NeqBroadcastTransactionMapping::BcastTickerAndMktIndex(bytes_to_struct(buf)),
        18705 => NeqBroadcastTransactionMapping::BcastOnlyMbp(bytes_to_struct(buf)),
        18707 => {
            NeqBroadcastTransactionMapping::BcastSecurityStatusChgPreopen(bytes_to_struct(buf))
        }
        18708 => NeqBroadcastTransactionMapping::BcastBuyBack(bytes_to_struct(buf)),
        18720 => NeqBroadcastTransactionMapping::BcastSecurityMstrChg(bytes_to_struct(buf)),
        _ => panic!("Invalid eq transaction id: {}", transaction_id),
    }
}

impl NeqBroadcastTransactionMapping {
    pub fn twiddle(&mut self) {
        match self {
            NeqBroadcastTransactionMapping::BcastContMsg(s) => s.twiddle(),
            NeqBroadcastTransactionMapping::BcastJrnlVctMsg(s) => s.twiddle(),
            NeqBroadcastTransactionMapping::BcastOpenMessage(s) => s.twiddle(),
            NeqBroadcastTransactionMapping::BcastCloseMessage(s) => s.twiddle(),
            NeqBroadcastTransactionMapping::BcastPreopenShutdownMsg(s) => s.twiddle(),
            NeqBroadcastTransactionMapping::BcastNormalMktPreopenEnded(s) => s.twiddle(),
            NeqBroadcastTransactionMapping::BcastAuctionStatusChange(s) => s.twiddle(),
            NeqBroadcastTransactionMapping::BcastMboMbpCedtc(s) => s.twiddle(),
            NeqBroadcastTransactionMapping::BcastMwRoundRobinCedtc(s) => s.twiddle(),
            NeqBroadcastTransactionMapping::BcastTickerAndMktIndex(s) => s.twiddle(),
            NeqBroadcastTransactionMapping::BcastSystemInformationOut(s) => s.twiddle(),
            NeqBroadcastTransactionMapping::BcastOnlyMbpCedtc(s) => s.twiddle(),
            NeqBroadcastTransactionMapping::BcastCallAuctionOrdCxlUpdate(s) => s.twiddle(),
            NeqBroadcastTransactionMapping::BcastCallAuctionMbpCedtc(s) => s.twiddle(),
            NeqBroadcastTransactionMapping::BcastCaMwCedtc(s) => s.twiddle(),
            NeqBroadcastTransactionMapping::BcastIndices(s) => s.twiddle(),
            NeqBroadcastTransactionMapping::BcastIndicesVix(s) => s.twiddle(),
            NeqBroadcastTransactionMapping::BcastPartMstrChg(s) => s.twiddle(),
            NeqBroadcastTransactionMapping::BcastSymbolStatusChangeAction(s) => s.twiddle(),
            NeqBroadcastTransactionMapping::BcastIndicativeIndices(s) => s.twiddle(),
            NeqBroadcastTransactionMapping::BcastTurnoverExceeded(s) => s.twiddle(),
            NeqBroadcastTransactionMapping::BcastBrokerReactivated(s) => s.twiddle(),
            NeqBroadcastTransactionMapping::BcastMarketStatsReportDataCedtcH(s) => s.twiddle(),
            NeqBroadcastTransactionMapping::BcastMarketStatsReportDataCedtcR(s) => s.twiddle(),
            NeqBroadcastTransactionMapping::BcastAuctionInquiryOut(s) => s.twiddle(),
            NeqBroadcastTransactionMapping::BcastSecurityStatusChg(s) => s.twiddle(),
            NeqBroadcastTransactionMapping::BcastSecurityStatusChgPreopen(s) => s.twiddle(),
            NeqBroadcastTransactionMapping::BcastOnlyMbp(s) => s.twiddle(),
            NeqBroadcastTransactionMapping::BcastBuyBack(s) => s.twiddle(),
            NeqBroadcastTransactionMapping::BcastSecurityMstrChg(s) => s.twiddle(),
        }
    }
}

#[derive(Debug, Twiddle, Clone, Copy)]
#[repr(C, packed(2))]
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

#[derive(Debug, Twiddle, Clone, Copy)]
#[repr(C, packed(2))]
pub struct BcastDestination {
    // trader_ws: u8, 1 bit
    // reserved1: u8, 7 bit
    reserved1: u8,
    reserved2: u8,
}

#[derive(Debug, Twiddle, Clone, Copy)]
#[repr(C, packed(2))]
pub struct BcastJournalMessage {
    bcast_header: BcastHeaders,
    branch_number: i16,
    broker_number: [u8; BROKERNUMBER_LEN],
    action_code: [u8; ACTIONCODE_LEN],
    reserved1: [u8; RESERVED_4],
    bcast_destination: BcastDestination,
    bcast_msg_length: i16,
    bcast_message: [u8; BCAST_MSG_LEN],
}

#[derive(Debug, Twiddle, Clone, Copy)]
#[repr(C, packed(2))]
pub struct BcastSECInfo {
    symbol: [u8; SYMBOL_LEN],
    series: [u8; SERIES_LEN],
}

#[derive(Debug, Twiddle, Clone, Copy)]
#[repr(C, packed(2))]
pub struct BcastSecurityEligibilityPerMarket {
    eligibility: u8,
    reserved: u8,
    status: i16,
}

#[derive(Debug, Twiddle, Clone, Copy)]
#[repr(C, packed(2))]
pub struct BcastEligibilityIndicators {
    // participate_in_market_index: u8, 1 bit
    // aon: u8, 1 bit
    // minimum_fill: u8, 1 bit
    reserved1: u8,
    reserved2: u8,
}

#[derive(Debug, Twiddle, Clone, Copy)]
#[repr(C, packed(2))]
pub struct BcastPurpose {
    // dividend: u8, 1 bit
    // rights: u8, 1 bit
    // bonus: u8, 1 bit
    // interest: u8, 1 bit
    // agm: u8, 1 bit
    // egm: u8, 1 bit
    reserved1: u8,
    reserved2: u8,
}

#[derive(Debug, Twiddle, Clone, Copy)]
#[repr(C, packed(2))]
pub struct BcastSecurityMasterUpdateInfo {
    bcast_header: BcastHeaders,
    token: i32,
    ec_info: BcastSECInfo,
    instrument_type: i16,
    permitted_to_trade: i16,
    lf_issued_capital: f64,
    ettlement_type: i16,
    freeze_percent: i16,
    credit_rating: [u8; CREDITRATING_LEN_19],
    reserved1: u8,
    eligibility_per_market: [BcastSecurityEligibilityPerMarket; 6],
    urv_ind: i16,
    issue_start_date: i32,
    interest_payment_date: i32,
    issue_maturity_date: i32,
    board_lot_quantity: i32,
    tick_size: i32,
    name: [u8; REMARKS_LEN],
    reserved2: u8,
    listing_date: i32,
    expulsion_date: i32,
    re_admission_date: i32,
    record_date: i32,
    expiry_date: i32,
    no_delivery_start_date: i32,
    no_delivery_end_date: i32,
    eligibility_indicators: BcastEligibilityIndicators,
    book_closure_start_date: i32,
    book_closure_end_date: i32,
    purpose: BcastPurpose,
    local_update_date_time: i32,
    delete_flag: u8,
    remark: [u8; REMARKS_LEN],
    face_value: i32,
    isin_number: [u8; ISINNUMBER_LEN],
    mkt_maker_spread: i32,
    mkt_maker_min_qty: i32,
    call_auction1_flag: i16,
}

#[derive(Debug, Twiddle, Clone, Copy)]
#[repr(C, packed(2))]
pub struct BcastParticipantMasterUpdateInfo {
    bcast_header: BcastHeaders,
    participant_id: [u8; PARTICIPANT_ID_LEN],
    participant_name: [u8; PARTICIPANT_NAME_LEN],
    participant_status: u8,
    participant_update_date_time: i32,
    delete_flag: u8,
    reserved: u8,
}

#[derive(Debug, Twiddle, Clone, Copy)]
#[repr(C, packed(2))]
pub struct BcastSecurityStatusPerMarket {
    tatus: i16,
}

#[derive(Debug, Twiddle, Clone, Copy)]
#[repr(C, packed(2))]
pub struct BcastTokenAndEligibility {
    token: i32,
    tatus_per_market: [BcastSecurityStatusPerMarket; MAX_SEC_STATUS_PERMARKET_IDX],
}

#[derive(Debug, Twiddle, Clone, Copy)]
#[repr(C, packed(2))]
pub struct BcastSecurityStatusUpdateInfo {
    bcast_header: BcastHeaders,
    number_of_records: i16,
    eligibility: [BcastTokenAndEligibility; MAX_TOKEN_ELIGIBILITY_IDX],
}

#[derive(Debug, Twiddle, Clone, Copy)]
#[repr(C, packed(2))]
pub struct BcastLimitExceeded {
    bcast_header: BcastHeaders,
    broker_code: [u8; BROKERCODE_LEN],
    counter_broker_code: [u8; COUNTER_BROKERCODE_LEN],
    warning_type: i16,
    ec_info: BcastSECInfo,
    trade_number: i32,
    trade_price: i32,
    trade_volume: i32,
    _final: u8,
}

#[derive(Debug, Twiddle, Clone, Copy)]
#[repr(C, packed(2))]
pub struct BcastSTAuctionINQInfo {
    token: i32,
    auction_number: i16,
    auction_status: i16,
    initiator_type: i16,
    total_buy_qty: i32,
    best_buy_price: i32,
    total_sell_qty: i32,
    best_sell_price: i32,
    auction_price: i32,
    auction_qty: i32,
    ettlement_period: i16,
}

#[derive(Debug, Twiddle, Clone, Copy)]
#[repr(C, packed(2))]
pub struct BcastAuctionINQData {
    bcast_header: BcastHeaders,
    auction_inq_info: BcastSTAuctionINQInfo,
}

#[derive(Debug, Twiddle, Clone, Copy)]
#[repr(C, packed(2))]
pub struct BcastAuctionStatusChange {
    bcast_header: BcastHeaders,
    ec_info: BcastSECInfo,
    auction_number: i16,
    auction_status: u8,
    action_code: [u8; ACTIONCODE_LEN],
    bcast_dest: BcastDestination,
    bcast_msg_length: i16,
    bcast_message: [u8; BCAST_MSG_LEN],
}

#[derive(Debug, Twiddle, Clone, Copy)]
#[repr(C, packed(2))]
pub struct BcastVCTMessages {
    bcast_header: BcastHeaders,
    ec_info: BcastSECInfo,
    market_type: i16,
    bcast_dest: BcastDestination,
    bcast_msg_length: i16,
    bcast_message: [u8; BCAST_MSG_LEN],
}

#[derive(Debug, Twiddle, Clone, Copy)]
#[repr(C, packed(2))]
pub struct BcastSymbolStatusChangeAction {
    bcast_header: BcastHeaders,
    ec_info: BcastSECInfo,
    market_type: i16,
    reserved: i16,
    action_code: i16,
}

#[derive(Debug, Twiddle, Clone, Copy)]
#[repr(C, packed(2))]
pub struct BcastTickerIndexInfo {
    token: i32,
    market_type: i16,
    fill_price: i32,
    fill_volume: i32,
    market_index_value: i32,
}

#[derive(Debug, Twiddle, Clone, Copy)]
#[repr(C, packed(2))]
pub struct BcastTickerTradeData {
    bcast_header: BcastHeaders,
    number_of_records: i16,
    ticker_index_info: [BcastTickerIndexInfo; MAX_TICKER_INDEX_INFO_IDX],
}

#[derive(Debug, Twiddle, Clone, Copy)]
#[repr(C, packed(2))]
pub struct BcastMBOMBPIndicator {
    // last_trade_more: u8, 1 bit
    // last_trade_less: u8, 1 bit
    // buy: u8, 1 bit
    // sell: u8, 1 bit
    reserved1: u8,
    reserved2: u8,
}

#[derive(Debug, Twiddle, Clone, Copy)]
#[repr(C, packed(2))]
pub struct BcastMBOMBPTerms {
    // mf: u8, 1 bit
    // aon: u8, 1 bit
    reserved1: u8,
    reserved: u8,
}

#[derive(Debug, Twiddle, Clone, Copy)]
#[repr(C, packed(2))]
pub struct BcastMBOInfo {
    trader_id: i32,
    qty: i32,
    price: i32,
    mbombp_terms: BcastMBOMBPTerms,
    min_fill_qty: i32,
}

#[derive(Debug, Twiddle, Clone, Copy)]
#[repr(C, packed(2))]
pub struct BcastMBPInfo {
    qty: u32,
    price: i32,
    number_of_orders: i16,
    bb_buy_sell_flag: i16,
}

#[derive(Debug, Twiddle, Clone, Copy)]
#[repr(C, packed(2))]
pub struct BcastMBPInfoCEDTC {
    qty: i64,
    price: i32,
    number_of_orders: i16,
    bb_buy_sell_flag: i16,
}

#[derive(Debug, Twiddle, Clone, Copy)]
#[repr(C, packed(2))]
pub struct BcastInteractiveMBOData {
    token: i32,
    book_type: i16,
    trading_status: i16,
    volume_traded_today: i64,
    last_traded_price: i32,
    net_change_indicator: u8,
    filler: u8,
    net_price_change_from_closing_price: i32,
    last_trade_quantity: i32,
    last_trade_time: i32,
    average_trade_price: i32,
    auction_number: i16,
    auction_status: i16,
    initiator_type: i16,
    initiator_price: i32,
    initiator_quantity: i32,
    auction_price: i32,
    auction_quantity: i32,
    mbo_info: [BcastMBOInfo; MAX_MBOINFO_IDX],
}

#[derive(Debug, Twiddle, Clone, Copy)]
#[repr(C, packed(2))]
pub struct BcastMBPIndicator {
    // last_trade_more: u8, 1 bit
    // last_trade_less: u8, 1 bit
    // buy: u8, 1 bit
    // sell: u8, 1 bit
    reserved1: u8,
    reserved2: u8,
}

#[derive(Debug, Twiddle, Clone, Copy)]
#[repr(C, packed(2))]
pub struct BcastInteractiveMBPData {
    token: i32,
    book_type: i16,
    trading_status: i16,
    volume_traded_today: u32,
    last_traded_price: i32,
    net_change_indicator: u8,
    filler: u8,
    net_price_change_from_closing_price: i32,
    last_trade_quantity: i32,
    last_trade_time: i32,
    average_trade_price: i32,
    auction_number: i16,
    auction_status: i16,
    initiator_type: i16,
    initiator_price: i32,
    initiator_quantity: i32,
    auction_price: i32,
    auction_quantity: i32,
    mbp_info: [BcastMBPInfo; MAX_MBPINFO_IDX],
    bb_total_buy_flag: i16,
    bb_total_sell_flag: i16,
    lf_total_buy_quantity: f64,
    lf_total_sell_quantity: f64,
    mbp_indicator: BcastMBPIndicator,
    closing_price: i32,
    open_price: i32,
    high_price: i32,
    low_price: i32,
}

#[derive(Debug, Twiddle, Clone, Copy)]
#[repr(C, packed(2))]
pub struct BcastOnlyMBP {
    bcast_header: BcastHeaders,
    no_of_records: i16,
    mbp_data: [BcastInteractiveMBPData; MAX_MBP_DATA_IDX],
}

#[derive(Debug, Twiddle, Clone, Copy)]
#[repr(C, packed(2))]
pub struct BcastInteractiveMBPDataCEDTC {
    token: i32,
    book_type: i16,
    trading_status: i16,
    volume_traded_today: i64,
    last_traded_price: i32,
    net_change_indicator: u8,
    filler: u8,
    net_price_change_from_closing_price: i32,
    last_trade_quantity: i32,
    last_trade_time: i32,
    average_trade_price: i32,
    auction_number: i16,
    auction_status: i16,
    initiator_type: i16,
    initiator_price: i32,
    initiator_quantity: i32,
    auction_price: i32,
    auction_quantity: i32,
    mbp_info: [BcastMBPInfoCEDTC; MAX_MBPINFO_IDX],
    bb_total_buy_flag: i16,
    bb_total_sell_flag: i16,
    total_buy_quantity: i64,
    total_sell_quantity: i64,
    mbp_indicator: BcastMBPIndicator,
    closing_price: i32,
    open_price: i32,
    high_price: i32,
    low_price: i32,
    indicative_closing_price: i32,
}

#[derive(Debug, Twiddle, Clone, Copy)]
#[repr(C, packed(2))]
pub struct BcastOnlyMBPCEDTC {
    bcast_header: BcastHeaders,
    no_of_records: i16,
    mbp_data: [BcastInteractiveMBPDataCEDTC; MAX_MBP_DATA_IDX],
}

#[derive(Debug, Twiddle, Clone, Copy)]
#[repr(C, packed(2))]
pub struct BcastMBOMBP {
    bcast_header: BcastHeaders,
    mbo_data: BcastInteractiveMBOData,
    mbp_info: [BcastMBPInfo; MAX_MBPINFO_IDX],
    bb_total_buy_flag: i16,
    bb_total_sell_flag: i16,
    total_buy_quantity: i64,
    total_sell_quantity: i64,
    mbombp_indicator: BcastMBOMBPIndicator,
    closing_price: i32,
    open_price: i32,
    high_price: i32,
    low_price: i32,
    reserved: [u8; 4],
}

#[derive(Debug, Twiddle, Clone, Copy)]
#[repr(C, packed(2))]
pub struct BcastMarketWiseInfo {
    mbombp_indicator: BcastMBOMBPIndicator,
    buy_volume: i64,
    buy_price: i32,
    ell_volume: i64,
    ell_price: i32,
    last_trade_price: i32,
    last_trade_time: i32,
}

#[derive(Debug, Twiddle, Clone, Copy)]
#[repr(C, packed(2))]
pub struct BcastMarketWatch {
    token: i32,
    market_wise_info: [BcastMarketWiseInfo; MAX_MARKET_WISE_INFO_IDX],
}

#[derive(Debug, Twiddle, Clone, Copy)]
#[repr(C, packed(2))]
pub struct BcastInquiryResponse {
    bcast_header: BcastHeaders,
    number_of_records: i16,
    market_watch: [BcastMarketWatch; MAX_MARKET_WATCH_IDX - 1],
}

#[derive(Debug, Twiddle, Clone, Copy)]
#[repr(C, packed(2))]
pub struct BcastCallAuctionMBPData {
    token: i32,
    book_type: i16,
    trading_status: i16,
    volume_traded_today: i64,
    indicative_traded_qty: i64,
    last_traded_price: i32,
    net_change_indicator: u8,
    filler: u8,
    net_price_change_from_closing_price: i32,
    last_trade_quantity: i32,
    last_trade_time: i32,
    average_trade_price: i32,
    first_open_price: i32,
    mbp_info: [BcastMBPInfo; MAX_MBPINFO_IDX],
    bb_total_buy_flag: i16,
    bb_total_sell_flag: i16,
    total_buy_quantity: i64,
    total_sell_quantity: i64,
    mbp_indicator: BcastMBPIndicator,
    closing_price: i32,
    open_price: i32,
    high_price: i32,
    low_price: i32,
}

#[derive(Debug, Twiddle, Clone, Copy)]
#[repr(C, packed(2))]
pub struct BcastCallAuctionMBP {
    bcast_header: BcastHeaders,
    number_of_records: i16,
    call_auction_mbp_data: [BcastCallAuctionMBPData; MAX_CA_MBP_DATA_IDX],
}

#[derive(Debug, Twiddle, Clone, Copy)]
#[repr(C, packed(2))]
pub struct BcastCAMarketWatch {
    token: i32,
    mkt_type: i16,
    mbombp_indicator: BcastMBOMBPIndicator,
    buy_volume: i64,
    buy_price: i32,
    ell_volume: i64,
    ell_price: i32,
    last_trade_price: i32,
    last_trade_time: i32,
}

#[derive(Debug, Twiddle, Clone, Copy)]
#[repr(C, packed(2))]
pub struct BcastCallAuctionMW {
    bcast_header: BcastHeaders,
    number_of_records: i16,
    market_watch: [BcastCAMarketWatch; MAX_CA_MARKET_WATCH_IDX],
}

#[derive(Debug, Twiddle, Clone, Copy)]
#[repr(C, packed(2))]
pub struct BcastSecurityEligibilityIndicators {
    aon: u8,
    minimum_fill: u8,
    books_merged: u8,
    reserved1: u8,
    reserved2: u8,
}

#[derive(Debug, Twiddle, Clone, Copy)]
#[repr(C, packed(2))]
pub struct BcastSystemInfoData {
    bcast_header: BcastHeaders,
    normal: i16,
    oddlot: i16,
    pot: i16,
    auction: i16,
    call_auction1: i16,
    call_auction2: i16,
    market_index: i32,
    default_periodn_normal: i16,
    default_period_spot: i16,
    default_period_auction: i16,
    competitor_period: i16,
    olicitor_period: i16,
    warninng_percent: i16,
    volume_freeze_percent: i16,
    reserved1: u8,
    terminal_idle_time: i16,
    board_lot_quantity: i32,
    tick_size: i32,
    maximum_gtc_days: i16,
    eligibility_indicators: BcastSecurityEligibilityIndicators,
    disclosed_qty_percent_allowed: i16,
    reserved2: [u8; 6],
}

#[derive(Debug, Twiddle, Clone, Copy)]
#[repr(C, packed(2))]
pub struct Ndices {
    index_name: [u8; INDEX_NAME_LEN],
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

#[derive(Debug, Twiddle, Clone, Copy)]
#[repr(C, packed(2))]
pub struct BcastIndices {
    bcast_header: BcastHeaders,
    number_of_records: i16,
    indices: [Ndices; MAX_INDICES_IDX],
}

#[derive(Debug, Twiddle, Clone, Copy)]
#[repr(C, packed(2))]
pub struct BcastBuyBackData {
    token: i32,
    symbol: [u8; SYMBOL_LEN],
    eries: [u8; SERIES_LEN],
    lf_pday_cum_vol: f64,
    pday_high_price: i32,
    pday_low_price: i32,
    pday_wt_avg: i32,
    lf_cday_cum_vol: f64,
    cday_high_price: i32,
    cday_low_price: i32,
    cday_wt_avg: i32,
    tart_date: i32,
    end_date: i32,
}

#[derive(Debug, Twiddle, Clone, Copy)]
#[repr(C, packed(2))]
pub struct BcastBuyBack {
    bcast_header: BcastHeaders,
    number_of_records: i16,
    buy_back_data: [BcastBuyBackData; MAX_BUY_BACK_IDX],
}

#[derive(Debug, Twiddle, Clone, Copy)]
#[repr(C, packed(2))]
pub struct BcastContMsg {
    bcast_header: BcastHeaders,
    tream_number: i16,
    tatus: i16,
    reserved: [u8; 200],
}

#[derive(Debug, Twiddle, Clone, Copy)]
#[repr(C, packed(2))]
pub struct BcastReportHdr {
    bcast_header: BcastHeaders,
    msg_type: u8,
    report_date: i32,
    user_type: i16,
    broker_id: [u8; BROKERCODE_LEN],
    broker_name: [u8; BROKER_NAME_LEN],
    trader_number: i16,
    trader_name: [u8; TRADER_NAME_LEN],
}

#[derive(Debug, Twiddle, Clone, Copy)]
#[repr(C, packed(2))]
pub struct BcastMktStatsData {
    ec_info: BcastSECInfo,
    market_type: i16,
    open_price: i32,
    high_price: i32,
    low_price: i32,
    closing_price: i64,
    total_quantity_traded: i64,
    total_value_traded: i32,
    previous_close_price: i32,
    fifty_two_week_high: i32,
    fifty_two_week_low: i32,
    corporate_action_indicator: i32,
}

#[derive(Debug, Twiddle, Clone, Copy)]
#[repr(C, packed(2))]
pub struct BcastReportMktStatsData {
    bcast_header: BcastHeaders,
    message_type: u8,
    reserved: u8,
    number_of_records: i16,
    market_stats: [BcastMktStatsData; 7],
}

#[derive(Debug, Twiddle, Clone, Copy)]
#[repr(C, packed(2))]
pub struct BcastInteractiveOrdCxlDetails {
    token: i32,
    filler: [u8; 4],
    buy_ord_cxl_count: i64,
    buy_ord_cxl_vol: i64,
    ell_ord_cxl_count: i64,
    ell_ord_cxl_vol: i64,
    reserved: [u8; 16],
}

#[derive(Debug, Twiddle, Clone, Copy)]
#[repr(C, packed(2))]
pub struct BcastCAOrdCxUpdate {
    bcast_header: BcastHeaders,
    number_of_records: i16,
    interactive_ord_cxl_details:
        [BcastInteractiveOrdCxlDetails; MAX_INTERACTIVE_ORD_CXL_DETAILS_IDX],
}
