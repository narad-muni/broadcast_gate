use std::{mem::MaybeUninit, net::Ipv4Addr};

use socket2::Socket;

use crate::{
    constants::{BUF_SIZE, UNRECOVERABLE_ERROR_KINDS}, global::INPUT_QUEUE, settings, types::packet::Packet,
    utils::{byte_utils::uninit_to_buf, udp_utils::build_socket},
};

enum SocketType {
    Primary,
    Secondary,
}

pub struct UdpInput<'a> {
    primary: Socket,
    secondary: Socket,
    current: Option<&'a Socket>,
    current_id: SocketType,
    auto_switch: bool,
    source_ip: Ipv4Addr,
}

impl<'a> UdpInput<'a> {
    pub fn new() -> UdpInput<'a> {
        let settings = settings::get();

        // Set timeout 0 if autoswitch is false
        let timeout = if settings.udp_auto_switch == true {
            settings.udp_switch_timeout
        } else {
            0
        } as u64;

        UdpInput {
            primary: build_socket(
                &settings.primary_mcast_ip,
                &settings.udp_local_ip,
                settings.primary_mcast_port as u16,
                timeout,
            ),
            secondary: build_socket(
                &settings.secondary_mcast_ip,
                &settings.udp_local_ip,
                settings.secondary_mcast_port as u16,
                timeout,
            ),
            current: None,
            current_id: SocketType::Primary,
            auto_switch: settings.udp_auto_switch,
            source_ip: settings.source_ip.parse().unwrap(),
        }
    }

    pub fn read(&'a mut self) {
        // Set current to primary
        self.current = Some(&self.primary);

        loop {
            // let mut buf = Packet([0; BUF_SIZE]);
            let mut buf: [MaybeUninit<u8>; BUF_SIZE] = unsafe { MaybeUninit::uninit().assume_init() };

            // Value can never be none
            debug_assert!(self.current.is_some());
            
            match self.current.unwrap().recv_from(&mut buf) {
                Ok((_, addr)) => {
                    // Drop packet if source ip doesn't match
                    if *addr.as_socket_ipv4().unwrap().ip() != self.source_ip {
                        continue;
                    }
                }
                Err(e) => {
                    // Check for client side errors
                    if UNRECOVERABLE_ERROR_KINDS.contains(&e.kind()) {
                        panic!("Unrecoverable io error in udp input: {}", e);
                    }

                    // If autoswitch is false, then don't rotate
                    if !self.auto_switch {
                        continue;
                    }

                    // Switch current id
                    self.current_id = match self.current_id {
                        SocketType::Primary => SocketType::Secondary,
                        SocketType::Secondary => SocketType::Primary,
                    };

                    // Based on current id select new
                    self.current = match self.current_id {
                        SocketType::Primary => Some(&self.primary),
                        SocketType::Secondary => Some(&self.secondary),
                    };

                    continue;
                }
            }

            let packet = Packet(uninit_to_buf(&buf), BUF_SIZE);
            INPUT_QUEUE.push(packet);
        }
    }
}
