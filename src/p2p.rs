// p2p.rs

use crate::tcp;
use crate::tcp::{MessageType, Message};
use serde::{Serialize, Deserialize};
use crate::state::{State, Peer};
use std::thread;
use std::sync::mpsc;
use std::sync::Arc;

#[derive(Debug)]
pub struct P2P {
    pub state: State,
    pub mscp_channel: mpsc::Sender<Message>,
    pub mscp_channel_recv: mpsc::Receiver<Message>,

}

impl P2P {
    pub fn new(ip: String, port: u32) -> P2P
    {
        let (tx, rx) = mpsc::channel();
        P2P {
            state: State::new(ip.clone(), port),
            mscp_channel: tx,
            mscp_channel_recv: rx,
        }
    }

    pub fn start_server(&self) -> Result<(), Box<dyn std::error::Error>> {
        tcp::start_server(8080).unwrap();
        Ok(())

    }

    pub fn send_message(&self, msg: String, id: String, m_type: MessageType, ip: String, port: u16) {
        tcp::send_message(msg, id, m_type, ip, port).unwrap();
    }



    pub fn add_peer(&mut self, peer: Peer) {
        self.state.add_peer(peer);

        
    }

    pub fn get_peers(&self) -> Vec<Peer> {
        self.state.get_peers().to_vec()
    }

    pub fn handle_msg(&mut self,msg: Message) {
        println!("Received message id: {} message: {} type: {:?}", msg.id, msg.message, msg.m_type);
        match msg.m_type {
            MessageType::Connect => {
                println!("Connect");
                // state add peer test
                let peer = Peer {
                    id: msg.id,
                    ip: msg.message,
                    port: 8080,
                };
                println!("Peer: {:?}", peer);
                self.add_peer(peer);
                
            },
            MessageType::Disconnect => {
                println!("Disconnect");
            },
            MessageType::AddPeer => {
                println!("AddPeer");
                
            },
            _ => {
                println!("Unknown");
            }
        }
    }

    pub fn handle_msg_loop(&mut self) {
        let rx = Arc::clone(&self.mscp_channel_recv);
        let mut p2p = self.clone();
        thread::spawn(move || {
            loop {
                let msg = rx.recv().unwrap();
                p2p.handle_msg(msg);
            }
        });
    }
}
