use crate::input::InputState;
use crate::world::{get_color, Tile, World};
use pixels::{Pixels, SurfaceTexture};
use std::error;
use winit::{event::WindowEvent, window::Window};

const TILE_SIZE: u64 = 8;
pub const WINDOW_WIDTH: u64 = 1024;
pub const WINDOW_HEIGHT: u64 = 768;

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

        let world = World::new((
            (win_size.width as u64 / TILE_SIZE),
            (win_size.height as u64 / TILE_SIZE),
        ));

        Ok(AppState {
            pixels,
            world,
            input_state: Default::default(),
        })
    }

    pub fn draw(&mut self) {
        let frame = self.pixels.get_frame();

        let size = self.world.size();

        for x in 0..size.0 {
            for y in 0..size.1 {
                let color = get_color(self.world.get_tile((x, y).into()).unwrap());
                for tx in 0..TILE_SIZE {
                    for ty in 0..TILE_SIZE {
                        let idx = ((TILE_SIZE * y + ty) * WINDOW_WIDTH * 4
                            + (TILE_SIZE * x + tx) * 4) as usize;
                        for offset in 0..4 {
                            frame[idx + offset] = color[offset];
                        }
                    }
                }
            }
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
            let world_pos = (pos.0 / TILE_SIZE, pos.1 / TILE_SIZE);
            self.world.set_tile(world_pos.into(), Tile::Sand);
        }
        self.world.step();
    }
}
