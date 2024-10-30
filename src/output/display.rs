use std::collections::HashMap;

use crate::{
    settings,
    types::{packet::Packet, settings::Exchange},
};

use super::OutputTrait;

pub struct DisplayOutput {
    exchange: Exchange,
    subscribed_tokens: Vec<u64>,
    latest_map: HashMap<u64, Packet>,
}

impl DisplayOutput {
    pub fn new() -> DisplayOutput {
        let settings = settings::get();

        DisplayOutput {
            exchange: settings.exchange,
            subscribed_tokens: settings.subscribed_tokens.clone(),
            latest_map: HashMap::new(),
        }
    }
}

impl OutputTrait for DisplayOutput {
    fn write(&mut self, data: &Packet) {
        let token = i32::from_be_bytes(data.0[0..4].try_into().unwrap());

        if self.subscribed_tokens.contains(&(token as u64)) {
            self.latest_map.insert(token as u64, *data);
        }

        print!("\x1B[2J\x1B[1;1H");
        println!("{:#?}", self.latest_map);
    }
}
