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
use entity::player::{Player, PlayerInfo};
use proto::packets::{SPacket, SPlayPlayerListItemData, SPlayPlayerListItemDataAction};
use world::{Dimension, LevelType, World};
use world::chunk::{ChunkPos, Chunk};
use self::playerlist::PlayerList;
use text::{self, Code, ChatPos};
use text::chat::{Chat, Component};
use util;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Gamemode {
    Survival,
    Creative,
    Adventure,
    Spectator,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Difficulty {
    Peaceful,
    Easy,
    Normal,
    Hard,
}

#[derive(Default)]
pub struct PacketList {
    /// this is sent to all players
    to_all_players: VecDeque<Arc<SPacket>>,
    /// a list of entries where every entry is a list of packets and a player to not send these to.
    to_all_player_except: HashMap<Uuid, VecDeque<Arc<SPacket>>>,
    /// packets are only sent to players specified.
    to_players: HashMap<Uuid, VecDeque<Arc<SPacket>>>,
}

impl PacketList {
    pub fn new() -> PacketList {
        Default::default()
    }

    pub fn insert_to_all_players(&mut self, packet: Arc<SPacket>) {
        self.to_all_players.push_back(packet);
    }

    pub fn insert_to_all_player_except(&mut self, player: Uuid, packet: Arc<SPacket>) {
        self.to_all_player_except.entry(player).or_insert(VecDeque::new()).push_back(packet);
    }

    pub fn insert_to_players(&mut self, player: Uuid, packet: Arc<SPacket>) {
        self.to_players.entry(player).or_insert(VecDeque::new()).push_back(packet);
    }

    pub fn clear(&mut self) {
        self.to_all_players.clear();
        self.to_all_player_except.clear();
        self.to_players.clear();
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
    pub player_view_distance: u8,
    pub tick: u64,
    pub players_info: HashMap<Uuid, PlayerInfo>,
}

pub struct Worlds {
    overworld: World,
    nether: World,
    end: World,
}

impl Worlds {
    fn new() -> Worlds {
        Worlds {
            overworld: World::new(Dimension::Overworld),
            nether: World::new(Dimension::Nether),
            end: World::new(Dimension::End),
        }
    }

    fn get_world(&mut self, dimension: Dimension) -> &mut World {
        match dimension {
            Dimension::Overworld => &mut self.overworld,
            Dimension::Nether => &mut self.nether,
            Dimension::End => &mut self.end,
        }
    }
}

pub struct Server {
    running: Arc<Mutex<bool>>,

    network_server: NetworkServer,
    player_list: PlayerList,
    incoming_players: Receiver<Player>,

    worlds: Worlds,

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

            worlds: Worlds::new(),

            packet_list: PacketList::new(),
            current_time: Instant::now(),
            start_time: Instant::now(),
            next_entity_id: 0,
            server_info: ServerInfo {
                view_distance: 10,
                player_view_distance: 3,
                tick: 0,
                players_info: HashMap::new(),
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

    fn add_new_player(&mut self, mut p: Player) {
        let entity_id = self.get_next_entity_id();
        p.set_entity_id(entity_id);
        p.send_packet(Arc::new(SPacket::PlayJoinGame {
            entity_id,
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
                    gamemode: p.get_gamemode() as i32,
                    ping: p.get_ping(),
                    properties: Vec::new(),
                    display_name: None,
                },
            }
        );

        self.packet_list.insert_to_all_players(Arc::new(SPacket::PlayPlayerListItem { players: vec![Arc::clone(&list_item_data)] }));
        let join_msg = Chat::from(text::parse_legacy(&format!("{}{} joined the game!", Code::Yellow, p.get_name())));
        self.packet_list.insert_to_all_players(Arc::new(SPacket::PlayChatMessage {
            position: ChatPos::Normal,
            message: join_msg.clone(),
        }));
        self.send_packet_list();
        // for newly joined player, we need to send all other players too.
        let mut list_item_datas = vec![list_item_data];
        for p in self.player_list.iter() {
            list_item_datas.push(Arc::new(SPlayPlayerListItemData {
                uuid: p.get_uuid(),
                action: SPlayPlayerListItemDataAction::AddPlayer {
                    name: p.get_name().into(),
                    gamemode: p.get_gamemode() as i32,
                    ping: p.get_ping(),
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

        self.send_initial_chunks_for_player(&mut p);
        self.worlds.get_world(p.get_dimension()).get_chunk(p.get_chunk_pos(), p.get_uuid()).insert_player(p.get_uuid());

        p.send_packet(Arc::new(SPacket::PlaySpawnPosition {
            location: ::proto::data::Position { x: 0, y: 82, z: 0 },
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

        self.server_info.players_info.insert(p.get_uuid(), p.create_player_info());
        self.player_list.add_player(p);
    }

    fn send_initial_chunks_for_player(&mut self, p: &mut Player) {
        let chunk_rect = p.get_chunk_rectangle(self.server_info.view_distance);

        let (uuid, dimension) = (p.get_uuid(), p.get_dimension());
        let sky_light_sent = self.worlds.get_world(dimension).get_properties().has_sky_light();

        util::iter_foreach_every(chunk_rect.chunks_iter()
                                           .map(|pos| self.worlds.get_world(dimension).get_chunk(pos, uuid).to_proto_map_chunk_bulk_data()),
                                 |i| i % 30 == 0 && i != 0,
                                 |q| {
                                     let mut chunks = Vec::new();
                                     while let Some(chunk) = q.pop_front() {
                                         chunks.push(chunk);
                                     }
                                     p.send_packet(Arc::new(SPacket::PlayMapChunkBulk {
                                         sky_light_sent,
                                         chunks,
                                     }));
                                 });
    }

    fn accept_new_players(&mut self) {
        loop {
            let p = self.incoming_players.try_recv();
            match p {
                Ok(player) => {
                    self.add_new_player(player);
                }
                Err(_) => break,
            }
        }
    }

    fn remove_disconnected_players(&mut self) {
        let mut to_remove = Vec::new();
        for p in self.player_list.iter() {
            if !p.get_connected() {
                to_remove.push(p.get_uuid());
            }
        }

        for u in to_remove {
            let mut p = self.player_list.remove_by_uuid(&u).unwrap();
            self.server_info.players_info.remove(&u);

            self.packet_list.insert_to_all_players(Arc::new(SPacket::PlayPlayerListItem {
                players: vec![Arc::new(SPlayPlayerListItemData {
                    uuid: u,
                    action: SPlayPlayerListItemDataAction::RemovePlayer,
                })]
            }));

            self.packet_list.insert_to_all_players(Arc::new(SPacket::PlayChatMessage {
                position: ChatPos::Normal,
                message: Chat::from(text::parse_legacy(&format!("{}{} left the game",
                                                                Code::Yellow,
                                                                p.get_name()))),
            }));

            {
                let world = self.worlds.get_world(p.get_dimension());
                world.get_chunk(p.get_chunk_pos(), p.get_uuid()).remove_player(&p.get_uuid());
                p.despawn(&mut self.server_info, world, &mut self.packet_list);

                for chunk_pos in p.get_chunk_rectangle(self.server_info.view_distance).chunks_iter() {
                    world.unload_chunk_if_required(chunk_pos, p.get_uuid());
                }
            }

            self.send_packet_list();
        }
    }

    fn tick_players(&mut self) {
        for p in self.player_list.iter_mut() {
            let world = self.worlds.get_world(p.get_dimension());
            p.tick(&self.server_info, world, &mut self.packet_list);
        }

        self.send_packet_list();
    }

    /// sends packets in packetlist accordingly, and clears it
    fn send_packet_list(&mut self) {
        while let Some(packet) = self.packet_list.to_all_players.pop_front() {
            for p in self.player_list.iter_mut() {
                p.send_packet(Arc::clone(&packet));
            }
        }

        for (&uuid, packets) in &mut self.packet_list.to_all_player_except {
            let mut packets_vec = Vec::new();
            while let Some(packet) = packets.pop_front() {
                packets_vec.push(packet);
            }

            for p in self.player_list.iter_mut() {
                if p.get_uuid() != uuid {
                    for packet in &packets_vec {
                        p.send_packet(Arc::clone(packet));
                    }
                }
            }
        }

        for (&uuid, packets) in &mut self.packet_list.to_players {
            if let Some(p) = self.player_list.get_by_uuid(&uuid) {
                while let Some(packet) = packets.pop_front() {
                    p.send_packet(packet);
                }
            }
        }

        self.packet_list.clear();
    }

    fn tick(&mut self) {
        self.accept_new_players();
        self.tick_players();
        self.remove_disconnected_players();
        self.server_info.tick += 1;
    }

    fn cleanup(&mut self) {}
}