use tokio::net::TcpStream;
use core::net::SocketAddr;
use tokio::io::AsyncReadExt;
use tokio::io;

pub struct Client {
    socket_addr: SocketAddr
}

impl Client {
    pub fn new(
        socket_addr: SocketAddr
    ) -> Self {
        Self {
            tcp_stream,
            socket_addr
        }
    }
    pub async fn start_reading(&self, stream: TcpStream) {

    }
    pub async fn read(stream: &mut TcpStream, buf: &mut [u8]) -> Result<usize, io::Error> {
        Ok(
            stream.read(buf).await?
        )
    }
}