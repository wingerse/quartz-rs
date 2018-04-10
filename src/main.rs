extern crate base64;
extern crate byteorder;
#[macro_use]
extern crate quick_error;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate uuid;

use std::net::{Ipv4Addr, SocketAddr};

mod nbt;
mod binary;
mod text;
mod proto;
mod server;
mod network;
mod world;
mod collections;

fn main() {
    let mut s = match server::Server::new(SocketAddr::new(Ipv4Addr::new(127, 0, 0, 1).into(), 25565)) {
        Ok(s) => s,
        Err(e) => panic!("error creating server: {}", e),
    };


    if let Err(e) = s.start() {
        panic!("error starting server: {}", e);
    }
}

// join game
// plugin message
// server difficulty
// spawn position
// player abilities
// held item change
// statistics
// player list item x 2
// player position and look
// world border
// time update
// window items
// set slot
// map chunk bulk
// entity metadata
// update health
// set experience
// 
