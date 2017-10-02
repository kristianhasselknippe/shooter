pub mod script;
pub mod lua;

use self::script::*;
use lua::*;

use super::na::Vector3;
use std::fs::File;
use std::path::Path;
use std::collections::HashMap;
use std::mem::transmute;

use super::entities::*;

#[derive(Clone,Debug)]
pub enum ScriptValue {
    Number(f64),
    Bool(bool),
    String(String),
    Null,
}

pub struct ScriptEngine {
    pub lua: Lua,
    script_watcher: ScriptWatcher, //TODO(Rename ScriptWatcher)
}

impl ScriptEngine {
    pub fn new() -> ScriptEngine {
        let mut lua = Lua::new();
        lua.open_libs();

        let mut sw = ScriptWatcher::new(&Path::new("scripts"));

        sw.new_script_from_file(&Path::new("scripts/globals.lua")).load(&mut lua);
        sw.new_script_from_file(&Path::new("scripts/scene.lua")).load(&mut lua);
        sw.new_script_from_file(&Path::new("scripts/main.lua")).load(&mut lua);

        ScriptEngine {
            lua: lua,
            script_watcher: sw,
        }
    }

    pub fn pre_update(&mut self) {
        self.script_watcher.tick(&mut self.lua);
    }

    pub fn call(&mut self, name: &str, args: &[ScriptValue]) -> ScriptValue {
        ScriptValue::Null
    }

    pub fn update_input(&mut self, left_down: bool,up_down: bool,right_down: bool,down_down: bool) {
        self.call("update_input", &[
            ScriptValue::Bool(left_down),
            ScriptValue::Bool(up_down),
            ScriptValue::Bool(right_down),
            ScriptValue::Bool(down_down)
        ]);
    }

    fn create_entity_from_lua_table(t: &mut LuaType) -> Entity {
        let mut x: f64 = 0.0;
        let mut y: f64 = 0.0;
        /*{
            let mut pos: LuaTable<_> = t.get("position").unwrap();
            x = pos.get("x").unwrap();
            y = pos.get("y").unwrap();
        }
        let name: String = t.get("name").unwrap();*/
        let name = "foobar";

        Entity {
            pos: Vector3::new(x as f32,y as f32,0.0),
            name: name.to_string(),
        }
    }

    pub fn get_entity(&mut self, id: u32) -> Entity {
        let mut fun: LuaType = self.lua.get("get_entity").unwrap();
        let mut table: LuaType = fun.call(&[]).unwrap();
        ScriptEngine::create_entity_from_lua_table(&mut table)
    }

    pub fn get_entities(&mut self) -> Vec<Entity> {
        println!("Getting lua");
        self.lua.call("get_some", &[
            LuaType::String("this is somethign ay".to_string()),
            LuaType::Number(42.123123)
        ]);
        println!("Get me some entities");
        let ret = Vec::new();
        let mut entities: LuaType = self.lua.get("entities").unwrap();
        for e in entities.iter() {
            println!("Getting entitites: {:?}", e);
            /*if let Some(e) = v {
                let entity = ScriptEngine::create_entity_from_lua_table(&e.1);
                ret.push(entity);
            }*/
        }
        ret
    }

    pub fn update_entities(&mut self, dt: f64) {
        self.call("update_entities", &[ScriptValue::Number(dt)]);
    }

    pub fn add_entity(&mut self, name: &str) -> f64 {
        println!("ADding entity");
        let r = self.call("create_entity", &[ScriptValue::String(name.to_string())]);
        /*match r {
            ScriptValue::Number(n) => n,
            _ => panic!("Add entity function didn't return a number"),
    }*/
        0.0
    }
}
