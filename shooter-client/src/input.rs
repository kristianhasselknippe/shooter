use sdl2::event::{Event};
use sdl2::EventPump;
use sdl2::keyboard::Keycode;
use na::Vector3;
use super::scripting::lua::lua52_sys::*;
use std::ptr::null_mut;
use std::ffi::{CStr};

pub struct Input {
    pub left_down: bool,
    pub right_down: bool,
    pub up_down: bool,
    pub down_down: bool,

    event_pump: EventPump,

    pub escape: bool,
}

luafunction!(get_input, L, {
    
});

impl NativeLuaLibrary for Input {
    nativelualib!(
        need to implement native lib for input
        )
}

impl Input {
    pub fn new(event_pump: EventPump) -> Input {
        Input {
            left_down: false,
            right_down: false,
            up_down: false,
            down_down: false,

            event_pump: event_pump,

            escape: false,
        }
    }

    pub fn normalized_input_vector(&self) -> Vector3<f32> {
        let mut x = 0.0;
        let mut y = 0.0;

        if self.left_down { x -= 1.0 }
        if self.right_down { x += 1.0 }
        if self.up_down { y += 1.0 }
        if self.down_down { y -= 1.0 }

        let v = Vector3::new(x,y,1.0);
        let mut ret = v.normalize();
        ret.z = 0.0;
        ret
    }

    pub fn update_sdl_input(&mut self) {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::KeyDown { keycode: Some(Keycode::W), .. } => { self.up_down = true; },
                Event::KeyDown { keycode: Some(Keycode::A), .. } => { self.left_down = true; },
                Event::KeyDown { keycode: Some(Keycode::S), .. } => { self.down_down = true; },
                Event::KeyDown { keycode: Some(Keycode::D), .. } => { self.right_down = true; },
                Event::KeyUp { keycode: Some(Keycode::W), .. } => { self.up_down = false; },
                Event::KeyUp { keycode: Some(Keycode::A), .. } => { self.left_down = false; },
                Event::KeyUp { keycode: Some(Keycode::S), .. } => { self.down_down = false; },
                Event::KeyUp { keycode: Some(Keycode::D), .. } => { self.right_down = false; },

                Event::KeyDown { keycode: Some(Keycode::Escape), .. } |
                Event::Quit { .. } => self.escape = true,
                _ => {}
            }
        }
    }
}
