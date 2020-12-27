use crate::{
    gui::Gui,
    input::InputState,
    world::{get_color, Tile, TileType, World},
    Position,
};
use pixels::{Pixels, SurfaceTexture};
use rand::{prelude::ThreadRng, thread_rng, Rng};
use std::error;
use winit::event::{ElementState, Event};
use winit::{event::WindowEvent, window::Window};

const TILE_SIZE: u64 = 4;
pub const WINDOW_WIDTH: u64 = 1024;
pub const WINDOW_HEIGHT: u64 = 768;

/// A struct storing current user state
pub struct UserState {
    pub current_tile: TileType,
    pub brush_size: u64,
    pub running: bool,
    pub edit_action_flag: Option<EditAction>,
    pub action_stack_size: usize,
}

pub enum EditAction {
    Undo,
    Clear,
}

pub struct AppState {
    pixels: Pixels<Window>,
    world: World,
    input_state: InputState,
    gui: Gui,
    user_state: UserState,
    action_stack: Vec<World>,
    rng: ThreadRng,
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
            action_stack: Vec::new(),
            user_state: UserState {
                running: true,
                brush_size: 4u64,
                current_tile: TileType::Sand,
                edit_action_flag: None,
                action_stack_size: 0,
            },
            rng: thread_rng(),
        })
    }

    pub fn draw(&mut self, window: &Window) {
        let frame = self.pixels.get_frame();

        let size = self.world.size();

        for x in 0..size.0 {
            for y in 0..size.1 {
                let tile = self.world.get_tile((x, y).into()).unwrap();
                let color = get_color(tile.tile_type, tile.variant);
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
        let state = &mut self.user_state;

        self.pixels
            .render_with(|encoder, render_target, context| {
                context.scaling_renderer.render(encoder, render_target);
                gui.render(encoder, render_target, context, state)
                    .expect("gui.render() failed");
            })
            .unwrap();
    }

    pub fn handle_input(&mut self, evt: &Event<()>, window: &Window) {
        if let Event::WindowEvent { event, .. } = evt {
            match event {
                WindowEvent::MouseInput { state, .. } => {
                    //whether the input was handled by gui and needs to be handled by the input state
                    let handle_input = !self.gui.handle_event(window, evt);

                    self.input_state.update_input(&event, handle_input);

                    if handle_input && *state == ElementState::Pressed {
                        self.action_stack.push(self.world.clone());
                    }
                }
                WindowEvent::CursorMoved { .. } => self
                    .input_state
                    .update_input(&event, !self.gui.handle_event(window, evt)),
                _ => {}
            }
        }
        self.user_state.action_stack_size = self.action_stack.len();
    }

    pub fn update(&mut self) {
        if self
            .input_state
            .is_button_pressed(winit::event::MouseButton::Left)
        {
            let pos = self.input_state.get_mouse_pos();
            let world_pos = (pos.0 / TILE_SIZE, pos.1 / TILE_SIZE);

            let half_brush_size = self.user_state.brush_size as i64;

            for dx in -half_brush_size..half_brush_size + 1 {
                for dy in -half_brush_size..half_brush_size + 1 {
                    if dx * dx + dy * dy > (half_brush_size * half_brush_size) - 1 {
                        continue;
                    };
                    let px = world_pos.0 + dx as u64;
                    let py = world_pos.1 + dy as u64;

                    self.place_tile((px, py).into(), self.user_state.current_tile);
                }
            }
        }

        if let Some(edit_action) = &self.user_state.edit_action_flag {
            match *edit_action {
                EditAction::Undo => {
                    let last_world = self.action_stack.pop();
                    self.world = last_world.unwrap();
                }
                EditAction::Clear => {
                    self.world.clear();
                }
            }
            self.user_state.edit_action_flag = None;
        }

        if self.user_state.running {
            self.world.step();
        }
    }

    fn place_tile(&mut self, pos: Position, tile: TileType) {
        let variant = self.rng.gen_range(0, 8);

        if let Some(clicked_tile) = self.world.get_tile(pos) {
            if clicked_tile.tile_type != TileType::Air && tile != TileType::Air {
                return;
            }
        }

        self.world.set_tile(
            pos,
            Tile {
                tile_type: tile,
                variant: variant,
            },
        );
    }
}
