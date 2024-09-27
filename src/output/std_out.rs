use crate::types::packet::Packet;

use super::OutputTrait;

pub struct StdOut {}

impl StdOut {
    pub fn new() -> StdOut {
        StdOut {}
    }
}

impl OutputTrait for StdOut {
    fn write(&mut self, data: Packet) {
        println!("{}", String::from_utf8_lossy(&data.0));
    }
}
