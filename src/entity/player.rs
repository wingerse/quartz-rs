use std::sync::{Arc, Mutex};
use std::sync::mpsc::{Receiver, Sender};
use std::net::SocketAddr;
use std::collections::HashMap;
use std::time::{Instant, Duration};

use uuid::Uuid;

use proto::packets::{CPacket, SPacket, SPlayPlayerListItemDataAction, SPlayPlayerListItemData};
use math::Vec3;
use world::chunk::ChunkPos;
use text::{self, ChatPos};
use text::chat::Chat;
use util;

#[derive(Default)]
pub struct PacketList {
    /// this is sent to all players
    pub to_all_players: Vec<SPacket>,
    /// a list of entries where every entry is a list of packets and a player to not send these to.
    pub to_all_player_except: HashMap<Uuid, Vec<SPacket>>,
}

#[derive(Debug)]
pub struct Player {
    name: String,
    uuid: Uuid,
    connected: Arc<Mutex<bool>>,
    packet_recv_queue: Receiver<CPacket>,
    packet_send_queue: Sender<Arc<SPacket>>,
    ip: SocketAddr,
    pos: Vec3,
    ping: i32,
    last_keep_alive: i32,
    time_of_last_keep_alive: Instant,
    yaw: f64,
    pitch: f64,
    on_ground: bool,
}

impl Player {
    pub fn new(name: String,
               uuid: Uuid,
               packet_send_queue: Sender<Arc<SPacket>>,
               packet_recv_queue: Receiver<CPacket>,
               ip: SocketAddr,
               connected: Arc<Mutex<bool>>,
    ) -> Player {
        Player {
            name,
            uuid,
            connected,
            packet_recv_queue,
            packet_send_queue,
            ip,
            pos: Vec3::default(),
            ping: 0,
            last_keep_alive: 0,
            time_of_last_keep_alive: Instant::now(),
            yaw: 0.0,
            pitch: 0.0,
            on_ground: true,
        }
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

    pub fn tick(&mut self, tick: u64, packet_list: &mut PacketList) {
        self.handle_client_packets(packet_list);

        let i = Instant::now();

        if (i - self.time_of_last_keep_alive).as_secs() >= 2 {
            self.send_packet(Arc::new(SPacket::PlayKeepAlive { id: tick as i32 }));
            self.last_keep_alive = tick as i32;
            self.time_of_last_keep_alive = i;
        }
    }

    fn handle_client_packets(&mut self, packet_list: &mut PacketList) {
        loop {
            let p = self.packet_recv_queue.try_recv();
            match p {
                Ok(p) => match p {
                    CPacket::PlayKeepAlive { id } => {
                        if id == self.last_keep_alive {
                            let current = Instant::now();
                            self.ping = util::duration_total_ms(current - self.time_of_last_keep_alive) as i32 / 2;
                            self.time_of_last_keep_alive = current;

                            packet_list.to_all_players.push(SPacket::PlayPlayerListItem {
                                players: vec![Arc::new(SPlayPlayerListItemData {
                                    action: SPlayPlayerListItemDataAction::UpdateLatency { ping: self.ping },
                                    uuid: self.uuid,
                                })],
                            });
                        }
                    }
                    CPacket::PlayChatMessage { message } => {
                        packet_list.to_all_players.push(SPacket::PlayChatMessage {
                            position: ChatPos::Normal,
                            message: Chat::from(text::parse_legacy_ex(&format!("{} > {}", self.name, message), '&')),
                        });
                    }
                    CPacket::PlayUseEntity { target, data } => {

                    }
                    CPacket::PlayPlayer { on_ground } => {
                        self.on_ground = true;
                    }
                    CPacket::PlayPlayerPosition { x, feet_y, z, on_ground } => {}
                    CPacket::PlayPlayerLook { yaw, pitch, on_ground } => {}
                    CPacket::PlayPlayerPositionAndLook { x, feet_y, z, yaw, pitch, on_ground } => {}
                    CPacket::PlayPlayerDigging { status, location, face } => {}
                    CPacket::PlayPlayerBlockPlacement { location, face, held_item, cursor_pos_x, cursor_pos_y, cursor_pos_z } => {}
                    CPacket::PlayHeldItemChange { slot } => {}
                    CPacket::PlayAnimation {} => {}
                    CPacket::PlayEntityAction { entity_id, action_id, action_param } => {}
                    CPacket::PlaySteerVehicle { sideways, forward, flags } => {}
                    CPacket::PlayCloseWindow { window_id } => {}
                    CPacket::PlayClickWindow { window_id, slot, button, action_num, mode, clicked_item } => {}
                    CPacket::PlayConfirmTransaction { window_id, action_num, accepted } => {}
                    CPacket::PlayCreativeInventoryAction { slot, clicked_item } => {}
                    CPacket::PlayEnchantItem { window_id, enchantment } => {}
                    CPacket::PlayUpdateSign { location, line1, line2, line3, line4 } => {}
                    CPacket::PlayPlayerAbilities { flags, flying_speed, walking_speed } => {}
                    CPacket::PlayTabComplete { text, pos } => {}
                    CPacket::PlayClientSettings { locale, view_distance, chat_mode, chat_colors, displayed_skin_parts } => {}
                    CPacket::PlayClientStatus { action_id } => {}
                    CPacket::PlayPluginMessage { channel, data } => {}
                    CPacket::PlaySpectate { target_player } => {}
                    CPacket::PlayResourcePackStatus { hash, result } => {}
                    _ => unreachable!(),
                }
                Err(_) => break,
            }
        }
    }
}