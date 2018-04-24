pub mod playerlist;

use std::fs::File;
use std::io::{self, BufRead, BufReader, Read, Write};
use std::net::SocketAddr;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::rc::Rc;
use std::sync::mpsc::{self, Sender, Receiver};
use std::time::{Instant, Duration};
use std::thread;
use std::collections::{HashMap, VecDeque, HashSet};
use std::cell::RefCell;

use base64;
use uuid::Uuid;

use network::{self, NetworkServer};
use entity::player::{Player};
use proto::packets::{SPacket, SPlayPlayerListItemData, SPlayPlayerListItemDataAction};
use world::{Dimension, LevelType, World};
use world::chunk::{ChunkPos, Chunk};
use self::playerlist::PlayerList;
use text::{self, Code, ChatPos};
use text::chat::{Chat, Component};
use util;

pub enum Gamemode {
    Survival,
    Creative,
    Adventure,
    Spectator,
}

pub enum Difficulty {
    Peaceful,
    Easy,
    Normal,
    Hard,
}

#[derive(Default)]
pub struct PacketList {
    /// this is sent to all players
    pub to_all_players: VecDeque<SPacket>,
    /// a list of entries where every entry is a list of packets and a player to not send these to.
    pub to_all_player_except: HashMap<Uuid, VecDeque<SPacket>>,
}

impl PacketList {
    pub fn new() -> PacketList {
        Default::default()
    }

    pub fn clear(&mut self) {
        self.to_all_players.clear();
        self.to_all_player_except.clear();
    }
}

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        IOError(err: io::Error) {
            description(err.description())
            display("io error: {}", err)
            from()
            cause(err)
        }
        NetworkError(err: network::Error) {
            description(err.description())
            display("network error: {}", err)
            from()
            cause(err)
        }
    }
}

pub const MS_PER_TICK: u64 = 50;

/// information players need to know about server
pub struct ServerInfo {
    pub view_distance: u8,
    pub tick: u64,
    pub player_identities: HashMap<Uuid, String>,
}

pub struct Server {
    running: Arc<Mutex<bool>>,

    network_server: NetworkServer,
    player_list: PlayerList,
    incoming_players: Receiver<Player>,

    overworld: World,
    nether: World,
    end: World,

    packet_list: PacketList,
    current_time: Instant,
    start_time: Instant,
    next_entity_id: i32,
    server_info: ServerInfo,
}

impl Server {
    fn load_favicon() -> Result<Option<String>, Error> {
        const FAVICON: &str = "favicon.png";
        if !Path::new(FAVICON).exists() {
            return Ok(None);
        }

        let mut f = File::open("favicon.png")?;
        let mut v = Vec::new();
        f.read_to_end(&mut v)?;

        Ok(Some(format!("data:image/png;base64,{}", base64::display::Base64Display::standard(&v))))
    }

    pub fn new(addr: SocketAddr) -> Result<Server, Error> {
        let running = Arc::new(Mutex::new(false));
        let favicon = Server::load_favicon()?;
        let (tx, rx) = mpsc::channel();
        let network_server = NetworkServer::new(
            addr,
            favicon,
            running.clone(),
            tx,
        );
        Ok(Server {
            running,

            network_server,
            player_list: PlayerList::new(),
            incoming_players: rx,

            overworld: World::new(Dimension::Overworld),
            nether: World::new(Dimension::Nether),
            end: World::new(Dimension::End),

            packet_list: PacketList::new(),
            current_time: Instant::now(),
            start_time: Instant::now(),
            next_entity_id: 0,
            server_info: ServerInfo {
                view_distance: 10,
                tick: 0,
                player_identities: HashMap::new(),
            },
        })
    }

    pub fn start(&mut self) -> Result<(), Error> {
        *self.running.lock().unwrap() = true;

        self.network_server.start()?;

        self.current_time = Instant::now();
        let mut accumulator = 0;

        while *self.running.lock().unwrap() {
            let current_time = Instant::now();
            let duration = current_time - self.current_time;
            let mut millis = util::duration_total_ms(duration) as u64;

            if millis > 2000 {
                println!("Ticks taking too long. Can't compensate too much. Skipping {} ticks", (millis - 2000) / MS_PER_TICK);
                millis = 2000;
            }

            accumulator += millis;
            self.current_time = current_time;

            while accumulator >= MS_PER_TICK {
                self.tick();
                accumulator -= MS_PER_TICK;
            }

            thread::sleep(Duration::from_millis(MS_PER_TICK - accumulator));
        }

        self.cleanup();
        Ok(())
    }

    fn get_next_entity_id(&mut self) -> i32 {
        let ret = self.next_entity_id;
        self.next_entity_id += 1;
        ret
    }

    fn send_packet_to_all_players(&mut self, p: Arc<SPacket>) {
        for player in self.player_list.iter_mut() {
            player.send_packet(Arc::clone(&p));
        }
    }

