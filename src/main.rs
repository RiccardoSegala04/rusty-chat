mod network;
use crate::network::Peer;
use std::io;

fn main() {
    //start_server();
    start_client();

    println!("Stopped!");
}

/*
fn dump_string(line: &str) {
    let bytes = line.as_bytes();

    print!("[");
    for byte in bytes {
        print!("{} ", byte);
    }
    println!("]\n");
}
*/

fn start_server() {
    println!("Server started!");   
    
    let mut peer = Peer::accept(4444, "Luca").unwrap();
    
    println!("Connected to: {}", peer.get_name());

    let stdin = io::stdin();
    loop {
        let mut line: String = peer.recieve().unwrap();

        if line.eq("quit") {
            peer.close();
            break;
        }

        println!("{}: {}", peer.get_name(), line);

        line.clear();
        
        stdin.read_line(&mut line).unwrap();
        line.pop();

        peer.send(line.as_str()).unwrap();

        if line.eq("quit") {
            peer.close();
            break;
        }
    }
}

fn start_client() {
    println!("Client started!");   
    
    let mut peer = Peer::connect("localhost:4444", "Paolo").unwrap();
    
    println!("Connected to: {}", peer.get_name());

    let stdin = io::stdin();
    let mut line = String::new();
    loop {

        line.clear();

        stdin.read_line(&mut line).unwrap();
        line.pop();

        peer.send(line.as_str()).unwrap();

        if line.eq("quit") {
            peer.close();
            break;
        }

        line = peer.recieve().unwrap();

        println!("{}: {}", peer.get_name(), line);

        if line.eq("quit") {
            peer.close();
            break;
        }
    }
}

