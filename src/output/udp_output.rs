use std::{io::Write, net::SocketAddrV4};

use socket2::{SockAddr, Socket};

use crate::{settings, types::packet::Packet, utils::udp_utils::build_socket};

use super::OutputTrait;
pub struct UdpOutput {
    socket: Socket,
}

impl UdpOutput {
    pub fn new() -> UdpOutput {
        let settings = settings::get();

        // Build a udp socket with mcast connection
        let socket = build_socket(
            &settings.output_udp_ip,
            &settings.udp_local_ip,
            settings.output_udp_port as u16,
            0,
        );

        // Connect to mcast ip
        // Required for sending packets
        socket
            .connect(&SockAddr::from(SocketAddrV4::new(
                settings.output_udp_ip.parse().unwrap(),
                settings.output_udp_port as u16,
            )))
            .unwrap();

        UdpOutput { socket }
    }
}

impl OutputTrait for UdpOutput {
    fn write(&mut self, data: &Packet) {
        let slice = &data.0[..data.1];

        self.socket.write(slice).unwrap();
    }
}
