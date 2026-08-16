use std::time::Duration;
use tokio::time::sleep;
use mcp_reimpl::network::mcp_server::McpServer;

#[tokio::main]
async fn main() {
    let server = McpServer::listen(25565, "127.0.0.1")
        .expect("Handler not implemented.");
    sleep(Duration::from_mins(10)).await;
}