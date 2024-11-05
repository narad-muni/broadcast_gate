use std::mem::size_of;

use crate::{
    constants::{
        ALPHA_CHAR_LEN, BEST_BID_VALUE, BEST_OFFER_VALUE, BSE_BCAST_MBP,
        COMPLEX_MBP_UNCOMPRESSED_DATA_LEN, DEBT_MBP_UNCOMPRESSED_DATA_LEN, MAX_MARKET_DEPTH_IDX,
        MBP_UNCOMPRESSED_DATA_LEN, MBP_UNCOMPRESSED_HEADER_LEN, TIMESTAMP_LEN, U16_MAX,
    },
    global::OUTPUT,
    types::{
        packet::Packet,
        packet_structures::{
            bse::{
                build_bse_struct, BcastComplexMarketPicture, BcastDebtMarketPicture,
                BcastMarketPicture,
            },
            depth_output::{TagMarketDepthInfo, TagMarketPictureBroadcast, TagMessageHeader},
        },
        work::Work,
    },
    utils::{
        byte_utils::{bytes_to_partial_struct, bytes_to_struct, create_empty, struct_to_bytes},
        time_utils::get_epoch_us,
    },
};

pub fn process_bse_compressed(packet: &mut Packet, _work: &Work) -> bool {
    let mut trans_code: i32 = bytes_to_struct(&packet.0);
    // Twiddle
    trans_code = trans_code.to_be();

    // Decompress packet according to transcode
    match trans_code {
        2020 => decompress_bcast_mbp(packet),
        2021 => decompress_bcast_mbp_complex_list(packet),
        2033 => decompress_bcast_debt_mbp(packet),
        _ => panic!("Invalid transcode {trans_code} for compressed bse packet"),
    };

    true
}

pub fn process_bse_uncompressed(packet: &mut Packet, _work: &Work) -> bool {
    let mut trans_code: i32 = bytes_to_struct(&packet.0);
    // Twiddle
    trans_code = trans_code.to_be();

    let mut bse_struct = build_bse_struct(trans_code as i16, &packet.0);
    bse_struct.twiddle();

    bse_struct.to_bytes(&mut packet.0);

    OUTPUT.write(&packet);

    true
}

