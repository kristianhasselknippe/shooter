use super::na::*;
use super::scripting::lua::{NativeLibrary, NativeLibraryProvider, luaL_Reg, luaL_checknumber};
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


luafunction!(set_size, L, {
    unsafe {
        let mut camera = (lua_touserdata(L, 1)) as *mut Camera;
        let x = luaL_checknumber(L, 2);
        let y = luaL_checknumber(L, 3);

        let old_size = (*camera).size;
        let new_camera_values = Camera::new_orthographic(old_size.0 + x as f32, old_size.1 + y as f32);
        
        (*camera).projection = new_camera_values.projection;
        (*camera).size = new_camera_values.size;
    }
    
    1
});

/*luafunction!(new_camera, L, {
    unsafe {
        let lua_camera = lua_newuserdata(L, size_of::<Camera>() as _) as *mut Camera;
        let x = luaL_checknumber(L, 1);
        let y = luaL_checknumber(L, 2);
        let cam = Camera::new_orthographic(x as f32, y as f32);

        (*lua_camera).projection = cam.projection;
        (*lua_camera).size = cam.size;
    }
    1
});*/

impl NativeLibraryProvider for Camera {
    fn get_native_library() -> NativeLibrary {       
        let native_lib = nativelualib!(
            "Camera",
            "set_size" => set_size
            //"new" => new_camera
        );
        println!("Native lib: {:?}", native_lib);
        native_lib
    }
}


