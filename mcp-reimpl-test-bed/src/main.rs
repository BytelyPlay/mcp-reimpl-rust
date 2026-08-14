use mcp_reimpl::network::mcp_server::McpServer;

fn main() {
    let server = McpServer::listen(25565, "127.0.0.1");
}