use std::{mem::size_of, sync::{
    atomic::{AtomicBool, AtomicPtr},
    OnceLock,
}};

use crate::{
    create_array, output::Output, types::{
        packet::Packet, packet_structures::neq::BcastHeaders, safe_hashmap::SafeHashMap,
        settings::{Exchange, Settings}, work::Work,
    }
};
use crossbeam::queue::SegQueue;
use lazy_static::lazy_static;


// No of work types
const TYPE_COUNT: usize = 258;

pub static INPUT_QUEUE: SegQueue<Packet> = SegQueue::new();
pub static TPOOL_QUEUE: SegQueue<Work> = SegQueue::new();
pub static WORK_QUEUES: [SegQueue<Packet>; TYPE_COUNT] = create_array!(SegQueue::new(); 258);
pub static WORK_LOCKS: [AtomicBool; TYPE_COUNT] = create_array!(AtomicBool::new(false); 258);

pub static mut EXCHANGE: &'static Exchange = &Exchange::NEQ;

pub static SETTINGS: OnceLock<Settings> = OnceLock::new();
pub static NSE_HEADER_SIZE: usize = size_of::<BcastHeaders>();

lazy_static! {
    pub static ref TOKEN_WISE_MAP: SafeHashMap<usize, AtomicPtr<Packet>> = SafeHashMap::new();
    pub static ref OUTPUT: Output = Output::new();
}
