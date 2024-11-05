use std::{
    net::TcpListener,
    sync::{Arc, Mutex},
    thread,
};

use crossbeam::queue::SegQueue;
use tungstenite::{accept, Message};

use crate::{
    constants::{
        BCAST_MBO_MBP, BCAST_ONLY_MBP, BCAST_ONLY_MBP_EQ, BSE_BCAST_MBP, SNAPSHOT_TEMPLATE_ID,
    },
    settings,
    types::{
        packet::Packet, packet_structures::depth_output::TagMarketPictureBroadcast,
        settings::Exchange,
    },
    utils::byte_utils::bytes_to_struct_ptr,
};

use super::OutputTrait;

pub struct Ws {
    message_code: Vec<i32>,
    queue: Arc<SegQueue<Packet>>,
}

impl Ws {
    pub fn new() -> Ws {
        let settings = settings::get();

        let message_code = match settings.exchange {
            Exchange::BSE => vec![BSE_BCAST_MBP as i32],
            Exchange::NEQ | Exchange::NFO | Exchange::NCD => {
                vec![
                    BCAST_ONLY_MBP as i32,
                    BCAST_ONLY_MBP_EQ as i32,
                    BCAST_MBO_MBP as i32,
                ]
            }
            Exchange::MCX => vec![SNAPSHOT_TEMPLATE_ID as i32],
        };
        let mq: Arc<SegQueue<Packet>> = Arc::new(SegQueue::new());
        let clients = Arc::new(Mutex::new(vec![]));

        let ws_url = settings.ws.as_ref().expect("Please provide `ws` in config");

        // Thread for handling new connections
        {
            let clients = clients.clone();
            thread::spawn(move || {
                let tcp_stream = TcpListener::bind(ws_url).unwrap();

                for stream in tcp_stream.incoming() {
                    match stream {
                        Ok(stream) => match accept(stream) {
                            Ok(ws) => {
                                println!("Connected");
                                clients.lock().unwrap().push(ws)
                            }
                            _ => {
                                println!("Error accept");
                            }
                        },
                        _ => {
                            println!("Error stream");
                        }
                    }
                }
            });
        }

        // Thread for sneding data
        {
            let clients = clients.clone();
            let mq = mq.clone();

            thread::spawn(move || loop {
                if let Some(packet) = mq.pop() {
                    let json = serde_json::to_string(bytes_to_struct_ptr::<
                        TagMarketPictureBroadcast,
                    >(&packet.0))
                    .unwrap();

                    clients.lock().unwrap().retain_mut(|client| {
                        let err = client.send(Message::text(&json));

                        err.is_ok()
                    });
                }
            });
        }

        Ws {
            message_code,
            queue: mq,
        }
    }
}

impl OutputTrait for Ws {
    fn write(&mut self, data: &Packet) {
        // get message code
        let message_code = i32::from_le_bytes(data.0[0..4].try_into().unwrap());

        if !self.message_code.contains(&message_code) {
            // println!("Invalid message code: {}", message_code);
            return;
        }

        self.queue.push(*data);
    }
}
