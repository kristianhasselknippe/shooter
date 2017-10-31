use std;
use libc::{c_int,c_void};
use entities::*;
use scripting::*;
#[macro_use] use scripting::lua::*;
use super::scripting::lua::lua52_sys::*;
use input::*;
use std::ptr::null_mut;
use std::ffi::{CStr};
use super::na::Vector2;
use std::path::Path;
use scripting::script::*;

#[derive(Debug)]
pub struct GameState {
    script_engine: ScriptEngine,
    ecs: EntityComponentStore,
    name: String,
}

impl GameState {
    pub fn new(name: &str) -> GameState {
        GameState {
            script_engine: ScriptEngine::new(),
            ecs: EntityComponentStore::new(),
            name: name.to_string(),
        }
    }

    pub fn register_native_library(&mut self, lib: &NativeLibrary) {
        self.script_engine.register_native_library(lib);
    }

    pub fn new_module(&mut self, p: &Path) {
        self.script_engine.new_module(p);
    }

    pub fn new_entity(&mut self, name: &str) -> EntityRef {
        self.ecs.add_entity(Entity::new(name, Vector2::new(0.0,0.0)))
    }

    pub fn new_entity_with_pos(&mut self, name: &str, pos: Vector2<f32>) -> EntityRef {
        self.ecs.add_entity(Entity::new(name, pos))
    }

    pub fn get_entity(&self, er: &EntityRef) -> Option<&Entity> {
        self.ecs.get_entity(er)
    }

    pub fn pre_update(&mut self) {
        //self.script_engine.pre_update();
    }

    pub fn call_script_function(&self, name: &str, val: &[LuaType]) -> Result<LuaType,()> {
        self.script_engine.call(name, val)
    }

    pub fn update_entities(&mut self, dt: f64) {
        for (er, scripts) in &self.ecs.scripts {
            for s in scripts {
                self.script_engine.update(er, s, self, dt);
            }
        }
    }

    pub fn register_script(&mut self, path: &Path, entity: &EntityRef) -> Script {
        let script = self.script_engine.new_script(path);
        self.ecs.add_script(entity, &script);
        script
    }

    pub fn register_global_pointer(&mut self, name: &str, ptr: *mut c_void) {
        self.script_engine.register_global_pointer(name, ptr);
    }
}

luafunction!(get_entity, L, {
    unsafe {
        let ptr = lua_touserdata(L, 1);
        let game_state = c_void_to_ref!(GameState, ptr);
        let entity_ref = EntityRef(lua_tonumberx(L, 2, std::ptr::null_mut()) as u32);
        if let Some(e) = game_state.ecs.entities.get_mut(&entity_ref) {
            push_value(L, &LuaType::LightUserdata(e as *mut _ as *mut c_void));
        } else {
            panic!("GameState(native): Could not find entity by ref: {:?}", entity_ref);
        }
    }
    1
});

impl NativeLibraryProvider for GameState {
   fn get_native_library() -> NativeLibrary {           
       NativeLibrary {
           name: "GameState".to_string(),
           functions: vec![("get_entity".to_string(), get_entity)],
       }
    }
}
