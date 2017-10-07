pub mod script;
pub mod lua;

use self::script::*;
use self::lua::*;

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
    Array,
    Object,
    Function,
    Null,
}

impl From<LuaType> for ScriptValue
{
    fn from(sv: LuaType) -> Self {
        match sv {
            LuaType::Object => { ScriptValue::Object },
            LuaType::Array => { ScriptValue::Array },
            LuaType::String(s) => { ScriptValue::String(s) },
            LuaType::Number(n) => { ScriptValue::Number(n) },
            LuaType::Bool(b) => { ScriptValue::Bool(b) },
            LuaType::Function => { ScriptValue::Function },
            LuaType::Null => { ScriptValue::Null },
        }
    }
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

    pub fn call(&mut self, name: &str, args: &[ScriptValue]) -> Result<ScriptValue, ()> {
        println!("Calling: {}", name);
        let lua_args: Vec<LuaType> = args.iter().map(|a| LuaType::from(a.clone())).collect();
        let ret = self.lua.call_global(name, &lua_args).and_then(|r| {
            Ok(ScriptValue::from(r))
        });
        println!("Done calling: {}", name);
        ret
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
        let mut table: LuaType = self.lua.call_global("get_entity", &[LuaType::Number(id as f64)]).unwrap();
        ScriptEngine::create_entity_from_lua_table(&mut table)
    }

    pub fn get_entities(&mut self) -> Vec<Entity> {
        /*println!("Getting lua");
        let result = self.lua.call("get_some", &[
            LuaType::String("this is somethign ay".to_string()),
            LuaType::Number(42.123123)
        ]).unwrap();
        println!("Result form calling get_some: {:?}", result);*/
        let ret = Vec::new();
        let mut entities: LuaType = self.lua.get_global("entities").unwrap();


        ret
    }

    pub fn update_entities(&mut self, dt: f64) {
        self.lua.call_global("update_entities", &[LuaType::Number(dt)]);
    }

    pub fn add_entity(&mut self, name: &str) -> f64 {
        println!("Adding entity");
        let r = self.lua.call_global("create_entity", &[LuaType::String(name.to_string())]);
        println!("Got back resutl: {:?}", r);
        /*match r {
            ScriptValue::Number(n) => n,
            _ => panic!("Add entity function didn't return a number"),
    }*/
        0.0
    }
}
