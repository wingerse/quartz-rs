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
use std::collections::HashMap;
use std::cell::RefCell;

use base64;
use uuid::Uuid;

use network::{self, NetworkServer};
use entity::player::Player;
use proto::packets::{SPacket, SPlayPlayerListItemData, SPlayPlayerListItemDataAction};
use world::{Dimension, LevelType};
use self::playerlist::PlayerList;
use world::chunk_provider::ChunkProvider;
use world::chunk::ChunkPos;
use text::{self, Code, ChatPos};
use text::chat::{Chat, Component};
use entity::player::PacketList;
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

pub struct Server {
    running: Arc<Mutex<bool>>,
    network_server: NetworkServer,
    player_list: PlayerList,
    incoming_players: Receiver<Player>,
    current_time: Instant,
    start_time: Instant,
    next_entity_id: i32,
    tick: u64,
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
            current_time: Instant::now(),
            start_time: Instant::now(),
            next_entity_id: 0,
            tick: 0,
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
            dimension: Dimension::Overworld as i8,
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

        for z in -5..5 {
            for x in -5..5 {
                let chunk = ChunkProvider::load_chunk(ChunkPos::new(x, z));
                p.send_packet(Arc::new(chunk.to_proto_chunk_data(true)));
            }
        }

        p.send_packet(Arc::new(SPacket::PlaySpawnPosition {
            location: ::proto::data::Position {x: 0, y: 82, z: 0},
        }));
        p.send_packet(Arc::new(SPacket::PlayPlayerPositionAndLook {
            x: 0.0,
            y: 82.0,
            z: 0.0,
            yaw: 0.0,
            pitch: 0.0,
            flags: 0,
        }));

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
        }
    }

    fn tick_players(&mut self, tick: u64) {
        let mut packet_list = PacketList::default();
        for p in self.player_list.iter_mut() {
            let player_packet_list = p.tick(tick, &mut packet_list);
        }

        self.send_packets(packet_list);
    }

    /// sends packets in packetlist accordingly
    fn send_packets(&mut self, packet_list: PacketList) {
        for packet in packet_list.to_all_players {
            let packet = Arc::new(packet);
            for p in self.player_list.iter_mut() {
                p.send_packet(Arc::clone(&packet));
            }
        }

        for (uuid, packets) in packet_list.to_all_player_except {
            let packets: Vec<Arc<SPacket>> = packets.into_iter().map(|packet| Arc::new(packet)).collect();
            for p in self.player_list.iter_mut() {
                if p.get_uuid() != uuid {
                    for packet in &packets {
                        p.send_packet(Arc::clone(packet));
                    }
                }
            }
        }
    }

    fn tick(&mut self) {
        let current_tick = self.tick;
        self.accept_new_players();
        self.tick_players(current_tick);
        self.remove_disconnected_players();
        self.tick += 1;
    }

    fn cleanup(&mut self) {

    }
}