use tokio::net::TcpStream;

pub struct Client {
    tcp_stream: TcpStream,
    socket_addr: SocketAddr
}