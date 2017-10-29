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

#[derive(Debug)]
pub struct GameState {
    pub script_engine: ScriptEngine,
    pub ecs: EntityComponentStore,
    name: String,
}

impl GameState {
    pub fn new(name: &str) -> GameState {
        let script_engine = ScriptEngine::new();
        GameState {
            script_engine: script_engine,
            ecs: EntityComponentStore::new(),
            name: name.to_string(),
        }
    }

    pub fn new_entity(&mut self, name: &str) -> EntityRef {
        self.ecs.add_entity(Entity::new(name, Vector2::new(0.0,0.0)))
    }

    pub fn get_entity(&self, er: &EntityRef) -> Option<&Entity> {
        self.ecs.get_entity(er)
    }

    pub fn pre_update(&mut self) {
        self.script_engine.pre_update();
    }

    pub fn update_entities(&mut self, dt: f64) {
        for (er, scripts) in &self.ecs.scripts {
            for s in scripts {
                self.script_engine.update(er, s, self, dt);
            }
        }
    }

    pub fn update_input(&mut self, input: &Input) {
        self.script_engine.call("update_input", &[
            LuaType::Bool(input.left_down),
            LuaType::Bool(input.up_down),
            LuaType::Bool(input.right_down),
            LuaType::Bool(input.down_down),
        ]).unwrap();
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
        nativelualib!(
            "GameState",
            "get_entity" => get_entity
        )
    }
}