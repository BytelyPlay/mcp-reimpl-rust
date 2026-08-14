use std::io::Write;
use std::net::TcpListener;
use crate::network::acceptor::Acceptor;

pub struct McpServer {
    acceptor: Acceptor
}

impl McpServer {
    pub fn listen(port: u16, ip: impl Into<String>) -> Result<Self, std::io::Error> {
        let acceptor = Acceptor::begin_accepting(
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