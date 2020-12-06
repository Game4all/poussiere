mod app;

use std::error;

use winit::{event::Event, event_loop::EventLoop, window::WindowBuilder};

fn main() -> Result<(), Box<dyn error::Error>> {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

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
            app.handle_input(event);
        }
        _ => (),
    });
}
