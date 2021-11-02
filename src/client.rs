use zmq;

#[derive(Clone, Debug)]
pub struct MatrixClientSettings {
    pub addr: String,
}

pub struct MatrixClient {
    pub opts: MatrixClientSettings,

    socket: zmq::Socket,
}

impl MatrixClient {
    pub fn new(opts: MatrixClientSettings) -> MatrixClient {
        let context = zmq::Context::new();
        let socket = context.socket(zmq::REQ).unwrap();
        socket.connect(&opts.addr).expect("Failed to connect to server!");

        MatrixClient {
            opts,
            socket
        }
    }

    pub fn send_frame(&self, frame: &[u8]) {
        self.socket.send(frame, 0).expect("Failed to send frame!");
        self.socket.recv_bytes(0).expect("Couldn't acknowledge sending frame!");
    }
}
