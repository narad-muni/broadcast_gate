use std::{
    cell::UnsafeCell,
    sync::atomic::{AtomicBool, Ordering},
};

use counter::Counter;
use kafka_output::KafkaOutput;
use std_out::StdOut;
use udp_output::UdpOutput;

use crate::{
    settings,
    types::{packet::Packet, settings::OutputTargets},
};

pub mod counter;
pub mod kafka_output;
pub mod std_out;
pub mod udp_output;

pub struct Output {
    kafka: UnsafeCell<KafkaOutput>,
    udp: UnsafeCell<UdpOutput>,
    stdout: UnsafeCell<StdOut>,
    counter: UnsafeCell<Counter>,
    lock: AtomicBool,
    output_targets: OutputTargets,
}

unsafe impl Send for Output {}
unsafe impl Sync for Output {}

trait OutputTrait {
    fn write(&mut self, data: &Packet);
}

impl Output {
    pub fn new() -> Self {
        let kafka = UnsafeCell::new(KafkaOutput::new());
        let udp = UnsafeCell::new(UdpOutput::new());
        let stdout = UnsafeCell::new(StdOut::new());
        let counter = UnsafeCell::new(Counter::new());
        let output_targets = settings::get().output_targets.clone();

        Self {
            kafka,
            udp,
            stdout,
            counter,
            output_targets,
            lock: AtomicBool::new(false),
        }
    }

    pub fn write(&self, packet: &Packet) {
        unsafe {
            // Acquire lock
            while self.lock.swap(true, Ordering::Relaxed) == false {}

            if self.output_targets.contains(OutputTargets::UDP) {
                (*self.udp.get()).write(packet);
            }

            if self.output_targets.contains(OutputTargets::KAFKA) {
                (*self.kafka.get()).write(packet);
            }

            if self.output_targets.contains(OutputTargets::STDOUT) {
                (*self.stdout.get()).write(packet);
            }

            if self.output_targets.contains(OutputTargets::COUNTER) {
                (*self.counter.get()).write(packet);
            }

            // release lock
            self.lock.store(false, Ordering::Relaxed);
        }
    }
}
