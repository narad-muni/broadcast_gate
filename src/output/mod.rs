pub mod counter;
pub mod kafka_output;
pub mod std_out;
pub mod udp_output;
pub mod ws;

use std::{
    cell::UnsafeCell,
    sync::atomic::{AtomicBool, Ordering},
};

use counter::Counter;
use kafka_output::KafkaOutput;
use std_out::StdOut;
use udp_output::UdpOutput;
use ws::Ws;

use crate::{
    settings,
    types::{packet::Packet, settings::OutputTargets},
};

pub struct Output {
    kafka: Option<UnsafeCell<KafkaOutput>>,
    ws: Option<UnsafeCell<Ws>>,
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
        let settings = settings::get();

        let kafka = if settings.output_targets.contains(OutputTargets::KAFKA) {
            Some(UnsafeCell::new(KafkaOutput::new()))
        } else {
            None
        };
        let udp = UnsafeCell::new(UdpOutput::new());
        let stdout = UnsafeCell::new(StdOut::new());
        let counter = UnsafeCell::new(Counter::new(settings.steps));

        let ws = if settings.output_targets.contains(OutputTargets::WS) {
            Some(UnsafeCell::new(Ws::new()))
        } else {
            None
        };
        let output_targets = settings::get().output_targets.clone();

        Self {
            kafka,
            udp,
            stdout,
            counter,
            ws,
            output_targets,
            lock: AtomicBool::new(false),
        }
    }

    pub fn write(&self, packet: &Packet) {
        unsafe {
            // Acquire lock
            while self.lock.swap(true, Ordering::Relaxed) == true {}

            if self.output_targets.contains(OutputTargets::UDP) {
                (*self.udp.get()).write(packet);
            }

            if self.output_targets.contains(OutputTargets::KAFKA) {
                (*self.kafka.as_ref().unwrap().get()).write(packet);
            }

            if self.output_targets.contains(OutputTargets::STDOUT) {
                (*self.stdout.get()).write(packet);
            }

            if self.output_targets.contains(OutputTargets::COUNTER) {
                (*self.counter.get()).write(packet);
            }

            if self.output_targets.contains(OutputTargets::WS) {
                (*self.ws.as_ref().unwrap().get()).write(packet);
            }

            // release lock
            self.lock.store(false, Ordering::Relaxed);
        }
    }

    pub fn touch(&self) {}
}
