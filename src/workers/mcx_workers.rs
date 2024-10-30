use std::{
    ptr::{self, drop_in_place},
    sync::atomic::Ordering,
    u32,
};

use crate::{
    constants::{ALPHA_CHAR_LEN, MAX_MARKET_DEPTH_IDX, SNAPSHOT_TEMPLATE_ID, TIMESTAMP_LEN},
    types::{
        packet::Packet,
        packet_structures::{
            depth_output::{TagMarketDepthInfo, TagMarketPictureBroadcast, TagMessageHeader},
            mcx::{DepthSnapshot, MDIncGrp, MDSshGrp, Message},
        },
        work::Work,
    },
    utils::{
        atomic_utils::compare_and_swap_gt,
        byte_utils::{bytes_to_struct, bytes_to_struct_mut, create_empty, struct_to_bytes},
        time_utils::get_epoch_us,
    },
};

pub fn process_mcx_depth(packet: &mut Packet, work: &Work) -> bool {
    // Swap atomic ptr with null, and add atomic ptr to work
    let mcx_state = work.mcx_state.clone().unwrap();

    let message: Message = bytes_to_struct(&packet.0[..]);

    if let Message::DepthSnapshotEmpty(()) = message {
        let raw_ptr = mcx_state.ptr.swap(ptr::null_mut(), Ordering::SeqCst);
        let mut ptr = unsafe { Box::from_raw(raw_ptr) };

        // Cast packet as depth snapshot
        let snapshot: &mut DepthSnapshot = bytes_to_struct_mut(&mut ptr.0[..]);

        mcx_state.seq_no.store(work.seq_no as u32, Ordering::SeqCst);
        let target_market_picture = snapshot_to_market_picture(snapshot);

        packet.1 = target_market_picture.msg_header.message_length as usize;
        struct_to_bytes(&target_market_picture, &mut packet.0);

        // Put ptr back into atomic ptr if it is null
        let swapped = mcx_state.ptr.compare_exchange(
            ptr::null_mut(),
            Box::into_raw(ptr),
            Ordering::SeqCst,
            Ordering::SeqCst,
        );

        // De allocate if current swap doesn't succeed
        if swapped.is_err() {
            unsafe {
                drop_in_place(raw_ptr);
            }
        }
    } else if let Message::MDIncGrp(md_incr_grp) = message {
        // Swap atomic ptr with null
        let raw_ptr = mcx_state.ptr.swap(ptr::null_mut(), Ordering::SeqCst);
        let mut ptr = unsafe { Box::from_raw(raw_ptr) };

        // Cast packet as depth snapshot
        let snapshot: &mut DepthSnapshot = bytes_to_struct_mut(&mut ptr.0[..]);

        // Update seq no only if it is after current
        let seq_no_update = compare_and_swap_gt(&mcx_state.seq_no, work.seq_no as u32);

        // If unable to update seq no, skip processing
        if seq_no_update.is_err() {
            // Put ptr back into atomic ptr if it is null
            let swapped = mcx_state.ptr.compare_exchange(
                ptr::null_mut(),
                Box::into_raw(ptr),
                Ordering::SeqCst,
                Ordering::SeqCst,
            );

            // De allocate if current swap doesn't succeed
            if swapped.is_err() {
                unsafe {
                    drop_in_place(raw_ptr);
                }
            }

            return false;
        }

        // Perform update based on MDUpdateAction
        match md_incr_grp.MDUpdateAction {
            0 => add_depth(snapshot, &md_incr_grp),
            1 => change_depth(snapshot, &md_incr_grp),
            2 => del_depth(snapshot, &md_incr_grp),
            3 => del_thru_depth(snapshot, &md_incr_grp),
            4 => del_from_depth(snapshot, &md_incr_grp),
            5 => overlay_depth(snapshot, &md_incr_grp),
            _ => panic!("Invalid MDUpdateAction: {}", md_incr_grp.MDUpdateAction),
        };

        snapshot.MsgSeqNum = Some(work.seq_no as u32);

        let target_market_picture = snapshot_to_market_picture(snapshot);

        packet.1 = target_market_picture.msg_header.message_length as usize;
        struct_to_bytes(&target_market_picture, &mut packet.0);

        // Put ptr back into atomic ptr if it is null
        let swapped = mcx_state.ptr.compare_exchange(
            ptr::null_mut(),
            Box::into_raw(ptr),
            Ordering::SeqCst,
            Ordering::SeqCst,
        );

        // De allocate if current swap doesn't succeed
        if swapped.is_err() {
            unsafe {
                drop_in_place(raw_ptr);
            }
        }
    } else {
        todo!();
    }

    true
}

