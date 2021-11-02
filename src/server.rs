use std::sync::Arc;
use std::sync::mpsc::{channel, Receiver, Sender};

use std::thread;
use zmq;

pub type ThreadedMatrixRx = Receiver<MatrixMessage>;
pub type ThreadedMatrixTx = Sender<MatrixMessage>;

#[non_exhaustive]
pub enum MatrixMessage {
    Frame(Vec<u8>),
}

#[derive(Clone, Debug)]
pub struct MatrixServerSettings {
    pub bind_address: String,
    pub width: u32,
    pub height: u32,
}

pub struct MatrixServer {
    socket: zmq::Socket,
    settings: MatrixServerSettings,
}

impl MatrixServer {
    pub fn new(settings: &MatrixServerSettings) -> Self {
        let context = zmq::Context::new();
        let socket = context.socket(zmq::SocketType::REP).unwrap();
        socket.bind(&settings.bind_address).unwrap();

        MatrixServer {
            socket,
            settings: settings.clone(),
        }
    }

    pub fn recv(&self) -> MatrixMessage {
        let bytes = self.socket.recv_bytes(0).unwrap();
        self.socket.send("", 0).unwrap();

        MatrixMessage::Frame(bytes)
    }
}

pub struct ThreadedMatrixServerHandle {
    pub rx: Arc<ThreadedMatrixRx>,
    pub settings: MatrixServerSettings,
}

pub struct ThreadedMatrixServer {
    thread: Option<thread::JoinHandle<()>>,
    rx: Arc<ThreadedMatrixRx>,
    settings: MatrixServerSettings,
}

impl ThreadedMatrixServer {
    pub fn new(settings: &MatrixServerSettings) -> Self {
        let (tx, rx) = channel();

        let thread_settings = settings.clone();
        let thread = thread::spawn(move || {
            let server = MatrixServer::new(&thread_settings);
            loop {
                let message = server.recv();
                tx.send(message).unwrap();
            }
        });

        ThreadedMatrixServer {
            thread: Some(thread),
            rx: Arc::new(rx),
            settings: settings.clone(),
        }
    }

    pub fn handle(&self) -> ThreadedMatrixServerHandle {
        ThreadedMatrixServerHandle {
            rx: Arc::clone(&self.rx),
            settings: self.settings.clone(),
        }
    }

    pub fn stop(&mut self) {
        match self.thread.take() {
            Some(thread) => {
                thread.join().unwrap();
            }
            None => {}
        }
    }
}
