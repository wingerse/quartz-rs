pub enum CPacket {
    Handshake {
        protocol_version: i32,
        server_address: String,
        server_port: u16,
        next_state: i32,
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
    }
    PlayChatMessage {
        message: String,
        position: i8,
    },
    PlayTimeUpdate {
        world_age: i64,
        time_of_day: i64,
    },
    PlayEntityEquipment {
        entity_id: i32,
        slot: i64
    }

}