fn snapshot_to_market_picture(depth_snapshot: &DepthSnapshot) -> TagMarketPictureBroadcast {
    let msg_header = TagMessageHeader {
        message_code: SNAPSHOT_TEMPLATE_ID,
        transaction_type: 0,
        log_time: depth_snapshot.MsgSeqNum.unwrap() as i32,
        alpha_char: [0; ALPHA_CHAR_LEN],
        trader_id: 0,
        error_code: 0,
        timestamp: get_epoch_us() as u64,
        timestamp1: [0; TIMESTAMP_LEN],
        timestamp2: [0; TIMESTAMP_LEN],
        message_length: 0,
    };

    let mut open_price = 0;
    let mut high_price = 0;
    let mut low_price = 0;
    let mut close_price = 0;
    let mut ltp = 0;
    let mut ltq = 0;
    let mut ltt = 0;
    let mut volume_traded_today = 0;
    let mut atp = 0;

    depth_snapshot.MDSshGrp.iter().for_each(|md_ssh_grp| {
        // Set vtt and atp
        if md_ssh_grp.MDEntryType == 9 {
            volume_traded_today = md_ssh_grp
                .TotalNumOfTrades
                .expect("TotalNumOfTrades must be present for B")
                as i64;

            atp = md_ssh_grp
                .AverageTradedPrice
                .expect("AveragePrice must be present for B") as i32;
        } else if md_ssh_grp.MDEntryType == 2 {
            // Set ohlc
            let trade_condition = md_ssh_grp
                .TradeCondition
                .expect("TradeCondition must be present for 2");

            // Set ltp, ltq, ltt
            if trade_condition & 1 == 1 {
                ltp = md_ssh_grp.MDEntryPx.unwrap() as i32;
                ltq = md_ssh_grp.MDEntrySize.unwrap() as i32;
                ltt = md_ssh_grp.MDEntryTime.unwrap() as i32;
            }
            // Set open, high, low, close
            if trade_condition & 2 == 2 {
                open_price = md_ssh_grp.MDEntryPx.unwrap() as i32;
            }
            if trade_condition & 4 == 4 {
                high_price = md_ssh_grp.MDEntryPx.unwrap() as i32;
            }
            if trade_condition & 8 == 8 {
                low_price = md_ssh_grp.MDEntryPx.unwrap() as i32;
            }
            if trade_condition & 16 == 16 {
                close_price = md_ssh_grp.MDEntryPx.unwrap() as i32;
            }
        }
    });

    let mut tag_market_picture_broadcast = TagMarketPictureBroadcast {
        msg_header,
        token: depth_snapshot.SecurityID,
        total_buy_qty: depth_snapshot.TotalBuyQuantity.unwrap_or(0.) as i64,
        total_sell_qty: depth_snapshot.TotalSellQuantity.unwrap_or(0.) as i64,
        volume_traded_today,
        open_price,
        close_price,
        high_price,
        low_price,
        ltp,
        ltq,
        ltt,
        atp,
        indicative_close_price: 0,
        lut: depth_snapshot.LastUpdateTime as i32,
        buy_depth_count: 0,
        sell_depth_count: 0,
        trading_status: 1,
        market_depth_info: create_empty(),
    };

    let mut buy_count = 0;
    let mut sell_count = 0;
    let mut idx = 0;

    for ssh_grp in &depth_snapshot.MDSshGrp {
        if ssh_grp.MDEntryType == 0 {
            buy_count += 1;
        } else if ssh_grp.MDEntryType == 1 {
            sell_count += 1;
        } else {
            continue;
        }

        // Add market depth
        tag_market_picture_broadcast.market_depth_info[idx] = TagMarketDepthInfo {
            qty: ssh_grp.MDEntrySize.unwrap() as i64,
            price: ssh_grp.MDEntryPx.unwrap() as i32,
            number_of_orders: ssh_grp.NumberOfOrders.unwrap() as i16,
        };

        idx += 1;
    }

    tag_market_picture_broadcast.buy_depth_count = buy_count;
    tag_market_picture_broadcast.sell_depth_count = sell_count;

    // Set message length
    tag_market_picture_broadcast.msg_header.message_length =
        (size_of::<TagMarketPictureBroadcast>()
            - (size_of::<TagMarketDepthInfo>() * (MAX_MARKET_DEPTH_IDX - idx))) as i16;

    tag_market_picture_broadcast
}

