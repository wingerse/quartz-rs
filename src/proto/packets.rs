use std::io;
use std::io::Read;
use std::sync::{Arc, Mutex};
use std::collections::HashSet;

use serde_json;
use uuid::Uuid;

use binary::*;
use nbt::NBT;
use proto::{data, Error, Result, State};
use entity::metadata::EntityMetadata;
use text;
use text::chat::Chat;
use world::BlockPos;

#[derive(Debug)]
pub enum CPacket {
    Handshake {
        protocol_version: i32,
        server_address: String,
        server_port: u16,
        next_state: i32,
    },
    PlayKeepAlive {
        id: i32,
    },
    PlayChatMessage {
        message: String,
    },
    PlayUseEntity {
        target: i32,
        data: CPlayUseEntityData,
    },
    PlayPlayer {
        on_ground: bool,
    },
    PlayPlayerPosition {
        x: f64,
        feet_y: f64,
        z: f64,
        on_ground: bool,
    },
    PlayPlayerLook {
        yaw: f32,
        pitch: f32,
        on_ground: bool,
    },
    PlayPlayerPositionAndLook {
        x: f64,
        feet_y: f64,
        z: f64,
        yaw: f32,
        pitch: f32,
        on_ground: bool,
    },
    PlayPlayerDigging {
        status: i8,
        location: BlockPos,
        face: i8,
    },
    PlayPlayerBlockPlacement {
        location: BlockPos,
        face: i8,
        held_item: data::SlotData,
        cursor_pos_x: i8,
        cursor_pos_y: i8,
        cursor_pos_z: i8,
    },
    PlayHeldItemChange {
        slot: i16,
    },
    PlayAnimation {},
    PlayEntityAction {
        entity_id: i32,
        action_id: i32,
        action_param: i32,
    },
    PlaySteerVehicle {
        sideways: f32,
        forward: f32,
        flags: u8,
    },
    PlayCloseWindow {
        window_id: u8,
    },
    PlayClickWindow {
        window_id: u8,
        slot: i16,
        button: i8,
        action_num: i16,
        mode: i8,
        clicked_item: data::SlotData,
    },
    PlayConfirmTransaction {
        window_id: i8,
        action_num: i16,
        accepted: bool,
    },
    PlayCreativeInventoryAction {
        slot: i16,
        clicked_item: data::SlotData,
    },
    PlayEnchantItem {
        window_id: i8,
        enchantment: i8,
    },
    PlayUpdateSign {
        location: BlockPos,
        line1: Chat,
        line2: Chat,
        line3: Chat,
        line4: Chat,
    },
    PlayPlayerAbilities {
        flags: i8,
        flying_speed: f32,
        walking_speed: f32,
    },
    PlayTabComplete {
        text: String,
        pos: Option<BlockPos>,
    },
    PlayClientSettings {
        locale: String,
        view_distance: i8,
        chat_mode: i8,
        chat_colors: bool,
        displayed_skin_parts: u8,
    },
    PlayClientStatus {
        action_id: i32,
    },
    PlayPluginMessage {
        channel: String,
        data: Vec<u8>,
    },
    PlaySpectate {
        target_player: Uuid,
    },
    PlayResourcePackStatus {
        hash: String,
        result: i32,
    },
    StatusRequest {},
    StatusPing {
        payload: i64,
    },
    LoginLoginStart {
        name: String,
    },
}

impl CPacket {
    pub fn read(r: &mut &[u8], state: State, id: i32) -> Result<Self> {
        match state {
            State::Handshake => {
                match id {
                    0 => Ok(CPacket::Handshake {
                            protocol_version: read_varint(r)?,
                            server_address: data::read_string(r)?,
                            server_port: read_ushort(r)?,
                            next_state: read_varint(r)?,
                    }),
                    _ => Err(Error::InvalidPacketId(id))
                }
            },
            State::Play => {
                match id {
                    0 => Ok(CPacket::PlayKeepAlive{id: read_varint(r)?}),
                    1 => Ok(CPacket::PlayChatMessage{message: data::read_string(r)?}),
                    2 => {
                        let target = read_varint(r)?;
                        let typ = read_varint(r)?;
                        let data = match typ {
                            0 => CPlayUseEntityData::Interact,
                            1 => CPlayUseEntityData::Attack,
                            2 => CPlayUseEntityData::InteractAt {
                                target_x: read_float(r)?,
                                target_y: read_float(r)?,
                                target_z: read_float(r)?,
                            },
                            _ => CPlayUseEntityData::Unknown,
                        };
                        Ok(CPacket::PlayUseEntity{target, data})
                    },
                    3 => Ok(CPacket::PlayPlayer{on_ground: read_bool(r)?}),
                    4 => Ok(CPacket::PlayPlayerPosition{
                            x: read_double(r)?,
                            feet_y: read_double(r)?,
                            z: read_double(r)?,
                            on_ground: read_bool(r)?,
                    }),
                    5 => Ok(CPacket::PlayPlayerLook {
                            yaw: read_float(r)?,
                            pitch: read_float(r)?,
                            on_ground: read_bool(r)?,
                    }),
                    6 => Ok(CPacket::PlayPlayerPositionAndLook {
                            x: read_double(r)?,
                            feet_y: read_double(r)?,
                            z: read_double(r)?,
                            yaw: read_float(r)?,
                            pitch: read_float(r)?,
                            on_ground: read_bool(r)?,
                    }),
                    7 => Ok(CPacket::PlayPlayerDigging {
                            status: read_byte(r)?,
                            location: BlockPos::read_proto(r)?,
                            face: read_byte(r)?,
                    }),
                    8 => Ok(CPacket::PlayPlayerBlockPlacement {
                            location: BlockPos::read_proto(r)?,
                            face: read_byte(r)?,
                            held_item: data::SlotData::read(r)?,
                            cursor_pos_x: read_byte(r)?,
                            cursor_pos_y: read_byte(r)?,
                            cursor_pos_z: read_byte(r)?,
                    }),
                    9 => Ok(CPacket::PlayHeldItemChange {
                        slot: read_ishort(r)?,
                    }),
                    10 => Ok(CPacket::PlayAnimation{}),
                    11 => Ok(CPacket::PlayEntityAction {
                        entity_id: read_varint(r)?,
                        action_id: read_varint(r)?,
                        action_param: read_varint(r)?,
                    }),
                    12 => Ok(CPacket::PlaySteerVehicle {
                        sideways: read_float(r)?,
                        forward: read_float(r)?,
                        flags: read_ubyte(r)?,
                    }),
                    13 => Ok(CPacket::PlayCloseWindow {
                        window_id: read_ubyte(r)?,
                    }),
                    14 => Ok(CPacket::PlayClickWindow {
                        window_id: read_ubyte(r)?,
                        slot: read_ishort(r)?,
                        button: read_byte(r)?,
                        action_num: read_ishort(r)?,
                        mode: read_byte(r)?,
                        clicked_item: data::SlotData::read(r)?,
                    }),
                    15 => Ok(CPacket::PlayConfirmTransaction {
                        window_id: read_byte(r)?,
                        action_num: read_ishort(r)?,
                        accepted: read_bool(r)?,
                    }),
                    16 => Ok(CPacket::PlayCreativeInventoryAction {
                        slot: read_ishort(r)?,
                        clicked_item: data::SlotData::read(r)?,
                    }),
                    17 => Ok(CPacket::PlayEnchantItem {
                        window_id: read_byte(r)?,
                        enchantment: read_byte(r)?,
                    }),
                    18 => Ok(CPacket::PlayUpdateSign {
                        location: BlockPos::read_proto(r)?,
                        line1: Chat::read_proto(r)?,
                        line2: Chat::read_proto(r)?,
                        line3: Chat::read_proto(r)?,
                        line4: Chat::read_proto(r)?,
                    }),
                    19 => Ok(CPacket::PlayPlayerAbilities {
                        flags: read_byte(r)?,
                        flying_speed: read_float(r)?,
                        walking_speed: read_float(r)?,
                    }),
                    20 => Ok(CPacket::PlayTabComplete {
                        text: data::read_string(r)?,
                        pos: {
                            let has = read_bool(r)?;
                            if has {
                                Some(BlockPos::read_proto(r)?)
                            } else {
                                None
                            }
                        }
                    }),
                    21 => Ok(CPacket::PlayClientSettings {
                        locale: data::read_string(r)?,
                        view_distance: read_byte(r)?,
                        chat_mode: read_byte(r)?,
                        chat_colors: read_bool(r)?,
                        displayed_skin_parts: read_ubyte(r)?,
                    }),
                    22 => Ok(CPacket::PlayClientStatus {
                        action_id: read_varint(r)?,
                    }),
                    23 => Ok(CPacket::PlayPluginMessage {
                        channel: data::read_string(r)?,
                        data: {
                            let mut vec = vec![0u8; r.len()];
                            r.read_exact(&mut vec)?;
                            vec
                        }
                    }),
                    24 => Ok(CPacket::PlaySpectate {
                        target_player: data::read_uuid(r)?,
                    }),
                    25 => Ok(CPacket::PlayResourcePackStatus {
                        hash: data::read_string(r)?,
                        result: read_varint(r)?,
                    }),
                    _ => Err(Error::InvalidPacketId(id))
                }
            },
            State::Status => {
                match id {
                    0 => Ok(CPacket::StatusRequest{}),
                    1 => Ok(CPacket::StatusPing {payload: read_long(r)?}),
                    _ => Err(Error::InvalidPacketId(id))
                }
            },
            State::Login => {
                match id {
                    0 => Ok(CPacket::LoginLoginStart {name: data::read_string(r)?}),
                    _ => Err(Error::InvalidPacketId(id))
                }
            }
        }
    }
}

