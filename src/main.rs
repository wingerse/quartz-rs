extern crate byteorder;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate quick_error;
extern crate uuid;

mod nbt;
mod binary;
mod text;
mod proto;

use std::io::{BufReader};
use std::net::{TcpListener, TcpStream};
use proto::{Reader, Writer};
use proto::packets::*;
use text::chat::{Chat, wrap, StringComponent};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:25565").unwrap();
    loop {
        let stream = listener.accept().unwrap();
        println!("{}", stream.1);
        handle_client(stream.0);    
    }
}

fn handle_client(stream: TcpStream) {
    let mut reader = Reader::new(BufReader::new(stream.try_clone().unwrap()));
    let mut writer = Writer::new(stream.try_clone().unwrap());

    let mut packet = reader.read_packet().unwrap();
    match packet {
        CPacket::Handshake{..} => {
            println!("{:?}", packet);
        },
        _ => panic!(),
    }
    reader.set_state(proto::State::Status);

    packet = reader.read_packet().unwrap();
    match packet {
        CPacket::StatusRequest {} => {},
        _ => panic!(),
    }

    let response = SPacket::StatusResponse {
        data: SStatusResponseData {
            version: SStatusResponseVersion {
                name: "1.8".into(), 
                protocol: proto::VERSION
            },
            players: SStatusResponsePlayers {
                max: 1000,
                online: 1,
                sample: None,
            },
            description: text::chat::Chat(wrap(StringComponent {text: "A minecraft server".into(), base: Default::default()})),
            favicon: None,
        },
    };

    writer.write_packet(&response).unwrap();

    packet = reader.read_packet().unwrap();
    let payload = match packet {
        CPacket::StatusPing {payload} => payload,
        _ => panic!(),
    };

    writer.write_packet(&SPacket::StatusPong {payload}).unwrap();
}
