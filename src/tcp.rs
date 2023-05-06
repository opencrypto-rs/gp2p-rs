use std::net::{TcpListener, TcpStream};
use std::io::{BufReader, Read};
use serde::{Deserialize, Serialize};
use std::io::Write;
use crate::p2p::P2P;
// serde_json 
use serde_json;

pub fn start_server(port: u16) -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port))?;
    print!("Server started on port {}", port);
    
    for stream in listener.incoming() {
        let mut stream = stream?;
        
        // Deserialize the received message using serde
        let mut buf_reader = BufReader::new(&stream);
        let mut buf = String::new();
        buf_reader.read_to_string(&mut buf)?;
        let msg: Message = serde_json::from_str(&buf)?;
        
        // Handle the received message
        // send it with mscp

    }
    
    Ok(())
}

#[derive(Deserialize, Serialize,Debug)]

pub enum MessageType {
    Connect,
    Disconnect,
    AddPeer,
}

#[derive(Deserialize, Serialize,Debug)]
pub struct Message {
    pub id: String,
    pub message: String,
    pub m_type: MessageType,
}



fn coreSendMsg(msg: Message, ip: String, port: u16) -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = TcpStream::connect(format!("{}:{}", ip, port))?;
    let msg = serde_json::to_string(&msg)?;
    stream.write(msg.as_bytes())?;
    Ok(())
}

pub fn send_message(msg: String, id: String, m_type: MessageType, ip: String, port: u16) -> Result<(), Box<dyn std::error::Error>> {
    let msg = Message {
        id,
        message: msg,
        m_type,
    };
    coreSendMsg(msg, ip, port)
} 



// EXAMPLE: send_message "test" "1" MessageType::Connect "