#[derive(Debug)]
pub enum CPlayUseEntityData {
    Interact,
    Attack,
    InteractAt {
        target_x: f32,
        target_y: f32,
        target_z: f32,
    },
    Unknown
}

#[derive(Debug)]
pub enum SPacket {
    PlayKeepAlive {
        id: i32,
    },
    PlayJoinGame {
        entity_id: i32,
        gamemode: u8,
        dimension: i8,
        difficulty: u8,
        max_players: u8,
        level_type: &'static str,
        reduced_debug_info: bool,
    },
    PlayChatMessage {
        message: Chat,
        position: text::ChatPos,
    },
    PlayTimeUpdate {
        world_age: i64,
        time_of_day: i64,
    },
    PlayEntityEquipment {
        entity_id: i32,
        slot: i16,
        item: data::SlotData,
    },
    PlaySpawnPosition {
        location: BlockPos,
    },
    PlayUpdateHealth {
        health: f32,
        food: i32,
        food_saturation: f32,
    },
    PlayRespawn {
        dimension: i32,
        difficulty: u8,
        gamemode: u8,
        level_type: &'static str,
    },
    PlayPlayerPositionAndLook {
        x: f64,
        y: f64,
        z: f64,
        yaw: f32,
        pitch: f32,
        flags: i8,
    },
    PlayHeldItemChange {
        slot: i8,
    },
    PlayUseBed {
        entity_id: i32,
        location: BlockPos,
    },
    PlayAnimation {
        entity_id: i32,
        animation: u8,
    },
    PlaySpawnPlayer {
        entity_id: i32,
        uuid: Uuid,
        x: f64,
        y: f64,
        z: f64,
        yaw: f64,
        pitch: f64,
        current_item: i16,
        metadata: EntityMetadata,
    },
    PlayCollectItem {
        collected_entity_id: i32,
        collector_entity_id: i32,
    },
    PlaySpawnObject {
        entity_id: i32,
        object_type: i8,
        x: f64,
        y: f64,
        z: f64,
        pitch: f64,
        yaw: f64,
        data: Option<SPlaySpawnObjectData>,
    },
    PlaySpawnMob {
        entity_id: i32,
        mob_type: u8,
        x: f64,
        y: f64,
        z: f64,
        yaw: f64,
        pitch: f64,
        head_pitch: f64,
        velocity_x: i16,
        velocity_y: i16,
        velocity_z: i16,
        metadata: EntityMetadata,
    },
    PlaySpawnPainting {
        entity_id: i32,
        title: String,
        location: BlockPos,
        direction: u8,
    },
    PlaySpawnExperienceOrb {
        entity_id: i32,
        x: f64,
        y: f64,
        z: f64,
        count: i16,
    },
    PlayEntityVelocity {
        entity_id: i32,
        velocity_x: i16,
        velocity_y: i16,
        velocity_z: i16,
    },
    PlayDestroyEntities {
        entity_ids: Vec<i32>,
    },
    PlayEntity {
        entity_id: i32,
    },
    PlayEntityRelativeMove {
        entity_id: i32,
        delta_x: i8,
        delta_y: i8,
        delta_z: i8,
        on_ground: bool,
    },
    PlayEntityLook {
        entity_id: i32,
        yaw: f64,
        pitch: f64,
        on_ground: bool,
    },
    PlayEntityLookAndRelativeMove {
        entity_id: i32,
        delta_x: i8,
        delta_y: i8,
        delta_z: i8,
        yaw: f64,
        pitch: f64,
        on_ground: bool,
    },
    PlayEntityTeleport {
        entity_id: i32,
        x: f64,
        y: f64,
        z: f64,
        yaw: f64,
        pitch: f64,
        on_ground: bool,
    },
    PlayEntityHeadLook {
        entity_id: i32,
        head_yaw: f64,
    },
    PlayEntityStatus {
        entity_id: i32,
        entity_status: i8,
    },
    PlayAttachEntity {
        entity_id: i32,
        vehicle_id: i32,
        leash: bool,
    },
    PlayEntityMetadata {
        entity_id: i32,
        metadata: EntityMetadata,
    },
    PlayEntityEffect {
        entity_id: i32,
        effect_id: i8,
        amplifier: i8,
        duration: i32,
        hide_particles: bool,
    },
    PlayRemoveEntityEffect {
        entity_id: i32,
        effect_id: i8,
    },
    PlaySetExperience {
        experience_bar: f32,
        level: i32,
        total_experience: i32,
    },
    PlayEntityProperties {
        entity_id: i32,
        properties: Vec<SPlayEntityPropertiesData>,
    },
    PlayChunkData {
        chunk_x: i32,
        chunk_z: i32,
        primary_bit_mask: u16,
        data: SPlayChunkDataData,
    },
    PlayMultiBlockChange {
        chunk_x: i32,
        chunk_z: i32,
        data: Vec<SPlayMultiBlockChangeData>,
    },
    PlayBlockChange {
        location: BlockPos,
        block_id: i32,
    },
    PlayBlockAction {
        location: BlockPos,
        byte1: u8,
        byte2: u8,
        block_type: i32,
    },
    PlayBlockBreakAnimation {
        entity_id: i32,
        location: BlockPos,
        destroy_stage: i8,
    },
    PlayMapChunkBulk {
        sky_light_sent: bool,
        chunks: Vec<SPlayMapChunkBulkData>,
    },
    PlayExplosion {
        x: f32,
        y: f32,
        z: f32,
        radius: f32,
        records: Vec<(i8, i8, i8)>,
        player_motion_x: f32,
        player_motion_y: f32,
        player_motion_z: f32,
    },
    PlayEffect {
        effect_id: i32,
        location: BlockPos,
        data: i32,
        disable_relative_volume: bool,
    },
    PlaySoundEffect {
        sound_name: &'static str,
        effect_pos_x: f64,
        effect_pos_y: f64,
        effect_pos_z: f64,
        volume: f32,
        pitch: u8,
    },
    PlayParticle {
        particle_id: i32,
        long_distance: bool,
        x: f32,
        y: f32,
        z: f32,
        offset_x: f32,
        offset_y: f32,
        offset_z: f32,
        particle_data: f32,
        particle_count: i32,
        data: Vec<i32>,
    },
    PlayChangeGameState {
        reason: u8,
        value: f32,
    },
    PlaySpawnGlobalEntity {
        entity_id: i32,
        entity_type: i8,
        x: f64,
        y: f64,
        z: f64,
    },
    PlayOpenWindow {
        window_id: u8,
        window_type: &'static str,
        window_title: Chat,
        slot_num: u8,
        entity_id: i32,
    },
    PlayCloseWindow {
        window_id: u8,
    },
    PlaySetSlot {
        window_id: i8,
        slot: i16,
        slot_data: data::SlotData,
    },
    PlayWindowItems {
        window_id: u8,
        slots: Vec<data::SlotData>,
    },
    PlayWindowProperty {
        window_id: u8,
        property: i16,
        value: i16,
    },
    PlayConfirmTransaction {
        window_id: i8,
        action_num: i16,
        accepted: bool,
    },
    PlayUpdateSign {
        location: BlockPos,
        line1: Chat,
        line2: Chat,
        line3: Chat,
        line4: Chat,
    },
    PlayMap {
        map_id: i32,
        scale: i8,
        icons: Vec<SPlayMapIcon>,
        data: Option<SPlayMapData>,
    },
    PlayUpdateBlockEntity {
        location: BlockPos,
        action: u8,
        nbt_data: NBT,
    },
    PlayOpenSignEditor {
        location: BlockPos,
    },
    PlayStatistics {
        statistics: Vec<(&'static str, i32)>,
    },
    PlayPlayerListItem {
        players: Vec<Arc<SPlayPlayerListItemData>>,
    },
    PlayPlayerAbilities {
        flags: i8,
        flying_speed: f32,
        field_of_view_modifier: f32,
    },
    PlayTabComplete {
        matches: Vec<String>,
    },
    PlayScoreboardObjective {
        objective_name: String,
        data: SPlayScoreboardObjectiveData,
    },
    PlayUpdateScore {
        score_name: String,
        objective_name: String,
        data: SPlayUpdateScoreData,
    },
    PlayDisplayScoreboard {
        position: i8,
        score_name: String,
    },
    PlayTeams {
        team_name: String,
        data: SPlayTeamsData,
    },
    PlayPluginMessage {
        channel: &'static str,
        data: Vec<u8>,
    },
    PlayDisconnect {
        reason: Chat,
    },
    PlayServerDifficulty {
        difficulty: u8,
    },
    PlayCombatEvent {
        data: SPlayCombatEventData,
    },
    PlayCamera {
        camera_id: i32,
    },
    PlayWorldBorder {
        data: SPlayWorldBorderData,
    },
    PlayTitle {
        data: SPlayTitleData,
    },
    PlaySetCompression {
        threshold: i32,
    },
    PlayPlayerListHeaderAndFooter {
        header: Chat,
        footer: Chat,
    },
    PlayResourcePackSend {
        url: String,
        hash: String,
    },
    PlayUpdateEntityNBT {
        entity_id: i32,
        tag: NBT,
    },
    StatusResponse {
        data: SStatusResponseData,
    },
    StatusPong {
        payload: i64,
    },
    LoginDisconnect {
        reason: Chat,
    },
    LoginLoginSuccess {
        uuid: String,
        username: String,
    },
    LoginSetCompression {
        threshold: i32,
    },
}

impl SPacket {
    pub fn id(&self) -> i32 {
        match *self {
            SPacket::PlayKeepAlive { .. } => 0,
            SPacket::PlayJoinGame { .. } => 1,
            SPacket::PlayChatMessage { .. } => 2,
            SPacket::PlayTimeUpdate { .. } => 3,
            SPacket::PlayEntityEquipment { .. } => 4,
            SPacket::PlaySpawnPosition { .. } => 5,
            SPacket::PlayUpdateHealth { .. } => 6,
            SPacket::PlayRespawn { .. } => 7,
            SPacket::PlayPlayerPositionAndLook { .. } => 8,
            SPacket::PlayHeldItemChange { .. } => 9,
            SPacket::PlayUseBed { .. } => 10,
            SPacket::PlayAnimation { .. } => 11,
            SPacket::PlaySpawnPlayer { .. } => 12,
            SPacket::PlayCollectItem { .. } => 13,
            SPacket::PlaySpawnObject { .. } => 14,
            SPacket::PlaySpawnMob { .. } => 15,
            SPacket::PlaySpawnPainting { .. } => 16,
            SPacket::PlaySpawnExperienceOrb { .. } => 18,
            SPacket::PlayEntityVelocity { .. } => 19,
            SPacket::PlayDestroyEntities { .. } => 19,
            SPacket::PlayEntity { .. } => 20,
            SPacket::PlayEntityRelativeMove { .. } => 21,
            SPacket::PlayEntityLook { .. } => 22,
            SPacket::PlayEntityLookAndRelativeMove { .. } => 23,
            SPacket::PlayEntityTeleport { .. } => 24,
            SPacket::PlayEntityHeadLook { .. } => 25,
            SPacket::PlayEntityStatus { .. } => 26,
            SPacket::PlayAttachEntity { .. } => 27,
            SPacket::PlayEntityMetadata { .. } => 28,
            SPacket::PlayEntityEffect { .. } => 29,
            SPacket::PlayRemoveEntityEffect { .. } => 30,
            SPacket::PlaySetExperience { .. } => 31,
            SPacket::PlayEntityProperties { .. } => 32,
            SPacket::PlayChunkData { .. } => 33,
            SPacket::PlayMultiBlockChange { .. } => 34,
            SPacket::PlayBlockChange { .. } => 35,
            SPacket::PlayBlockAction { .. } => 36,
            SPacket::PlayBlockBreakAnimation { .. } => 37,
            SPacket::PlayMapChunkBulk { .. } => 38,
            SPacket::PlayExplosion { .. } => 39,
            SPacket::PlayEffect { .. } => 40,
            SPacket::PlaySoundEffect { .. } => 41,
            SPacket::PlayParticle { .. } => 42,
            SPacket::PlayChangeGameState { .. } => 43,
            SPacket::PlaySpawnGlobalEntity { .. } => 44,
            SPacket::PlayOpenWindow { .. } => 45,
            SPacket::PlayCloseWindow { .. } => 46,
            SPacket::PlaySetSlot { .. } => 47,
            SPacket::PlayWindowItems { .. } => 48,
            SPacket::PlayWindowProperty { .. } => 49,
            SPacket::PlayConfirmTransaction { .. } => 50,
            SPacket::PlayUpdateSign { .. } => 51,
            SPacket::PlayMap { .. } => 52,
            SPacket::PlayUpdateBlockEntity { .. } => 53,
            SPacket::PlayOpenSignEditor { .. } => 54,
            SPacket::PlayStatistics { .. } => 55,
            SPacket::PlayPlayerListItem { .. } => 56,
            SPacket::PlayPlayerAbilities { .. } => 57,
            SPacket::PlayTabComplete { .. } => 58,
            SPacket::PlayScoreboardObjective { .. } => 59,
            SPacket::PlayUpdateScore { .. } => 60,
            SPacket::PlayDisplayScoreboard { .. } => 61,
            SPacket::PlayTeams { .. } => 62,
            SPacket::PlayPluginMessage { .. } => 63,
            SPacket::PlayDisconnect { .. } => 64,
            SPacket::PlayServerDifficulty { .. } => 65,
            SPacket::PlayCombatEvent { .. } => 66,
            SPacket::PlayCamera { .. } => 67,
            SPacket::PlayWorldBorder { .. } => 68,
            SPacket::PlayTitle { .. } => 69,
            SPacket::PlaySetCompression { .. } => 70,
            SPacket::PlayPlayerListHeaderAndFooter { .. } => 71,
            SPacket::PlayResourcePackSend { .. } => 72,
            SPacket::PlayUpdateEntityNBT { .. } => 73,
            SPacket::StatusResponse { .. } => 0,
            SPacket::StatusPong { .. } => 1,
            SPacket::LoginDisconnect { .. } => 0,
            SPacket::LoginLoginSuccess { .. } => 2,
            SPacket::LoginSetCompression { .. } => 3,
        }
    }

