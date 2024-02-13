use winit::{
    dpi::PhysicalSize,
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

const SCREEN_WIDTH: u16 = 2000;
const SCREEN_HEIGHT: u16 = 1000;

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
    let builder = WindowBuilder::new().build(&event_loop).unwrap();
    //    Window::new(&event_loop).unwrap();
    event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);
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
