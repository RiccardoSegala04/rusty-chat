mod network;
use crate::network::Peer;
use std::io;
use std::thread;
use std::sync::{Arc, Mutex};

fn main() {
    //start_server();
    start_client();

    println!("Stopped!");
}

fn start_server() {
    println!("Server started!");   
    
    let mut peer = Peer::accept(4444, "Luca").unwrap();
    
    println!("Connected to: {}", peer.get_name());

    chat_loop(&mut peer);
}

fn start_client() {
    println!("Client started!");   
    
    let mut peer = Peer::connect("localhost:4444", "Paolo").unwrap();
    
    println!("Connected to: {}", peer.get_name());

    chat_loop(&mut peer);
}

fn chat_loop(peer: &mut Peer) {

    let sender = Arc::new(Mutex::new(peer.clone()));

    thread::spawn(move || {
        let stdin = io::stdin();
        let mut line = String::new();

        let mut stream = sender.lock().unwrap();

        loop {
            stdin.read_line(&mut line).unwrap();

            if line.eq("quit\n") {
                stream.close();
                break;
            }

            stream.send(line.as_str()).unwrap();

            line.clear();
        }
    });

    let reciever = Arc::new(Mutex::new(peer.clone()));

    let recv_thread = thread::spawn(move || {
        let mut stream = reciever.lock().unwrap();
        loop {
            let line = stream.recieve().unwrap();

            print!("{}: {}", stream.get_name(), line);

            if line.eq("quit\n") || line.len()==0 {
                stream.close();
                break;
            }

        }
    });

    recv_thread.join().unwrap();
}

