use std::net::{TcpStream, TcpListener, SocketAddr, Shutdown};
use std::io::{Write, Read};

pub struct Peer {
    stream: TcpStream,
    name: String,
    ip_str: String,
}

impl Peer {
    pub fn connect(address: &str, name: &str) -> std::io::Result<Self> {

        let mut stream = TcpStream::connect(address)?;

        stream.write(name.as_bytes())?;

        let mut buffer = [0 as u8; 64];
        stream.read(&mut buffer)?;

        Ok(Peer { 
            ip_str: stream.local_addr().unwrap().ip().to_string(),
            stream, 
            name: std::str::from_utf8(&buffer).unwrap().to_string(),
        })
    }

    pub fn accept(port: u16, name: &str) -> std::io::Result<Self> {
        let listener = TcpListener::bind(
            SocketAddr::from(([0,0,0,0], port))
        )?;

        let (mut stream, _) = listener.accept()?;

        let mut buffer = [0 as u8; 64];
        stream.read(&mut buffer)?;

        stream.write(name.as_bytes())?;

        Ok(Peer {
            ip_str: stream.local_addr().unwrap().ip().to_string(),
            stream,
            name: std::str::from_utf8(&buffer).unwrap().to_string(),
        })
    }

    pub fn close(&self) {
        let _ = self.stream.shutdown(Shutdown::Both);
    }

    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }

    pub fn get_ip_str(&self) -> &str {
        self.ip_str.as_str()
    }

    pub fn send(&mut self, message: &str) -> Result<usize, std::io::Error> {
        self.stream.write(message.as_bytes())
    }

    pub fn recieve(&mut self) -> Result<String, std::io::Error> {
        let mut buf = [0 as u8; 64];
        let len = self.stream.read(&mut buf)?;
        Ok(std::str::from_utf8(&buf[0..len]).unwrap().to_string())
    }

    pub fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            stream: self.stream.try_clone().unwrap(),
            ip_str: self.ip_str.clone(),
        }
    }

}
