use std::{mem::offset_of, sync::Arc, thread};

use crossbeam::queue::SegQueue;
use web_view::Content;

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

pub struct DepthView {
    message_code: Vec<i32>,
    subscribed_tokens: Vec<u64>,
    mq: Arc<SegQueue<Packet>>,
}

impl DepthView {
    pub fn new() -> DepthView {
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

        let mq2 = mq.clone();
        thread::spawn(move || {
            let html = include_str!("../../depth_view/dist/index.html");

            let mut webview = web_view::builder()
                .title("My Project")
                .content(Content::Html(html))
                // .content(Content::Url("http://localhost:5173"))
                .size(960, 540)
                .resizable(true)
                .debug(true)
                .user_data(())
                .invoke_handler(|webview, arg| Ok(()))
                .build()
                .unwrap();

            webview.eval("let customEvent;").unwrap();
            let handle = webview.handle();

            // Continuously get data from queue and send to webview
            thread::spawn(move || loop {
                if let Some(packet) = mq2.pop() {
                    handle
                        .dispatch(move |webview| {
                            webview
                                .eval(&format!(
                                    "customEvent = new CustomEvent(\"depth_data_event\", {{
                                        detail: {}
                                    }});
                                    document.dispatchEvent(customEvent);",
                                    serde_json::to_string(bytes_to_struct_ptr::<
                                        TagMarketPictureBroadcast,
                                    >(
                                        &packet.0
                                    ))
                                    .unwrap()
                                ))
                                .unwrap();
                            Ok(())
                        })
                        .unwrap();
                }
            });

            webview.run().unwrap();
        });

        DepthView {
            message_code,
            subscribed_tokens: settings.subscribed_tokens.clone(),
            mq,
        }
    }
}

impl OutputTrait for DepthView {
    fn write(&mut self, data: &Packet) {
        // get message code
        let message_code = i32::from_le_bytes(data.0[0..4].try_into().unwrap());

        if !self.message_code.contains(&message_code) {
            // println!("Invalid message code: {}", message_code);
            return;
        }

        // Get token
        let token_offset = offset_of!(TagMarketPictureBroadcast, token);
        let token_sz = size_of::<i64>();

        let token = i64::from_le_bytes(
            data.0[token_offset..token_offset + token_sz]
                .try_into()
                .unwrap(),
        );

        if !self.subscribed_tokens.is_empty() && !self.subscribed_tokens.contains(&(token as u64)) {
            // println!("Invalid token: {}", token);
            return;
        }

        self.mq.push(*data);
    }
}
