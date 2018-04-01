use text::chat::Chat;
use text;
use super::data;
use uuid;

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
        location: data::Position,
        face: i8,
    },
    PlayPlayerBlockPlacement {
        location: data::Position,
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
        location: data::Position,
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
        pos: Option<data::Position>,
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
        target_player: uuid::Uuid,
    },
    PlayResourcePackSend {
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

pub enum CPlayUseEntityData {
    Interact,
    Attack,
    InteractAt {
        target_x: f32,
        target_y: f32,
        target_z: f32,
    }
}

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
        position: i8,
    },
    PlayTimeUpdate {
        world_age: i64,
        time_of_day: i64,
    },
    PlayEntityEquipment {
        entity_id: i32,
        slot: i64,
        item: data::SlotData,
    },
    PlaySpawnPosition {
        location: data::Position,
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
        yaw: f64,
        pitch: f64,
        flags: u8,
    },
    PlayHeldItemChange {
        slot: u8,
    },
    PlayUseBed {
        entity_id: i32,
        location: data::Position,
    },
    PlayAnimation {
        entity_id: i32,
        animation: u8,
    },
    PlaySpawnPlayer {
        entity_id: i32,
        uuid: uuid::Uuid,
        x: f64,
        y: f64,
        z: f64,
        yaw: f64,
        pitch: f64,
        current_item: i16,
        metadata: data::EntityMetadata,
    },
    PlayCollectItem {
        collected_entity_id: i32,
        collector_entity_id: i32,
    },
    PlaySpawnObject {
        entity_id: i32,
        type: i8,
        x: f64,
        y: f64,
        z: f64,
        pitch: f64,
        yaw: f64,
        data: Option<SPlaySpawnObjectData>
    },
    PlaySpawnMob {
        entity_id: i32,
        type: u8,
        x: f64,
        y: f64,
        z: f64,
        yaw: f64,
        pitch: f64,
        head_pitch: f64,
        velocity_x: i16,
        velocity_y: i16,
        velocity_z: i16,
        metadata: data::EntityMetadata,
    },
    PlaySpawnPainting {
        entity_id: i32,
        title: String,
        location: data::Position,
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
        count: i32,
        entity_ids: Vec<i32>,
    },
    PlayEntity {
        entity_id: i32,
    },
    PlayEntityRelativeMove {
        entity_id: i32,
        delta_x: f64,
        delta_y: f64,
        delta_z: f64,
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
        delta_x: f64,
        delta_y: f64,
        delta_z: f64,
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
        metadata: data::EntityMetadata,
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
        location: data::Position,
        block_id: i32,
    },
    PlayBlockAction {
        location: data::Position,
        byte1: u8,
        byte2: u8,
        block_type: i32,
    },
    PlayBlockBreakAnimation {
        entity_id: i32,
        location: data::Position,
        destroy_stage: i8,
    },
    PlayMapChunkBulk {
        sky_light_sent: bool,
        data: Vec<SPlayMapChunkBulkData>,
        chunks: Vec<data::GroundUpContinuous>,
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
        location: data::Position,
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
        type: u8,
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
        location: data::Position,
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
        location: data::Position,
        action: u8,
        nbt_data: nbt::NBT,
    },
    PlayOpenSignEditor {
        location: data::Position,
    },
    PlayStatistics {
        statistics: Vec<(&'static str, i32)>,
    },
    PlayPlayerListItem {
        players: Vec<SPlayPlayerListItemData>,
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
        tag: nbt::NBT,
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
        threashold: i32,
    }
}

#[derive(Debug, Serialize)]
pub struct SStatusResponseData {
    pub version: SStatusResponseVersion,
    pub players: SStatusResponsePlayers,
    pub description: Chat,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub favicon: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct SStatusResponseVersion {
    pub name: String,
    pub protocol: i32,
}

#[derive(Debug, Serialize)]
pub struct SStatusResponsePlayers {
    pub max: i32,
    pub online: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample: Option<Vec<SStatusResponsePlayer>>,
}

#[derive(Debug, Serialize)]
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
    }
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
        friendly_fire, u8,
        name_tag_visibility: &'static str,
        color: text::Code,
        players: Vec<String>,
    },
    RemoveTeam,
    UpdateTeam {
        display_name: String,
        prefix: String,
        suffix: String,
        friendly_fire, u8,
        name_tag_visibility: &'static str,
        color: text::Code,
    },
    AddPlayers {
        players: Vec<String>,
    },
    RemovePlayers {
        players: Vec<String>,
    }
}

#[derive(Debug)]
pub enum SPlayUpdateScoreData {
    CreateOrUpdateItem {
        value: i32,
    },
    RemoveItem,
}

#[derive(Debug)]
pub enum SPlayScoreboardObjectiveData {
    CreateScoreboard {
        objective_value: String,
        type: String,
    },
    RemoveScoreboard,
    UpdateScoreboard {
        objective_value: String,
        type: String,
    }
}

#[derive(Debug)]
pub struct SPlayerListItemProperty {
    pub name: String,
    pub value: String,
    pub signature: Option<String>,
}

#[derive(Debug)]
pub enum SPlayPlayerListItemData {
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
    pub direction: u8,
    pub type: u8,
    pub x: u8,
    pub z: u8,
}

#[derive(Debug)]
pub struct SPlayMapChunkBulkData {
    pub chunk_x: i32,
    pub chunk_z: i32,
    pub primary_bit_mask: u16,
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
    NotGroundUpContinuous {
        sections: Vec<data::ChunkSection>,
    }
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