fn add_depth(depth_snapshot: &mut DepthSnapshot, md_incr_grp: &MDIncGrp) {
    let entry_type = md_incr_grp.MDEntryType;

    let new_md_ssh_grp = MDSshGrp::from_md_incr_grp(md_incr_grp);

    let pos = get_new_depth_idx(&depth_snapshot.MDSshGrp, &new_md_ssh_grp);

    depth_snapshot.MDSshGrp.insert(pos, new_md_ssh_grp);

    // Increment price level for higher prices
    depth_snapshot
        .MDSshGrp
        .iter_mut()
        .skip(pos + 1)
        .for_each(|mdssh_grp| {
            if mdssh_grp.MDEntryType == entry_type && mdssh_grp.MDPriceLevel.is_some() {
                mdssh_grp.MDPriceLevel = Some(mdssh_grp.MDPriceLevel.unwrap() + 1);
            }
        });
}

fn change_depth(depth_snapshot: &mut DepthSnapshot, md_incr_grp: &MDIncGrp) {
    let level = md_incr_grp
        .MDPriceLevel
        .expect("MDPriceLevel must be present for change");

    let pos = depth_snapshot.MDSshGrp.iter().position(|mdssh_grp| {
        mdssh_grp.MDPriceLevel == Some(level) && mdssh_grp.MDEntryType == md_incr_grp.MDEntryType
    });

    let mdssh_grp = &mut depth_snapshot.MDSshGrp[pos.expect("Unable to find position to change")];

    mdssh_grp.MDEntrySize = md_incr_grp.MDEntrySize.or(mdssh_grp.MDEntrySize);
    mdssh_grp.NumberOfOrders = md_incr_grp.NumberOfOrders.or(mdssh_grp.NumberOfOrders);
    mdssh_grp.MDEntryTime = md_incr_grp.MDEntryTime.or(mdssh_grp.MDEntryTime);
    mdssh_grp.PotentialSecurityTradingEvent = md_incr_grp
        .PotentialSecurityTradingEvent
        .or(mdssh_grp.PotentialSecurityTradingEvent);
    mdssh_grp.QuoteCondition = md_incr_grp.QuoteCondition.or(mdssh_grp.QuoteCondition);
}

fn del_depth(depth_snapshot: &mut DepthSnapshot, md_incr_grp: &MDIncGrp) {
    let level = md_incr_grp
        .MDPriceLevel
        .expect("MDPriceLevel must be present for add");
    let entry_type = md_incr_grp.MDEntryType;

    depth_snapshot.MDSshGrp.retain(|mdssh_grp| {
        // Either entry type or price level is not same
        mdssh_grp.MDEntryType != entry_type || (mdssh_grp.MDPriceLevel != Some(level))
    });

    // Decrement price level for higher prices
    depth_snapshot.MDSshGrp.iter_mut().for_each(|mdssh_grp| {
        if mdssh_grp.MDEntryType == entry_type
            && mdssh_grp.MDPriceLevel.is_some()
            && mdssh_grp.MDPriceLevel.unwrap() > level
        {
            mdssh_grp.MDPriceLevel = Some(mdssh_grp.MDPriceLevel.unwrap() - 1);
        }
    });
}

