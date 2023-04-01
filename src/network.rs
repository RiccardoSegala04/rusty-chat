use std::net::{TcpStream, TcpListener, SocketAddr, Shutdown};

pub struct Peer {
    stream: TcpStream,
    name: String,
}

impl Peer {
    pub fn connect(address: &str) -> std::io::Result<Self> {

        let stream = TcpStream::connect(address)?;

        Ok(Peer { 
            stream, 
            name: String::from("CON_NAME")
        })
    }

    pub fn accept(port: u16) -> std::io::Result<Self> {
        let listener = TcpListener::bind(
            SocketAddr::from(([127,0,0,1], port))
        )?;

        let (stream, _) = listener.accept()?;

        Ok(Peer {
            stream,
            name: String::from("ACC_NAME")
        })
    }

    pub fn close(&self) {
        match self.stream.shutdown(Shutdown::Both) {
            _ => ()
        }
    }

    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }
}