    fn add_new_player(&mut self, mut p: Player) {
        p.send_packet(Arc::new(SPacket::PlayJoinGame {
            entity_id: self.get_next_entity_id(),
            gamemode: Gamemode::Creative as u8,
            dimension: Dimension::End as i8,
            difficulty: Difficulty::Peaceful as u8,
            max_players: 100,
            level_type: LevelType::Flat.as_str(),
            reduced_debug_info: false,
        }));
        p.send_packet(Arc::new(SPacket::PlayPluginMessage {
            channel: "MC|Brand",
            data: "Quartz".as_bytes().to_vec(),
        }));
        p.send_packet(Arc::new(SPacket::PlayServerDifficulty {
            difficulty: Difficulty::Peaceful as u8,
        }));
        p.send_packet(Arc::new(SPacket::PlayPlayerAbilities {
            flags: 0x08 | 0x04,
            flying_speed: 0.05,
            field_of_view_modifier: 0.5,
        }));

        p.send_packet(Arc::new(SPacket::PlayPlayerListHeaderAndFooter {
            header: Chat::from(text::parse_legacy_ex("&4&lQuartz server", '&')),
            footer: Chat::from(text::parse_legacy_ex("&6&lAll hail Emperor", '&')),
        }));

        let list_item_data = Arc::new(
            SPlayPlayerListItemData {
                uuid: p.get_uuid(),
                action: SPlayPlayerListItemDataAction::AddPlayer {
                    name: p.get_name().into(),
                    gamemode: Gamemode::Creative as i32,
                    ping: 0,
                    properties: Vec::new(),
                    display_name: None,
                },
            }
        );

        self.send_packet_to_all_players(Arc::new(SPacket::PlayPlayerListItem { players: vec![Arc::clone(&list_item_data)] }));
        let join_msg = Chat::from(text::parse_legacy(&format!("{}{} joined the game!", Code::Yellow, p.get_name())));
        self.send_packet_to_all_players(Arc::new(SPacket::PlayChatMessage {
            position: ChatPos::Normal,
            message: join_msg.clone(),
        }));
        // for newly joined player, we need to send all other players too.
        let mut list_item_datas = vec![list_item_data];
        for p in self.player_list.iter() {
            list_item_datas.push(Arc::new(SPlayPlayerListItemData {
                uuid: p.get_uuid(),
                action: SPlayPlayerListItemDataAction::AddPlayer {
                    name: p.get_name().into(),
                    gamemode: Gamemode::Creative as i32,
                    ping: 0,
                    properties: Vec::new(),
                    display_name: None,
                },
            }));
        }

        p.send_packet(Arc::new(SPacket::PlayPlayerListItem { players: list_item_datas }));
        p.send_packet(Arc::new(SPacket::PlayChatMessage {
            position: ChatPos::Normal,
            message: join_msg,
        }));

        let chunk_rect = p.get_chunk_rectangle(self.server_info.view_distance);
        for pos in chunk_rect.chunks_iter() {
            let chunk = self.get_world(p.get_dimension()).get_chunk(pos);
            p.send_packet(Arc::new(chunk.to_proto_chunk_data(Chunk::FULL_BIT_MASK)));
        }

        p.send_packet(Arc::new(SPacket::PlaySpawnPosition {
            location: ::proto::data::Position {x: 0, y: 82, z: 0},
        }));
        let (x, y, z, yaw, pitch) = (p.get_pos().x, p.get_pos().y, p.get_pos().z, p.get_yaw(), p.get_pitch());
        p.send_packet(Arc::new(SPacket::PlayPlayerPositionAndLook {
            x,
            y,
            z,
            yaw: yaw as f32,
            pitch: pitch as f32,
            flags: 0,
        }));

        self.server_info.player_identities.insert(p.get_uuid(), p.get_name().into());
        self.player_list.add_player(p);
    }

    fn accept_new_players(&mut self) {
        loop {
            let p = self.incoming_players.try_recv();
            match p {
                Ok(player) => {
                    self.add_new_player(player);
                },
                Err(_) => break,
            }
        }
    }

    fn remove_disconnected_players(&mut self) {
        let mut to_remove = Vec::new();
        for p in self.player_list.iter_mut() {
            if !p.get_connected() {
                to_remove.push((p.get_uuid(), String::from(p.get_name())));
            }
        }

        for (u, name) in to_remove {
            self.send_packet_to_all_players(Arc::new(SPacket::PlayPlayerListItem { players: vec![Arc::new(SPlayPlayerListItemData {
                uuid: u,
                action: SPlayPlayerListItemDataAction::RemovePlayer,
            })] }));
            self.send_packet_to_all_players(Arc::new(SPacket::PlayChatMessage {
                position: ChatPos::Normal,
                message: Chat::from(text::parse_legacy(&format!("{}{} left the game", Code::Yellow, name))),
            }));
            self.player_list.remove_by_uuid(&u);
            self.server_info.player_identities.remove(&u);
        }
    }

    fn tick_players(&mut self) {
        for p in self.player_list.iter_mut() {
            let world = match p.get_dimension() {
                Dimension::Overworld => &mut self.overworld,
                Dimension::Nether => &mut self.nether,
                Dimension::End => &mut self.end,
            };
            p.tick(&self.server_info, world, &mut self.packet_list);
        }

        self.send_packet_list();
        self.packet_list.clear();
    }

    fn get_world(&mut self, dimension: Dimension) -> &mut World {
        match dimension {
            Dimension::Overworld => &mut self.overworld,
            Dimension::Nether => &mut self.nether,
            Dimension::End => &mut self.end,
        }
    }

    /// sends packets in packetlist accordingly
    fn send_packet_list(&mut self) {
        while let Some(packet) = self.packet_list.to_all_players.pop_front() {
            let packet = Arc::new(packet);
            for p in self.player_list.iter_mut() {
                p.send_packet(Arc::clone(&packet));
            }
        }

        for (&uuid, packets) in &mut self.packet_list.to_all_player_except {
            let mut packets_vec = Vec::new();
            while let Some(packet) = packets.pop_front() {
                packets_vec.push(Arc::new(packet));
            }

            for p in self.player_list.iter_mut() {
                if p.get_uuid() != uuid {
                    for packet in &packets_vec {
                        p.send_packet(Arc::clone(packet));
                    }
                }
            }
        }
    }

    fn tick(&mut self) {
        self.accept_new_players();
        self.tick_players();
        self.remove_disconnected_players();
        self.server_info.tick += 1;
    }

    fn cleanup(&mut self) {

    }
}