pub fn decompress_bcast_mbp(packet: &mut Packet) {
    // Load uncompressed header
    let mut bcast_market_picture: BcastMarketPicture = create_empty();
    let mut offset = MBP_UNCOMPRESSED_HEADER_LEN;
    let mut buy_count = 0;
    let mut sell_count = 0;

    // Only cast header
    bytes_to_partial_struct(&mut bcast_market_picture, &packet.0[..offset]);

    bcast_market_picture.twiddle();

    for i in 0..bcast_market_picture.no_of_records as usize {
        // Copy uncompressed data
        bytes_to_partial_struct(
            &mut bcast_market_picture.mbp_details[i],
            &packet.0[offset..offset + MBP_UNCOMPRESSED_DATA_LEN],
        );
        bcast_market_picture.mbp_details[i].twiddle();

        offset += MBP_UNCOMPRESSED_DATA_LEN;

        bcast_market_picture.mbp_details[i].open_rate = decompress_field(
            bcast_market_picture.mbp_details[i].ltp,
            &packet.0,
            &mut offset,
        );
        bcast_market_picture.mbp_details[i].prev_close_rate = decompress_field(
            bcast_market_picture.mbp_details[i].ltp,
            &packet.0,
            &mut offset,
        );
        bcast_market_picture.mbp_details[i].high_rate = decompress_field(
            bcast_market_picture.mbp_details[i].ltp,
            &packet.0,
            &mut offset,
        );
        bcast_market_picture.mbp_details[i].low_rate = decompress_field(
            bcast_market_picture.mbp_details[i].ltp,
            &packet.0,
            &mut offset,
        );
        bcast_market_picture.mbp_details[i].block_deal_ref_rate = decompress_field(
            bcast_market_picture.mbp_details[i].ltp,
            &packet.0,
            &mut offset,
        );
        bcast_market_picture.mbp_details[i].indicative_equilibrium_price = decompress_field(
            bcast_market_picture.mbp_details[i].ltp,
            &packet.0,
            &mut offset,
        );
        bcast_market_picture.mbp_details[i].indicative_equilibrium_qty = decompress_field(
            bcast_market_picture.mbp_details[i].ltq,
            &packet.0,
            &mut offset,
        );
        bcast_market_picture.mbp_details[i].total_bid_qty = decompress_field(
            bcast_market_picture.mbp_details[i].ltq,
            &packet.0,
            &mut offset,
        );
        bcast_market_picture.mbp_details[i].total_offer_qty = decompress_field(
            bcast_market_picture.mbp_details[i].ltq,
            &packet.0,
            &mut offset,
        );
        bcast_market_picture.mbp_details[i].lower_price_band = decompress_field(
            bcast_market_picture.mbp_details[i].ltp,
            &packet.0,
            &mut offset,
        );
        bcast_market_picture.mbp_details[i].upper_price_band = decompress_field(
            bcast_market_picture.mbp_details[i].ltp,
            &packet.0,
            &mut offset,
        );
        bcast_market_picture.mbp_details[i].weighted_avg_price = decompress_field(
            bcast_market_picture.mbp_details[i].ltp,
            &packet.0,
            &mut offset,
        );

        // For buy
        for count in 0..bcast_market_picture.mbp_details[i].no_of_price_points as usize {
            buy_count = count + 1;

            if count == 0 {
                bcast_market_picture.mbp_details[i].mbp_data[count].best_bid_rate =
                    decompress_field(
                        bcast_market_picture.mbp_details[i].ltp,
                        &packet.0,
                        &mut offset,
                    );

                // This is last packet
                if bcast_market_picture.mbp_details[i].mbp_data[count].best_bid_rate
                    == BEST_BID_VALUE as i32
                {
                    bcast_market_picture.mbp_details[i].mbp_data[count].best_bid_rate = 0;
                    break;
                }
                bcast_market_picture.mbp_details[i].mbp_data[count].total_bid_qty =
                    decompress_field(
                        bcast_market_picture.mbp_details[i].ltq,
                        &packet.0,
                        &mut offset,
                    );
                bcast_market_picture.mbp_details[i].mbp_data[count].no_of_bid_at_price_point =
                    decompress_field(
                        bcast_market_picture.mbp_details[i].ltq,
                        &packet.0,
                        &mut offset,
                    );
                bcast_market_picture.mbp_details[i].mbp_data[count].implied_buy_qty =
                    decompress_field(
                        bcast_market_picture.mbp_details[i].ltq,
                        &packet.0,
                        &mut offset,
                    );
            } else {
                bcast_market_picture.mbp_details[i].mbp_data[count].best_bid_rate =
                    decompress_field(
                        bcast_market_picture.mbp_details[i].mbp_data[count - 1].best_bid_rate,
                        &packet.0,
                        &mut offset,
                    );

                // This is last packet
                if bcast_market_picture.mbp_details[i].mbp_data[count].best_bid_rate
                    == BEST_BID_VALUE as i32
                {
                    bcast_market_picture.mbp_details[i].mbp_data[count].best_bid_rate = 0;
                    break;
                }
                bcast_market_picture.mbp_details[i].mbp_data[count].total_bid_qty =
                    decompress_field(
                        bcast_market_picture.mbp_details[i].mbp_data[count - 1].total_bid_qty,
                        &packet.0,
                        &mut offset,
                    );
                bcast_market_picture.mbp_details[i].mbp_data[count].no_of_bid_at_price_point =
                    decompress_field(
                        bcast_market_picture.mbp_details[i].mbp_data[count - 1]
                            .no_of_bid_at_price_point,
                        &packet.0,
                        &mut offset,
                    );
                bcast_market_picture.mbp_details[i].mbp_data[count].implied_buy_qty =
                    decompress_field(
                        bcast_market_picture.mbp_details[i].mbp_data[count - 1].implied_buy_qty,
                        &packet.0,
                        &mut offset,
                    );
            }
        } // Buy loop end

        // For sell
        for count in 0..bcast_market_picture.mbp_details[i].no_of_price_points as usize {
            sell_count = count + 1;

            if count == 0 {
                bcast_market_picture.mbp_details[i].mbp_data[count].best_offer_rate =
                    decompress_field(
                        bcast_market_picture.mbp_details[i].ltp,
                        &packet.0,
                        &mut offset,
                    );

                // This is last packet
                if bcast_market_picture.mbp_details[i].mbp_data[count].best_offer_rate
                    == BEST_OFFER_VALUE as i32
                {
                    bcast_market_picture.mbp_details[i].mbp_data[count].best_offer_rate = 0;
                    break;
                }
                bcast_market_picture.mbp_details[i].mbp_data[count].total_offer_qty =
                    decompress_field(
                        bcast_market_picture.mbp_details[i].ltq,
                        &packet.0,
                        &mut offset,
                    );
                bcast_market_picture.mbp_details[i].mbp_data[count].no_of_offer_at_price_point =
                    decompress_field(
                        bcast_market_picture.mbp_details[i].ltq,
                        &packet.0,
                        &mut offset,
                    );
                bcast_market_picture.mbp_details[i].mbp_data[count].implied_sell_qty =
                    decompress_field(
                        bcast_market_picture.mbp_details[i].ltq,
                        &packet.0,
                        &mut offset,
                    );
            } else {
                bcast_market_picture.mbp_details[i].mbp_data[count].best_offer_rate =
                    decompress_field(
                        bcast_market_picture.mbp_details[i].mbp_data[count - 1].best_offer_rate,
                        &packet.0,
                        &mut offset,
                    );

                // This is last packet
                if bcast_market_picture.mbp_details[i].mbp_data[count].best_offer_rate
                    == BEST_OFFER_VALUE as i32
                {
                    bcast_market_picture.mbp_details[i].mbp_data[count].best_offer_rate = 0;
                    break;
                }
                bcast_market_picture.mbp_details[i].mbp_data[count].total_offer_qty =
                    decompress_field(
                        bcast_market_picture.mbp_details[i].mbp_data[count - 1].total_offer_qty,
                        &packet.0,
                        &mut offset,
                    );
                bcast_market_picture.mbp_details[i].mbp_data[count].no_of_offer_at_price_point =
                    decompress_field(
                        bcast_market_picture.mbp_details[i].mbp_data[count - 1]
                            .no_of_offer_at_price_point,
                        &packet.0,
                        &mut offset,
                    );
                bcast_market_picture.mbp_details[i].mbp_data[count].implied_sell_qty =
                    decompress_field(
                        bcast_market_picture.mbp_details[i].mbp_data[count - 1].implied_sell_qty,
                        &packet.0,
                        &mut offset,
                    );
            }
        } // sell loop end
    }

    for i in 0..bcast_market_picture.no_of_records {
        let mut bcast_market_picture = bcast_market_picture.clone();
        bcast_market_picture.no_of_records = i;

        let market_picture =
            bcast_mbp_to_market_picture(&bcast_market_picture, buy_count, sell_count);

        struct_to_bytes(&market_picture, &mut packet.0);

        OUTPUT.write(&packet);
    }
}

