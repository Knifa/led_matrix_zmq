mod viewer;

use std::clone::Clone;
use structopt::StructOpt;
use led_matrix_zmq::server::{ZmqOpts, ZmqServer};

#[derive(Clone, Debug, StructOpt)]
#[structopt(name = "led-matrix-zmq-virtual")]
pub struct Opt {
    #[structopt(short, long, default_value = "64")]
    /// Matrix width, in pixels.
    pub width: u32,

    #[structopt(short, long, default_value = "32")]
    /// Matrix height, in pixels.
    pub height: u32,

    #[structopt(short, long, default_value = "16")]
    /// Matrix scale, for the size of the window.
    pub scale: f32,

    #[structopt(short, long, default_value = "tcp://*:42024")]
    /// Address to bind ZMQ server to.
    pub bind_address: String,
}

impl Opt {
    fn print_summary(&self) {
        println!("Listening on {}", self.bind_address);
        println!("Matrix size: {}x{}", self.width, self.height);
        println!("Expected bytes per frame: {}", self.width * self.height * 3);
    }
}

impl Into<ZmqOpts> for Opt {
    fn into(self) -> ZmqOpts {
        ZmqOpts {
            bind_address: self.bind_address,
            width: self.width,
            height: self.height,
        }
    }
}

fn main() {
    let opt = Opt::from_args();
    opt.print_summary();

    let server = ZmqServer::new(opt.clone().into());
    let (server_handle, join_handle) = server.run_thread();

    viewer::run(
        viewer::ViewerOpts { scale: opt.scale },
        server_handle.clone(),
    );
    join_handle.join().unwrap();
}
