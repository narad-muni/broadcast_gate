use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Message {
    MDPacketHeader(MDPacketHeader),
    FastReset(FastReset),
    DepthSnapshot(DepthSnapshot),
    DepthIncremental(DepthIncremental),
    QuoteRequest(QuoteRequest),
    CrossRequest(CrossRequest),
    TopOfBookImplied(TopOfBookImplied),
    FlexibleInstrumentUpdate(FlexibleInstrumentUpdate),
    InstrumentStateChange(InstrumentStateChange),
    IndexStats(IndexStats),
    ProductStateChange(ProductStateChange),
    MassInstrumentStateChange(MassInstrumentStateChange),
    ComplexInstrumentUpdate(ComplexInstrumentUpdate),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FastReset {}

#[derive(Serialize, Deserialize, Debug)]
pub struct MDPacketHeader {
    SenderCompID: u32,
    #[serde(with = "serde_bytes")]
    PacketSeqNum: Vec<u8>,
    #[serde(with = "serde_bytes")]
    SendingTime: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DepthSnapshot {
    MsgType: String,
    MsgSeqNum: Option<u32>,
    SenderCompID: u32,
    LastMsgSeqNumProcessed: Option<u32>,
    RefreshIndicator: Option<u32>,
    MarketSegmentID: u32,
    SecurityID: i64,
    SecurityIDSource: String,
    ProductComplex: u32,
    SecurityStatus: u32,
    TESSecurityStatus: Option<u32>,
    LastUpdateTime: i64,
    TotalBuyQuantity: Option<f64>,
    TotalSellQuantity: Option<f64>,
    MDSshGrp: Vec<MDSshGrp>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MDSshGrp {
    MDOriginType: u32,
    MDEntryType: u32,
    MDBookType: Option<u32>,
    MDSubBookType: Option<u32>,
    TrdType: Option<u32>,
    TradingSessionID: Option<u32>,
    TradingSessionSubID: Option<u32>,
    TESTradSesStatus: Option<u32>,
    SecurityTradingStatus: Option<u32>,
    MarketCondition: Option<u32>,
    FastMarketIndicator: Option<u32>,
    SecurityTradingEvent: Option<u32>,
    PotentialSecurityTradingEvent: Option<u32>,
    SoldOutIndicator: Option<u32>,
    TradeCondition: Option<u64>,
    MultiLegReportingType: Option<u32>,
    MultiLegPriceModel: Option<u32>,
    QuoteCondition: Option<u32>,
    MDEntryPx: Option<f64>,
    MDEntrySize: Option<f64>,
    NumberOfOrders: Option<u32>,
    MDPriceLevel: Option<u32>,
    MDEntryTime: Option<i64>,
    NonDisclosedTradeVolume: Option<f64>,
    TotalTradedValue: Option<f64>,
    AverageTradedPrice: Option<f64>,
    TotalNumOfTrades: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
struct MassInstrumentStateChange {
    MsgType: String,
    MsgSeqNum: u32,
    SenderCompID: u32,
    MarketSegmentID: u32,
    InstrumentScopeProductComplex: u32,
    SecurityMassStatus: u32,
    SecurityMassTradingStatus: Option<u32>,
    MassMarketCondition: u32,
    FastMarketIndicator: u32,
    SecurityMassTradingEvent: Option<u32>,
    MassSoldOutIndicator: Option<u32>,
    TransactTime: i64,
    TESSecurityMassStatus: Option<u32>,
    SecMassStatGrp: Vec<SecMassStatGrp>, // sequence of sub-structures
    LastFragment: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct SecMassStatGrp {
    SecurityID: i64,
    SecurityIDSource: String,
    SecurityStatus: u32,
    SecurityTradingStatus: Option<u32>,
    MarketCondition: u32,
    SecurityTradingEvent: Option<u32>,
    SoldOutIndicator: Option<u32>,
    TESSecurityStatus: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ProductStateChange {
    MsgType: String,
    MsgSeqNum: u32,
    SenderCompID: u32,
    MarketSegmentID: u32,
    TradingSessionID: u32,
    TradingSessionSubID: u32,
    TradSesStatus: u32,
    MarketCondition: Option<u32>,
    FastMarketIndicator: u32,
    TransactTime: i64,
    TESTradSesStatus: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
struct InstrumentStateChange {
    MsgType: String,
    MsgSeqNum: u32,
    SenderCompID: u32,
    MarketSegmentID: u32,
    SecurityID: i64,
    SecurityIDSource: String,
    SecurityStatus: u32,
    SecurityTradingStatus: Option<u32>,
    MarketCondition: u32,
    FastMarketIndicator: u32,
    SecurityTradingEvent: Option<u32>,
    SoldOutIndicator: Option<u32>,
    TransactTime: i64,
    TESSecurityStatus: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
struct DepthIncremental {
    MsgType: String,
    MsgSeqNum: u32,
    SenderCompID: u32,
    MarketSegmentID: u32,
    MDIncGrp: Vec<MDIncGrp>,
}

#[derive(Debug, Serialize, Deserialize)]
struct MDIncGrp {
    MDOriginType: u32,
    MDUpdateAction: u32,
    MDEntryType: u32,
    SecurityID: i64,
    SecurityIDSource: String,
    MDEntryPx: Option<f64>,
    MDEntrySize: Option<f64>,
    NumberOfOrders: Option<u32>,
    MDPriceLevel: Option<u32>,
    MDEntryTime: Option<i64>,
    PotentialSecurityTradingEvent: Option<u32>,
    QuoteCondition: Option<u32>,
    TotalBuyQuantity: Option<f64>,
    TotalSellQuantity: Option<f64>,
    TradeEntryGrp: Option<TradeEntryGrp>, // Optional group of sub-structure
}

#[derive(Debug, Serialize, Deserialize)]
struct TradeEntryGrp {
    TrdType: Option<u32>,
    AlgorithmicTradeIndicator: Option<u32>,
    TradeCondition: Option<u64>,
    MultiLegReportingType: Option<u32>,
    MultiLegPriceModel: Option<u32>,
    AggressorTime: Option<i64>,
    RequestTime: Option<i64>,
    AggressorSide: Option<u32>,
    NumberOfBuyOrders: Option<u32>,
    NumberOfSellOrders: Option<u32>,
    TotalNumOfTrades: Option<u32>,
    RestingCxlQty: Option<f64>,
    MDEntryID: Option<u32>,
    TotalTradedValue: Option<f64>,
    AverageTradedPrice: Option<f64>,
    NonDisclosedTradeVolume: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ComplexInstrumentUpdate {
    MsgType: String,
    MsgSeqNum: u32,
    SenderCompID: u32,
    SecurityUpdateAction: String,
    SecurityID: i64,
    SecurityIDSource: String,
    SecurityDesc: String,
    SecurityType: u32,
    SecuritySubType: Option<u32>,
    ProductComplex: u32,
    LegRatioMultiplier: Option<u32>,
    InstrmtLegGrp: Vec<InstrmtLegGrp>,       // sequence of legs
    MarketSegmentGrp: Vec<MarketSegmentGrp>, // sequence of market segments
    TransactTime: i64,
}

#[derive(Debug, Serialize, Deserialize)]
struct InstrmtLegGrp {
    LegSymbol: u32,
    LegSecurityID: i64,
    LegSecurityIDSource: String,
    LegSecurityType: u32,
    LegRatioQty: u32,
    LegSide: u32,
    LegPrice: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
struct MarketSegmentGrp {
    MarketSegmentID: u32,
    ImpliedMarketIndicator: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct QuoteRequest {
    MsgType: String,
    MsgSeqNum: u32,
    SenderCompID: u32,
    MarketSegmentID: u32,
    QuotReqGrp: Vec<QuotReqGrp>, // sequence of quote request group
}

#[derive(Debug, Serialize, Deserialize)]
struct QuotReqGrp {
    SecurityID: i64,
    SecurityIDSource: String,
    Side: Option<u32>,
    OrderQty: Option<f64>,
    TransactTime: i64,
}

#[derive(Debug, Serialize, Deserialize)]
struct CrossRequest {
    MsgType: String,
    MsgSeqNum: u32,
    SenderCompID: u32,
    MarketSegmentID: u32,
    SecurityID: i64,
    SecurityIDSource: String,
    OrderQty: Option<f64>,
    CrossRequestType: u32,
    CrossRequestSideGrp: Option<Vec<CrossRequestSideGrp>>, // optional sequence of side group
    MDEntryPx: Option<f64>,
    TransactTime: i64,
}

#[derive(Debug, Serialize, Deserialize)]
struct CrossRequestSideGrp {
    Side: Option<u32>,
    InputSource: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct IndexStats {
    MsgType: String,
    MsgSeqNum: Option<u32>,
    SenderCompID: u32,
    MarketSegmentID: Option<u32>,
    IndexHigh: Option<f64>,
    IndexLow: Option<f64>,
    IndexOpen: Option<f64>,
    IndexClose: Option<f64>,
    IndexValue: Option<f64>,
    IndexLifeHigh: Option<f64>,
    IndexLifeLow: Option<f64>,
    Index52WeekHigh: Option<f64>,
    Index52WeekLow: Option<f64>,
    CloseIndexFlag: Option<u32>,
    TransactTime: i64,
}

#[derive(Debug, Serialize, Deserialize)]
struct TopOfBookImplied {
    MsgType: String,
    MsgSeqNum: u32,
    SenderCompID: u32,
    MarketSegmentID: u32,
    MDIncGrp: Vec<MDEntry>,
}

#[derive(Debug, Serialize, Deserialize)]
struct MDEntry {
    MDUpdateAction: u32,
    MDEntryType: u32,
    MDBookType: u32,
    MDSubBookType: u32,
    SecurityID: i64,
    SecurityIDSource: String,
    MDEntryPx: Option<f64>,
    MDEntrySize: Option<f64>,
    MDEntryTime: Option<i64>,
    QuoteCondition: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
struct FlexibleInstrumentUpdate {
    MsgType: String,
    MsgSeqNum: u32,
    SenderCompID: u32,
    SecurityUpdateAction: String,
    SecurityID: i64,
    SecurityIDSource: String,
    SecurityDesc: String,
    SecurityType: u32,
    ProductComplex: u32,
    MaturityDate: u32,
    StrikePrice: Option<f64>,
    PutOrCall: Option<u32>,
    OptAttribute: Option<u32>,
    ExerciseStyle: Option<u32>,
    SettlMethod: u32,
    MarketSegmentGrp: Vec<MarketSegmentGrp2>,
    TransactTime: i64,
}

#[derive(Debug, Serialize, Deserialize)]
struct MarketSegmentGrp2 {
    MarketSegmentID: u32,
}
