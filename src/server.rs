use base64;
use network::{self, NetworkServer};
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read, Write};
use std::net::SocketAddr;
use std::path::Path;
use std::sync::{Arc, Mutex};

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

pub struct Server {
    running: Arc<Mutex<bool>>,
    network_server: NetworkServer,
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
        let network_server = NetworkServer::new(addr, favicon, running.clone());
        Ok(Server { running, network_server })
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