pub fn decompress_bcast_mbp_complex_list(packet: &mut Packet) {
    // Load uncompressed header
    let mut complex_market_picture: BcastComplexMarketPicture = create_empty();
    let mut offset = MBP_UNCOMPRESSED_HEADER_LEN;

    // Only cast header
    bytes_to_partial_struct(&mut complex_market_picture, &packet.0[..offset]);

    complex_market_picture.twiddle();

    for i in 0..complex_market_picture.no_of_records as usize {
        // Copy uncompressed data
        bytes_to_partial_struct(
            &mut complex_market_picture.mbp_details[i],
            &packet.0[offset..offset + COMPLEX_MBP_UNCOMPRESSED_DATA_LEN],
        );
        complex_market_picture.mbp_details[i].twiddle();

        offset += COMPLEX_MBP_UNCOMPRESSED_DATA_LEN;

        complex_market_picture.mbp_details[i].open_rate = decompress_field(
            complex_market_picture.mbp_details[i].ltp,
            &packet.0,
            &mut offset,
        );
        complex_market_picture.mbp_details[i].prev_close_rate = decompress_field(
            complex_market_picture.mbp_details[i].ltp,
            &packet.0,
            &mut offset,
        );
        complex_market_picture.mbp_details[i].high_rate = decompress_field(
            complex_market_picture.mbp_details[i].ltp,
            &packet.0,
            &mut offset,
        );
        complex_market_picture.mbp_details[i].low_rate = decompress_field(
            complex_market_picture.mbp_details[i].ltp,
            &packet.0,
            &mut offset,
        );
        complex_market_picture.mbp_details[i].block_deal_ref_rate = decompress_field(
            complex_market_picture.mbp_details[i].ltp,
            &packet.0,
            &mut offset,
        );
        complex_market_picture.mbp_details[i].indicative_equilibrium_price = decompress_field(
            complex_market_picture.mbp_details[i].ltp,
            &packet.0,
            &mut offset,
        );
        complex_market_picture.mbp_details[i].indicative_equilibrium_qty = decompress_field(
            complex_market_picture.mbp_details[i].ltq,
            &packet.0,
            &mut offset,
        );
        complex_market_picture.mbp_details[i].total_bid_qty = decompress_field(
            complex_market_picture.mbp_details[i].ltq,
            &packet.0,
            &mut offset,
        );
        complex_market_picture.mbp_details[i].total_offer_qty = decompress_field(
            complex_market_picture.mbp_details[i].ltq,
            &packet.0,
            &mut offset,
        );
        complex_market_picture.mbp_details[i].lower_price_band = decompress_field(
            complex_market_picture.mbp_details[i].ltp,
            &packet.0,
            &mut offset,
        );
        complex_market_picture.mbp_details[i].upper_price_band = decompress_field(
            complex_market_picture.mbp_details[i].ltp,
            &packet.0,
            &mut offset,
        );
        complex_market_picture.mbp_details[i].weighted_avg_price = decompress_field(
            complex_market_picture.mbp_details[i].ltp,
            &packet.0,
            &mut offset,
        );

        // For buy
        for count in 0..complex_market_picture.mbp_details[i].no_of_price_points as usize {
            if count == 0 {
                complex_market_picture.mbp_details[i].mbp_data[count].best_bid_rate =
                    decompress_field(
                        complex_market_picture.mbp_details[i].ltp,
                        &packet.0,
                        &mut offset,
                    );

                // This is last packet
                if complex_market_picture.mbp_details[i].mbp_data[count].best_bid_rate
                    == BEST_BID_VALUE as i32
                {
                    complex_market_picture.mbp_details[i].mbp_data[count].best_bid_rate = 0;
                    break;
                }
                complex_market_picture.mbp_details[i].mbp_data[count].total_bid_qty =
                    decompress_field(
                        complex_market_picture.mbp_details[i].ltq,
                        &packet.0,
                        &mut offset,
                    );
                complex_market_picture.mbp_details[i].mbp_data[count].no_of_bid_at_price_point =
                    decompress_field(
                        complex_market_picture.mbp_details[i].ltq,
                        &packet.0,
                        &mut offset,
                    );
                complex_market_picture.mbp_details[i].mbp_data[count].implied_buy_qty =
                    decompress_field(
                        complex_market_picture.mbp_details[i].ltq,
                        &packet.0,
                        &mut offset,
                    );
            } else {
                complex_market_picture.mbp_details[i].mbp_data[count].best_bid_rate =
                    decompress_field(
                        complex_market_picture.mbp_details[i].mbp_data[count - 1].best_bid_rate,
                        &packet.0,
                        &mut offset,
                    );

                // This is last packet
                if complex_market_picture.mbp_details[i].mbp_data[count].best_bid_rate
                    == BEST_BID_VALUE as i32
                {
                    complex_market_picture.mbp_details[i].mbp_data[count].best_bid_rate = 0;
                    break;
                }
                complex_market_picture.mbp_details[i].mbp_data[count].total_bid_qty =
                    decompress_field(
                        complex_market_picture.mbp_details[i].mbp_data[count - 1].total_bid_qty,
                        &packet.0,
                        &mut offset,
                    );
                complex_market_picture.mbp_details[i].mbp_data[count].no_of_bid_at_price_point =
                    decompress_field(
                        complex_market_picture.mbp_details[i].mbp_data[count - 1]
                            .no_of_bid_at_price_point,
                        &packet.0,
                        &mut offset,
                    );
                complex_market_picture.mbp_details[i].mbp_data[count].implied_buy_qty =
                    decompress_field(
                        complex_market_picture.mbp_details[i].mbp_data[count - 1].implied_buy_qty,
                        &packet.0,
                        &mut offset,
                    );
            }
        } // Buy loop end

        // For sell
        for count in 0..complex_market_picture.mbp_details[i].no_of_price_points as usize {
            if count == 0 {
                complex_market_picture.mbp_details[i].mbp_data[count].best_offer_rate =
                    decompress_field(
                        complex_market_picture.mbp_details[i].ltp,
                        &packet.0,
                        &mut offset,
                    );

                // This is last packet
                if complex_market_picture.mbp_details[i].mbp_data[count].best_offer_rate
                    == BEST_OFFER_VALUE as i32
                {
                    complex_market_picture.mbp_details[i].mbp_data[count].best_offer_rate = 0;
                    break;
                }
                complex_market_picture.mbp_details[i].mbp_data[count].total_offer_qty =
                    decompress_field(
                        complex_market_picture.mbp_details[i].ltq,
                        &packet.0,
                        &mut offset,
                    );
                complex_market_picture.mbp_details[i].mbp_data[count].no_of_offer_at_price_point =
                    decompress_field(
                        complex_market_picture.mbp_details[i].ltq,
                        &packet.0,
                        &mut offset,
                    );
                complex_market_picture.mbp_details[i].mbp_data[count].implied_sell_qty =
                    decompress_field(
                        complex_market_picture.mbp_details[i].ltq,
                        &packet.0,
                        &mut offset,
                    );
            } else {
                complex_market_picture.mbp_details[i].mbp_data[count].best_offer_rate =
                    decompress_field(
                        complex_market_picture.mbp_details[i].mbp_data[count - 1].best_offer_rate,
                        &packet.0,
                        &mut offset,
                    );

                // This is last packet
                if complex_market_picture.mbp_details[i].mbp_data[count].best_offer_rate
                    == BEST_OFFER_VALUE as i32
                {
                    complex_market_picture.mbp_details[i].mbp_data[count].best_offer_rate = 0;
                    break;
                }
                complex_market_picture.mbp_details[i].mbp_data[count].total_offer_qty =
                    decompress_field(
                        complex_market_picture.mbp_details[i].mbp_data[count - 1].total_offer_qty,
                        &packet.0,
                        &mut offset,
                    );
                complex_market_picture.mbp_details[i].mbp_data[count].no_of_offer_at_price_point =
                    decompress_field(
                        complex_market_picture.mbp_details[i].mbp_data[count - 1]
                            .no_of_offer_at_price_point,
                        &packet.0,
                        &mut offset,
                    );
                complex_market_picture.mbp_details[i].mbp_data[count].implied_sell_qty =
                    decompress_field(
                        complex_market_picture.mbp_details[i].mbp_data[count - 1].implied_sell_qty,
                        &packet.0,
                        &mut offset,
                    );
            }
        } // sell loop end
    }

    for i in 0..complex_market_picture.no_of_records {
        let mut complex_market_picture = complex_market_picture.clone();
        complex_market_picture.no_of_records = i;

        struct_to_bytes(&complex_market_picture, &mut packet.0);

        OUTPUT.write(&packet);
    }
}

