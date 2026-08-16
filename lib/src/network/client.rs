use tokio::net::TcpStream;
use core::net::SocketAddr;
use log::warn;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub struct Client {
    socket_addr: SocketAddr
}

impl Client {
    pub fn new(
        socket_addr: SocketAddr
    ) -> Self {
        Self {
            socket_addr
        }
    }
    pub fn start_reading(&self, mut stream: TcpStream) {
        tokio::spawn(
            async move {
                loop {
                    if !Self::read(&mut stream).await {
                        return;
                    }
                }
            }
        );
    }
    /// Returns whether it should continue reading or not, if false is returned it disconnects.
    pub async fn read(stream: &mut TcpStream) -> bool {
        let mut buf = [0u8; 1024];

        let read_result =
            stream.read(&mut buf).await;

        match read_result {
            Ok(result) => {
                stream.write_all(
                    &buf[0..result]
                ).await.expect("Implementation for handling this error isn't implements yet.");
                true
            },
            Err(err) => {
                warn!("Reading from a connection returned an error. \
                Disconnecting. \
                The connection may have just been closed, \
                but we aren't checking that yet. Err: {}", err);
                false
            }
        }
    }
}