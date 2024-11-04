use std::mem::size_of;

use crate::{
    constants::{
        BCAST_MBO_MBP, BCAST_ONLY_MBP, BCAST_ONLY_MBP_EQ, MAX_BUY_SELL_DEPTH_IDX,
        MAX_MARKET_DEPTH_IDX, MAX_MBPINFO_IDX, SKIP_BYTES,
    },
    global::{EXCHANGE, NSE_HEADER_SIZE},
    types::{
        packet::Packet,
        packet_structures::{
            depth_output::{TagMarketDepthInfo, TagMarketPictureBroadcast, TagMessageHeader},
            ncd::{build_ncd_struct, NcdBroadcastTransactionMapping},
            neq::{
                self, build_neq_struct, BcastHeaders, BcastInteractiveMBPDataCEDTC,
                BcastOnlyMBPCEDTC, NeqBroadcastTransactionMapping,
            },
            nfo::{self, build_nfo_struct, NfoBroadcastTransactionMapping},
        },
        settings::Exchange,
        work::Work,
    },
    utils::{
        byte_utils::{create_empty, struct_to_bytes},
        time_utils::get_epoch_us,
    },
};

pub fn cast_and_twiddle_nfo(packet: &mut Packet, _work: &Work) -> bool {
    let trans_code = BcastHeaders::get_trans_code(&packet.0);

    if let Some(mut nfo_struct) = build_nfo_struct(trans_code, &packet.0[SKIP_BYTES..]) {
        nfo_struct.twiddle();

        // Convert struct to custom struct for 7208 and 7200
        if let NfoBroadcastTransactionMapping::BcastMboMbpUpdate(s) = &mut nfo_struct {
            let st = convert_mbo_mbp(s, &mut packet.1);
            packet.1 = struct_to_bytes(&st, &mut packet.0);
        } else if let NfoBroadcastTransactionMapping::BcastOnlyMbp(s) = &mut nfo_struct {
            let st = convert_only_mbp(s, &mut packet.1);
            packet.1 = struct_to_bytes(&st, &mut packet.0);
        } else {
            nfo_struct.to_bytes(&mut packet.0);
        }
    };

    true
}

pub fn cast_and_twiddle_neq(packet: &mut Packet, _work: &Work) -> bool {
    let trans_code = BcastHeaders::get_trans_code(&packet.0);

    if let Some(mut neq_struct) = build_neq_struct(trans_code, &packet.0[SKIP_BYTES..]) {
        neq_struct.twiddle();

        // Convert struct to custom struct for 7208 and 7200
        if let NeqBroadcastTransactionMapping::BcastMboMbpCedtc(s) = &mut neq_struct {
            let st = convert_mbo_mbp_eq(s, &mut packet.1);
            packet.1 = struct_to_bytes(&st, &mut packet.0);
        } else if let NeqBroadcastTransactionMapping::BcastOnlyMbpCedtc(s) = &mut neq_struct {
            let st = convert_only_mbp_cedtc(s, &mut packet.1);
            packet.1 = struct_to_bytes(&st, &mut packet.0);
        } else if let NeqBroadcastTransactionMapping::BcastOnlyMbp(s) = &mut neq_struct {
            let st = convert_only_mbp_eq(s, &mut packet.1);
            packet.1 = struct_to_bytes(&st, &mut packet.0);
        } else {
            neq_struct.to_bytes(&mut packet.0);
        };
    }

    true
}

pub fn cast_and_twiddle_ncd(packet: &mut Packet, _work: &Work) -> bool {
    let trans_code = BcastHeaders::get_trans_code(&packet.0);

    if let Some(mut ncd_struct) = build_ncd_struct(trans_code, &packet.0[SKIP_BYTES..]) {
        ncd_struct.twiddle();

        // Convert struct to custom struct for 7208 and 7200
        if let NcdBroadcastTransactionMapping::BcastMboMbpUpdate(s) = &mut ncd_struct {
            let st = convert_mbo_mbp(s, &mut packet.1);

            packet.1 = struct_to_bytes(&st, &mut packet.0);
        } else if let NcdBroadcastTransactionMapping::BcastOnlyMbp(s) = &mut ncd_struct {
            let st = convert_only_mbp(s, &mut packet.1);

            packet.1 = struct_to_bytes(&st, &mut packet.0);
        } else {
            ncd_struct.to_bytes(&mut packet.0);
        };
    }

    true
}

