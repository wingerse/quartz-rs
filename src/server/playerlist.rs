use std::collections::HashMap;
use std::collections::hash_map::{ValuesMut, Values};
use std::rc::Rc;
use std::sync::Arc;
use std::cell::RefCell;

use uuid::Uuid;

use entity::player::Player;
use proto::packets::SPacket;

pub struct PlayerList {
    by_uuid: HashMap<Uuid, Player>,
    // this hashmap owns player
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
        self.by_uuid.insert(uuid, player);
        self.by_name.insert(name, uuid);
    }

    pub fn get_by_name(&mut self, name: &str) -> Option<&mut Player> {
        let u = self.by_name.get(name);
        match u {
            Some(u) => self.by_uuid.get_mut(&u),
            None => None,
        }
    }

    pub fn get_by_uuid(&mut self, uuid: &Uuid) -> Option<&mut Player> {
        self.by_uuid.get_mut(uuid)
    }

    pub fn remove_by_name(&mut self, name: &str) -> Option<Player> {
        let u = self.by_name.remove(name);
        if let Some(u) = u {
            let mut p = self.by_uuid.remove(&u).unwrap();
            p.set_connected(false);
            Some(p)
        } else { None }
    }

    pub fn remove_by_uuid(&mut self, uuid: &Uuid) -> Option<Player> {
        let p = self.by_uuid.remove(uuid);
        if let Some(mut p) = p {
            self.by_name.remove(p.get_name());
            p.set_connected(false);
            Some(p)
        } else { None }
    }

    pub fn count(&self) -> usize {
        self.by_uuid.len()
    }

    pub fn iter(&self) -> Values<Uuid, Player> {
        self.by_uuid.values()
    }

    pub fn iter_mut(&mut self) -> ValuesMut<Uuid, Player> {
        self.by_uuid.values_mut()
    }
}