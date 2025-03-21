mod bindings;

use bindings::wasi::sockets::instance_network::instance_network;
use bindings::wasi::sockets::network::{IpAddressFamily, IpSocketAddress, Ipv4SocketAddress};
use bindings::wasi::sockets::udp::{IncomingDatagram, OutgoingDatagram};
use bindings::wasi::sockets::udp_create_socket::create_udp_socket;

/// The main entry point for the UDP echo server.
struct Component;
bindings::export!(Component with_types_in bindings);
impl bindings::exports::wasi::cli::run::Guest for Component {
    fn run() -> Result<(), ()> {
        // Create the UDP socket
        let family = IpAddressFamily::Ipv4;
        let socket = create_udp_socket(family).unwrap();

        // Bind the socket to an address
        let network = instance_network();
        let addr = IpSocketAddress::Ipv4(Ipv4SocketAddress {
            address: (127, 0, 0, 1),
            port: 8080,
        });
        socket.start_bind(&network, addr).unwrap();
        socket.subscribe().block();
        socket.finish_bind().unwrap();

        // Begin receiving packets
        let (receiver, sender) = socket.stream(None).unwrap();
        loop {
            // Wait for packets to be received
            receiver.subscribe().block();
            let max_results = 1024;
            let data = receiver.receive(max_results).unwrap();
            let data: Vec<_> = data.into_iter().map(to_outgoing_datagram).collect();
            let mut data = data.as_slice();

            // Echo the packets back to the sender
            loop {
                // 1. Check if the sender is ready
                let n = sender.check_send().unwrap();
                if n == 0 {
                    sender.subscribe().block();
                }

                // 2. Send as much data as possible
                let n = data.len().min(n as usize);
                let (ready, remainder) = data.split_at(n);
                sender.send(ready).unwrap();

                // 3. Loop again if more data needs to be sent
                data = remainder;
                if data.is_empty() {
                    break;
                }
            }
        }
    }
}

/// Convert an incoming datagram to an outgoing datagram
fn to_outgoing_datagram(incoming: IncomingDatagram) -> OutgoingDatagram {
    OutgoingDatagram {
        data: incoming.data,
        remote_address: Some(incoming.remote_address),
    }
}
