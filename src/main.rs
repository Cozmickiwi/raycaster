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

const SCREEN_WIDTH: u16 = 1500;
const SCREEN_HEIGHT: u16 = 900;
const HALF_HEIGHT: u16 = 450;
const SCALE: u16 = 5;

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

    /*
    for pixel in frame.chunks_exact_mut(4) {
        pixel[0] = 255; // R
        pixel[1] = 27; // G
        pixel[2] = 71; // B
        pixel[3] = 0xff; // A
    }*/
    //draw_square(&mut frame, &window_size, 500, 400, 200, 200);
    //draw_square(&mut frame, &window_size, 300, 150, 100, 50);
    //    raycast(&player, &mut raycaster, frame, &window_size);
    //    pixels.render().unwrap();
    event_loop
        .run(move |event, elwt| {
            let surface_texture =
                SurfaceTexture::new(window_size.width, window_size.height, &builder);
            let mut pixels =
                Pixels::new(window_size.width, window_size.height, surface_texture).unwrap();
            //    pixels.clear_color(Color::BLUE);
            //pixels.clear_color(Color::RED);

            let mut frame = pixels.frame_mut();
            for pixel in frame.chunks_exact_mut(4) {
                pixel[0] = 255; // R
                pixel[1] = 27; // G
                pixel[2] = 71; // B
                pixel[3] = 0xff; // A
            }
            raycast(&player, &mut raycaster, frame, &window_size);
            pixels.render().unwrap();
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

                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::KeyboardInput {
                        device_id,
                        event,
                        is_synthetic,
                    } => match event.physical_key {
                        winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::KeyA) => {
                            player.angle -= 2;
                            if player.angle < 0 {
                                player.angle = 360;
                            }
                        }
                        _ => {}
                    },
                    _ => {}
                },
                _ => (),
            }
        })
        .unwrap();
}

const TESTCOLOR: [u8; 4] = [0, 27, 71, 0];
const BLUE1: [u8; 4] = [25, 122, 154, 0];
const PURPLE1: [u8; 4] = [131, 60, 169, 0];
const RED1: [u8; 4] = [154, 25, 70, 0];

fn draw_square(
    frame: &mut [u8],
    window_size: &PhysicalSize<u32>,
    x: u32,
    y: u32,
    width: usize,
    height: usize,
    color: [u8; 4],
) {
    let w_height = window_size.height;
    for row in (0..min(height, window_size.height as usize)).rev() {
        for a in (0..width).step_by(4) {
            let new_x = x as usize + a;
            let new_y = w_height as usize - (y as usize + row);
            let pixel_index = (new_y * window_size.width as usize + (new_x)) * 4;
            if pixel_index > frame.len() - 3 {
                continue;
            }
            for i in frame[pixel_index..pixel_index + 4].chunks_exact_mut(4) {
                i[0] = color[0];
                i[1] = color[1];
                i[2] = color[2];
            }
            /*
            frame[pixel_index] = color[0];
            frame[pixel_index + 1] = color[1];
            frame[pixel_index + 2] = color[2];
            frame[pixel_index + 3] = color[3];
            */
        }
    }
}

struct Ray {
    x: f32,
    y: f32,
}

fn raycast(
    player: &Player,
    ray: &mut Raycaster,
    frame: &mut [u8],
    window_size: &PhysicalSize<u32>,
) {
    let mut ray_angle = player.angle as f32 - player.half_fov as f32;
    for i in 0..window_size.width as u16 / SCALE {
        let mut ray_struct = Ray {
            x: player.x,
            y: player.y,
        };
        // Increment
        ray_angle += ray.increment_angle * SCALE as f32;
        let ray_rad = ray_angle.to_radians();
        let pres = ray.precision as f32;
        let ray_cos = ray_rad.cos() / pres;
        let ray_sin = ray_rad.sin() / pres;

        // Wall check
        let mut wall = 0;
        while wall == 0 {
            ray_struct.x += ray_cos;
            ray_struct.y += ray_sin;
            wall = MAP[ray_struct.y as usize][ray_struct.x as usize];
        }
        let mut distance =
            ((player.x - ray_struct.x).powi(2) + (player.y - ray_struct.y).powi(2)).sqrt();
        let panglef = ray_angle - player.angle as f32;
        // wall height
        distance = distance * (panglef.to_radians()).cos();
        let wall_height = (window_size.height / 2) as f32 / distance;
        let mut wall_start = window_size.height / 2 - wall_height as u32;
        if distance <= 1.0 {
            wall_start = 1;
        }
        let true_height = wall_height as u32 * 2;
        draw_square(
            frame,
            window_size,
            i as u32 * SCALE as u32,
            wall_start - 1,
            SCALE as usize,
            wall_height as usize * 2,
            PURPLE1,
        );
        draw_square(
            frame,
            window_size,
            i as u32 * SCALE as u32,
            0,
            SCALE as usize,
            wall_start as usize,
            BLUE1,
        );
        draw_square(
            frame,
            window_size,
            i as u32 * SCALE as u32,
            window_size.height - wall_start - 1,
            SCALE as usize,
            //            wall_height as usize - (wall_start + (wall_height as u32 * 2)) as usize,
            wall_start as usize + 4,
            RED1,
        );
    }
}
