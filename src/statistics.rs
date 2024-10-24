use std::{cell::UnsafeCell, io};

use crate::global::STATISTICS;

pub struct StatisticsData {
    pub udp_packets_count: u64,
    pub other_packets_count: u64,
    pub mbo_packets_count: u64,
    pub filtered_packets_count: u64,
}

pub struct Statistics {
    inner: UnsafeCell<StatisticsData>,
}

unsafe impl Send for Statistics {}
unsafe impl Sync for Statistics {}

impl Statistics {
    pub const fn new() -> Self {
        Self {
            inner: UnsafeCell::new(StatisticsData {
                udp_packets_count: 0,
                other_packets_count: 0,
                mbo_packets_count: 0,
                filtered_packets_count: 0,
            }),
        }
    }

    pub fn get(&self) -> &mut StatisticsData {
        unsafe { &mut *self.inner.get() }
    }

    pub fn run() {
        let mut command = String::new();

        loop {
            Self::list_options();
            io::stdin().read_line(&mut command).unwrap();

            match command.trim() {
                "1" => println!(
                    "7208 / 7200 / 18705 Packets Count : {}",
                    STATISTICS.get().mbo_packets_count
                ),
                "2" => println!(
                    "Other Packets Processed : {}",
                    STATISTICS.get().other_packets_count
                ),
                "3" => println!("Total UDP Packets : {}", STATISTICS.get().udp_packets_count),
                "4" => println!(
                    "Filtered UDP Packets : {}",
                    STATISTICS.get().filtered_packets_count
                ),
                _ => println!("Unknown command"),
            }

            io::stdin().read_line(&mut command).unwrap();
            command.clear();
        }
    }

    pub fn list_options() {
        // Clears terminal
        print!("\x1B[2J\x1B[1;1H");

        let options = r#"
Select an option
1. 7208 / 7200 / 18705 Packets Count
2. Other Packets Processed
3. Total UDP Packets
4. Filtered UDP Packets
        "#;

        println!("{options}");
    }
}
