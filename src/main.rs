mod network;
use crate::network::Peer;

fn main() {
    let peer = Peer::connect("127.0.0.1:4444").unwrap();

    println!("{}", peer.get_name());

    peer.close();
}
