use std::fs;

use fastlib::Decoder;

use crate::settings;

use super::Distribute;

pub struct McxDistributor {
    decoder: Decoder,
}

// Required for Decoder, safe because is used by only single thread
unsafe impl Send for McxDistributor {}

impl McxDistributor {
    pub fn new() -> Self {
        let settings = settings::get().clone();
        let template = fs::read_to_string(&settings.fast_template.expect("Fast template path required for mcx")).unwrap();

        let decoder = Decoder::new_from_xml(&template).unwrap();
        
        Self {
            decoder
        }
    }
}

impl Distribute for McxDistributor {
    fn distribute(&mut self, packet: crate::types::packet::Packet) {}
}
