use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Message {
    MDPacketHeader(MDPacketHeader),
    FastReset(FastReset),
    DepthSnapshot(DepthSnapshot),
    DepthSnapshotEmpty(()),
    MDIncGrp(MDIncGrp),
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FastReset {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MDPacketHeader {
    SenderCompID: u32,
    #[serde(with = "serde_bytes")]
    PacketSeqNum: Vec<u8>,
    #[serde(with = "serde_bytes")]
    SendingTime: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DepthSnapshot {
    pub MsgType: String,
    pub MsgSeqNum: Option<u32>,
    pub SenderCompID: u32,
    pub LastMsgSeqNumProcessed: Option<u32>,
    pub RefreshIndicator: Option<u32>,
    pub MarketSegmentID: u32,
    pub SecurityID: i64,
    pub SecurityIDSource: String,
    pub ProductComplex: u32,
    pub SecurityStatus: u32,
    pub TESSecurityStatus: Option<u32>,
    pub LastUpdateTime: i64,
    pub TotalBuyQuantity: Option<f64>,
    pub TotalSellQuantity: Option<f64>,
    pub MDSshGrp: Vec<MDSshGrp>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MDSshGrp {
    pub MDOriginType: u32,
    pub MDEntryType: u32,
    pub MDBookType: Option<u32>,
    pub MDSubBookType: Option<u32>,
    pub TrdType: Option<u32>,
    pub TradingSessionID: Option<u32>,
    pub TradingSessionSubID: Option<u32>,
    pub TESTradSesStatus: Option<u32>,
    pub SecurityTradingStatus: Option<u32>,
    pub MarketCondition: Option<u32>,
    pub FastMarketIndicator: Option<u32>,
    pub SecurityTradingEvent: Option<u32>,
    pub PotentialSecurityTradingEvent: Option<u32>,
    pub SoldOutIndicator: Option<u32>,
    pub TradeCondition: Option<u64>,
    pub MultiLegReportingType: Option<u32>,
    pub MultiLegPriceModel: Option<u32>,
    pub QuoteCondition: Option<u32>,
    pub MDEntryPx: Option<f64>,
    pub MDEntrySize: Option<f64>,
    pub NumberOfOrders: Option<u32>,
    pub MDPriceLevel: Option<u32>,
    pub MDEntryTime: Option<i64>,
    pub NonDisclosedTradeVolume: Option<f64>,
    pub TotalTradedValue: Option<f64>,
    pub AverageTradedPrice: Option<f64>,
    pub TotalNumOfTrades: Option<u32>,
}

impl MDSshGrp {
    pub fn from_md_incr_grp(md_incr_grp: &MDIncGrp) -> Self {
        MDSshGrp {
            MDOriginType: md_incr_grp.MDOriginType,
            MDEntryType: md_incr_grp.MDEntryType,
            MDBookType: None,
            MDSubBookType: None,
            TrdType: None,
            TradingSessionID: None,
            TradingSessionSubID: None,
            TESTradSesStatus: None,
            SecurityTradingStatus: None,
            MarketCondition: None,
            FastMarketIndicator: None,
            SecurityTradingEvent: None,
            PotentialSecurityTradingEvent: md_incr_grp.PotentialSecurityTradingEvent,
            SoldOutIndicator: None,
            TradeCondition: None,
            MultiLegReportingType: None,
            MultiLegPriceModel: None,
            QuoteCondition: md_incr_grp.QuoteCondition,
            MDEntryPx: md_incr_grp.MDEntryPx,
            MDEntrySize: md_incr_grp.MDEntrySize,
            NumberOfOrders: md_incr_grp.NumberOfOrders,
            MDPriceLevel: md_incr_grp.MDPriceLevel,
            MDEntryTime: md_incr_grp.MDEntryTime,
            NonDisclosedTradeVolume: None,
            TotalTradedValue: None,
            AverageTradedPrice: None,
            TotalNumOfTrades: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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

#[derive(Debug, Serialize, Deserialize, Clone)]
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

#[derive(Debug, Serialize, Deserialize, Clone)]
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

#[derive(Debug, Serialize, Deserialize, Clone)]
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DepthIncremental {
    pub MsgType: String,
    pub MsgSeqNum: u32,
    pub SenderCompID: u32,
    pub MarketSegmentID: u32,
    pub MDIncGrp: Vec<MDIncGrp>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MDIncGrp {
    pub MDOriginType: u32,
    pub MDUpdateAction: u32,
    pub MDEntryType: u32,
    pub SecurityID: i64,
    pub SecurityIDSource: String,
    pub MDEntryPx: Option<f64>,
    pub MDEntrySize: Option<f64>,
    pub NumberOfOrders: Option<u32>,
    pub MDPriceLevel: Option<u32>,
    pub MDEntryTime: Option<i64>,
    pub PotentialSecurityTradingEvent: Option<u32>,
    pub QuoteCondition: Option<u32>,
    pub TotalBuyQuantity: Option<f64>,
    pub TotalSellQuantity: Option<f64>,
    pub TradeEntryGrp: Option<TradeEntryGrp>, // Optional group of sub-structure
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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

#[derive(Debug, Serialize, Deserialize, Clone)]
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

#[derive(Debug, Serialize, Deserialize, Clone)]
struct InstrmtLegGrp {
    LegSymbol: u32,
    LegSecurityID: i64,
    LegSecurityIDSource: String,
    LegSecurityType: u32,
    LegRatioQty: u32,
    LegSide: u32,
    LegPrice: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct MarketSegmentGrp {
    MarketSegmentID: u32,
    ImpliedMarketIndicator: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct QuoteRequest {
    MsgType: String,
    MsgSeqNum: u32,
    SenderCompID: u32,
    MarketSegmentID: u32,
    QuotReqGrp: Vec<QuotReqGrp>, // sequence of quote request group
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct QuotReqGrp {
    SecurityID: i64,
    SecurityIDSource: String,
    Side: Option<u32>,
    OrderQty: Option<f64>,
    TransactTime: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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

#[derive(Debug, Serialize, Deserialize, Clone)]
struct CrossRequestSideGrp {
    Side: Option<u32>,
    InputSource: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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

#[derive(Debug, Serialize, Deserialize, Clone)]
struct TopOfBookImplied {
    MsgType: String,
    MsgSeqNum: u32,
    SenderCompID: u32,
    MarketSegmentID: u32,
    MDIncGrp: Vec<MDEntry>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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

#[derive(Debug, Serialize, Deserialize, Clone)]
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

#[derive(Debug, Serialize, Deserialize, Clone)]
struct MarketSegmentGrp2 {
    MarketSegmentID: u32,
}
