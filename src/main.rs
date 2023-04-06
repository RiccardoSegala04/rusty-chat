mod network;
use crate::network::Peer;

mod app;
use crate::app::App;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about=None)]
struct Args {

    #[command(subcommand)]
    mode: ChatMode,

}

#[derive(Subcommand)]
enum ChatMode {
    Connect {
        #[arg(short, long, value_name="IP:PORT")]
        destination: String,

        #[arg(short, long)] 
        name: String,
    },
    Accept {
        #[arg(short, long)]
        port: u16,
        
        #[arg(short, long)] 
        name: String,
    },
}

fn main() {
    let args = Args::parse();

    match args.mode {
        ChatMode::Connect{destination, name} => start_client(
            destination.as_str(), name.as_str()),

        ChatMode::Accept{port, name} => start_server(port, name.as_str()),
    }
}

fn start_server(port: u16, name: &str) {
    
    let peer = Peer::accept(port, name).unwrap();
    
    chat_loop(peer);
}

fn start_client(address: &str, name: &str) {
    let peer = Peer::connect(address, name).unwrap();
    
    chat_loop(peer);
}

fn chat_loop(peer: Peer) {
    
    let mut app = App::new(peer);

    app.run().unwrap();

}

