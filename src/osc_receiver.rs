extern crate rosc;

use rosc::OscPacket;

pub fn handle_packet(packet: OscPacket) {
    match packet {
        OscPacket::Message(msg) => {
            println!("OSC address: {}", msg.addr);
            println!("OSC args: {:?}", msg.args);
        }
        _ => {
            println!("Auch!");
        }
//        OscPacket::Bundle(bundle) => {
//            println!("OSC Bundle: (:?)", bundle);
//        }
    }
}
