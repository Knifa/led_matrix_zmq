mod viewer;

use std::clone::Clone;
use std::sync::Arc;
use structopt::StructOpt;
use led_matrix_zmq::server::{MatrixServerSettings, ThreadedMatrixServer};

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

impl Into<MatrixServerSettings> for Opt {
    fn into(self) -> MatrixServerSettings {
        MatrixServerSettings {
            bind_address: self.bind_address,
            width: self.width,
            height: self.height,
        }
    }
}

fn main() {
    let opt = Opt::from_args();
    opt.print_summary();

    let mut server = ThreadedMatrixServer::new(&opt.clone().into());
    let server_handle = server.handle();
    let server_handle = Arc::new(server_handle);

    viewer::run(
        viewer::ViewerOpts { scale: opt.scale },
        server_handle.clone(),
    );

    server.stop();
}