pub fn decompress_bcast_debt_mbp(packet: &mut Packet) {
    // Load uncompressed header
    let mut debt_market_picture: BcastDebtMarketPicture = create_empty();
    let mut offset = MBP_UNCOMPRESSED_HEADER_LEN;

    // Only cast header
    bytes_to_partial_struct(&mut debt_market_picture, &packet.0[..offset]);

    debt_market_picture.twiddle();

    for i in 0..debt_market_picture.no_of_records as usize {
        // Copy uncompressed data
        bytes_to_partial_struct(
            &mut debt_market_picture.mbp_details[i],
            &packet.0[offset..offset + DEBT_MBP_UNCOMPRESSED_DATA_LEN],
        );
        debt_market_picture.mbp_details[i].twiddle();

        offset += DEBT_MBP_UNCOMPRESSED_DATA_LEN;

        debt_market_picture.mbp_details[i].open_rate = decompress_field(
            debt_market_picture.mbp_details[i].ltp,
            &packet.0,
            &mut offset,
        );
        debt_market_picture.mbp_details[i].prev_close_rate = decompress_field(
            debt_market_picture.mbp_details[i].ltp,
            &packet.0,
            &mut offset,
        );
        debt_market_picture.mbp_details[i].high_rate = decompress_field(
            debt_market_picture.mbp_details[i].ltp,
            &packet.0,
            &mut offset,
        );
        debt_market_picture.mbp_details[i].low_rate = decompress_field(
            debt_market_picture.mbp_details[i].ltp,
            &packet.0,
            &mut offset,
        );
        debt_market_picture.mbp_details[i].reserved11 = decompress_field(
            debt_market_picture.mbp_details[i].ltp,
            &packet.0,
            &mut offset,
        );
        debt_market_picture.mbp_details[i].indicative_equilibrium_price = decompress_field(
            debt_market_picture.mbp_details[i].ltp,
            &packet.0,
            &mut offset,
        );
        debt_market_picture.mbp_details[i].indicative_equilibrium_qty = decompress_field(
            debt_market_picture.mbp_details[i].ltq,
            &packet.0,
            &mut offset,
        );
        debt_market_picture.mbp_details[i].total_bid_qty = decompress_field(
            debt_market_picture.mbp_details[i].ltq,
            &packet.0,
            &mut offset,
        );
        debt_market_picture.mbp_details[i].total_offer_qty = decompress_field(
            debt_market_picture.mbp_details[i].ltq,
            &packet.0,
            &mut offset,
        );
        debt_market_picture.mbp_details[i].lower_price_band = decompress_field(
            debt_market_picture.mbp_details[i].ltp,
            &packet.0,
            &mut offset,
        );
        debt_market_picture.mbp_details[i].upper_price_band = decompress_field(
            debt_market_picture.mbp_details[i].ltp,
            &packet.0,
            &mut offset,
        );
        debt_market_picture.mbp_details[i].weighted_avg_price = decompress_field(
            debt_market_picture.mbp_details[i].ltp,
            &packet.0,
            &mut offset,
        );

        // For buy
        for count in 0..debt_market_picture.mbp_details[i].no_of_price_points as usize {
            if count == 0 {
                debt_market_picture.mbp_details[i].mbp_data[count].best_bid_rate = decompress_field(
                    debt_market_picture.mbp_details[i].ltp,
                    &packet.0,
                    &mut offset,
                );

                // This is last packet
                if debt_market_picture.mbp_details[i].mbp_data[count].best_bid_rate
                    == BEST_BID_VALUE as i32
                {
                    debt_market_picture.mbp_details[i].mbp_data[count].best_bid_rate = 0;
                    break;
                }
                debt_market_picture.mbp_details[i].mbp_data[count].total_bid_qty = decompress_field(
                    debt_market_picture.mbp_details[i].ltq,
                    &packet.0,
                    &mut offset,
                );
                debt_market_picture.mbp_details[i].mbp_data[count].buy_ytm = decompress_field(
                    debt_market_picture.mbp_details[i].ltq,
                    &packet.0,
                    &mut offset,
                );
                debt_market_picture.mbp_details[i].mbp_data[count].buy_ytp = decompress_field(
                    debt_market_picture.mbp_details[i].ltq,
                    &packet.0,
                    &mut offset,
                );
                debt_market_picture.mbp_details[i].mbp_data[count].buy_ytc = decompress_field(
                    debt_market_picture.mbp_details[i].ltq,
                    &packet.0,
                    &mut offset,
                );
                debt_market_picture.mbp_details[i].mbp_data[count].no_of_bid_at_price_point =
                    decompress_field(
                        debt_market_picture.mbp_details[i].ltq,
                        &packet.0,
                        &mut offset,
                    );
                debt_market_picture.mbp_details[i].mbp_data[count].filler1 = decompress_field(
                    debt_market_picture.mbp_details[i].ltq,
                    &packet.0,
                    &mut offset,
                );
            } else {
                debt_market_picture.mbp_details[i].mbp_data[count].best_bid_rate = decompress_field(
                    debt_market_picture.mbp_details[i].mbp_data[count - 1].best_bid_rate,
                    &packet.0,
                    &mut offset,
                );

                // This is last packet
                if debt_market_picture.mbp_details[i].mbp_data[count].best_bid_rate
                    == BEST_BID_VALUE as i32
                {
                    debt_market_picture.mbp_details[i].mbp_data[count].best_bid_rate = 0;
                    break;
                }
                debt_market_picture.mbp_details[i].mbp_data[count].total_bid_qty = decompress_field(
                    debt_market_picture.mbp_details[i].mbp_data[count - 1].total_bid_qty,
                    &packet.0,
                    &mut offset,
                );
                debt_market_picture.mbp_details[i].mbp_data[count].buy_ytm = decompress_field(
                    debt_market_picture.mbp_details[i].mbp_data[count - 1].buy_ytm,
                    &packet.0,
                    &mut offset,
                );
                debt_market_picture.mbp_details[i].mbp_data[count].buy_ytp = decompress_field(
                    debt_market_picture.mbp_details[i].mbp_data[count - 1].buy_ytp,
                    &packet.0,
                    &mut offset,
                );
                debt_market_picture.mbp_details[i].mbp_data[count].buy_ytc = decompress_field(
                    debt_market_picture.mbp_details[i].mbp_data[count - 1].buy_ytc,
                    &packet.0,
                    &mut offset,
                );
                debt_market_picture.mbp_details[i].mbp_data[count].no_of_bid_at_price_point =
                    decompress_field(
                        debt_market_picture.mbp_details[i].mbp_data[count - 1]
                            .no_of_bid_at_price_point,
                        &packet.0,
                        &mut offset,
                    );
                debt_market_picture.mbp_details[i].mbp_data[count].filler1 = decompress_field(
                    debt_market_picture.mbp_details[i].mbp_data[count - 1].filler1,
                    &packet.0,
                    &mut offset,
                );
            }
        } // Buy loop end

        // For sell
        for count in 0..debt_market_picture.mbp_details[i].no_of_price_points as usize {
            if count == 0 {
                debt_market_picture.mbp_details[i].mbp_data[count].best_offer_rate =
                    decompress_field(
                        debt_market_picture.mbp_details[i].ltp,
                        &packet.0,
                        &mut offset,
                    );

                // This is last packet
                if debt_market_picture.mbp_details[i].mbp_data[count].best_offer_rate
                    == BEST_OFFER_VALUE as i32
                {
                    debt_market_picture.mbp_details[i].mbp_data[count].best_offer_rate = 0;
                    break;
                }
                debt_market_picture.mbp_details[i].mbp_data[count].total_offer_qty =
                    decompress_field(
                        debt_market_picture.mbp_details[i].ltq,
                        &packet.0,
                        &mut offset,
                    );
                debt_market_picture.mbp_details[i].mbp_data[count].sell_ytm = decompress_field(
                    debt_market_picture.mbp_details[i].ltq,
                    &packet.0,
                    &mut offset,
                );
                debt_market_picture.mbp_details[i].mbp_data[count].sell_ytp = decompress_field(
                    debt_market_picture.mbp_details[i].ltq,
                    &packet.0,
                    &mut offset,
                );
                debt_market_picture.mbp_details[i].mbp_data[count].sell_ytc = decompress_field(
                    debt_market_picture.mbp_details[i].ltq,
                    &packet.0,
                    &mut offset,
                );
                debt_market_picture.mbp_details[i].mbp_data[count].no_of_offer_at_price_point =
                    decompress_field(
                        debt_market_picture.mbp_details[i].ltq,
                        &packet.0,
                        &mut offset,
                    );
                debt_market_picture.mbp_details[i].mbp_data[count].filler2 = decompress_field(
                    debt_market_picture.mbp_details[i].ltq,
                    &packet.0,
                    &mut offset,
                );
            } else {
                debt_market_picture.mbp_details[i].mbp_data[count].best_offer_rate =
                    decompress_field(
                        debt_market_picture.mbp_details[i].mbp_data[count - 1].best_offer_rate,
                        &packet.0,
                        &mut offset,
                    );

                // This is last packet
                if debt_market_picture.mbp_details[i].mbp_data[count].best_offer_rate
                    == BEST_OFFER_VALUE as i32
                {
                    debt_market_picture.mbp_details[i].mbp_data[count].best_offer_rate = 0;
                    break;
                }
                debt_market_picture.mbp_details[i].mbp_data[count].total_offer_qty =
                    decompress_field(
                        debt_market_picture.mbp_details[i].mbp_data[count - 1].total_offer_qty,
                        &packet.0,
                        &mut offset,
                    );
                debt_market_picture.mbp_details[i].mbp_data[count].sell_ytm = decompress_field(
                    debt_market_picture.mbp_details[i].mbp_data[count - 1].sell_ytm,
                    &packet.0,
                    &mut offset,
                );
                debt_market_picture.mbp_details[i].mbp_data[count].sell_ytp = decompress_field(
                    debt_market_picture.mbp_details[i].mbp_data[count - 1].sell_ytp,
                    &packet.0,
                    &mut offset,
                );
                debt_market_picture.mbp_details[i].mbp_data[count].sell_ytc = decompress_field(
                    debt_market_picture.mbp_details[i].mbp_data[count - 1].sell_ytc,
                    &packet.0,
                    &mut offset,
                );
                debt_market_picture.mbp_details[i].mbp_data[count].no_of_offer_at_price_point =
                    decompress_field(
                        debt_market_picture.mbp_details[i].mbp_data[count - 1]
                            .no_of_offer_at_price_point,
                        &packet.0,
                        &mut offset,
                    );
                debt_market_picture.mbp_details[i].mbp_data[count].filler2 = decompress_field(
                    debt_market_picture.mbp_details[i].mbp_data[count - 1].filler2,
                    &packet.0,
                    &mut offset,
                );
            }
        } // sell loop end
    }

    for i in 0..debt_market_picture.no_of_records {
        let mut debt_market_picture = debt_market_picture.clone();
        debt_market_picture.no_of_records = i;

        struct_to_bytes(&debt_market_picture, &mut packet.0);

        OUTPUT.write(&packet);
    }
}

