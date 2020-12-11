use winit::event::{ElementState, MouseButton, WindowEvent};

#[derive(Default)]
pub struct InputState {
    mouse_pos: (u64, u64),
    mouse_buttons: [bool; 4],
}

impl InputState {
    pub fn is_button_pressed(&self, btn: MouseButton) -> bool {
        self.mouse_buttons[id_for_button(btn)]
    }

    pub fn get_mouse_pos(&self) -> (u64, u64) {
        self.mouse_pos
    }

    pub fn update_input(&mut self, event: &WindowEvent, update_mouse_buttons: bool) {
        match event {
            WindowEvent::CursorMoved { position, .. } => {
                self.mouse_pos = (position.x as u64, position.y as u64)
            }
            WindowEvent::MouseInput { button, state, .. } if update_mouse_buttons => {
                self.mouse_buttons[id_for_button(*button)] = if *state == ElementState::Pressed {
                    true
                } else {
                    false
                }
            }
            _ => {}
        }
    }
}

fn id_for_button(btn: MouseButton) -> usize {
    match btn {
        MouseButton::Left => 0,
        MouseButton::Right => 1,
        MouseButton::Middle => 2,
        MouseButton::Other(_) => 3,
    }
}
