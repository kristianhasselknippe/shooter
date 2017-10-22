use std;
use libc::c_int;
use entities::*;
use scripting::*;
#[macro_use] use scripting::lua::*;
use super::scripting::lua::lua52_sys::*;
use input::*;

pub struct GameState {
    pub script_engine: ScriptEngine,

    entities: Vec<Entity>,
    components: Vec<Component>,
}

impl GameState {
    pub fn new() -> GameState {
        let script_engine = ScriptEngine::new();

        GameState {
            script_engine: script_engine,
            components: Vec::new(),
            entities: Vec::new(),
        }
    }

    pub fn new_entity(&mut self, name: &str) {
        let id = self.script_engine.add_entity(name);
    }

    pub fn get_entities(&mut self) -> Vec<Entity> {
        self.script_engine.get_entities()
    }

    pub fn get_entity(&self, name: &str) -> Option<Entity> {
        self.script_engine.get_entity(name)
    }

    pub fn pre_update(&mut self) {
        self.script_engine.pre_update();
    }

    pub fn update_entities(&mut self, dt: f64) {
        self.script_engine.update_entities(dt);
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

type GameStatePtr = *mut GameState;

luafunction!(get_entity, L, {

    //let game_state = lua_GameStatePtr
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
