use std::{ptr, sync::atomic::Ordering, u32};

use crate::{
    constants::MCX_BID,
    types::{
        packet::Packet,
        packet_structures::mcx::{DepthSnapshot, MDIncGrp, MDSshGrp, Message},
        work::Work,
    },
    utils::{
        atomic_utils::compare_and_swap_gt,
        byte_utils::{bytes_to_struct, bytes_to_struct_mut},
    },
};

pub fn process_mcx_depth(packet: &mut Packet, work: &Work) {
    // Swap atomic ptr with null, and add atomic ptr to work
    let mcx_state = work.mcx_state.clone().unwrap();

    let message: Message = bytes_to_struct(&packet.0[..]);

    if let Message::DepthSnapshotEmpty(()) = message {
        mcx_state.seq_no.store(work.seq_no as u32, Ordering::SeqCst);
    } else if let Message::MDIncGrp(md_incr_grp) = message {
        // Swap atomic ptr with null
        let ptr = mcx_state.ptr.swap(ptr::null_mut(), Ordering::SeqCst);
        let mut ptr = unsafe { Box::from_raw(ptr) };

        // Cast packet as depth snapshot
        let snapshot: &mut DepthSnapshot = bytes_to_struct_mut(&mut ptr.0[..]);

        // Update seq no only if it is after current
        let seq_no_update = compare_and_swap_gt(&mcx_state.seq_no, work.seq_no as u32);

        // If unable to update seq no, skip processing
        if seq_no_update.is_err() {
            // Put ptr back into atomic ptr if it is null
            let _ = mcx_state.ptr.compare_exchange(
                ptr::null_mut(),
                Box::into_raw(ptr),
                Ordering::SeqCst,
                Ordering::SeqCst,
            );
            return;
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

        // Put ptr back into atomic ptr if it is null
        let _ = mcx_state.ptr.compare_exchange(
            ptr::null_mut(),
            Box::into_raw(ptr),
            Ordering::SeqCst,
            Ordering::SeqCst,
        );
    } else {
        todo!();
    }
}

pub fn add_depth(depth_snapshot: &mut DepthSnapshot, md_incr_grp: &MDIncGrp) {
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

pub fn change_depth(depth_snapshot: &mut DepthSnapshot, md_incr_grp: &MDIncGrp) {
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

pub fn del_depth(depth_snapshot: &mut DepthSnapshot, md_incr_grp: &MDIncGrp) {
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

pub fn del_thru_depth(depth_snapshot: &mut DepthSnapshot, md_incr_grp: &MDIncGrp) {
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

pub fn del_from_depth(depth_snapshot: &mut DepthSnapshot, md_incr_grp: &MDIncGrp) {
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

pub fn overlay_depth(depth_snapshot: &mut DepthSnapshot, md_incr_grp: &MDIncGrp) {
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

        if depth.MDPriceLevel.unwrap_or(u32::MAX) >= new_depth.MDPriceLevel.unwrap() && entry_started {
            break;
        }
        pos += 1;
    }

    pos
}