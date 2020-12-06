mod app;
mod world;

pub use world::*;

use std::error;

use winit::{event::Event, event::WindowEvent, event_loop::EventLoop, window::WindowBuilder};

fn main() -> Result<(), Box<dyn error::Error>> {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_resizable(false)
        .build(&event_loop)?;

    let mut app = app::AppState::create(&window)?;

    event_loop.run(move |event, _, control_flow| match event {
        Event::MainEventsCleared => {
            app.update();
            window.request_redraw();
        }
        Event::RedrawRequested(_) => {
            app.draw();
        }
        Event::WindowEvent { event, .. } => {
            if event == WindowEvent::CloseRequested {
                *control_flow = winit::event_loop::ControlFlow::Exit;
            } else {
                app.handle_input(event);
            }
        }
        _ => (),
    });
}
