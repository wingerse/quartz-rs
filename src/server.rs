use base64;
use network::{self, NetworkServer};
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read, Write};
use std::net::SocketAddr;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::rc::Rc;
use std::sync::mpsc::{self, Sender, Receiver};

use entity::player::Player;
use std::collections::HashMap;
use uuid::Uuid;
use std::cell::RefCell;

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

    pub fn add_player(&mut self, player: Player) {
        let name = player.get_name().into();
        let uuid = player.get_uuid();
        let p = Rc::new(RefCell::new(player));
        self.by_name.insert(name, Rc::clone(&p));
        self.by_uuid.insert(uuid, p);
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

pub struct Server {
    running: Arc<Mutex<bool>>,
    network_server: NetworkServer,
    player_list: PlayerList,
    incoming_players: Receiver<Player>,
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
        Ok(Server { running, network_server, player_list: PlayerList::new(), incoming_players: rx })
    }

    pub fn start(&mut self) -> Result<(), Error> {
        {
            *self.running.lock().unwrap() = true;
        }

        self.network_server.start()?;

        loop {}

        Ok(())
    }
}