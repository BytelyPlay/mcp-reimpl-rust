use crate::network::acceptor::Acceptor;
use std::net::TcpListener;

pub struct McpServer {
    acceptor: Acceptor
}

impl McpServer {
    pub fn listen(port: u16, ip: impl Into<String>) -> Result<Self, std::io::Error> {
        let acceptor = Acceptor::new();

        acceptor.begin_accepting(
            TcpListener::bind(
                format!("{}:{}", ip.into(), port)
            )?
        );

        Ok(
            Self {
                acceptor
            }
        )
    }
}