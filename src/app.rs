use pixels::{Pixels, SurfaceTexture};
use std::error;
use winit::{event::WindowEvent, window::Window};

pub struct AppState {
    pixels: Pixels<Window>,
}

impl AppState {
    pub fn create(window: &Window) -> Result<AppState, Box<dyn error::Error>> {
        let win_size = window.inner_size();
        let surface = SurfaceTexture::new(win_size.width, win_size.height, window);
        let pixels = Pixels::new(win_size.width, win_size.height, surface)?;

        Ok(AppState { pixels: pixels })
    }

    pub fn draw(&mut self) {
        self.pixels.render().unwrap();
    }

    pub fn handle_input(&mut self, event: WindowEvent) {}

    pub fn update(&mut self) {}
}
