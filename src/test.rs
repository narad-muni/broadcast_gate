use std::{alloc::{dealloc, Layout}, sync::atomic::{AtomicPtr, Ordering}, thread, time::Duration};

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

static q: SegQueue<i32> = SegQueue::<i32>::new();

fn main() {

    let boxed = Box::into_raw(Box::new(10));
    MAP.insert(1, AtomicPtr::new(boxed));

    let t1 = thread::spawn(|| {
        loop {unsafe{
            
            let data = q.pop();

            if data.is_none() {
                continue;
            }

            let ptr = MAP.get(&1).unwrap();

            let boxed = Box::into_raw(Box::new(data.unwrap()));

            let old = ptr.swap(boxed, Ordering::SeqCst);

            dealloc(old as *mut u8, Layout::new::<i32>());
        }}
    });

    let t2 = thread::spawn(|| {
        loop {unsafe{
            let data = q.pop();

            if data.is_none() {
                continue;
            }
            let ptr = MAP.get(&1).unwrap();

            let boxed = Box::into_raw(Box::new(data.unwrap()));

            let old = ptr.swap(boxed, Ordering::SeqCst);

            dealloc(old as *mut u8, Layout::new::<i32>());
        }}
    });

    loop {
        q.push(10);

        thread::sleep(Duration::from_millis(10));
    }
}