// 7200 for fao, cd
pub fn convert_mbo_mbp(
    bcast_mbo_mbp: &mut nfo::BcastMBOMBP,
    packet_size: &mut usize,
) -> TagMarketPictureBroadcast {
    let header = TagMessageHeader {
        message_code: bcast_mbo_mbp.bcast_header.trans_code as i32,
        transaction_type: 0,
        log_time: bcast_mbo_mbp.bcast_header.log_time,
        alpha_char: bcast_mbo_mbp.bcast_header.alpha_char,
        trader_id: bcast_mbo_mbp.bcast_header.bc_seq_no,
        error_code: bcast_mbo_mbp.bcast_header.error_code,
        timestamp: get_epoch_us() as u64,
        timestamp1: bcast_mbo_mbp.bcast_header.time_stamp2,
        timestamp2: bcast_mbo_mbp.bcast_header.time_stamp2,
        message_length: bcast_mbo_mbp.bcast_header.message_length,
    };

    let mut market_depth_info: [TagMarketDepthInfo; MAX_MARKET_DEPTH_IDX] = create_empty();

    let mut buy_depth_count = 0;
    let mut sell_depth_count = 0;
    let mut mkt_depth_cnt = 0;

    let mut i = 0;
    while i < MAX_MBPINFO_IDX {
        if bcast_mbo_mbp.mbp_info[i].qty > 0 {
            market_depth_info[mkt_depth_cnt].qty = bcast_mbo_mbp.mbp_info[i].qty as i64;
            market_depth_info[mkt_depth_cnt].price = bcast_mbo_mbp.mbp_info[i].price;
            market_depth_info[mkt_depth_cnt].number_of_orders =
                bcast_mbo_mbp.mbp_info[i].number_of_orders;

            if i <= MAX_BUY_SELL_DEPTH_IDX - 1 {
                buy_depth_count += 1;
            } else {
                sell_depth_count += 1;
            }

            mkt_depth_cnt += 1;
        } else if i < MAX_BUY_SELL_DEPTH_IDX {
            i = MAX_BUY_SELL_DEPTH_IDX - 1;
        } else {
            break;
        }

        i += 1;
    }

    let picture = TagMarketPictureBroadcast {
        msg_header: header,
        token: bcast_mbo_mbp.mbo_data.token as i64,
        total_buy_qty: bcast_mbo_mbp.lf_total_buy_quantity as i64,
        total_sell_qty: bcast_mbo_mbp.lf_total_sell_quantity as i64,
        volume_traded_today: bcast_mbo_mbp.mbo_data.volume_traded_today as i64,
        open_price: bcast_mbo_mbp.open_price,
        close_price: bcast_mbo_mbp.closing_price,
        high_price: bcast_mbo_mbp.high_price,
        low_price: bcast_mbo_mbp.low_price,
        ltp: bcast_mbo_mbp.mbo_data.last_traded_price,
        ltq: bcast_mbo_mbp.mbo_data.last_trade_quantity,
        ltt: bcast_mbo_mbp.mbo_data.last_trade_time as i64,
        atp: bcast_mbo_mbp.mbo_data.average_trade_price,
        indicative_close_price: 0,
        lut: bcast_mbo_mbp.bcast_header.log_time as i64,
        buy_depth_count,
        sell_depth_count,
        trading_status: bcast_mbo_mbp.mbo_data.trading_status,
        market_depth_info,
    };

    *packet_size = size_of::<TagMarketPictureBroadcast>()
        - (size_of::<TagMarketDepthInfo>() * (MAX_MARKET_DEPTH_IDX - mkt_depth_cnt));

    picture
}

