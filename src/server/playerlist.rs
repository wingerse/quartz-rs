use std::collections::HashMap;
use std::collections::hash_map::{ValuesMut, Values};
use std::rc::Rc;
use std::sync::Arc;
use std::cell::{RefCell, RefMut};

use uuid::Uuid;

use entity::player::Player;
use proto::packets::SPacket;

/// List of players in the server. This allows you to get by name and uuid.
/// # Important
/// While a player is borrowed from this (eg: calling `tick`), it is important to not borrow it again (which can happen when you call one of send_packet*
/// methods).
/// The send packet except method should be used in that case where you send the packet to your borrowed player yourself and to other players
/// using except method. `Player` has a private method to do just that.
pub struct PlayerList {
    // this hashmap owns the player
    by_uuid: HashMap<Uuid, RefCell<Player>>,
    by_name: HashMap<String, Uuid>,
}

impl PlayerList {
    pub fn new() -> PlayerList {
        PlayerList {
            by_name: HashMap::new(),
            by_uuid: HashMap::new(),
        }
    }

    pub fn add_player(&mut self, player: Player) {
        let name = player.get_name().into();
        let uuid = player.get_uuid();
        self.by_uuid.insert(uuid, RefCell::new(player));
        self.by_name.insert(name, uuid);
    }

    pub fn get_by_name(&self, name: &str) -> Option<RefMut<Player>> {
        self.by_name.get(name).and_then(|u| Some(self.by_uuid.get(&u).unwrap().borrow_mut()))
    }

    pub fn get_by_uuid(&self, uuid: &Uuid) -> Option<RefMut<Player>> {
        self.by_uuid.get(uuid).and_then(|r| Some(r.borrow_mut()))
    }

    pub fn remove_by_name(&mut self, name: &str) -> Option<Player> {
        self.by_name.remove(name).and_then(|u| {
            let p = self.by_uuid.remove(&u).unwrap();
            p.borrow_mut().set_connected(false);
            Some(p.into_inner())
        })
    }

    pub fn remove_by_uuid(&mut self, uuid: &Uuid) -> Option<Player> {
        self.by_uuid.remove(uuid).and_then(|p| {
            self.by_name.remove(p.borrow().get_name());
            p.borrow_mut().set_connected(false);
            Some(p.into_inner())
        })
    }

    pub fn count(&self) -> usize {
        self.by_uuid.len()
    }

    pub fn iter(&self) -> Values<Uuid, RefCell<Player>> {
        self.by_uuid.values()
    }

    pub fn send_packet_to_all_players(&self, packet: Arc<SPacket>) {
        for p in self.iter() {
            p.borrow_mut().send_packet(Arc::clone(&packet));
        }
    }

    pub fn send_packet_to_player(&self, player: Uuid, packet: Arc<SPacket>) {
        if let Some(mut p) = self.get_by_uuid(&player) {
            p.send_packet(Arc::clone(&packet))
        }
    }

    pub fn send_packet_to_players_except(&self, player: Uuid, packet: Arc<SPacket>) {
        for (&uuid, p) in self.by_uuid.iter() {
            if uuid != player {
                p.borrow_mut().send_packet(Arc::clone(&packet));
            }
        }
    }
}