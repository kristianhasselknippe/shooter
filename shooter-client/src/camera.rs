use super::na::*;
use super::scripting::lua::{UserData, UserDataProvider, luaL_Reg};
use super::scripting::lua::lua52_sys::*;
use std::mem::size_of;
use libc::{c_int};

pub struct Camera {
    projection: Matrix4<f32>,
}

impl Camera {
    pub fn new_orthographic(w: f32, h: f32) -> Camera {
        let w = w/2.0;
        let h = h/2.0;
        let proj = Matrix4::new_orthographic(-w, w, -h, h,0.0,1000.0);
        Camera {
            projection: proj,
        }
    }

    pub fn camera_matrix(&self) -> Matrix4<f32> {
        self.projection
    }
}


extern "C" fn set_size(L: *mut lua_State) -> c_int{
    /*unsafe {
        let camera = lua_touserdata(L, -1) as *mut Camera;
}*/

    println!("123123: We got called from lua");
    println!("123123: We got called from lua");
    println!("123123: We got called from lua");
    println!("123123: We got called from lua");
    1
}

impl UserDataProvider for Camera {
    fn get_userdata() -> UserData {
        
        let mut ret = Vec::new();
        ret.push(luaL_Reg::new("set_size", set_size));
        ret.push(luaL_Reg::null());
        
        UserData {
            name: "Camera".to_string(),
            size: size_of::<Camera>() as i32,
            methods: ret,
        }
    }
}


