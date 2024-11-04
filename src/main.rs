use std::thread;

use distributor::Distributor;
use global::OUTPUT;
use input::UdpInput;
use output::Output;
use statistics::Statistics;
use threadpool::ThreadPoolMaster;

mod constants;
mod distributor;
mod global;
mod input;
mod macros;
mod output;
mod settings;
mod statistics;
mod threadpool;
mod types;
mod utils;
mod workers;

fn main() {
    settings::init();
    // Because behind lazy static, we need to init it at start
    OUTPUT.touch();

    let distributor = Distributor::new();
    let tpool_master = ThreadPoolMaster::new(settings::get().thread_count);

    let input_thread = thread::spawn(|| UdpInput::new().read());
    let distributor_thread = distributor.start_distributor();
    let tpool_master_thread = tpool_master.start_tpool();

    // Runs in main thread in loop
    Statistics::run();

    input_thread.join().unwrap();
    distributor_thread.join().unwrap();
    tpool_master_thread.join().unwrap();
}
