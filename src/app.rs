use crate::gui::Gui;
use crate::input::InputState;
use crate::world::{get_color, World};
use pixels::{Pixels, SurfaceTexture};
use std::error;
use winit::event::Event;
use winit::{event::WindowEvent, window::Window};

const TILE_SIZE: u64 = 8;
pub const WINDOW_WIDTH: u64 = 1024;
pub const WINDOW_HEIGHT: u64 = 768;

pub struct AppState {
    pixels: Pixels<Window>,
    world: World,
    input_state: InputState,
    gui: Gui,
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

        let gui = Gui::new(window, &pixels);

        Ok(AppState {
            pixels,
            world,
            input_state: Default::default(),
            gui,
        })
    }

    pub fn draw(&mut self, window: &Window) {
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

        self.gui.prepare(window).expect("so");

        let gui = &mut self.gui;

        self.pixels
            .render_with(|encoder, render_target, context| {
                context.scaling_renderer.render(encoder, render_target);
                gui.render(encoder, render_target, context)
                    .expect("gui.render() failed");
            })
            .unwrap();
    }

    pub fn handle_input(&mut self, evt: &Event<()>, window: &Window) {
        if let Event::WindowEvent { event, .. } = evt {
            match event {
                WindowEvent::MouseInput { .. } | WindowEvent::CursorMoved { .. } => {
                    self.input_state
                        .update_input(&event, !self.gui.handle_event(window, evt));
                }
                _ => {}
            }
        }
    }

    pub fn update(&mut self) {
        if self
            .input_state
            .is_button_pressed(winit::event::MouseButton::Left)
        {
            let pos = self.input_state.get_mouse_pos();
            let world_pos = (pos.0 / TILE_SIZE, pos.1 / TILE_SIZE);

            let half_brush_size = self.gui.user_state.brush_size as i64;

            for dx in -half_brush_size..half_brush_size + 1 {
                for dy in -half_brush_size..half_brush_size + 1 {
                    if dx * dx + dy * dy > (half_brush_size * half_brush_size) - 1 {
                        continue;
                    };
                    let px = world_pos.0 + dx as u64;
                    let py = world_pos.1 + dy as u64;

                    self.world
                        .set_tile((px, py).into(), self.gui.user_state.current_tile);
                }
            }
        }
        self.world.step();
    }
}
