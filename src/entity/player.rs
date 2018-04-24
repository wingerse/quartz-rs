use std::sync::{Arc, Mutex};
use std::sync::mpsc::{Receiver, Sender};
use std::net::SocketAddr;
use std::collections::HashMap;
use std::time::{Instant, Duration};

use uuid::Uuid;

use proto::packets::{CPacket, SPacket, SPlayPlayerListItemDataAction, SPlayPlayerListItemData};
use math::Vec3;
use world::chunk::{ChunkPos, Chunk};
use world::{World, Dimension, ChunkRectangle};
use server::{PacketList, ServerInfo};
use text::{self, ChatPos};
use text::chat::Chat;
use util;

#[derive(Debug)]
pub struct Player {
    name: String,
    uuid: Uuid,

    connected: Arc<Mutex<bool>>,
    packet_recv_queue: Receiver<CPacket>,
    packet_send_queue: Sender<Arc<SPacket>>,

    ip: SocketAddr,
    ping: i32,
    last_keep_alive: i32,
    time_of_last_keep_alive: Instant,

    pos: Vec3,
    yaw: f64,
    pitch: f64,
    on_ground: bool,
    dimension: Dimension,
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

            ping: 0,
            last_keep_alive: 0,
            time_of_last_keep_alive: Instant::now(),

            pos: Vec3::new(7.5, 82.0, 7.5),
            yaw: 0.0,
            pitch: 0.0,
            on_ground: true,
            dimension: Dimension::End,
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

    pub fn get_dimension(&self) -> Dimension {
        self.dimension
    }

    pub fn get_chunk_rectangle(&self, view_distance: u8) -> ChunkRectangle {
        let chunk_pos = self.get_chunk_pos();
        ChunkRectangle::new(ChunkPos::new(chunk_pos.x - view_distance as i32, chunk_pos.z - view_distance as i32),
                            ChunkPos::new(chunk_pos.x + view_distance as i32, chunk_pos.z + view_distance as i32))
    }

    pub fn send_packet(&mut self, p: Arc<SPacket>) {
        // ignore because if other side is dropped,
        // it's due to error and player will have to disconnect anyway
        let _ = self.packet_send_queue.send(p);
    }

    pub fn get_pos(&self) -> Vec3 {
        self.pos
    }

    pub fn get_yaw(&self) -> f64 {
        self.yaw
    }

    pub fn get_pitch(&self) -> f64 {
        self.pitch
    }

    pub fn get_chunk_pos(&self) -> ChunkPos {
        ChunkPos::new((self.pos.x / 16.0).floor() as i32, (self.pos.z / 16.0).floor() as i32)
    }

    pub fn tick(&mut self, server_info: &ServerInfo, world: &mut World, packet_list: &mut PacketList) {
        self.handle_client_packets(server_info, world, packet_list);

        let i = Instant::now();

        if (i - self.time_of_last_keep_alive).as_secs() >= 2 {
            self.send_packet(Arc::new(SPacket::PlayKeepAlive { id: server_info.tick as i32 }));
            self.last_keep_alive = server_info.tick as i32;
            self.time_of_last_keep_alive = i;
        }
    }

    fn handle_motion_recv(&mut self, x: f64, y: f64, z: f64, yaw: f64, pitch: f64, on_ground: bool,
                          moved: bool, rotated: bool,
                          server_info: &ServerInfo, world: &mut World,
                          packet_list: &mut PacketList) {
        if rotated {
            self.yaw = yaw;
            self.pitch = pitch;
        }

        if moved {
            let prev_chunk = self.get_chunk_pos();
            let prev_chunk_rect = self.get_chunk_rectangle(server_info.view_distance);
            self.pos.x = x;
            self.pos.y = y;
            self.pos.z = z;
            let new_chunk = self.get_chunk_pos();
            let new_chunk_rect = self.get_chunk_rectangle(server_info.view_distance);

            if new_chunk != prev_chunk {
                for chunk_pos in new_chunk_rect.subtract_iter(prev_chunk_rect) {
                    let chk = world.get_chunk(chunk_pos);
                    self.send_packet(Arc::new(chk.to_proto_chunk_data(Chunk::FULL_BIT_MASK)));
                }
                for chunk_pos in prev_chunk_rect.subtract_iter(new_chunk_rect) {
                    let chk = world.get_chunk(chunk_pos);
                    self.send_packet(Arc::new(chk.empty_proto_chunk_data()));
                }
            }
        }

        self.on_ground = on_ground;
    }

    fn handle_client_packets(&mut self, server_info: &ServerInfo, world: &mut World, packet_list: &mut PacketList) {
        loop {
            let p = self.packet_recv_queue.try_recv();
            match p {
                Ok(p) => match p {
                    CPacket::PlayKeepAlive { id } => {
                        if id == self.last_keep_alive {
                            let current = Instant::now();
                            self.ping = util::duration_total_ms(current - self.time_of_last_keep_alive) as i32 / 2;
                            self.time_of_last_keep_alive = current;

                            packet_list.to_all_players.push_back(SPacket::PlayPlayerListItem {
                                players: vec![Arc::new(SPlayPlayerListItemData {
                                    action: SPlayPlayerListItemDataAction::UpdateLatency { ping: self.ping },
                                    uuid: self.uuid,
                                })],
                            });
                        }
                    }
                    CPacket::PlayChatMessage { message } => {
                        packet_list.to_all_players.push_back(SPacket::PlayChatMessage {
                            position: ChatPos::Normal,
                            message: Chat::from(text::parse_legacy_ex(&format!("{} > {}", self.name, message), '&')),
                        });
                    }
                    CPacket::PlayUseEntity { target, data } => {}
                    CPacket::PlayPlayer { on_ground } => {
                        let (x, y, z, yaw, pitch, on_ground) = (self.pos.x, self.pos.y, self.pos.z, self.yaw, self.pitch, on_ground);
                        self.handle_motion_recv(x, y, z, yaw, pitch, on_ground,
                                                false, false,
                                                server_info, world,
                                                packet_list);
                    }
                    CPacket::PlayPlayerPosition { x, feet_y, z, on_ground } => {
                        let (x, y, z, yaw, pitch, on_ground) = (x, feet_y, z, self.yaw, self.pitch, on_ground);
                        self.handle_motion_recv(x, y, z, yaw, pitch, on_ground,
                                                true, false,
                                                server_info, world,
                                                packet_list);
                    }
                    CPacket::PlayPlayerLook { yaw, pitch, on_ground } => {
                        let (x, y, z, yaw, pitch, on_ground) = (self.pos.x, self.pos.y, self.pos.z, yaw as f64, pitch as f64, on_ground);
                        self.handle_motion_recv(x, y, z, yaw, pitch, on_ground,
                                                false, true,
                                                server_info, world,
                                                packet_list);
                    }
                    CPacket::PlayPlayerPositionAndLook { x, feet_y, z, yaw, pitch, on_ground } => {
                        let (x, y, z, yaw, pitch, on_ground) = (x, feet_y, z, yaw as f64, pitch as f64, on_ground);
                        self.handle_motion_recv(x, y, z, yaw, pitch, on_ground,
                                                true, true,
                                                server_info,
                                                world, packet_list);
                    }
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