mod osc_receiver;

use std::env;
use std::net::{SocketAddrV4, UdpSocket};
use std::str::FromStr;



fn main() {
    println!("I am scbuffer, hear me roar!");

    let addr = match SocketAddrV4::from_str("127.0.0.1:1234") {
        Ok(addr) => addr,
        Err(_) => panic!("OISANN!")
    };
    let sock = UdpSocket::bind(addr).unwrap();
    println!("Listening to {}", addr);

    let mut buf = [0u8; rosc::decoder::MTU];

    loop{
        match sock.recv_from(&mut buf) {
            Ok((size, addr)) => {
                println!("Received packet with size {} from {}", size, addr);
                let packet = rosc::decoder::decode(&buf[..size]).unwrap();
                osc_receiver::handle_packet(packet);
            }
            Err(e) => {
                println!("Error receiving from socket: {}", e);
                break;
            }
        }
    }
}
