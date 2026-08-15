use tokio::net::TcpListener;
use tokio::net::unix::SocketAddr;
use crate::network::acceptor::Acceptor;

pub struct McpServer {
    acceptor: Acceptor
}

impl McpServer {
    pub fn listen(port: u16, ip: impl Into<String>) -> Result<Self, std::io::Error> {
        let acceptor = Acceptor::new();

        acceptor.begin_accepting(
            TcpListener::bind(
                SocketAddr
            )?
        );

        Ok(
            Self {
                acceptor
            }
        )
    }
}