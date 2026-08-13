use std::net::TcpListener;

pub struct McpServer {
    listener: TcpListener
}

impl McpServer {
    pub fn listen(port: u16, ip: impl Into<String>) -> Result<Self, std::io::Error> {
        let result = Self {
                listener: TcpListener::bind(
                    format!("{}:{}", ip.into(), port)
                )?
            };

        result.begin_accepting();

        Ok(
            result
        )
    }
    fn begin_accepting(&self) {
        tokio::spawn(
            async {
                let mut buf = [0u8; 1024];

                self.listener.accept();

                loop {

                }
            }
        );
    }
}