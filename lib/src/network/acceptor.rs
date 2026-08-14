use std::net::TcpListener;
use std::sync::atomic::{AtomicBool, Ordering};

pub struct Acceptor {
    continue_accepting: AtomicBool,
    listener: TcpListener
}

impl Acceptor {
    pub fn new(listener: TcpListener) -> Acceptor {
        Self {
            continue_accepting: AtomicBool::new(true),
            listener: listener
        }
    }
    pub fn begin_accepting(self) {
        tokio::spawn(async move {
            loop {
                if !self.continue_accepting.load(Ordering::SeqCst) {

                }
                match self.listener.accept() {
                    Ok(mut conn) => {

                    },
                    Err(err) => {
                        println!("Err: {}", err)
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
}