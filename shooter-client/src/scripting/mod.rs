pub mod script;
#[macro_use] pub mod lua;
pub mod userdata;

use std;
use self::script::*;
use self::lua::*;
use super::na::Vector3;
use std::path::Path;
use of::OrderedFloat;
use libc::c_void;

use std::rc::Rc;

use super::entities::*;
use super::camera::Camera;
use super::game_state::GameState;
use super::input::Input;

#[macro_export]
macro_rules! c_void_to_ref {
    ($to:ty, $e:expr) => {
        std::mem::transmute::<_,&mut $to>($e)
    }
}

#[macro_export]
macro_rules! c_ref_to_void {
    ($e:expr) => {
        std::mem::transmute::<_,*mut c_void>($e)
    }
}

#[macro_export]
macro_rules! nativelualib {
    ($name:expr, $( $x:expr => $y:expr ),* ) => {
        {
            let library = {
                let mut library = Vec::new();
                $(
                    library.push(($x.to_string(), $y));
                )*
                library
            };
            
            NativeLibrary {
                name: $name.to_string(),
                functions: library,
            }
        }
    };
}

#[macro_export]
macro_rules! luafunction {
    ($name:ident, $lua:ident, $body:expr) => {
        extern "C" fn $name($lua: *mut lua_State) -> c_int {
            $body
        }
    }
}

#[derive(Debug)]
pub struct ScriptEngine {
    pub lua: Rc<Lua>,
    script_watcher: ScriptWatcher, //TODO(Rename ScriptWatcher)
}

impl ScriptEngine {
    pub fn new() -> ScriptEngine {
        let mut lua = Rc::new(Lua::new());
        lua.open_libs();
        //TODO: This should be extendable (script as a path)
        let mut sw = ScriptWatcher::new(&Path::new("scripts"));

        ScriptEngine {
            lua: lua,
            script_watcher: sw,
        } 
    }

    pub fn register_native_library(&mut self, lib: &NativeLibrary) {
        self.lua.new_native_library(lib);
    }

    pub fn register_global_pointer(&self, name: &str, ptr: *mut c_void) {
        self.lua.set_global(name, &LuaType::LightUserdata(ptr));
    }

    pub fn new_module(&mut self, p: &Path) {
        self.script_watcher.new_module(p, &self.lua)
    }

    pub fn new_script(&mut self, p: &Path) -> Script {
        self.script_watcher.new_script(p, &self.lua)
    }

    pub fn pre_update(&mut self) {
        self.script_watcher.tick();
    }

    pub fn call(&self, name: &str, args: &[LuaType]) -> Result<LuaType, ()> {
        self.lua.call_global(name, args)
    }

    pub fn update_input(&mut self, left_down: bool,up_down: bool,right_down: bool,down_down: bool) {
        self.call("update_input", &[
            LuaType::Bool(left_down),
            LuaType::Bool(up_down),
            LuaType::Bool(right_down),
            LuaType::Bool(down_down)
        ]).unwrap();
    }

    pub fn update(&self, e: &EntityRef, script: &Script, gs: &GameState, dt: f64) {
        let script_id = script.get_string_id();
       
        self.call(&format!("__entity_scripts.{}.update", script_id), &[
            LuaType::LightUserdata(unsafe {std::mem::transmute::<_,*mut c_void>(gs)}),
            LuaType::Number(OrderedFloat(dt)),
            LuaType::Number(OrderedFloat(e.0 as f64))
        ]);
    }
}
