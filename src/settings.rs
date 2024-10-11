use std::{env, fs};

use crate::{global::{EXCHANGE, SETTINGS}, types::settings::Settings};

pub fn init() {
    let args = env::args().collect::<Vec<String>>();

    let settings_path = args.get(1).expect("Please provide config file path");

    // Read settings to string
    let settings = fs::read_to_string(settings_path).expect("Cannot find config file");

    // Create struct from settings file
    let settings =
        serde_json::from_str::<Settings>(&settings).expect("Unable to parse settings file");

    // Initialize settings
    SETTINGS.get_or_init(|| settings);

    // Set global exchange
    unsafe{
        EXCHANGE = &SETTINGS.get().unwrap().exchange;
    }
}

pub fn get() -> &'static Settings {
    // Initialize settings
    SETTINGS.get().unwrap()
}
