use na::Vector3;
use super::scripting::lua::lua52_sys::*;
use std::ptr::null_mut;
use std::ffi::{CStr};
use libc::{c_void, c_int};
use super::scripting::lua::*;
use super::scripting::*;
use std;

pub struct Input {
    pub left_down: bool,
    pub right_down: bool,
    pub up_down: bool,
    pub down_down: bool,

    //event_pump: EventPump,

    pub escape: bool,
}

luafunction!(get_input, L, {
    unsafe {
        let input = c_void_to_ref!(Input, lua_touserdata(L, 1));
        push_new_table(L)
            .with_value("left_down", &LuaType::Bool(input.left_down))
            .with_value("up_down", &LuaType::Bool(input.up_down))
            .with_value("right_down", &LuaType::Bool(input.right_down))
            .with_value("down_down", &LuaType::Bool(input.down_down));
        1
    }
});

impl NativeLibraryProvider for Input {
    fn get_native_library() -> NativeLibrary {
        NativeLibrary {
            name: "Input".to_string(),
            functions: vec![("get_input".to_string(), get_input)],
        }
    }
}

impl Input {
    pub fn new() -> Input {
        Input {
            left_down: false,
            right_down: false,
            up_down: false,
            down_down: false,

            //event_pump: event_pump,

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

    /*pub fn update_sdl_input(&mut self) {
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
    }*/
}
