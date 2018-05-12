use na::Vector2;
use glutin;

#[derive(Debug)]
pub struct Input {
    pub forward: bool,
    pub backward: bool,
    pub left: bool,
    pub right: bool,
    pub up: bool,
    pub down: bool,

    pub escape: bool,

    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,

    pub mouse_left: bool,
    pub mouse_right: bool,
    pub mouse_middle: bool,

    pub mouse_delta: Vector2<f32>,
}

macro_rules! pressed_state_to_bool {
    ($e:expr) => {
        {
            if $e == glutin::ElementState::Pressed {
                true
            } else {
                false
            }
        }
    }
}

impl Input {
    pub fn new() -> Input {
        Input {
            forward: false,
            backward: false,
            left: false,
            right: false,
            up: false,
            down: false,

            shift: false,
            ctrl: false,
            alt: false,

            escape: false,

            mouse_left: false,
            mouse_right: false,
            mouse_middle: false,

            mouse_delta: Vector2::new(0.0, 0.0),
        }
    }

    pub fn update_mouse_buttons(&mut self,
                                button: &glutin::MouseButton,
                                state: &glutin::ElementState,
                                _modifiers: &glutin::ModifiersState) {
        match button {
            &glutin::MouseButton::Left => {
                self.mouse_left = pressed_state_to_bool!(*state);
            }
            &glutin::MouseButton::Right => {
                self.mouse_right = pressed_state_to_bool!(*state);
            }
            &glutin::MouseButton::Middle => {
                self.mouse_middle = pressed_state_to_bool!(*state);
            }
            _ => {}
        }
    }

    pub fn update_glutin_keyboard_input(&mut self, input: &glutin::KeyboardInput) {
        if let Some(keycode) = input.virtual_keycode {
            match keycode {
                glutin::VirtualKeyCode::A => {
                    self.left = pressed_state_to_bool!(input.state);
                }
                glutin::VirtualKeyCode::D => {
                    self.right = pressed_state_to_bool!(input.state);
                }
                glutin::VirtualKeyCode::W => {
                    self.forward = pressed_state_to_bool!(input.state);
                }
                glutin::VirtualKeyCode::S => {
                    self.backward = pressed_state_to_bool!(input.state);
                }
                glutin::VirtualKeyCode::Q => {
                    self.down = pressed_state_to_bool!(input.state);
                }
                glutin::VirtualKeyCode::E => {
                    self.up = pressed_state_to_bool!(input.state);
                }
                glutin::VirtualKeyCode::LShift |
                glutin::VirtualKeyCode::RShift => {
                    self.shift = pressed_state_to_bool!(input.state);
                }
                glutin::VirtualKeyCode::LAlt |
                glutin::VirtualKeyCode::RAlt => {
                    self.alt = pressed_state_to_bool!(input.state);
                }
                glutin::VirtualKeyCode::LControl |
                glutin::VirtualKeyCode::RControl => {
                    self.ctrl = pressed_state_to_bool!(input.state);
                }
                glutin::VirtualKeyCode::Escape => {
                    self.escape = pressed_state_to_bool!(input.state);
                }
                _ => {}
            }
        }
    }

    pub fn reset_mouse_delta(&mut self) {
        self.mouse_delta = Vector2::new(0.0, 0.0);
    }

    pub fn update_glutin_mouse_delta(&mut self, axis: u32, value: f32) {
        if axis == 0 {
            self.mouse_delta += Vector2::new(value, 0.0);
        } else {
            self.mouse_delta -= Vector2::new(0.0, value);
        }
    }
}
