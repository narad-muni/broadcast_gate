use crate::types::packet::Packet;

use super::OutputTrait;

pub struct StdOut {}

impl StdOut {
    pub fn new() -> StdOut {
        StdOut {}
    }
}

impl OutputTrait for StdOut {
    fn write(&mut self, data: &Packet) {
        let slice = &data.0[..data.1];

        println!("{}", String::from_utf8_lossy(slice));
    }
}
