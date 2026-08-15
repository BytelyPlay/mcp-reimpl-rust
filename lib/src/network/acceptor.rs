use std::sync::{Arc};
use std::sync::atomic::{AtomicBool, Ordering};
use futures::executor::block_on;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::RwLock;
use crate::network::client::Client;
use core::net::SocketAddr;

type ThreadSafeClientList = Arc<RwLock<Vec<Client>>>;

pub struct Acceptor {
    continue_accepting: Arc<AtomicBool>,
    client_list: ThreadSafeClientList
}

impl Acceptor {
    pub fn new() -> Acceptor {
        Self {
            continue_accepting: Arc::new(
                AtomicBool::new(true)
            ),
            client_list: Arc::new(RwLock::new(Vec::new())),
        }
    }
    pub fn begin_accepting(&self, listener: TcpListener) {
        let accepting = self.continue_accepting.clone();
        let client_list = self.client_list.clone();

        tokio::spawn(async move {
            loop {
                if !accepting.load(Ordering::SeqCst) {
                    return;
                }
                match listener.accept().await {
                    Ok(conn) => {
                        Self::accept(conn, client_list.clone());
                    },
                    Err(err) => {
                        // TODO: Use an ACTUAL logger.
                        println!("Error while accepting: {}", err)
                    }
                };
            }
        });
    }
    pub fn stop_listening_and_disconnect_all_clients(&self) {
        if self.continue_accepting.load(
            Ordering::SeqCst
        ) {
            self.continue_accepting.store(
                false, Ordering::SeqCst
            )
        }
    }
    fn accept(conn: (TcpStream, SocketAddr), client_list: ThreadSafeClientList) {
        let mut vec = block_on(
            client_list.write()
        );
        (*vec).push(Client::new(conn.0, conn.1));
    }
}