// 7200 for eq
pub fn convert_mbo_mbp_eq(
    bcast_mbo_mbp: &mut neq::BcastMBOMBP,
    packet_size: &mut usize,
) -> TagMarketPictureBroadcast {
    let header = TagMessageHeader {
        message_code: bcast_mbo_mbp.bcast_header.trans_code as i32,
        transaction_type: 0,
        log_time: bcast_mbo_mbp.bcast_header.log_time,
        alpha_char: bcast_mbo_mbp.bcast_header.alpha_char,
        trader_id: bcast_mbo_mbp.bcast_header.bc_seq_no,
        error_code: bcast_mbo_mbp.bcast_header.error_code,
        timestamp: get_epoch_us() as u64,
        timestamp1: bcast_mbo_mbp.bcast_header.time_stamp2,
        timestamp2: bcast_mbo_mbp.bcast_header.time_stamp2,
        message_length: bcast_mbo_mbp.bcast_header.message_length,
    };

    let mut market_depth_info: [TagMarketDepthInfo; MAX_MARKET_DEPTH_IDX] = create_empty();

    let mut buy_depth_count = 0;
    let mut sell_depth_count = 0;
    let mut mkt_depth_cnt = 0;

    let mut i = 0;
    while i < MAX_MBPINFO_IDX {
        if bcast_mbo_mbp.mbp_info[i].qty > 0 {
            market_depth_info[mkt_depth_cnt].qty = bcast_mbo_mbp.mbp_info[i].qty as i64;
            market_depth_info[mkt_depth_cnt].price = bcast_mbo_mbp.mbp_info[i].price;
            market_depth_info[mkt_depth_cnt].number_of_orders =
                bcast_mbo_mbp.mbp_info[i].number_of_orders;

            if i <= MAX_BUY_SELL_DEPTH_IDX - 1 {
                buy_depth_count += 1;
            } else {
                sell_depth_count += 1;
            }

            mkt_depth_cnt += 1;
        } else if i < MAX_BUY_SELL_DEPTH_IDX {
            i = MAX_BUY_SELL_DEPTH_IDX - 1;
        } else {
            break;
        }

        i += 1;
    }

    let picture = TagMarketPictureBroadcast {
        msg_header: header,
        token: bcast_mbo_mbp.mbo_data.token as i64,
        total_buy_qty: bcast_mbo_mbp.total_buy_qty as i64,
        total_sell_qty: bcast_mbo_mbp.total_sell_qty as i64,
        volume_traded_today: bcast_mbo_mbp.mbo_data.volume_traded_today as i64,
        open_price: bcast_mbo_mbp.open_price,
        close_price: bcast_mbo_mbp.closing_price,
        high_price: bcast_mbo_mbp.high_price,
        low_price: bcast_mbo_mbp.low_price,
        ltp: bcast_mbo_mbp.mbo_data.last_traded_price,
        ltq: bcast_mbo_mbp.mbo_data.last_trade_quantity,
        ltt: bcast_mbo_mbp.mbo_data.last_trade_time as i64,
        atp: bcast_mbo_mbp.mbo_data.average_trade_price,
        indicative_close_price: 0,
        lut: bcast_mbo_mbp.bcast_header.log_time as i64,
        buy_depth_count,
        sell_depth_count,
        trading_status: bcast_mbo_mbp.mbo_data.trading_status,
        market_depth_info,
    };

    *packet_size = size_of::<TagMarketPictureBroadcast>()
        - (size_of::<TagMarketDepthInfo>() * (MAX_MARKET_DEPTH_IDX - mkt_depth_cnt));

    picture
}

