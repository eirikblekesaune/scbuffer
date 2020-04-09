extern crate rosc;

use rosc::{OscMessage, OscPacket};

pub fn handle_packet(packet: OscPacket) {
    match packet {
        OscPacket::Message(msg) => {
            println!("OSC address: {}", msg.addr);
            println!("OSC args: {:?}", msg.args);
            parse_message(msg);
        }
        _ => {
            println!("Auch!");
        }
//        OscPacket::Bundle(bundle) => {
//            println!("OSC Bundle: (:?)", bundle);
//        }
    }
}

fn parse_message(msg: OscMessage) {
    let path: String = msg.addr;
    match path.as_str() {
        "/b_alloc" => {
            println!("I got /b_alloc");
        }
        "/b_allocRead" => {
            println!("I got /b_allocRead");
        }
        "/b_allocReadChannel" => {
            println!("I got /b_allocReadChannel");
        }
        "/b_read" => {
            println!("Got /b_read");
        }
        "/b_readChannel" => {
            println!("I got /b_readChannel");
        }
        "/b_write" => {
            println!("I got /b_write");
        }
        "/b_free" => {
            println!("I got /b_free");
        }
        "/b_zero" => {
            println!("I got /b_zero");
        }
        "/b_set" => {
            println!("I got /b_set");
        }
        "/b_setn" => {
            println!("I got /b_setn");
        }
        "/b_fill" => {
            println!("I got /b_fill");
        }
        "/b_gen" => {
            println!("I got /b_gen");
        }
        "/b_close" => {
            println!("I got /b_close");
        }
        "/b_query" => {
            println!("I got /b_query");
        }
        "/b_get" => {
            println!("I got /b_get");
        }
        "/b_getn" => {
            println!("I got /b_getn");
        }
        _ => {
            println!("Got uknown addr: {}", path);
        }
    }
}
