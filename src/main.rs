use std::cmp::min;

use pixels::{wgpu::Color, Pixels, SurfaceTexture};
use winit::{
    dpi::{LogicalSize, PhysicalSize},
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};

struct Raycaster {
    increment_angle: f32,
    precision: u8,
}

struct Player {
    fov: u16,
    half_fov: u16,
    x: f32,
    y: f32,
    angle: i16,
}

const MAP: [[u8; 10]; 10] = [
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 1, 1, 0, 1, 0, 0, 1],
    [1, 0, 0, 1, 0, 0, 1, 0, 0, 1],
    [1, 0, 0, 1, 0, 0, 1, 0, 0, 1],
    [1, 0, 0, 1, 0, 1, 1, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
];

const SCREEN_WIDTH: u32 = 1500;
const SCREEN_HEIGHT: u32 = 900;

fn main() {
    let mut player = Player {
        fov: 60,
        half_fov: 30,
        x: 1.5,
        y: 1.5,
        angle: 45,
    };
    let mut raycaster = Raycaster {
        increment_angle: player.fov as f32 / SCREEN_WIDTH as f32,
        precision: 64,
    };
    let event_loop = EventLoop::new().unwrap();
    let builder = WindowBuilder::new()
        .with_inner_size(LogicalSize::new(SCREEN_WIDTH, SCREEN_HEIGHT))
        .build(&event_loop)
        .unwrap();
    event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);
    let window_size = builder.inner_size();
    let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &builder);
    let mut pixels = Pixels::new(window_size.width, window_size.height, surface_texture).unwrap();
//    pixels.clear_color(Color::BLUE);
    let mut frame = pixels.frame_mut();
    for pixel in frame.chunks_exact_mut(4) {
        pixel[0] = 255; // R
        pixel[1] = 27; // G
        pixel[2] = 71; // B
        pixel[3] = 0xff; // A
    }
    draw_square(&mut frame, &window_size, 500, 400, 200, 200);
    pixels.render().unwrap();
    event_loop.run(move |event, elwt| {
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                println!("The close button was pressed; stopping");
                elwt.exit();
            }
            Event::AboutToWait => {
                // Application update code.

                // Queue a RedrawRequested event.
                //
                // You only need to call this if you've determined that you need to redraw in
                // applications which do not always need to. Applications that redraw continuously
                // can render here instead.
                builder.request_redraw();
            }
            Event::WindowEvent {
                event: WindowEvent::RedrawRequested,
                ..
            } => {
                // Application update code.

                // Queue a RedrawRequested event.
                //
                // You only need to call this if you've determined that you need to redraw in
                // applications which do not always need to. Applications that redraw continuously
                // can render here instead.
                builder.request_redraw();
            }
            _ => (),
        }
    });
}

const TESTCOLOR: [u8; 4] = [0, 27, 71, 0];

fn draw_square(
    frame: &mut [u8],
    window_size: &PhysicalSize<u32>,
    x: usize,
    y: usize,
    width: usize,
    height: usize,
) {
    for row in 0..min(height, window_size.height as usize) {
        for a in 0..width {
            let new_x = x + a;
            let new_y = y + row;
            let pixel_index = (new_y * window_size.width as usize + (new_x)) * 4;
            frame[pixel_index] = TESTCOLOR[0];
            frame[pixel_index + 1] = TESTCOLOR[1];
            frame[pixel_index + 2] = TESTCOLOR[2];
            frame[pixel_index + 3] = TESTCOLOR[3];
        }
    }
}
