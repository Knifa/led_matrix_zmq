use std::sync::Arc;

use gfx::{self, *};
use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Color};
use ggez::{Context, ContextBuilder, GameResult};
use glam::Vec2;

use led_matrix_zmq::server::{ThreadedMatrixServerHandle, MatrixMessage};

const VERT_SHADER: &str = include_str!("matrix.glslv");
const FRAG_SHADER: &str = include_str!("matrix.glslf");

gfx::gfx_defines! {
    constant MatrixPixelShader {
        width: f32 = "u_Width",
        height: f32 = "u_Height",
    }
}

pub struct ViewerOpts {
    pub scale: f32,
}

struct ViewerState {
    frame: Option<graphics::Image>,
    matrix_shader: graphics::Shader<MatrixPixelShader>,
    opts: ViewerOpts,
    zmq_handle: Arc<ThreadedMatrixServerHandle>,
}

impl ViewerState {
    pub fn new(opts: ViewerOpts, zmq_handle: Arc<ThreadedMatrixServerHandle>, ctx: &mut Context) -> GameResult<ViewerState> {
        ggez::input::mouse::set_cursor_hidden(ctx, false);

        let mps: MatrixPixelShader = MatrixPixelShader {
            width: zmq_handle.settings.width as f32,
            height: zmq_handle.settings.height as f32,
        };

        let shader = graphics::Shader::from_u8(
            ctx,
            VERT_SHADER.as_bytes(),
            FRAG_SHADER.as_bytes(),
            mps,
            "MatrixPixelShader",
            None,
        )?;

        Ok(ViewerState {
            frame: None,
            matrix_shader: shader,
            opts,
            zmq_handle,
        })
    }
}

impl EventHandler<ggez::GameError> for ViewerState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        let zmq_msg = match self.zmq_handle.rx.try_recv() {
            Ok(m) => m,
            Err(_) => return Ok(()),
        };

        match zmq_msg {
            MatrixMessage::Frame(frame) => {
                let rgba = frame
                    .chunks(3)
                    .flat_map(|chunk| [chunk[0], chunk[1], chunk[2], 255])
                    .collect::<Vec<_>>();

                let mut img = graphics::Image::from_rgba8(
                    ctx,
                    self.zmq_handle.settings.width as u16,
                    self.zmq_handle.settings.height as u16,
                    &rgba,
                )
                .unwrap();
                img.set_filter(graphics::FilterMode::Nearest);

                self.frame = Some(img);
            },
            _ => (),
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, Color::BLACK);

        if let Some(frame) = self.frame.as_ref() {
            let screen_coords = graphics::screen_coordinates(ctx);
            let screen_center = Vec2::new(screen_coords.w / 2.0, screen_coords.h / 2.0);

            let largest_frame_dim =
                self.zmq_handle.settings.width.max(self.zmq_handle.settings.height) as f32;
            let largest_screen_dim = screen_coords.w.max(screen_coords.h) as f32;
            let scale = largest_screen_dim / largest_frame_dim;

            let draw_param = graphics::DrawParam::new()
                .dest(screen_center)
                .scale(Vec2::new(scale, scale))
                .offset(Vec2::new(0.5, 0.5));

            {
                let _lock = graphics::use_shader(ctx, &self.matrix_shader);
                graphics::draw(ctx, frame, draw_param).unwrap();
            }
        }

        graphics::present(ctx)
    }
}

pub fn run(opts: ViewerOpts, zmq_handle: Arc<ThreadedMatrixServerHandle>) {
    let (mut ctx, event_loop) = ContextBuilder::new("Matrix Viewer", "")
        .window_mode(
            ggez::conf::WindowMode::default()
                .dimensions(
                    zmq_handle.settings.width as f32 * opts.scale,
                    zmq_handle.settings.height as f32 * opts.scale,
                )
        )
        .build()
        .unwrap();

    let viewer = ViewerState::new(opts, zmq_handle, &mut ctx).unwrap();
    event::run(ctx, event_loop, viewer);
}