    pub fn write<W: io::Write>(&self, w: &mut W) -> io::Result<()> {
        match *self {
            SPacket::PlayKeepAlive { id } => {
                write_varint(w, id)?;
            }
            SPacket::PlayJoinGame {
                entity_id,
                gamemode,
                dimension,
                difficulty,
                max_players,
                level_type,
                reduced_debug_info,
            } => {
                write_int(w, entity_id)?;
                write_ubyte(w, gamemode)?;
                write_byte(w, dimension)?;
                write_ubyte(w, difficulty)?;
                write_ubyte(w, max_players)?;
                data::write_string(w, level_type)?;
                write_bool(w, reduced_debug_info)?;
            }
            SPacket::PlayChatMessage {
                ref message,
                position,
            } => {
                message.write_proto(w)?;
                write_byte(w, position as i8)?;
            }
            SPacket::PlayTimeUpdate {
                world_age,
                time_of_day,
            } => {
                write_long(w, world_age)?;
                write_long(w, time_of_day)?;
            }
            SPacket::PlayEntityEquipment {
                entity_id,
                slot,
                ref item,
            } => {
                write_varint(w, entity_id)?;
                write_ishort(w, slot)?;
                item.write(w)?;
            }
            SPacket::PlaySpawnPosition { location } => {
                location.write_proto(w)?;
            }
            SPacket::PlayUpdateHealth {
                health,
                food,
                food_saturation,
            } => {
                write_float(w, health)?;
                write_varint(w, food)?;
                write_float(w, food_saturation)?;
            }
            SPacket::PlayRespawn {
                dimension,
                difficulty,
                gamemode,
                level_type,
            } => {
                write_int(w, dimension)?;
                write_ubyte(w, difficulty)?;
                write_ubyte(w, gamemode)?;
                data::write_string(w, level_type)?;
            }
            SPacket::PlayPlayerPositionAndLook {
                x,
                y,
                z,
                yaw,
                pitch,
                flags,
            } => {
                write_double(w, x)?;
                write_double(w, y)?;
                write_double(w, z)?;
                write_float(w, yaw)?;
                write_float(w, pitch)?;
                write_byte(w, flags)?;
            }
            SPacket::PlayHeldItemChange { slot } => {
                write_byte(w, slot)?;
            }
            SPacket::PlayUseBed {
                entity_id,
                location,
            } => {
                write_varint(w, entity_id)?;
                location.write_proto(w)?;
            }
            SPacket::PlayAnimation {
                entity_id,
                animation,
            } => {
                write_varint(w, entity_id)?;
                write_ubyte(w, animation)?;
            }
            SPacket::PlaySpawnPlayer {
                entity_id,
                uuid,
                x,
                y,
                z,
                yaw,
                pitch,
                current_item,
                ref metadata,
            } => {
                write_varint(w, entity_id)?;
                data::write_uuid(w, &uuid)?;
                write_int(w, double_to_fixed_point(x))?;
                write_int(w, double_to_fixed_point(y))?;
                write_int(w, double_to_fixed_point(z))?;
                data::write_angle(w, yaw)?;
                data::write_angle(w, pitch)?;
                write_ishort(w, current_item)?;
                metadata.write_proto(w)?;
            }
            SPacket::PlayCollectItem {
                collected_entity_id,
                collector_entity_id,
            } => {
                write_varint(w, collected_entity_id)?;
                write_varint(w, collector_entity_id)?;
            }
            SPacket::PlaySpawnObject {
                entity_id,
                object_type,
                x,
                y,
                z,
                pitch,
                yaw,
                ref data,
            } => {
                write_varint(w, entity_id)?;
                write_byte(w, object_type)?;
                write_int(w, double_to_fixed_point(x))?;
                write_int(w, double_to_fixed_point(y))?;
                write_int(w, double_to_fixed_point(z))?;
                data::write_angle(w, pitch)?;
                data::write_angle(w, yaw)?;
                match *data {
                    Some(ref x) => {
                        write_int(w, x.data)?;
                        write_ishort(w, x.velocity_x)?;
                        write_ishort(w, x.velocity_y)?;
                        write_ishort(w, x.velocity_z)?;
                    }
                    None => write_int(w, 0)?,
                }
            }
            SPacket::PlaySpawnMob {
                entity_id,
                mob_type,
                x,
                y,
                z,
                yaw,
                pitch,
                head_pitch,
                velocity_x,
                velocity_y,
                velocity_z,
                ref metadata,
            } => {
                write_varint(w, entity_id)?;
                write_ubyte(w, mob_type)?;
                write_int(w, double_to_fixed_point(x))?;
                write_int(w, double_to_fixed_point(y))?;
                write_int(w, double_to_fixed_point(z))?;
                data::write_angle(w, yaw)?;
                data::write_angle(w, pitch)?;
                data::write_angle(w, head_pitch)?;
                write_ishort(w, velocity_x)?;
                write_ishort(w, velocity_y)?;
                write_ishort(w, velocity_z)?;
                metadata.write_proto(w)?;
            }
            SPacket::PlaySpawnPainting {
                entity_id,
                ref title,
                location,
                direction,
            } => {
                write_varint(w, entity_id)?;
                data::write_string(w, title)?;
                location.write_proto(w)?;
                write_ubyte(w, direction)?;
            }
            SPacket::PlaySpawnExperienceOrb {
                entity_id,
                x,
                y,
                z,
                count,
            } => {
                write_varint(w, entity_id)?;
                write_int(w, double_to_fixed_point(x))?;
                write_int(w, double_to_fixed_point(y))?;
                write_int(w, double_to_fixed_point(z))?;
                write_ishort(w, count)?;
            }
            SPacket::PlayEntityVelocity {
                entity_id,
                velocity_x,
                velocity_y,
                velocity_z,
            } => {
                write_varint(w, entity_id)?;
                write_ishort(w, velocity_x)?;
                write_ishort(w, velocity_y)?;
                write_ishort(w, velocity_z)?;
            }
            SPacket::PlayDestroyEntities {
                ref entity_ids,
            } => {
                write_varint(w, entity_ids.len() as i32)?;
                for &entity_id in entity_ids.iter() {
                    write_varint(w, entity_id)?;
                }
            }
            SPacket::PlayEntity { entity_id } => {
                write_varint(w, entity_id)?;
            }
            SPacket::PlayEntityRelativeMove {
                entity_id,
                delta_x,
                delta_y,
                delta_z,
                on_ground,
            } => {
                write_varint(w, entity_id)?;
                write_byte(w, delta_x)?;
                write_byte(w, delta_y)?;
                write_byte(w, delta_z)?;
                write_bool(w, on_ground)?;
            }
            SPacket::PlayEntityLook {
                entity_id,
                yaw,
                pitch,
                on_ground,
            } => {
                write_varint(w, entity_id)?;
                data::write_angle(w, yaw)?;
                data::write_angle(w, pitch)?;
                write_bool(w, on_ground)?;
            }
            SPacket::PlayEntityLookAndRelativeMove {
                entity_id,
                delta_x,
                delta_y,
                delta_z,
                yaw,
                pitch,
                on_ground,
            } => {
                write_varint(w, entity_id)?;
                write_byte(w, delta_x)?;
                write_byte(w, delta_y)?;
                write_byte(w, delta_z)?;
                data::write_angle(w, yaw)?;
                data::write_angle(w, pitch)?;
                write_bool(w, on_ground)?;
            }
            SPacket::PlayEntityTeleport {
                entity_id,
                x,
                y,
                z,
                yaw,
                pitch,
                on_ground,
            } => {
                write_varint(w, entity_id)?;
                write_int(w, double_to_fixed_point(x))?;
                write_int(w, double_to_fixed_point(y))?;
                write_int(w, double_to_fixed_point(z))?;
                data::write_angle(w, yaw)?;
                data::write_angle(w, pitch)?;
                write_bool(w, on_ground)?;
            }
            SPacket::PlayEntityHeadLook {
                entity_id,
                head_yaw,
            } => {
                write_varint(w, entity_id)?;
                data::write_angle(w, head_yaw)?;
            }
            SPacket::PlayEntityStatus {
                entity_id,
                entity_status,
            } => {
                write_int(w, entity_id)?;
                write_byte(w, entity_status)?;
            }
            SPacket::PlayAttachEntity {
                entity_id,
                vehicle_id,
                leash,
            } => {
                write_int(w, entity_id)?;
                write_int(w, vehicle_id)?;
                write_bool(w, leash)?;
            }
            SPacket::PlayEntityMetadata {
                entity_id,
                ref metadata,
            } => {
                write_varint(w, entity_id)?;
                metadata.write_proto(w)?;
            }
            SPacket::PlayEntityEffect {
                entity_id,
                effect_id,
                amplifier,
                duration,
                hide_particles,
            } => {
                write_varint(w, entity_id)?;
                write_byte(w, effect_id)?;
                write_byte(w, amplifier)?;
                write_varint(w, duration)?;
                write_bool(w, hide_particles)?;
            }
            SPacket::PlayRemoveEntityEffect {
                entity_id,
                effect_id,
            } => {
                write_varint(w, entity_id)?;
                write_byte(w, effect_id)?;
            }
            SPacket::PlaySetExperience {
                experience_bar,
                level,
                total_experience,
            } => {
                write_float(w, experience_bar)?;
                write_varint(w, level)?;
                write_varint(w, total_experience)?;
            }
            SPacket::PlayEntityProperties {
                entity_id,
                ref properties,
            } => {
                write_varint(w, entity_id)?;
                write_int(w, properties.len() as i32)?;
                for p in properties.iter() {
                    data::write_string(w, &p.key)?;
                    write_double(w, p.value)?;
                    write_varint(w, p.modifiers.len() as i32)?;
                    for m in &p.modifiers {
                        m.write(w)?;
                    }
                }
            }
            SPacket::PlayChunkData {
                chunk_x,
                chunk_z,
                primary_bit_mask,
                ref data,
            } => {
                write_int(w, chunk_x)?;
                write_int(w, chunk_z)?;
                match *data {
                    SPlayChunkDataData::GroundUpContinuous(_) => write_bool(w, true)?,
                    SPlayChunkDataData::GroundUpNonContinuous(_) => write_bool(w, false)?,
                }
                write_ushort(w, primary_bit_mask)?;
                match *data {
                    SPlayChunkDataData::GroundUpContinuous(ref g) => {
                        write_varint(w, g.len() as i32)?;
                        g.write(w)?;
                    }
                    SPlayChunkDataData::GroundUpNonContinuous(ref g) => {
                        write_varint(w, g.len() as i32)?;
                        g.write(w)?;
                    }
                }
            }
            SPacket::PlayMultiBlockChange {
                chunk_x,
                chunk_z,
                ref data,
            } => {
                write_int(w, chunk_x)?;
                write_int(w, chunk_z)?;
                write_varint(w, data.len() as i32)?;
                for d in data.iter() {
                    let b = (d.x << 4) | (d.z & 0x0f);
                    write_ubyte(w, b)?;
                    write_ubyte(w, d.y)?;
                    write_varint(w, d.block_id)?;
                }
            }
            SPacket::PlayBlockChange { location, block_id } => {
                location.write_proto(w)?;
                write_varint(w, block_id)?;
            }
            SPacket::PlayBlockAction {
                location,
                byte1,
                byte2,
                block_type,
            } => {
                location.write_proto(w)?;
                write_ubyte(w, byte1)?;
                write_ubyte(w, byte2)?;
                write_varint(w, block_type)?;
            }
            SPacket::PlayBlockBreakAnimation {
                entity_id,
                location,
                destroy_stage,
            } => {
                write_varint(w, entity_id)?;
                location.write_proto(w)?;
                write_byte(w, destroy_stage)?;
            }
            SPacket::PlayMapChunkBulk {
                sky_light_sent,
                ref chunks,
            } => {
                write_bool(w, sky_light_sent)?;
                write_varint(w, chunks.len() as i32)?;
                for chk in chunks.iter() {
                    write_int(w, chk.chunk_x)?;
                    write_int(w, chk.chunk_z)?;
                    write_ushort(w, chk.primary_bit_mask)?;
                }
                for chk in chunks.iter() {
                    chk.chunk.write(w)?;
                }
            }
            SPacket::PlayExplosion {
                x,
                y,
                z,
                radius,
                ref records,
                player_motion_x,
                player_motion_y,
                player_motion_z,
            } => {
                write_float(w, x)?;
                write_float(w, y)?;
                write_float(w, z)?;
                write_float(w, radius)?;
                write_int(w, records.len() as i32)?;
                for r in records.iter() {
                    write_byte(w, r.0)?;
                    write_byte(w, r.1)?;
                    write_byte(w, r.2)?;
                }
                write_float(w, player_motion_x)?;
                write_float(w, player_motion_y)?;
                write_float(w, player_motion_z)?;
            }
            SPacket::PlayEffect {
                effect_id,
                location,
                data,
                disable_relative_volume,
            } => {
                write_int(w, effect_id)?;
                location.write_proto(w)?;
                write_int(w, data)?;
                write_bool(w, disable_relative_volume)?;
            }
            SPacket::PlaySoundEffect {
                sound_name,
                effect_pos_x,
                effect_pos_y,
                effect_pos_z,
                volume,
                pitch,
            } => {
                data::write_string(w, sound_name)?;
                write_int(w, double_to_fixed_point(effect_pos_x))?;
                write_int(w, double_to_fixed_point(effect_pos_y))?;
                write_int(w, double_to_fixed_point(effect_pos_z))?;
                write_float(w, volume)?;
                write_ubyte(w, pitch)?;
            }
            SPacket::PlayParticle {
                particle_id,
                long_distance,
                x,
                y,
                z,
                offset_x,
                offset_y,
                offset_z,
                particle_data,
                particle_count,
                ref data,
            } => {
                write_int(w, particle_id)?;
                write_bool(w, long_distance)?;
                write_float(w, x)?;
                write_float(w, y)?;
                write_float(w, z)?;
                write_float(w, offset_x)?;
                write_float(w, offset_y)?;
                write_float(w, offset_z)?;
                write_float(w, particle_data)?;
                write_int(w, particle_count)?;
                for &d in data.iter() {
                    write_int(w, d)?;
                }
            }
            SPacket::PlayChangeGameState { reason, value } => {
                write_ubyte(w, reason)?;
                write_float(w, value)?;
            }
            SPacket::PlaySpawnGlobalEntity {
                entity_id,
                entity_type,
                x,
                y,
                z,
            } => {
                write_varint(w, entity_id)?;
                write_byte(w, entity_type)?;
                write_int(w, double_to_fixed_point(x))?;
                write_int(w, double_to_fixed_point(y))?;
                write_int(w, double_to_fixed_point(z))?;
            }
            SPacket::PlayOpenWindow {
                window_id,
                window_type,
                ref window_title,
                slot_num,
                entity_id,
            } => {
                write_ubyte(w, window_id)?;
                data::write_string(w, window_type)?;
                window_title.write_proto(w)?;
                write_ubyte(w, slot_num)?;
                if window_type == "EntityHorse" {
                    write_int(w, entity_id)?;
                }
            }
            SPacket::PlayCloseWindow { window_id } => {
                write_ubyte(w, window_id)?;
            }
            SPacket::PlaySetSlot {
                window_id,
                slot,
                ref slot_data,
            } => {
                write_byte(w, window_id)?;
                write_ishort(w, slot)?;
                slot_data.write(w)?;
            }
            SPacket::PlayWindowItems {
                window_id,
                ref slots,
            } => {
                write_ubyte(w, window_id)?;
                write_ishort(w, slots.len() as i16)?;
                for s in slots.iter() {
                    s.write(w)?;
                }
            }
            SPacket::PlayWindowProperty {
                window_id,
                property,
                value,
            } => {
                write_ubyte(w, window_id)?;
                write_ishort(w, property)?;
                write_ishort(w, value)?;
            }
            SPacket::PlayConfirmTransaction {
                window_id,
                action_num,
                accepted,
            } => {
                write_byte(w, window_id)?;
                write_ishort(w, action_num)?;
                write_bool(w, accepted)?;
            }
            SPacket::PlayUpdateSign {
                location,
                ref line1,
                ref line2,
                ref line3,
                ref line4,
            } => {
                location.write_proto(w)?;
                line1.write_proto(w)?;
                line2.write_proto(w)?;
                line3.write_proto(w)?;
                line4.write_proto(w)?;
            }
            SPacket::PlayMap {
                map_id,
                scale,
                ref icons,
                ref data,
            } => {
                write_varint(w, map_id)?;
                write_byte(w, scale)?;
                for i in icons.iter() {
                    let b = (i.direction << 4) | (i.icon_type & 0x0f);
                    write_byte(w, b)?;
                    write_byte(w, i.x)?;
                    write_byte(w, i.z)?;
                }
                match *data {
                    Some(ref x) => {
                        write_byte(w, x.columns)?;
                        write_byte(w, x.rows)?;
                        write_byte(w, x.x)?;
                        write_byte(w, x.z)?;
                        write_varint(w, x.data.len() as i32)?;
                        w.write_all(&x.data)?;
                    }
                    None => write_byte(w, 0)?,
                }
            }
            SPacket::PlayUpdateBlockEntity {
                location,
                action,
                ref nbt_data,
            } => {
                location.write_proto(w)?;
                write_ubyte(w, action)?;
                nbt_data.write(w)?;
            }
            SPacket::PlayOpenSignEditor { location } => {
                location.write_proto(w)?;
            }
            SPacket::PlayStatistics { ref statistics } => {
                write_varint(w, statistics.len() as i32)?;
                for s in statistics.iter() {
                    data::write_string(w, &s.0)?;
                    write_varint(w, s.1)?;
                }
            }
            SPacket::PlayPlayerListItem { ref players } => {
                if players.is_empty() {
                    write_varint(w, 0)?;
                    write_varint(w, 0)?;
                } else {
                    let action = match players[0].action {
                        SPlayPlayerListItemDataAction::AddPlayer { .. } => 0,
                        SPlayPlayerListItemDataAction::UpdateGamemode { .. } => 1,
                        SPlayPlayerListItemDataAction::UpdateLatency { .. } => 2,
                        SPlayPlayerListItemDataAction::UpdateDisplayName { .. } => 3,
                        SPlayPlayerListItemDataAction::RemovePlayer { .. } => 4,
                    };
                    write_varint(w, action)?;
                    write_varint(w, players.len() as i32)?;
                }

                for p in players {
                    data::write_uuid(w, &p.uuid)?;
                    match p.action {
                        SPlayPlayerListItemDataAction::AddPlayer {
                            ref name,
                            ref properties,
                            gamemode,
                            ping,
                            ref display_name,
                        } => {
                            data::write_string(w, name)?;
                            write_varint(w, properties.len() as i32)?;
                            for p in properties {
                                data::write_string(w, &p.name)?;
                                data::write_string(w, &p.value)?;
                                match p.signature {
                                    Some(ref sig) => {
                                        write_bool(w, true)?;
                                        data::write_string(w, sig)?;
                                    }
                                    None => write_bool(w, false)?,
                                }
                            }
                            write_varint(w, gamemode)?;
                            write_varint(w, ping)?;
                            match *display_name {
                                Some(ref name) => {
                                    write_bool(w, true)?;
                                    name.write_proto(w)?;
                                }
                                None => write_bool(w, false)?,
                            }
                        }
                        SPlayPlayerListItemDataAction::UpdateGamemode { gamemode } => {
                            write_varint(w, gamemode)?
                        }
                        SPlayPlayerListItemDataAction::UpdateLatency { ping } => write_varint(w, ping)?,
                        SPlayPlayerListItemDataAction::UpdateDisplayName { ref display_name } => {
                            match *display_name {
                                Some(ref name) => {
                                    write_bool(w, true)?;
                                    name.write_proto(w)?;
                                }
                                None => write_bool(w, false)?,
                            }
                        }
                        SPlayPlayerListItemDataAction::RemovePlayer => (),
                    }
                }
            }
            SPacket::PlayPlayerAbilities {
                flags,
                flying_speed,
                field_of_view_modifier,
            } => {
                write_byte(w, flags)?;
                write_float(w, flying_speed)?;
                write_float(w, field_of_view_modifier)?;
            }
            SPacket::PlayTabComplete { ref matches } => {
                write_varint(w, matches.len() as i32)?;
                for m in matches {
                    data::write_string(w, m)?;
                }
            }
            SPacket::PlayScoreboardObjective {
                ref objective_name,
                ref data,
            } => {
                data::write_string(w, objective_name)?;
                match *data {
                    SPlayScoreboardObjectiveData::CreateScoreboard {
                        ref objective_value,
                        ref objective_type,
                    } => {
                        data::write_string(w, objective_value)?;
                        data::write_string(w, objective_type)?;
                    }
                    SPlayScoreboardObjectiveData::RemoveScoreboard => (),
                    SPlayScoreboardObjectiveData::UpdateScoreboard {
                        ref objective_value,
                        ref objective_type,
                    } => {
                        data::write_string(w, objective_value)?;
                        data::write_string(w, objective_type)?;
                    }
                }
            }
            SPacket::PlayUpdateScore {
                ref score_name,
                ref objective_name,
                ref data,
            } => {
                data::write_string(w, score_name)?;
                let action = match *data {
                    SPlayUpdateScoreData::CreateOrUpdateItem { .. } => 0,
                    SPlayUpdateScoreData::RemoveItem => 1,
                };
                write_byte(w, action)?;
                data::write_string(w, objective_name)?;
                match *data {
                    SPlayUpdateScoreData::CreateOrUpdateItem { value } => write_varint(w, value)?,
                    SPlayUpdateScoreData::RemoveItem => (),
                }
            }
            SPacket::PlayDisplayScoreboard {
                position,
                ref score_name,
            } => {
                write_byte(w, position)?;
                data::write_string(w, score_name)?;
            }
            SPacket::PlayTeams {
                ref team_name,
                ref data,
            } => {
                data::write_string(w, team_name)?;
                match *data {
                    SPlayTeamsData::CreateTeam {
                        ref display_name,
                        ref prefix,
                        ref suffix,
                        friendly_fire,
                        name_tag_visibility,
                        color,
                        ref players,
                    } => {
                        write_byte(w, 0)?;
                        data::write_string(w, display_name)?;
                        data::write_string(w, prefix)?;
                        data::write_string(w, suffix)?;
                        write_byte(w, friendly_fire)?;
                        data::write_string(w, name_tag_visibility)?;
                        write_byte(w, color as i8)?;
                        write_varint(w, players.len() as i32)?;
                        for p in players {
                            data::write_string(w, p)?;
                        }
                    }
                    SPlayTeamsData::RemoveTeam => write_byte(w, 1)?,
                    SPlayTeamsData::UpdateTeam {
                        ref display_name,
                        ref prefix,
                        ref suffix,
                        friendly_fire,
                        name_tag_visibility,
                        color,
                    } => {
                        write_byte(w, 2)?;
                        data::write_string(w, display_name)?;
                        data::write_string(w, prefix)?;
                        data::write_string(w, suffix)?;
                        write_byte(w, friendly_fire)?;
                        data::write_string(w, name_tag_visibility)?;
                        write_byte(w, color as i8)?;
                    }
                    SPlayTeamsData::AddPlayers { ref players } => {
                        write_byte(w, 3)?;
                        for p in players {
                            data::write_string(w, p)?;
                        }
                    }
                    SPlayTeamsData::RemovePlayers { ref players } => {
                        write_byte(w, 4)?;
                        for p in players {
                            data::write_string(w, p)?;
                        }
                    }
                }
            }
            SPacket::PlayPluginMessage { channel, ref data } => {
                data::write_string(w, channel)?;
                w.write_all(data)?;
            }
            SPacket::PlayDisconnect { ref reason } => {
                reason.write_proto(w)?;
            }
            SPacket::PlayServerDifficulty { difficulty } => {
                write_ubyte(w, difficulty)?;
            }
            SPacket::PlayCombatEvent { ref data } => match *data {
                SPlayCombatEventData::EnterCombat => write_varint(w, 0)?,
                SPlayCombatEventData::EndCombat {
                    duration,
                    entity_id,
                } => {
                    write_varint(w, 1)?;
                    write_varint(w, duration)?;
                    write_varint(w, entity_id)?;
                }
                SPlayCombatEventData::EntityDead {
                    player_id,
                    entity_id,
                    ref message,
                } => {
                    write_varint(w, 2)?;
                    write_varint(w, player_id)?;
                    write_int(w, entity_id)?;
                    data::write_string(w, message)?;
                }
            },
            SPacket::PlayCamera { camera_id } => {
                write_varint(w, camera_id)?;
            }
            SPacket::PlayWorldBorder { ref data } => match *data {
                SPlayWorldBorderData::SetSize { radius } => {
                    write_varint(w, 0)?;
                    write_double(w, radius)?;
                }
                SPlayWorldBorderData::LerpSize {
                    old_radius,
                    new_radius,
                    speed,
                } => {
                    write_varint(w, 1)?;
                    write_double(w, old_radius)?;
                    write_double(w, new_radius)?;
                    write_varlong(w, speed)?;
                }
                SPlayWorldBorderData::SetCenter { x, z } => {
                    write_varint(w, 2)?;
                    write_double(w, x)?;
                    write_double(w, z)?;
                }
                SPlayWorldBorderData::Initialize {
                    x,
                    z,
                    old_radius,
                    new_radius,
                    speed,
                    portal_teleport_boundary,
                    warning_time,
                    warning_blocks,
                } => {
                    write_varint(w, 3)?;
                    write_double(w, x)?;
                    write_double(w, z)?;
                    write_double(w, old_radius)?;
                    write_double(w, new_radius)?;
                    write_varlong(w, speed)?;
                    write_varint(w, portal_teleport_boundary)?;
                    write_varint(w, warning_time)?;
                    write_varint(w, warning_blocks)?;
                }
                SPlayWorldBorderData::SetWarningTime { warning_time } => {
                    write_varint(w, 4)?;
                    write_varint(w, warning_time)?;
                }
                SPlayWorldBorderData::SetWarningBlocks { warning_blocks } => {
                    write_varint(w, 4)?;
                    write_varint(w, warning_blocks)?;
                }
            },
            SPacket::PlayTitle { ref data } => match *data {
                SPlayTitleData::SetTitle { ref text } => {
                    write_varint(w, 0)?;
                    text.write_proto(w)?;
                }
                SPlayTitleData::SetSubtitle { ref subtitle } => {
                    write_varint(w, 1)?;
                    subtitle.write_proto(w)?;
                }
                SPlayTitleData::SetTimesAndDisplay {
                    fade_in,
                    stay,
                    fade_out,
                } => {
                    write_varint(w, 2)?;
                    write_int(w, fade_in)?;
                    write_int(w, stay)?;
                    write_int(w, fade_out)?;
                }
                SPlayTitleData::Hide => write_varint(w, 3)?,
                SPlayTitleData::Reset => write_varint(w, 4)?,
            },
            SPacket::PlaySetCompression { threshold } => {
                write_varint(w, threshold)?;
            }
            SPacket::PlayPlayerListHeaderAndFooter {
                ref header,
                ref footer,
            } => {
                header.write_proto(w)?;
                footer.write_proto(w)?;
            }
            SPacket::PlayResourcePackSend { ref url, ref hash } => {
                data::write_string(w, url)?;
                data::write_string(w, hash)?;
            }
            SPacket::PlayUpdateEntityNBT { entity_id, ref tag } => {
                write_varint(w, entity_id)?;
                tag.write(w)?;
            }
            SPacket::StatusResponse { ref data } => {
                data::write_string(w, &serde_json::to_string(data).unwrap())?;
            }
            SPacket::StatusPong { payload } => {
                write_long(w, payload)?;
            }
            SPacket::LoginDisconnect { ref reason } => {
                reason.write_proto(w)?;
            }
            SPacket::LoginLoginSuccess {
                ref uuid,
                ref username,
            } => {
                data::write_string(w, uuid)?;
                data::write_string(w, username)?;
            }
            SPacket::LoginSetCompression { threshold } => {
                write_varint(w, threshold)?;
            }
        }

        Ok(())
    }
}

#[derive(Debug, Serialize)]
pub struct SStatusResponseData {
    pub version: SStatusResponseVersion,
    pub players: SStatusResponsePlayers,
    pub description: Chat,
    #[serde(skip_serializing_if = "Option::is_none")] pub favicon: Arc<Option<String>>,
}

#[derive(Debug, Serialize)]
pub struct SStatusResponseVersion {
    pub name: &'static str,
    pub protocol: i32,
}

#[derive(Debug, Serialize)]
pub struct SStatusResponsePlayers {
    pub max: i32,
    pub online: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample: Option<Arc<Mutex<HashSet<SStatusResponsePlayer>>>>,
}

#[derive(Debug, Serialize, Hash, Clone, PartialEq, Eq)]
pub struct SStatusResponsePlayer {
    pub name: String,
    pub id: String,
}

#[derive(Debug)]
pub enum SPlayTitleData {
    SetTitle {
        text: Chat,
    },
    SetSubtitle {
        subtitle: Chat,
    },
    SetTimesAndDisplay {
        fade_in: i32,
        stay: i32,
        fade_out: i32,
    },
    Hide,
    Reset,
}

#[derive(Debug)]
pub enum SPlayWorldBorderData {
    SetSize {
        radius: f64,
    },
    LerpSize {
        old_radius: f64,
        new_radius: f64,
        speed: i64,
    },
    SetCenter {
        x: f64,
        z: f64,
    },
    Initialize {
        x: f64,
        z: f64,
        old_radius: f64,
        new_radius: f64,
        speed: i64,
        portal_teleport_boundary: i32,
        warning_time: i32,
        warning_blocks: i32,
    },
    SetWarningTime {
        warning_time: i32,
    },
    SetWarningBlocks {
        warning_blocks: i32,
    },
}

#[derive(Debug)]
pub enum SPlayCombatEventData {
    EnterCombat,
    EndCombat {
        duration: i32,
        entity_id: i32,
    },
    EntityDead {
        player_id: i32,
        entity_id: i32,
        message: String,
    },
}

#[derive(Debug)]
pub enum SPlayTeamsData {
    CreateTeam {
        display_name: String,
        prefix: String,
        suffix: String,
        friendly_fire: i8,
        name_tag_visibility: &'static str,
        color: text::Code,
        players: Vec<String>,
    },
    RemoveTeam,
    UpdateTeam {
        display_name: String,
        prefix: String,
        suffix: String,
        friendly_fire: i8,
        name_tag_visibility: &'static str,
        color: text::Code,
    },
    AddPlayers {
        players: Vec<String>,
    },
    RemovePlayers {
        players: Vec<String>,
    },
}

#[derive(Debug)]
pub enum SPlayUpdateScoreData {
    CreateOrUpdateItem { value: i32 },
    RemoveItem,
}

#[derive(Debug)]
pub enum SPlayScoreboardObjectiveData {
    CreateScoreboard {
        objective_value: String,
        objective_type: String,
    },
    RemoveScoreboard,
    UpdateScoreboard {
        objective_value: String,
        objective_type: String,
    },
}

#[derive(Debug)]
pub struct SPlayerListItemProperty {
    pub name: String,
    pub value: String,
    pub signature: Option<String>,
}

#[derive(Debug)]
pub struct SPlayPlayerListItemData {
    pub uuid: Uuid,
    pub action: SPlayPlayerListItemDataAction,
}

#[derive(Debug)]
pub enum SPlayPlayerListItemDataAction {
    AddPlayer {
        name: String,
        properties: Vec<SPlayerListItemProperty>,
        gamemode: i32,
        ping: i32,
        display_name: Option<Chat>,
    },
    UpdateGamemode {
        gamemode: i32,
    },
    UpdateLatency {
        ping: i32,
    },
    UpdateDisplayName {
        display_name: Option<Chat>,
    },
    RemovePlayer,
}

#[derive(Debug)]
pub struct SPlayMapData {
    pub columns: i8,
    pub rows: i8,
    pub x: i8,
    pub z: i8,
    pub data: Vec<u8>,
}

#[derive(Debug)]
pub struct SPlayMapIcon {
    pub direction: i8,
    pub icon_type: i8,
    pub x: i8,
    pub z: i8,
}

#[derive(Debug)]
pub struct SPlayMapChunkBulkData {
    pub chunk_x: i32,
    pub chunk_z: i32,
    pub primary_bit_mask: u16,
    pub chunk: data::GroundUpContinuous,
}

#[derive(Debug)]
pub struct SPlayMultiBlockChangeData {
    pub x: u8,
    pub z: u8,
    pub y: u8,
    pub block_id: i32,
}

#[derive(Debug)]
pub enum SPlayChunkDataData {
    GroundUpContinuous(data::GroundUpContinuous),
    GroundUpNonContinuous(data::GroundUpNonContinuous),
}

#[derive(Debug)]
pub struct SPlayEntityPropertiesData {
    pub key: String,
    pub value: f64,
    pub modifiers: Vec<data::ModifierData>,
}

#[derive(Debug)]
pub struct SPlaySpawnObjectData {
    pub data: i32,
    pub velocity_x: i16,
    pub velocity_y: i16,
    pub velocity_z: i16,
}
