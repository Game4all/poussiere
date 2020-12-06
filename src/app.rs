use crate::input::InputState;
use crate::world::{get_color, Tile, World};
use pixels::{Pixels, SurfaceTexture};
use std::error;
use winit::{event::WindowEvent, window::Window};

pub struct AppState {
    pixels: Pixels<Window>,
    world: World,
    input_state: InputState,
}

impl AppState {
    pub fn create(window: &Window) -> Result<AppState, Box<dyn error::Error>> {
        let win_size = window.inner_size();
        let surface = SurfaceTexture::new(win_size.width, win_size.height, window);
        let pixels = Pixels::new(win_size.width, win_size.height, surface)?;

        let mut world = World::new((win_size.width as u64, win_size.height as u64));

        Ok(AppState {
            pixels,
            world,
            input_state: Default::default(),
        })
    }

    pub fn draw(&mut self) {
        let frame = self.pixels.get_frame();

        for (pixel, tile) in frame.chunks_exact_mut(4).zip(self.world.get_tiles().iter()) {
            pixel.copy_from_slice(get_color(tile));
        }

        self.pixels.render().unwrap();
    }

    pub fn handle_input(&mut self, event: WindowEvent) {
        match event {
            WindowEvent::MouseInput { .. } | WindowEvent::CursorMoved { .. } => {
                self.input_state.update_input(&event)
            }
            _ => {}
        }
    }

    pub fn update(&mut self) {
        if self
            .input_state
            .is_button_pressed(winit::event::MouseButton::Left)
        {
            let pos = self.input_state.get_mouse_pos();
            self.world.set_tile(pos.into(), Tile::Sand);
        }
        self.world.step();
    }
}
