use std::sync::{Arc, Mutex};
use std::sync::mpsc::{Receiver, Sender};
use std::net::SocketAddr;

use uuid::Uuid;

use proto::packets::{CPacket, SPacket};
use math::Vec3;
use world::chunk::ChunkPos;

#[derive(Debug)]
pub struct Player {
    name: String,
    uuid: Uuid,
    connected: Arc<Mutex<bool>>,
    packet_recv_queue: Receiver<CPacket>,
    packet_send_queue: Sender<Arc<SPacket>>,
    ip: SocketAddr,
    pos: Vec3,
}

impl Player {
    pub fn new(name: String,
               uuid: Uuid,
               packet_send_queue: Sender<Arc<SPacket>>,
               packet_recv_queue: Receiver<CPacket>,
               ip: SocketAddr,
               connected: Arc<Mutex<bool>>,
    ) -> Player {
        Player { name, uuid, connected, packet_recv_queue, packet_send_queue, ip, pos: Vec3::default() }
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

    pub fn send_packet(&mut self, p: Arc<SPacket>) {
        // ignore because if other side is dropped,
        // it's due to error and player will have to disconnect anyway
        let _ = self.packet_send_queue.send(p);
    }

    pub fn get_pos(&self) -> Vec3 {
        self.pos
    }

    pub fn set_pos(&mut self, pos: Vec3) {
        self.pos = pos;
    }

    pub fn get_chunk_pos(&self) -> ChunkPos {
        ChunkPos::new((self.pos.x / 16.0) as i32, (self.pos.z / 16.0) as i32)
    }
}