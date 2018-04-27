use std::sync::{Arc, Mutex};
use std::sync::mpsc::{Receiver, Sender};
use std::net::SocketAddr;
use std::collections::{HashMap, HashSet};
use std::time::{Instant, Duration};

use uuid::Uuid;

use proto::packets::{CPacket, SPacket, SPlayPlayerListItemDataAction, SPlayPlayerListItemData};
use math::Vec3;
use world::chunk::{ChunkPos, Chunk, BlockID};
use world::{World, Dimension, ChunkRectangle, LevelType, BlockPos};
use entity::metadata::{EntityMetadata, MetadataEntry};
use server::{ServerContext, ServerInfo, Gamemode, Difficulty, TICKS_PER_SEC};
use server::playerlist::PlayerList;
use text::{self, ChatPos, Code};
use text::chat::Chat;
use binary::double_to_fixed_point;
use proto::data::SlotData;
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
    join_tick: u64,

    pos: Vec3,
    yaw: f64,
    pitch: f64,
    on_ground: bool,

    dimension: Dimension,
    gamemode: Gamemode,
    entity_id: i32,
    players_in_vicinity: HashSet<Uuid>,
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
            join_tick: 0,

            pos: Vec3::new(7.5, 82.0, 7.5),
            yaw: 0.0,
            pitch: 0.0,
            on_ground: true,

            dimension: Dimension::Overworld,
            gamemode: Gamemode::Creative,
            entity_id: 0,
            players_in_vicinity: HashSet::new(),
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_uuid(&self) -> Uuid {
        self.uuid
    }

    pub fn get_ping(&self) -> i32 {
        self.ping
    }

    pub fn set_join_tick(&mut self, join_tick: u64) {
        self.join_tick = join_tick;
    }

    pub fn get_dimension(&self) -> Dimension {
        self.dimension
    }

    pub fn get_gamemode(&self) -> Gamemode {
        self.gamemode
    }

    pub fn get_connected(&self) -> bool {
        *self.connected.lock().unwrap()
    }

    pub fn set_connected(&mut self, c: bool) {
        *self.connected.lock().unwrap() = c;
    }

    pub fn get_entity_id(&self) -> i32 {
        self.entity_id
    }

    pub fn set_entity_id(&mut self, entity_id: i32) {
        self.entity_id = entity_id;
    }

    /// Creates a chunk rectangle centered around this player whith given view distance.
    pub fn get_chunk_rectangle(&self, view_distance: u8) -> ChunkRectangle {
        ChunkRectangle::centered(self.get_chunk_pos(), view_distance)
    }

    pub fn send_packet(&mut self, p: Arc<SPacket>) {
        // ignore because if other side is dropped,
        // it's due to error and player will have to disconnect anyway
        let _ = self.packet_send_queue.send(p);
    }

    fn send_packet_to_all_players(&mut self, ctx: &ServerContext, packet: Arc<SPacket>) {
        self.send_packet(Arc::clone(&packet));
        ctx.player_list.send_packet_to_players_except(self.uuid, Arc::clone(&packet));
    }

    fn send_packet_to_vicinity(&self, ctx: &ServerContext, packet: Arc<SPacket>) {
        for &p in &self.players_in_vicinity {
            ctx.player_list.send_packet_to_player(p, Arc::clone(&packet));
        }
    }

    fn send_packet_to_chunk_vicinity(&self, chunk_pos: ChunkPos, ctx: &mut ServerContext, packet: Arc<SPacket>) {
        let chunk = ctx.world.get_chunk(chunk_pos, self.uuid);
        for &p in chunk.players_in_vicinity_iter() {
            if p != self.uuid {
                ctx.player_list.send_packet_to_player(p, Arc::clone(&packet));
            }
        }
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
        self.pos.into()
    }

    pub fn tick(&mut self, ctx: &mut ServerContext) {
        self.handle_client_packets(ctx);

        // every 2 secs.
        if (ctx.server_info.tick - self.join_tick) % (TICKS_PER_SEC * 2) == 0 {
            self.send_packet(Arc::new(SPacket::PlayKeepAlive { id: ctx.server_info.tick as i32 }));
            self.last_keep_alive = ctx.server_info.tick as i32;
            self.time_of_last_keep_alive = Instant::now();
        }

        self.update_players_in_vicinity(ctx);
    }

    fn update_players_in_vicinity(&mut self, ctx: &mut ServerContext) {
        let mut new_who_can_see = HashSet::new();
        let rect = self.get_chunk_rectangle(ctx.server_info.player_view_distance);
        for chunk_pos in rect.chunks_iter() {
            let chunk = ctx.world.get_chunk(chunk_pos, self.uuid);
            for &p in chunk.players_iter().filter(|&&p| p != self.uuid) {
                new_who_can_see.insert(p);
            }
        }

        let spawn_packet = Arc::new(SPacket::PlaySpawnPlayer {
            entity_id: self.entity_id,
            uuid: self.uuid,
            x: self.pos.x,
            y: self.pos.y,
            z: self.pos.z,
            yaw: self.yaw,
            pitch: self.pitch,
            current_item: 0,
            metadata: {
                let mut metadata = EntityMetadata::new();
                metadata.insert(6, MetadataEntry::Float(20.0));
                metadata
            },
        });
        for &new_player in new_who_can_see.difference(&self.players_in_vicinity) {
            ctx.player_list.send_packet_to_player(new_player, Arc::clone(&spawn_packet));
        }

        let destroy_packket = Arc::new(SPacket::PlayDestroyEntities { entity_ids: vec![self.entity_id] });
        for &gone_player in self.players_in_vicinity.difference(&new_who_can_see) {
            ctx.player_list.send_packet_to_player(gone_player, Arc::clone(&destroy_packket));
        }

        self.players_in_vicinity = new_who_can_see;
    }

    /// Joins the player to server. Sends chunks, announces to other players, etc..
    pub fn join(&mut self, ctx: &mut ServerContext) {
        let (entity_id, gamemode, dimension) = (self.entity_id, self.gamemode, self.dimension);
        self.send_packet(Arc::new(SPacket::PlayJoinGame {
            entity_id,
            gamemode: gamemode as u8,
            dimension: dimension as i8,
            difficulty: ctx.server_info.difficulty as u8,
            max_players: 100,
            level_type: ctx.server_info.level_type.as_str(),
            reduced_debug_info: false,
        }));
        self.send_packet(Arc::new(SPacket::PlayPluginMessage {
            channel: "MC|Brand",
            data: "Quartz".as_bytes().to_vec(),
        }));
        self.send_packet(Arc::new(SPacket::PlayServerDifficulty {
            difficulty: ctx.server_info.difficulty as u8,
        }));
        self.send_packet(Arc::new(SPacket::PlayPlayerAbilities {
            flags: 0x08 | 0x04,
            flying_speed: 0.05,
            field_of_view_modifier: 1.0,
        }));

        self.send_packet(Arc::new(SPacket::PlayPlayerListHeaderAndFooter {
            header: Chat::from(text::parse_legacy_ex("&4&lQuartz server", '&')),
            footer: Chat::from(text::parse_legacy_ex("&6&lAll hail Emperor", '&')),
        }));

        let list_item_data = Arc::new(
            SPlayPlayerListItemData {
                uuid: self.uuid,
                action: SPlayPlayerListItemDataAction::AddPlayer {
                    name: self.name.clone(),
                    gamemode: self.gamemode as i32,
                    ping: self.ping,
                    properties: Vec::new(),
                    display_name: None,
                },
            }
        );

        ctx.player_list.send_packet_to_all_players(Arc::new(SPacket::PlayPlayerListItem { players: vec![Arc::clone(&list_item_data)] }));
        let join_msg = Chat::from(text::parse_legacy(&format!("{}{} joined the game!", Code::Yellow, self.name)));
        ctx.player_list.send_packet_to_all_players(Arc::new(SPacket::PlayChatMessage {
            position: ChatPos::Normal,
            message: join_msg.clone(),
        }));

        // for newly joined player, we need to send all other players too.
        let mut list_item_datas = vec![list_item_data];
        for p in ctx.player_list.iter() {
            let p = p.borrow();
            list_item_datas.push(Arc::new(SPlayPlayerListItemData {
                uuid: p.uuid,
                action: SPlayPlayerListItemDataAction::AddPlayer {
                    name: p.name.clone(),
                    gamemode: p.gamemode as i32,
                    ping: p.ping,
                    properties: Vec::new(),
                    display_name: None,
                },
            }));
        }

        self.send_packet(Arc::new(SPacket::PlayPlayerListItem { players: list_item_datas }));
        self.send_packet(Arc::new(SPacket::PlayChatMessage {
            position: ChatPos::Normal,
            message: join_msg,
        }));

        self.send_initial_chunks(ctx);
        ctx.world.get_chunk(self.get_chunk_pos(), self.uuid).insert_player(self.uuid);

        self.send_packet(Arc::new(SPacket::PlaySpawnPosition {
            location: BlockPos::new(0, 80, 0),
        }));
        let (x, y, z, yaw, pitch) = (self.pos.x, self.pos.y, self.pos.z, self.yaw, self.pitch);
        self.send_packet(Arc::new(SPacket::PlayPlayerPositionAndLook {
            x,
            y,
            z,
            yaw: yaw as f32,
            pitch: pitch as f32,
            flags: 0,
        }));
    }

    fn send_initial_chunks(&mut self, ctx: &mut ServerContext) {
        let chunk_rect = self.get_chunk_rectangle(ctx.server_info.view_distance);

        let (uuid, dimension) = (self.uuid, self.dimension);
        let sky_light_sent = ctx.world.get_properties().has_sky_light();

        util::iter_foreach_every(chunk_rect.chunks_iter()
                                           .map(|pos| ctx.world.get_chunk(pos, uuid).to_proto_map_chunk_bulk_data()),
                                 |i| i % 8 == 0 && i != 0,
                                 |q| {
                                     let mut chunks = Vec::new();
                                     while let Some(chunk) = q.pop_front() {
                                         chunks.push(chunk);
                                     }
                                     self.send_packet(Arc::new(SPacket::PlayMapChunkBulk {
                                         sky_light_sent,
                                         chunks,
                                     }));
                                 });
    }

    /// Despawns this player to nearby players, unloads chunks if required, and announces leave. This should be called when player is disconnected.
    pub fn leave(self, ctx: &mut ServerContext) {
        ctx.player_list.send_packet_to_all_players(Arc::new(SPacket::PlayPlayerListItem {
            players: vec![Arc::new(SPlayPlayerListItemData {
                uuid: self.uuid,
                action: SPlayPlayerListItemDataAction::RemovePlayer,
            })]
        }));

        ctx.player_list.send_packet_to_all_players(Arc::new(SPacket::PlayChatMessage {
            position: ChatPos::Normal,
            message: Chat::from(text::parse_legacy(&format!("{}{} left the game",
                                                            Code::Yellow,
                                                            self.name))),
        }));

        ctx.world.get_chunk(self.get_chunk_pos(), self.uuid).remove_player(&self.uuid);

        let destroy_packet = Arc::new(SPacket::PlayDestroyEntities { entity_ids: vec![self.entity_id] });
        for &p in &self.players_in_vicinity {
            ctx.player_list.send_packet_to_player(p, Arc::clone(&destroy_packet));
        }

        for chunk_pos in self.get_chunk_rectangle(ctx.server_info.view_distance).chunks_iter() {
            ctx.world.unload_chunk_if_required(chunk_pos, self.uuid);
        }
    }

    fn handle_motion_recv(&mut self, x: f64, y: f64, z: f64, yaw: f64, pitch: f64, on_ground: bool,
                          moved: bool, rotated: bool,
                          ctx: &mut ServerContext) {
        let (prev_fp_x, prev_fp_y, prev_fp_z) = (double_to_fixed_point(self.pos.x), double_to_fixed_point(self.pos.y), double_to_fixed_point(self.pos.z));
        let (fp_x, fp_y, fp_z) = (double_to_fixed_point(x), double_to_fixed_point(y), double_to_fixed_point(z));
        let (delta_fp_x, delta_fp_y, delta_fp_z) = (fp_x - prev_fp_x, fp_y - prev_fp_y, fp_z - prev_fp_z);

        fn fp_fits_byte(fp: i32) -> bool { fp >= -128 && fp <= 127 }

        if rotated {
            self.yaw = yaw;
            self.pitch = pitch;
        }

        if moved {
            let prev_chunk = self.get_chunk_pos();
            let prev_chunk_rect = self.get_chunk_rectangle(ctx.server_info.view_distance);
            self.pos.x = x;
            self.pos.y = y;
            self.pos.z = z;
            let new_chunk = self.get_chunk_pos();
            let new_chunk_rect = self.get_chunk_rectangle(ctx.server_info.view_distance);

            if new_chunk != prev_chunk {
                ctx.world.get_chunk(prev_chunk, self.uuid).remove_player(&self.uuid);
                ctx.world.get_chunk(new_chunk, self.uuid).insert_player(self.uuid);
                let uuid = self.uuid;
                let sky_light_sent = ctx.world.get_properties().has_sky_light();
                util::iter_foreach_every(new_chunk_rect.subtract_iter(prev_chunk_rect).map(|pos| {
                    ctx.world.get_chunk(pos, uuid).to_proto_map_chunk_bulk_data()
                }),
                                         |i| i % 8 == 0 && i != 0,
                                         |q| {
                                             let mut chunks = Vec::new();
                                             while let Some(chunk) = q.pop_front() {
                                                 chunks.push(chunk);
                                             }
                                             self.send_packet(Arc::new(SPacket::PlayMapChunkBulk {
                                                 sky_light_sent,
                                                 chunks,
                                             }));
                                         });
                for chunk_pos in prev_chunk_rect.subtract_iter(new_chunk_rect) {
                    ctx.world.unload_chunk_if_required(chunk_pos, self.uuid);
                    self.send_packet(Arc::new(Chunk::empty_proto_chunk_data(chunk_pos)));
                }
            }
        }

        self.on_ground = on_ground;

        let mut packets = Vec::new();

        if moved && fp_fits_byte(delta_fp_x) && fp_fits_byte(delta_fp_y) && fp_fits_byte(delta_fp_z) {
            if rotated {
                packets.push(Arc::new(SPacket::PlayEntityLookAndRelativeMove {
                    entity_id: self.entity_id,
                    delta_x: delta_fp_x as i8,
                    delta_y: delta_fp_y as i8,
                    delta_z: delta_fp_z as i8,
                    yaw: self.yaw,
                    pitch: self.pitch,
                    on_ground: self.on_ground,
                }));
            } else {
                packets.push(Arc::new(SPacket::PlayEntityRelativeMove {
                    entity_id: self.entity_id,
                    delta_x: delta_fp_x as i8,
                    delta_y: delta_fp_y as i8,
                    delta_z: delta_fp_z as i8,
                    on_ground: self.on_ground,
                }));
            }
        } else if (rotated && moved) || moved {
            packets.push(Arc::new(SPacket::PlayEntityTeleport {
                entity_id: self.entity_id,
                x: self.pos.x,
                y: self.pos.y,
                z: self.pos.z,
                yaw: self.yaw,
                pitch: self.pitch,
                on_ground: self.on_ground,
            }));
        } else {
            packets.push(Arc::new(SPacket::PlayEntityLook {
                entity_id: self.entity_id,
                yaw: self.yaw,
                pitch: self.pitch,
                on_ground: self.on_ground,
            }));
        }
        if rotated {
            packets.push(Arc::new(SPacket::PlayEntityHeadLook {
                entity_id: self.entity_id,
                head_yaw: self.yaw,
            }));
        }


        for &p in &self.players_in_vicinity {
            for packet in &packets {
                ctx.player_list.send_packet_to_player(p, Arc::clone(packet));
            }
        }
    }

    fn handle_client_packets(&mut self, ctx: &mut ServerContext) {
        loop {
            let p = self.packet_recv_queue.try_recv();
            match p {
                Ok(p) => match p {
                    CPacket::PlayKeepAlive { id } => {
                        if id == self.last_keep_alive {
                            let current = Instant::now();
                            self.ping = util::duration_total_ms(current - self.time_of_last_keep_alive) as i32 / 2;
                            self.time_of_last_keep_alive = current;

                            let (uuid, ping) = (self.uuid, self.ping);
                            self.send_packet_to_all_players(ctx, Arc::new(SPacket::PlayPlayerListItem {
                                players: vec![Arc::new(SPlayPlayerListItemData {
                                    action: SPlayPlayerListItemDataAction::UpdateLatency { ping },
                                    uuid,
                                })],
                            }));
                        }
                    }
                    CPacket::PlayChatMessage { message } => {
                        let message = Chat::from(text::parse_legacy_ex(&format!("{} > {}", self.name, message), '&'));
                        self.send_packet_to_all_players(ctx, Arc::new(SPacket::PlayChatMessage {
                            position: ChatPos::Normal,
                            message,
                        }));
                    }
                    // CPacket::PlayUseEntity { target, data } => {}
                    CPacket::PlayPlayer { on_ground } => {
                        let (x, y, z, yaw, pitch, on_ground) = (self.pos.x, self.pos.y, self.pos.z, self.yaw, self.pitch, on_ground);
                        self.handle_motion_recv(x, y, z, yaw, pitch, on_ground,
                                                false, false,
                                                ctx);
                    }
                    CPacket::PlayPlayerPosition { x, feet_y, z, on_ground } => {
                        let (x, y, z, yaw, pitch, on_ground) = (x, feet_y, z, self.yaw, self.pitch, on_ground);
                        self.handle_motion_recv(x, y, z, yaw, pitch, on_ground,
                                                true, false,
                                                ctx);
                    }
                    CPacket::PlayPlayerLook { yaw, pitch, on_ground } => {
                        let (x, y, z, yaw, pitch, on_ground) = (self.pos.x, self.pos.y, self.pos.z, yaw as f64, pitch as f64, on_ground);
                        self.handle_motion_recv(x, y, z, yaw, pitch, on_ground,
                                                false, true,
                                                ctx);
                    }
                    CPacket::PlayPlayerPositionAndLook { x, feet_y, z, yaw, pitch, on_ground } => {
                        let (x, y, z, yaw, pitch, on_ground) = (x, feet_y, z, yaw as f64, pitch as f64, on_ground);
                        self.handle_motion_recv(x, y, z, yaw, pitch, on_ground,
                                                true, true,
                                                ctx);
                    }
                    CPacket::PlayPlayerDigging { status, location, face } => {
                        match status {
                            0 => {
                                if self.gamemode == Gamemode::Creative {
                                    ctx.world.set_block(location, BlockID::AIR);
                                    let chunk_pos = ChunkPos::from(location);
                                    let packet = Arc::new(SPacket::PlayBlockChange {
                                        location,
                                        block_id: BlockID::AIR.to_u16() as i32,
                                    });
                                    self.send_packet_to_chunk_vicinity(chunk_pos, ctx, packet);
                                }
                            },
                            _ => (),
                        }
                    }
                    CPacket::PlayPlayerBlockPlacement { location, face, held_item, cursor_pos_x, cursor_pos_y, cursor_pos_z } => {
                        if self.gamemode == Gamemode::Creative {
                            if let SlotData::Some{block_id, item_damage, ..} = held_item {
                                let block = BlockID::new(block_id as u8, item_damage as u8);

                                ctx.world.set_block(location, block);

                                let chunk_pos = ChunkPos::from(location);
                                let packet = Arc::new(SPacket::PlayBlockChange {
                                    location,
                                    block_id: block.to_u16() as i32,
                                });
                                self.send_packet_to_chunk_vicinity(chunk_pos, ctx, packet);
                            }
                        }
                    }
                    /*CPacket::PlayHeldItemChange { slot } => {}*/
                    CPacket::PlayAnimation {} => {
                        self.send_packet_to_vicinity(ctx, Arc::new(SPacket::PlayAnimation {
                            entity_id: self.entity_id,
                            animation: 0,
                        }));
                    }
                    CPacket::PlayEntityAction { entity_id, action_id, action_param } => {
                        match action_id {
                            0 => {
                                for &p in &self.players_in_vicinity {
                                    ctx.player_list.send_packet_to_player(p, Arc::new(SPacket::PlayEntityMetadata {
                                        entity_id: self.entity_id,
                                        metadata: {
                                            let mut metadata = EntityMetadata::new();
                                            metadata.insert(0, MetadataEntry::Byte(0x02));
                                            metadata
                                        }
                                    }));
                                }
                            },
                            1 => {
                                for &p in &self.players_in_vicinity {
                                    ctx.player_list.send_packet_to_player(p, Arc::new(SPacket::PlayEntityMetadata {
                                        entity_id: self.entity_id,
                                        metadata: {
                                            let mut metadata = EntityMetadata::new();
                                            metadata.insert(0, MetadataEntry::Byte(0));
                                            metadata
                                        }
                                    }));
                                }
                            },
                            _ => (),
                        }
                    }
                    /*CPacket::PlaySteerVehicle { sideways, forward, flags } => {}
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
                    CPacket::PlayResourcePackStatus { hash, result } => {}*/
                    x @ _ => println!("{:?}", x),
                }
                Err(_) => break,
            }
        }
    }
}