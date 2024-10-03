use std::time::Instant;

use crate::types::packet::Packet;

use super::OutputTrait;

pub struct Counter {
    i: usize,
    time: Instant,
}

impl Counter {
    pub fn new() -> Counter {
        Counter {
            i: 0,
            time: Instant::now(),
        }
    }
}

impl OutputTrait for Counter {
    fn write(&mut self, _data: &Packet) {
        self.i += 1;

        if self.i % 10 == 0 {
            println!("{} {:?}", self.i, self.time.elapsed());

            self.time = Instant::now();
        }
    }
}
