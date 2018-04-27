extern crate base64;
extern crate byteorder;
#[macro_use]
extern crate quick_error;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate uuid;
extern crate flate2;
extern crate failure;
#[macro_use]
extern crate failure_derive;

mod nbt;
mod binary;
mod text;
mod proto;
mod server;
mod network;
mod world;
mod collections;
mod entity;
mod math;
mod inventory;
mod util;

use std::net::{Ipv4Addr, SocketAddr};

fn main() {
    let mut s = match server::Server::new(SocketAddr::new(Ipv4Addr::new(0, 0, 0, 0).into(), 25565)) {
        Ok(s) => s,
        Err(e) => panic!("error creating server: {}", e),
    };

    if let Err(e) = s.start() {
        panic!("error starting server: {}", e);
    }
}