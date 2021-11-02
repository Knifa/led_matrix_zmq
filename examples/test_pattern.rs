use led_matrix_zmq::client::{MatrixClient, MatrixClientSettings};

fn main() {
    let opts = MatrixClientSettings {
        addr: "tcp://matryx-pi:42024".to_string()
    };

    let client = MatrixClient::new(opts);
    let mut frame = [0u8; 64 * 32 * 3];

    for y in 0..32 {
        for x in 0..64 {
            let i = (y * 64 + x) * 3;
            frame[i] = (x as f32 / 64.0 * 255.0) as u8;
            frame[i + 1] = (y as f32 / 32.0 * 255.0) as u8;
            frame[i + 2] = 0;
        }
    }

    loop {
        client.send_frame(&frame);
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