// 7208 fao, cd
pub fn convert_only_mbp(
    bcast_only_mbp: &mut nfo::BcastOnlyMBP,
    packet_size: &mut usize,
) -> TagMarketPictureBroadcast {
    let header = TagMessageHeader {
        message_code: bcast_only_mbp.bcast_header.trans_code as i32,
        transaction_type: 0,
        log_time: bcast_only_mbp.bcast_header.log_time,
        alpha_char: bcast_only_mbp.bcast_header.alpha_char,
        trader_id: bcast_only_mbp.bcast_header.bc_seq_no,
        error_code: bcast_only_mbp.bcast_header.error_code,
        timestamp: get_epoch_us() as u64,
        timestamp1: bcast_only_mbp.bcast_header.time_stamp2,
        timestamp2: bcast_only_mbp.bcast_header.time_stamp2,
        message_length: bcast_only_mbp.bcast_header.message_length,
    };

    let mut market_depth_info: [TagMarketDepthInfo; MAX_MARKET_DEPTH_IDX] = create_empty();

    let mut buy_depth_count = 0;
    let mut sell_depth_count = 0;
    let mut mkt_depth_cnt = 0;

    // Used as index
    let idx = bcast_only_mbp.no_of_records as usize;

    let mut i = 0;
    while i < MAX_MBPINFO_IDX {
        if bcast_only_mbp.mbp_data[idx].mbp_info[i].qty > 0 {
            market_depth_info[mkt_depth_cnt].qty =
                bcast_only_mbp.mbp_data[idx].mbp_info[i].qty as i64;
            market_depth_info[mkt_depth_cnt].price = bcast_only_mbp.mbp_data[idx].mbp_info[i].price;
            market_depth_info[mkt_depth_cnt].number_of_orders =
                bcast_only_mbp.mbp_data[idx].mbp_info[i].number_of_orders;

            if i <= MAX_BUY_SELL_DEPTH_IDX - 1 {
                buy_depth_count += 1;
            } else {
                sell_depth_count += 1;
            }

            mkt_depth_cnt += 1;
        } else if i < MAX_BUY_SELL_DEPTH_IDX {
            i = MAX_BUY_SELL_DEPTH_IDX - 1;
        } else {
            break;
        }

        i += 1;
    }

    let picture = TagMarketPictureBroadcast {
        msg_header: header,
        token: bcast_only_mbp.mbp_data[idx].token as i64,
        total_buy_qty: bcast_only_mbp.mbp_data[idx].total_buy_quantity as i64,
        total_sell_qty: bcast_only_mbp.mbp_data[idx].total_sell_quantity as i64,
        volume_traded_today: bcast_only_mbp.mbp_data[idx].volume_traded_today as i64,
        open_price: bcast_only_mbp.mbp_data[idx].open_price,
        close_price: bcast_only_mbp.mbp_data[idx].closing_price,
        high_price: bcast_only_mbp.mbp_data[idx].high_price,
        low_price: bcast_only_mbp.mbp_data[idx].low_price,
        ltp: bcast_only_mbp.mbp_data[idx].last_traded_price,
        ltq: bcast_only_mbp.mbp_data[idx].last_trade_quantity,
        ltt: bcast_only_mbp.mbp_data[idx].last_trade_time as i64,
        atp: bcast_only_mbp.mbp_data[idx].average_trade_price,
        indicative_close_price: 0,
        lut: bcast_only_mbp.bcast_header.log_time as i64,
        buy_depth_count,
        sell_depth_count,
        trading_status: bcast_only_mbp.mbp_data[idx].trading_status,
        market_depth_info,
    };

    *packet_size = size_of::<TagMarketPictureBroadcast>()
        - (size_of::<TagMarketDepthInfo>() * (MAX_MARKET_DEPTH_IDX - mkt_depth_cnt));

    picture
}

