use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::Arc;

use std::thread;
use zmq;

pub type ZmqChannelRx = Receiver<ZqmServerMessage>;
pub type ZmqChannelTx = Sender<ZqmServerMessage>;

pub enum ZqmServerMessage {
    Frame(Vec<u8>),
}

#[derive(Clone, Debug)]
pub struct ZmqOpts {
    pub bind_address: String,
    pub width: u32,
    pub height: u32,
}

pub struct ZmqServer {
    opts: ZmqOpts,
}

pub struct ZmqHandle {
    pub rx: ZmqChannelRx,
    pub opts: ZmqOpts,
}

impl ZmqServer {
    pub fn new(opts: ZmqOpts) -> Self {
        ZmqServer { opts }
    }

    pub fn run_thread(self) -> (Arc<ZmqHandle>, std::thread::JoinHandle<()>) {
        let (tx, rx): (ZmqChannelTx, ZmqChannelRx) = channel();
        let cloned_opts = self.opts.clone();

        let join_handle = thread::spawn(move || {
            self.run(tx);
        });

        return (
            Arc::new(ZmqHandle {
                rx,
                opts: cloned_opts,
            }),
            join_handle,
        );
    }

    fn run(self, tx: Sender<ZqmServerMessage>) {
        let context = zmq::Context::new();
        let socket = context.socket(zmq::SocketType::REP).unwrap();
        socket.bind(&self.opts.bind_address).unwrap();

        loop {
            let msg_bytes = socket.recv_bytes(0);
            socket.send(zmq::Message::new(), 0).unwrap();

            match msg_bytes {
                Ok(bytes) => {
                    let msg = ZqmServerMessage::Frame(bytes);
                    tx.send(msg).unwrap();
                }
                Err(e) => {
                    println!("Error: {}", e);
                }
            }
        }
    }
}
