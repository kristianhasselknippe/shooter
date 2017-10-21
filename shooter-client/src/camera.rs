use super::na::*;
use super::scripting::lua::{UserData, UserDataProvider, luaL_Reg, luaL_checknumber};
use super::scripting::lua::lua52_sys::*;
use std::mem::{size_of,transmute};
use libc::{c_int};

#[derive(Debug)]
pub struct Camera {
    pub projection: Matrix4<f32>,
    pub size: (f32,f32)
}

impl Camera {
    pub fn new_orthographic(width: f32, height: f32) -> Camera {
        let w = width/2.0;
        let h = height/2.0;
        let proj = Matrix4::new_orthographic(-w, w, -h, h,0.0,1000.0);
        Camera {
            projection: proj,
            size: (width,height),
        }
    }

    pub fn camera_matrix(&self) -> Matrix4<f32> {
        self.projection
    }
}


extern "C" fn set_size(L: *mut lua_State) -> c_int {
    unsafe {
        let mut camera = (luaL_checknumber(L, 1)) as usize;
        let x = luaL_checknumber(L, 2);
        let y = luaL_checknumber(L, 3);


        let mut camera = transmute::<_, *mut Camera>(camera);
        
        let cam = &(*camera);
        

        let old_size = (*camera).size;

        println!("Old size {:?}", old_size);

        let new_camera_values = Camera::new_orthographic(old_size.0 + x as f32, old_size.1 + y as f32);

        println!("New size {:?}", new_camera_values.size);
        
        (*camera).projection = new_camera_values.projection;
        (*camera).size = new_camera_values.size;
    }

    println!("Setting camera size");
    
    1
}

impl UserDataProvider for Camera {
    fn get_userdata() -> UserData {       
        let mut ret = Vec::new();
        ret.push(luaL_Reg::new("set_size", set_size));
        ret.push(luaL_Reg::null());
        
        UserData {
            name: "Camera".to_string(),
            size: size_of::<*const Camera>() as i32,
            methods: ret,
        }
    }
}