// 18705 eq
pub fn convert_only_mbp_eq(
    bcast_only_mbp: &mut neq::BcastOnlyMBP,
    packet_size: &mut usize,
) -> TagMarketPictureBroadcast {
    let header = TagMessageHeader {
        message_code: bcast_only_mbp.bcast_header.trans_code as i32,
        transaction_type: 0,
        log_time: bcast_only_mbp.bcast_header.log_time,
        alpha_char: bcast_only_mbp.bcast_header.alpha_char,
        trader_id: bcast_only_mbp.bcast_header.bc_seq_no,
        error_code: bcast_only_mbp.bcast_header.error_code,
        timestamp: get_epoch_us() as u64,
        timestamp1: bcast_only_mbp.bcast_header.time_stamp2,
        timestamp2: bcast_only_mbp.bcast_header.time_stamp2,
        message_length: bcast_only_mbp.bcast_header.message_length,
    };

    let mut market_depth_info: [TagMarketDepthInfo; MAX_MARKET_DEPTH_IDX] = create_empty();

    let mut buy_depth_count = 0;
    let mut sell_depth_count = 0;
    let mut mkt_depth_cnt = 0;

    // Used as index
    let idx = bcast_only_mbp.no_of_records as usize;

    let mut i = 0;
    while i < MAX_MBPINFO_IDX {
        if bcast_only_mbp.mbp_data[idx].mbp_info[i].qty > 0 {
            market_depth_info[mkt_depth_cnt].qty =
                bcast_only_mbp.mbp_data[idx].mbp_info[i].qty as i64;
            market_depth_info[mkt_depth_cnt].price = bcast_only_mbp.mbp_data[idx].mbp_info[i].price;
            market_depth_info[mkt_depth_cnt].number_of_orders =
                bcast_only_mbp.mbp_data[idx].mbp_info[i].number_of_orders;

            if i <= MAX_BUY_SELL_DEPTH_IDX - 1 {
                buy_depth_count += 1;
            } else {
                sell_depth_count += 1;
            }

            mkt_depth_cnt += 1;
        } else if i < MAX_BUY_SELL_DEPTH_IDX {
            i = MAX_BUY_SELL_DEPTH_IDX - 1;
        } else {
            break;
        }

        i += 1;
    }

    let picture = TagMarketPictureBroadcast {
        msg_header: header,
        token: bcast_only_mbp.mbp_data[idx].token as i64,
        total_buy_qty: bcast_only_mbp.mbp_data[idx].lf_total_buy_quantity as i64,
        total_sell_qty: bcast_only_mbp.mbp_data[idx].lf_total_sell_quantity as i64,
        volume_traded_today: bcast_only_mbp.mbp_data[idx].volume_traded_today as i64,
        open_price: bcast_only_mbp.mbp_data[idx].open_price,
        close_price: bcast_only_mbp.mbp_data[idx].closing_price,
        high_price: bcast_only_mbp.mbp_data[idx].high_price,
        low_price: bcast_only_mbp.mbp_data[idx].low_price,
        ltp: bcast_only_mbp.mbp_data[idx].last_traded_price,
        ltq: bcast_only_mbp.mbp_data[idx].last_trade_quantity,
        ltt: bcast_only_mbp.mbp_data[idx].last_trade_time as i64 * 1000,
        atp: bcast_only_mbp.mbp_data[idx].average_trade_price,
        indicative_close_price: 0,
        lut: bcast_only_mbp.bcast_header.log_time as i64 * 1000,
        buy_depth_count,
        sell_depth_count,
        trading_status: bcast_only_mbp.mbp_data[idx].trading_status,
        market_depth_info,
    };

    *packet_size = size_of::<TagMarketPictureBroadcast>()
        - (size_of::<TagMarketDepthInfo>() * (MAX_MARKET_DEPTH_IDX - mkt_depth_cnt));

    picture
}

