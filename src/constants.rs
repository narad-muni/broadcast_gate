use std::io::ErrorKind;

pub const BUF_SIZE: usize = 1024;
pub const SKIP_BYTES: usize = 8;

// For packet structures
pub const ALPHA_CHAR_LEN: usize = 2;
pub const SERIES_LEN: usize = 2;
pub const ACTIONCODE_LEN: usize = 3;
pub const LTP_MILLI_SEC_LEN: usize = 3;
pub const RESERVED_4: usize = 4;
pub const BROKERNUMBER_LEN: usize = 5;
pub const BROKERCODE_LEN: usize = 5;
pub const COUNTER_BROKERCODE_LEN: usize = 5;
pub const INSTRUMENT_NAME_LEN: usize = 6;
pub const ASSET_INSTRUMENT_LEN: usize = 6;
pub const INDEX_ID_LEN: usize = 7;
pub const SYMBOL_LEN: usize = 10;
pub const ASSET_NAME_LEN: usize = 10;
pub const DATE_LEN: usize = 11;
pub const NOTICE_NUMBER_LEN: usize = 11;
pub const PARTICIPANT_ID_LEN: usize = 12;
pub const ISINNUMBER_LEN: usize = 12;
pub const CREDITRATING_LEN_12: usize = 12;
pub const CREDITRATING_LEN_19: usize = 19;
pub const INDEX_NAME_LEN: usize = 21;
pub const REMARKS_LEN: usize = 25;
pub const PARTICIPANT_NAME_LEN: usize = 25;
pub const INSTRUMENT_DESC_LEN: usize = 25;
pub const BROKER_NAME_LEN: usize = 25;
pub const TRADER_NAME_LEN: usize = 26;
pub const NEWS_HEADLINE_MSG_LEN: usize = 40;
pub const BCAST_MSG_LEN: usize = 240;
pub const MAX_MBP_DATA_IDX: usize = 2;
pub const MAX_CA_MBP_DATA_IDX: usize = 2;
pub const MAX_MARKET_WISE_INFO_IDX: usize = 3;
pub const MAX_MARKET_WATCH_IDX: usize = 5;
pub const MAX_ASSEST_MBP_INFO_IDX: usize = 5;
pub const MAX_BSE_MBP_DATA_IDX: usize = 5;
pub const MAX_AUCTION_MBP_DATA_IDX: usize = 5;
pub const MAX_BSE_MBP_DEATIL_IDX: usize = 6;
pub const MAX_INDICES_IDX: usize = 6;
pub const MAX_BUY_BACK_IDX: usize = 6;
pub const MAX_SEC_STATUS_PERMARKET_IDX: usize = 6;
pub const MAX_INTERACTIVE_ORD_CXL_DETAILS_IDX: usize = 8;
pub const MAX_MBOINFO_IDX: usize = 10;
pub const MAX_MBPINFO_IDX: usize = 10;
pub const MAX_MBA_DATA_IDX: usize = 10;
pub const MAX_AUCTION_MBP_DETAILS_IDX: usize = 10;
pub const MAX_CA_MARKET_WATCH_IDX: usize = 11;
pub const MAX_IMPLIED_VOLATILITY_IDX: usize = 13;
pub const MAX_INDUSTRY_INDICES_IDX: usize = 20;
pub const MAX_BSE_LPP_RANGE_IDX: usize = 20;
pub const MAX_BSE_CA_CXL_QTY_IDX: usize = 20;
pub const MAX_BSE_INDEX_DEATIL_IDX: usize = 24;
pub const MAX_LPP_RANGE_INDX: usize = 25;
pub const MAX_TOKEN_ELIGIBILITY_IDX: usize = 25;
pub const MAX_TRADE_EXEC_RANGE_DETAILS_IDX: usize = 25;
pub const MAX_BSE_OPEN_INTEREST_IDX: usize = 26;
pub const MAX_TICKER_INDEX_INFO_IDX: usize = 28;
pub const MAX_BSE_VAR_IDX: usize = 40;
pub const MAX_ASSET_OI_IDX: usize = 58;
pub const MAX_BSE_CLOSE_PRICE_IDX: usize = 80;
pub const BCAST_ONLY_MBP: i16 = 7208;
pub const BCAST_ONLY_MBP_EQ: i16 = 18705;
pub const BCAST_MBO_MBP: i16 = 7200;
pub const MAX_SUB_PACKETS: usize = 12;

// For BSE
pub const U16_MAX: i16 = 32767;
pub const BEST_BID_VALUE: i16 = 32766;
pub const BEST_OFFER_VALUE: i16 = -32766;

pub const MBP_UNCOMPRESSED_HEADER_LEN: usize = 28;
pub const MBP_UNCOMPRESSED_DATA_LEN: usize = 56;
pub const COMPLEX_MBP_UNCOMPRESSED_DATA_LEN: usize = 60;
pub const DEBT_MBP_UNCOMPRESSED_DATA_LEN: usize = 68;

// Custom types
pub const MAX_MARKET_DEPTH_IDX: usize = 200; // Define this constant as per your requirement
pub const TIMESTAMP_LEN: usize = 8; // Define this constant as per your requirement
pub const MAX_BUY_SELL_DEPTH_IDX: usize = 5;

// Error kinds
pub const UNRECOVERABLE_ERROR_KINDS: [ErrorKind; 7] = [
    ErrorKind::NotFound,
    ErrorKind::PermissionDenied,
    ErrorKind::AddrInUse,
    ErrorKind::AddrNotAvailable,
    ErrorKind::BrokenPipe,
    ErrorKind::Unsupported,
    ErrorKind::OutOfMemory,
];