mod network;
use crate::network::Peer;
use std::io;
use std::thread;
use std::sync::{Arc, Mutex};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about=None)]
struct Args {

    #[command(subcommand)]
    mode: ChatMode,
    
    #[arg(short, long)] 
    name: String,

}

#[derive(Subcommand)]
enum ChatMode {
    Connect {
        #[arg(short, long)]
        destination: String,
    },
    Accept {
        #[arg(short, long, default_value_t=4444)]
        port: u16,
    },
}

fn main() {
    let args = Args::parse();

    match args.mode {
        ChatMode::Connect{destination} => start_client(
            destination.as_str(), args.name.as_str()),

        ChatMode::Accept{port} => start_server(port, args.name.as_str()),
    }

    println!("Stopped!");
}

fn start_server(port: u16, name: &str) {
    println!("Server started!");   
    
    let mut peer = Peer::accept(port, name).unwrap();
    
    println!("Connected to: {}", peer.get_name());

    chat_loop(&mut peer);
}

fn start_client(address: &str, name: &str) {
    println!("Client started!");   
    
    let mut peer = Peer::connect(address, name).unwrap();
    
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

            if line.eq("quit\n") || line.len()==0 {
                stream.close();
                break;
            }

            print!("{}: {}", stream.get_name(), line);

        }
    });

    recv_thread.join().unwrap();
}

