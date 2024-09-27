use std::thread;

use distributor::Distributor;
use global::TOKEN_WISE_MAP;
use input::UdpInput;
use output::Output;
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

fn main() {
    settings::init();
    TOKEN_WISE_MAP.init();

    let distributor = Distributor::new();
    let tpool_master = ThreadPoolMaster::new(settings::get().thread_count);
    let output = Output::new();

    let input_thread = thread::spawn(|| UdpInput::new().read());
    let distributor_thread = distributor.start_distributor();
    let tpool_master_thread = tpool_master.start_tpool();
    let output_thread = output.write();

    input_thread.join().unwrap();
    distributor_thread.join().unwrap();
    tpool_master_thread.join().unwrap();
    output_thread.join().unwrap();
}
