mod app;
mod gui;
mod input;
mod world;

pub use world::*;

use std::error;

use winit::{
    dpi::{PhysicalSize, Size},
    event::Event,
    event::WindowEvent,
    event_loop::EventLoop,
    window::WindowBuilder,
};

fn main() -> Result<(), Box<dyn error::Error>> {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_inner_size(Size::Physical(PhysicalSize::new(
            app::WINDOW_WIDTH as u32,
            app::WINDOW_HEIGHT as u32,
        )))
        .with_title("poussière")
        .with_resizable(false)
        .build(&event_loop)?;

    let mut app = app::AppState::create(&window)?;

    event_loop.run(move |evt, _, control_flow| match &evt {
        Event::MainEventsCleared => {
            app.update();
            window.request_redraw();
        }
        Event::RedrawRequested(_) => app.draw(&window),
        Event::WindowEvent { event, .. } => match event {
            WindowEvent::CloseRequested => {
                *control_flow = winit::event_loop::ControlFlow::Exit;
            }
            _ => app.handle_event(&evt, &window),
        },
        _ => (),
    });
}
