mod network;
use crate::network::Peer;

mod app;
use crate::app::App;

use std::io;
use clap::{Parser, Subcommand};

use tui::{Terminal, backend::CrosstermBackend};
use crossterm::{
    execute, 
    terminal::{
        disable_raw_mode, enable_raw_mode, EnterAlternateScreen, 
        LeaveAlternateScreen,
    },
    cursor::{SetCursorShape, EnableBlinking, CursorShape},
    event::{EnableMouseCapture, DisableMouseCapture},
};

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
    
    let mut app = App::new(peer.get_name().to_string(), "192.168.1.69".to_string());

    enable_raw_mode().unwrap();
    let mut stdout = io::stdout();
    execute!(
        stdout, 
        EnterAlternateScreen, 
        EnableMouseCapture, 
        SetCursorShape(CursorShape::Line)
    ).unwrap();
    let backend = CrosstermBackend::new(stdout);
    let mut term = Terminal::new(backend).unwrap();

    app.run(&mut term, peer).unwrap();

    disable_raw_mode().unwrap();
    execute!(
        term.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture,
        SetCursorShape(CursorShape::Block),
        EnableBlinking,
    ).unwrap();
    term.show_cursor().unwrap();

}

