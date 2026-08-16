use tokio::net::TcpListener;
use crate::network::acceptor::Acceptor;
use futures::executor::block_on;

pub struct McpServer {
    acceptor: Acceptor
}

impl McpServer {
    pub fn listen(port: u16, ip: impl Into<String>) -> Result<Self, std::io::Error> {
        let acceptor = Acceptor::new();

        acceptor.begin_accepting(
            block_on(
                TcpListener::bind(
                    format!("{}:{}", ip.into(), port)
                )
            )?
        );

        Ok(
            Self {
                acceptor
            }
        )
    }
    /// This blocks until McpServer is shutdown.
    /// It doesn't send any signal to shut down.
    pub async fn await_until_shutdown() {
        
    }
}