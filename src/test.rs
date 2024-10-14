use std::{alloc::{dealloc, Layout}, ptr, sync::atomic::{AtomicPtr, Ordering}, thread, time::Duration};

use crossbeam::queue::SegQueue;
use types::safe_hashmap::SafeHashMap;

mod constants;
mod distributor;
mod global;
mod input;
mod output;
mod settings;
mod threadpool;
mod types;
mod utils;
mod workers;
mod macros;

lazy_static::lazy_static! {
    static ref MAP: SafeHashMap<i32, AtomicPtr<i32>> = SafeHashMap::new();
}

fn main() {

    // let boxed = Box::into_raw(Box::new(10));
    // MAP.insert(1, AtomicPtr::new(boxed));

    // let t1 = thread::spawn(|| {
    //     let ptr = MAP.get(&1).unwrap();
    //     loop {unsafe{
    //         let boxed = Box::into_raw(Box::new(10));

    //         let old = ptr.swap(boxed, Ordering::SeqCst);

    //         dealloc(old as *mut u8, Layout::new::<i32>());
    //         dealloc(old as *mut u8, Layout::new::<i32>());
    //     }}
    // });

    // let t2 = thread::spawn(|| {
    //     let ptr = MAP.get(&1).unwrap();

    //     loop {unsafe{
    //         let old = ptr.swap(ptr::null_mut(), Ordering::SeqCst);

    //         // let _ = Box::from_raw(old);
    //         dealloc(old as *mut u8, Layout::new::<i32>());
    //     }}
    // });

    // t1.join().unwrap();
    // t2.join().unwrap();
}