pub fn decompress_field(base_value: i32, buf: &[u8], offset: &mut usize) -> i32 {
    let final_value: i32;
    let mut stop_bit_value: i16 = bytes_to_struct(&buf[*offset..*offset + size_of::<i16>()]);
    // Twiddle
    stop_bit_value = stop_bit_value.to_be();

    *offset += size_of::<i16>();

    match stop_bit_value {
        // This is last field
        BEST_BID_VALUE | BEST_OFFER_VALUE => stop_bit_value as i32,
        // Bytes exceed, read next 4 bytes
        U16_MAX => {
            final_value = bytes_to_struct(&buf[*offset..*offset + size_of::<i32>()]);

            *offset += size_of::<i32>();

            // Return twiddled
            final_value.to_be()
        }
        // Add stop bit value to base value
        _ => stop_bit_value as i32 + base_value,
    }
}

fn bcast_mbp_to_market_picture(
    bcast_mbp: &BcastMarketPicture,
    buy_count: usize,
    sell_count: usize,
) -> TagMarketPictureBroadcast {
    // No of records is used as index
    let bcast_detail = bcast_mbp.mbp_details[bcast_mbp.no_of_records as usize];

    let msg_header = TagMessageHeader {
        message_code: BSE_BCAST_MBP as i32,
        transaction_type: 0,
        log_time: 0,
        alpha_char: [0; ALPHA_CHAR_LEN],
        trader_id: 0,
        error_code: 0,
        timestamp: get_epoch_us() as u64,
        timestamp1: [0; TIMESTAMP_LEN],
        timestamp2: [0; TIMESTAMP_LEN],
        message_length: 0,
    };

    let mut market_depth_info: [TagMarketDepthInfo; MAX_MARKET_DEPTH_IDX] = create_empty();

    // Buy
    for i in 0..buy_count {
        let market_depth = TagMarketDepthInfo {
            qty: bcast_mbp.mbp_details[bcast_mbp.no_of_records as usize].mbp_data[i]
                .no_of_bid_at_price_point as i64,
            price: bcast_mbp.mbp_details[bcast_mbp.no_of_records as usize].mbp_data[i].best_bid_rate
                as i32,
            number_of_orders: bcast_mbp.mbp_details[bcast_mbp.no_of_records as usize].mbp_data[i]
                .no_of_bid_at_price_point as i16,
        };

        market_depth_info[i] = market_depth;
    }

    // Sell
    for i in 0..sell_count {
        let market_depth = TagMarketDepthInfo {
            qty: bcast_mbp.mbp_details[bcast_mbp.no_of_records as usize].mbp_data[i]
                .no_of_offer_at_price_point as i64,
            price: bcast_mbp.mbp_details[bcast_mbp.no_of_records as usize].mbp_data[i]
                .best_offer_rate as i32,
            number_of_orders: bcast_mbp.mbp_details[bcast_mbp.no_of_records as usize].mbp_data[i]
                .no_of_offer_at_price_point as i16,
        };

        market_depth_info[i + buy_count] = market_depth;
    }

    let market_picture = TagMarketPictureBroadcast {
        msg_header,
        token: bcast_detail.instrument as i64,
        total_buy_qty: bcast_detail.total_bid_qty as i64,
        total_sell_qty: bcast_detail.total_offer_qty as i64,
        volume_traded_today: bcast_detail.traded_volume as i64,
        open_price: bcast_detail.open_rate,
        close_price: bcast_detail.close_rate,
        high_price: bcast_detail.high_rate,
        low_price: bcast_detail.low_rate,
        ltp: bcast_detail.ltp,
        ltq: bcast_detail.ltq,
        ltt: 0,
        atp: bcast_detail.weighted_avg_price,
        indicative_close_price: 0,
        lut: bcast_detail.timestamp,
        buy_depth_count: buy_count as i32,
        sell_depth_count: sell_count as i32,
        trading_status: bcast_detail.session_number,
        market_depth_info,
    };

    market_picture
}