// 7208 eq
pub fn convert_only_mbp_cedtc(
    bcast_only_mbp_cedtc: &mut BcastOnlyMBPCEDTC,
    packet_size: &mut usize,
) -> TagMarketPictureBroadcast {
    let header = TagMessageHeader {
        message_code: bcast_only_mbp_cedtc.bcast_header.trans_code as i32,
        transaction_type: 0,
        log_time: bcast_only_mbp_cedtc.bcast_header.log_time,
        alpha_char: bcast_only_mbp_cedtc.bcast_header.alpha_char,
        trader_id: bcast_only_mbp_cedtc.bcast_header.bc_seq_no,
        error_code: bcast_only_mbp_cedtc.bcast_header.error_code,
        timestamp: get_epoch_us() as u64,
        timestamp1: bcast_only_mbp_cedtc.bcast_header.time_stamp2,
        timestamp2: bcast_only_mbp_cedtc.bcast_header.time_stamp2,
        message_length: bcast_only_mbp_cedtc.bcast_header.message_length,
    };

    let mut market_depth_info: [TagMarketDepthInfo; MAX_MARKET_DEPTH_IDX] = create_empty();

    let mut buy_depth_count = 0;
    let mut sell_depth_count = 0;
    let mut mkt_depth_cnt = 0;

    // Used as index
    let idx = bcast_only_mbp_cedtc.no_of_records as usize;

    let mut i = 0;
    while i < MAX_MBPINFO_IDX {
        if bcast_only_mbp_cedtc.mbp_data[idx].mbp_info[i].qty > 0 {
            market_depth_info[mkt_depth_cnt].qty =
                bcast_only_mbp_cedtc.mbp_data[idx].mbp_info[i].qty as i64;
            market_depth_info[mkt_depth_cnt].price =
                bcast_only_mbp_cedtc.mbp_data[idx].mbp_info[i].price;
            market_depth_info[mkt_depth_cnt].number_of_orders =
                bcast_only_mbp_cedtc.mbp_data[idx].mbp_info[i].number_of_orders;

            if i <= MAX_BUY_SELL_DEPTH_IDX - 1 {
                buy_depth_count += 1;
            } else {
                sell_depth_count += 1;
            }

            mkt_depth_cnt += 1;
        } else if i < MAX_BUY_SELL_DEPTH_IDX {
            i = MAX_BUY_SELL_DEPTH_IDX - 1;
        } else {
            break;
        }

        i += 1;
    }

    let picture = TagMarketPictureBroadcast {
        msg_header: header,
        token: bcast_only_mbp_cedtc.mbp_data[idx].token as i64,
        total_buy_qty: bcast_only_mbp_cedtc.mbp_data[idx].total_buy_quantity as i64,
        total_sell_qty: bcast_only_mbp_cedtc.mbp_data[idx].total_sell_quantity as i64,
        volume_traded_today: bcast_only_mbp_cedtc.mbp_data[idx].volume_traded_today as i64,
        open_price: bcast_only_mbp_cedtc.mbp_data[idx].open_price,
        close_price: bcast_only_mbp_cedtc.mbp_data[idx].closing_price,
        high_price: bcast_only_mbp_cedtc.mbp_data[idx].high_price,
        low_price: bcast_only_mbp_cedtc.mbp_data[idx].low_price,
        ltp: bcast_only_mbp_cedtc.mbp_data[idx].last_traded_price,
        ltq: bcast_only_mbp_cedtc.mbp_data[idx].last_trade_quantity,
        ltt: bcast_only_mbp_cedtc.mbp_data[idx].last_trade_time as i64,
        atp: bcast_only_mbp_cedtc.mbp_data[idx].average_trade_price,
        indicative_close_price: 0,
        lut: bcast_only_mbp_cedtc.bcast_header.log_time as i64,
        buy_depth_count,
        sell_depth_count,
        trading_status: bcast_only_mbp_cedtc.mbp_data[idx].trading_status,
        market_depth_info,
    };

    *packet_size = size_of::<TagMarketPictureBroadcast>()
        - (size_of::<TagMarketDepthInfo>() * (MAX_MARKET_DEPTH_IDX - mkt_depth_cnt));

    picture
}

pub fn get_token(trans_code: i16, buf: &[u8], idx: usize) -> i32 {
    let exchange = unsafe { EXCHANGE };

    let mut token = 0;

    if exchange == Exchange::NEQ {
        if trans_code == BCAST_MBO_MBP {
            let token_start = SKIP_BYTES + NSE_HEADER_SIZE;
            let token_end = token_start + size_of::<i32>();

            token = i32::from_be_bytes(buf[token_start..token_end].try_into().unwrap());
        } else if trans_code == BCAST_ONLY_MBP {
            let packet_sz = idx * size_of::<BcastInteractiveMBPDataCEDTC>();
            let token_start = SKIP_BYTES + NSE_HEADER_SIZE + size_of::<i16>() + packet_sz;
            let token_end = token_start + size_of::<i32>();

            token = i32::from_be_bytes(buf[token_start..token_end].try_into().unwrap());
        } else if trans_code == BCAST_ONLY_MBP_EQ {
            let packet_sz = idx * size_of::<neq::BcastInteractiveMBPData>();
            let token_start = SKIP_BYTES + NSE_HEADER_SIZE + size_of::<i16>() + packet_sz;
            let token_end = token_start + size_of::<i32>();

            token = i32::from_be_bytes(buf[token_start..token_end].try_into().unwrap());
        }
    } else {
        if trans_code == BCAST_MBO_MBP {
            let token_start = SKIP_BYTES + NSE_HEADER_SIZE;
            let token_end = token_start + size_of::<i32>();

            token = i32::from_be_bytes(buf[token_start..token_end].try_into().unwrap());
        } else {
            let packet_sz = idx * size_of::<nfo::BcastInteractiveMBPData>();
            let token_start = SKIP_BYTES + NSE_HEADER_SIZE + size_of::<i16>() + packet_sz;
            let token_end = token_start + size_of::<i32>();

            token = i32::from_be_bytes(buf[token_start..token_end].try_into().unwrap());
        }
    }

    token
}
