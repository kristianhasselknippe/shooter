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
                self.script_engine.update(er, s);
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

macro_rules! c_void_to_ref {
    ($to:ty, $e:expr) => {
        std::mem::transmute::<_,&mut $to>($e)
    }
}

luafunction!(get_entity, L, {
    unsafe {
        let ptr = lua_touserdata(L, 1);
        println!("Got tr: {:?}", ptr);
        let game_state_ptr = c_void_to_ref!(GameState, ptr);

        println!("Got gamestate: {:?}", game_state_ptr.name);
        
        let entity_name = luaL_checklstring(L, 2, null_mut());
        let c_str = CStr::from_ptr(entity_name as _);
        println!("Got arg: {:?}", c_str);
        let mut got_entity = false;

        for (_,e) in &game_state_ptr.ecs.entities {
            if e.name == c_str.to_str().unwrap() {
                println!("Pusing got value");
                push_value(L, &LuaType::String("gotcha back :D, coulda been an entity".to_string()));
                got_entity = true;
            }
        }
        if !got_entity {
            println!("Pushing didn't get value");
            push_value(L, &LuaType::String("Didn't find the entity".to_string()));
        }
    }
    1
});

impl UserDataProvider for GameState {
   fn get_userdata() -> UserData {
        userdata!(
            "GameState",
            "get_entity" => get_entity
        )
    }
}
