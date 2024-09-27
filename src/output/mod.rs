use std::thread::{self, JoinHandle};

use kafka_output::KafkaOutput;
use std_out::StdOut;
use udp_output::UdpOutput;

use crate::{
    global::OUTPUT_QUEUE,
    settings,
    types::{packet::Packet, settings::OutputTargets},
};

pub mod kafka_output;
pub mod std_out;
pub mod udp_output;

pub struct Output {}

trait OutputTrait {
    fn write(&mut self, data: Packet);
}

impl Output {
    pub fn new() -> Self {
        Self {}
    }

    pub fn write(self) -> JoinHandle<()> {
        thread::spawn(move || {
            let mut kafka = KafkaOutput::new();
            let mut udp = UdpOutput::new();
            let mut stdout = StdOut::new();

            let settings = settings::get();

            loop {
                if let Some(packet) = OUTPUT_QUEUE.pop() {
                    if packet.0[0] == 1 {
                        break;
                    }
                    if settings.output_targets.contains(OutputTargets::UDP) {
                        udp.write(packet);
                    }

                    if settings.output_targets.contains(OutputTargets::KAFKA) {
                        kafka.write(packet);
                    }

                    if settings.output_targets.contains(OutputTargets::STDOUT) {
                        stdout.write(packet);
                    }
                } else {
                    thread::yield_now();
                }
            }
        })
    }
}
