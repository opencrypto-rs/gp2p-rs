// state.rs
#[derive(Debug, Clone)]

pub struct Peer {
    
    pub id: String,
    pub ip: String,
    pub port: u32,
}
#[derive(Debug, Clone)]

struct PeerList {
    peers: Vec<Peer>,
}
#[derive(Debug, Clone)]

struct Me {
    id: String,
    ip: String,
    port: u32,
}
#[derive(Debug, Clone)]
pub struct State {
    me: Me,
    peers: PeerList,
}

impl State {
    pub fn new(ip: String, port: u32) -> State {
        State {
            me: Me {
                id: "1".to_string(),
                ip,
                port,
            },
            peers: PeerList { peers: vec![] },
        }
    }

    pub fn add_peer(&mut self, peer: Peer) {
        self.peers.peers.push(peer);
    }

    pub fn get_peers(&self) -> &Vec<Peer> {
        &self.peers.peers
    }
}

