use math::Vec3;
use uuid::Uuid;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{Receiver, Sender};
use proto::packets::{CPacket, SPacket};
use std::net::SocketAddr;

#[derive(Debug)]
pub struct Player {
    name: String,
    uuid: Uuid,
    connected: Arc<Mutex<bool>>,
    packet_recv_queue: Receiver<CPacket>,
    packet_send_queue: Sender<SPacket>,
    ip: SocketAddr,
}

impl Player {
    pub fn new(name: String,
               uuid: Uuid,
               packet_send_queue: Sender<SPacket>,
               packet_recv_queue: Receiver<CPacket>,
               ip: SocketAddr,
               connected: Arc<Mutex<bool>>,
    ) -> Player {
        Player { name, uuid, connected, packet_recv_queue, packet_send_queue, ip }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_uuid(&self) -> Uuid {
        self.uuid
    }

    pub fn get_connected(&self) -> bool {
        *self.connected.lock().unwrap()
    }

    pub fn set_connected(&mut self, c: bool) {
        *self.connected.lock().unwrap() = c;
    }
}