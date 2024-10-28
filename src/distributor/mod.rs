use std::thread::{self, JoinHandle};
use std::{
    alloc::{dealloc, Layout},
    sync::{
        atomic::{AtomicPtr, Ordering},
        Arc,
    },
};

use crate::types::state::NseTokenState;
use crate::{
    global::{NSE_TOKEN_WISE_MAP, PACKET_QUEUES, TPOOL_QUEUE, WORK_LOCKS},
    types::{packet::Packet, work::Work},
};

use bse_distributor::BseDistributor;
use mcx_distributor::McxDistributor;
use ncd_distributor::NcdDistributor;
use neq_distributor::NeqDistributor;
use nfo_distributor::NfoDistributor;

use crate::{global::INPUT_QUEUE, settings, types::settings::Exchange};

pub mod bse_distributor;
pub mod mcx_distributor;
pub mod ncd_distributor;
pub mod neq_distributor;
pub mod nfo_distributor;

pub struct Distributor {}

impl Distributor {
    pub fn new() -> Self {
        Self {}
    }

    pub fn start_distributor(self) -> JoinHandle<()> {
        let settings = settings::get();
        // let mcx_distributor = McxDistributor { decoder: Decoder::new_from_xml("").unwrap() };

        // NSE or BSE processing function
        let mut distributor: Box<dyn Distribute + Send> = match settings.exchange {
            Exchange::BSE => Box::new(BseDistributor::new()),
            Exchange::NEQ => Box::new(NeqDistributor::new()),
            Exchange::NFO => Box::new(NfoDistributor::new()),
            Exchange::NCD => Box::new(NcdDistributor::new()),
            Exchange::MCX => Box::new(McxDistributor::new()),
        };

        thread::spawn(move || loop {
            if let Some(packet) = INPUT_QUEUE.pop() {
                distributor.distribute(packet);
            }
        })
    }
}

pub fn distribute_to_queue(packet: Packet, work: Work) {
    let work_id = work.work_type.get_id();

    let packet_queue = &PACKET_QUEUES[work_id];
    let work_lock = &WORK_LOCKS[work_id];

    packet_queue.push(packet);

    // If no work of current type in threadpool
    if work_lock
        .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
        .is_ok()
    {
        TPOOL_QUEUE.push(work);
    }
}

pub fn distribute_to_map(packet: Packet, mut work: Work) {
    let new_packet_ptr = Box::into_raw(Box::new(packet));

    let nse_token_state = NSE_TOKEN_WISE_MAP.get(&work.work_type.get_id());

    if let Some(nse_token_state) = nse_token_state {
        work.atomic_ptr = Some(nse_token_state.ptr.clone());
        // If value exists
        // retreive old packet by swaping with new value
        let old_packet_ptr = nse_token_state.ptr.swap(new_packet_ptr, Ordering::SeqCst);

        // If old packet ptr was set to null
        // create new work
        if old_packet_ptr.is_null() {
            TPOOL_QUEUE.push(work);
        } else {
            // If old packet was not null
            // means it is still allocated in heap
            // manually create box from it and drop
            unsafe {
                dealloc(old_packet_ptr as *mut u8, Layout::new::<Packet>());
            }
        }
    } else {
        let atomic_ptr = Arc::new(AtomicPtr::new(new_packet_ptr));
        work.atomic_ptr = Some(atomic_ptr.clone());

        NSE_TOKEN_WISE_MAP.insert(work.work_type.get_id(), NseTokenState { ptr: atomic_ptr });

        TPOOL_QUEUE.push(work);
    }
}

pub trait Distribute {
    fn distribute(&mut self, packet: Packet);
}
