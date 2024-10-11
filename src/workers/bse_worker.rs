use std::mem::size_of;

use crate::{
    constants::{
        BEST_BID_VALUE, BEST_OFFER_VALUE, COMPLEX_MBP_UNCOMPRESSED_DATA_LEN,
        DEBT_MBP_UNCOMPRESSED_DATA_LEN, MBP_UNCOMPRESSED_DATA_LEN, MBP_UNCOMPRESSED_HEADER_LEN,
        U16_MAX,
    },
    types::{
        packet::Packet,
        packet_structures::bse::{
            build_bse_struct, BcastComplexMarketPicture, BcastDebtMarketPicture, BcastMarketPicture,
        },
    },
    utils::byte_utils::{bytes_to_partial_struct, bytes_to_struct, create_empty, struct_to_bytes},
};

pub fn process_bse_compressed(packet: &mut Packet) {
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
}

pub fn decompress_bcast_mbp(packet: &mut Packet) {
    // Load uncompressed header
    let mut bcast_market_picture: BcastMarketPicture = create_empty();
    let mut offset = MBP_UNCOMPRESSED_HEADER_LEN;

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

    struct_to_bytes(&bcast_market_picture, &mut packet.0);
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

    struct_to_bytes(&complex_market_picture, &mut packet.0);
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

    struct_to_bytes(&debt_market_picture, &mut packet.0);
}

pub fn process_bse_uncompressed(packet: &mut Packet) {
    let mut trans_code: i32 = bytes_to_struct(&packet.0);
    // Twiddle
    trans_code = trans_code.to_be();

    let mut bse_struct = build_bse_struct(trans_code as i16, &packet.0);
    bse_struct.twiddle();

    struct_to_bytes(&bse_struct, &mut packet.0);
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