fn del_thru_depth(depth_snapshot: &mut DepthSnapshot, md_incr_grp: &MDIncGrp) {
    let level = md_incr_grp
        .MDPriceLevel
        .expect("MDPriceLevel must be present for del_thru");
    let entry_type = md_incr_grp.MDEntryType;

    depth_snapshot.MDSshGrp.retain(|mdssh_grp| {
        // Either entry type is not same
        mdssh_grp.MDEntryType != entry_type ||
        // Or price level is higher than specified
        (mdssh_grp.MDPriceLevel > Some(level))
    });

    let mut price_level = 1;
    // Decrement price level for higher prices
    depth_snapshot.MDSshGrp.iter_mut().for_each(|mdssh_grp| {
        if mdssh_grp.MDEntryType == entry_type
            && mdssh_grp.MDPriceLevel.is_some()
            && mdssh_grp.MDPriceLevel.unwrap() > level
        {
            mdssh_grp.MDPriceLevel = Some(price_level);
            price_level += 1;
        }
    });
}

fn del_from_depth(depth_snapshot: &mut DepthSnapshot, md_incr_grp: &MDIncGrp) {
    let level = md_incr_grp
        .MDPriceLevel
        .expect("MDPriceLevel must be present for del_thru");
    let entry_type = md_incr_grp.MDEntryType;

    depth_snapshot.MDSshGrp.retain(|mdssh_grp| {
        // Either entry type is not same
        mdssh_grp.MDEntryType != entry_type ||
        // Or price level is lower than specified
        (mdssh_grp.MDPriceLevel < Some(level))
    });
}

fn overlay_depth(depth_snapshot: &mut DepthSnapshot, md_incr_grp: &MDIncGrp) {
    let level = md_incr_grp
        .MDPriceLevel
        .expect("MDPriceLevel must be present for overlay");

    // Add overlay
    depth_snapshot.MDSshGrp.iter_mut().for_each(|mdssh_grp| {
        // If price level doesn't match, skip
        // Or md entry type
        // md  entry type is 0 = bid(buy), 1 = offer(sell)
        if mdssh_grp.MDPriceLevel != Some(level) || mdssh_grp.MDEntryType != md_incr_grp.MDEntryType
        {
            return;
        }

        mdssh_grp.MDEntryPx = md_incr_grp.MDEntryPx.or(mdssh_grp.MDEntryPx);
        mdssh_grp.MDEntrySize = md_incr_grp.MDEntrySize.or(mdssh_grp.MDEntrySize);
        mdssh_grp.NumberOfOrders = md_incr_grp.NumberOfOrders.or(mdssh_grp.NumberOfOrders);
        mdssh_grp.MDEntryTime = md_incr_grp.MDEntryTime.or(mdssh_grp.MDEntryTime);
        mdssh_grp.PotentialSecurityTradingEvent = md_incr_grp
            .PotentialSecurityTradingEvent
            .or(mdssh_grp.PotentialSecurityTradingEvent);
        mdssh_grp.QuoteCondition = md_incr_grp.QuoteCondition.or(mdssh_grp.QuoteCondition);
    });
}

fn get_new_depth_idx(depths: &Vec<MDSshGrp>, new_depth: &MDSshGrp) -> usize {
    let mut pos = 0;
    let mut entry_started = false;

    for depth in depths {
        if new_depth.MDEntryType == depth.MDEntryType {
            entry_started = true;
        }

        if entry_started && depth.MDEntryType != new_depth.MDEntryType {
            break;
        }

        if depth.MDPriceLevel >= new_depth.MDPriceLevel && entry_started {
            break;
        }
        pos += 1;
    }

    pos
}
