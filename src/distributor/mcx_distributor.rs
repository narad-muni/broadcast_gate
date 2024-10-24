use std::fs;

use bytes::Bytes;
use fastlib::{Decoder, ModelFactory};
use serde::Deserialize;

use crate::{settings, types::packet_structures::mcx::Message};

use super::Distribute;

pub struct McxDistributor {
    decoder: Decoder,
}

// Required for Decoder, safe because is used by only single thread
unsafe impl Send for McxDistributor {}

impl McxDistributor {
    pub fn new() -> Self {
        let settings = settings::get().clone();
        let template = fs::read_to_string(
            &settings
                .fast_template
                .expect("Fast template path required for mcx"),
        )
        .unwrap();

        let decoder = Decoder::new_from_xml(&template).unwrap();

        Self { decoder }
    }
}

impl Distribute for McxDistributor {
    fn distribute(&mut self, packet: crate::types::packet::Packet) {
        let mut raw = Bytes::from(packet.0[packet.1..].to_owned());

        loop {
            let mut msg = ModelFactory::new();
            let err = self.decoder.decode_reader(&mut raw, &mut msg);

            if err.is_err() {
                println!("Error: {:?}", err);
                break;
            }

            let st = Message::deserialize(msg.data.unwrap().clone());

            if st.is_err() {
                println!("Error: {:?}", st);
                break;
            }

            let st = st.unwrap();

            println!("{:#?}", st);

            if let Message::FastReset(_) = st {
                self.decoder.reset();
            }

            if raw.is_empty() {
                break;
            }
        }
    }
}
