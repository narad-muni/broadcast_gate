use std::thread;

use distributor::Distributor;
use input::UdpInput;
use statistics::Statistics;
use threadpool::ThreadPoolMaster;

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
mod statistics;
mod macros;

fn main() {
    settings::init();

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
