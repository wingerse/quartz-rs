use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

use uuid::Uuid;

use entity::player::Player;

pub struct PlayerList {
    by_name: HashMap<String, Rc<RefCell<Player>>>,
    by_uuid: HashMap<Uuid, Rc<RefCell<Player>>>,
}

impl PlayerList {
    pub fn new() -> PlayerList {
        PlayerList {
            by_name: HashMap::new(),
            by_uuid: HashMap::new(),
        }
    }

    pub fn add_player(&mut self, player: Rc<RefCell<Player>>) {
        let name = player.borrow().get_name().into();
        self.by_name.insert(name, Rc::clone(&player));
        let uuid = player.borrow().get_uuid();
        self.by_uuid.insert(uuid, player);
    }

    pub fn get_by_name(&self, name: &str) -> Option<Rc<RefCell<Player>>> {
        self.by_name.get(name).map(Rc::clone)
    }

    pub fn get_by_uuid(&self, uuid: Uuid) -> Option<Rc<RefCell<Player>>> {
        self.by_uuid.get(&uuid).map(Rc::clone)
    }

    pub fn remove_by_name(&mut self, name: &str) {
        let p = self.by_name.remove(name);
        if let Some(p) = p {
            let mut pl = p.borrow_mut();
            pl.set_connected(false);
            self.by_uuid.remove(&pl.get_uuid());
        }
    }

    pub fn remove_by_uuid(&mut self, uuid: Uuid) {
        let p = self.by_uuid.remove(&uuid);
        if let Some(p) = p {
            let mut pl = p.borrow_mut();
            pl.set_connected(false);
            self.by_name.remove(pl.get_name());
        }
    }

    pub fn count(&self) -> usize {
        self.by_uuid.len()
    }

    pub fn iter(&self) -> ::std::collections::hash_map::Iter<Uuid, Rc<RefCell<Player>>> {
        self.by_uuid.iter()
    }
}