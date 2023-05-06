use gp2p_rs::p2p;
use std::thread;
use gp2p_rs::tcp::{MessageType, Message};
use std::sync::Arc;
// mutex
use std::sync::Mutex;
fn main() {
    // let p2p_node = Arc::new(p2p::P2P::new("127.0.0.1".to_string(), 8080));
    let p2p_node = Arc::new(Mutex::new(p2p::P2P::new("127.0.0.1".to_string(), 8080)));
    let server_node = Arc::clone(&p2p_node);
    let (tx, rx) = flume::unbounded();

    thread::spawn(move || {
        let server_node = server_node.lock().unwrap();
        server_node.start_server();

    });
    


    thread::sleep_ms(100);

    println!("Server started on port 8080");
    println!("Sending message");
    //w ait
    // wait
    p2p_node.send_message("Hello".to_string(), "123".to_string(), MessageType::Connect, "127.0.0.1".to_string(), 8080);
    println!("Message sent");
    // get peer list
    let peers = p2p_node.get_peers();
    println!("Peers: {:?}", peers);
    loop {
        thread::sleep_ms(1000);
        let peers = p2p_node.get_peers();
        println!("Peers: {:?}", peers);
    }



}