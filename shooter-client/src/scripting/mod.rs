pub extern crate hlua;

pub mod script;

use self::script::*;

use super::na::Vector3;
use self::hlua::{Lua,LuaFunction,LuaRead,Push,PushGuard,AsMutLua,AnyLuaValue,LuaFunctionCallError,LuaTable};
use std::fs::File;
use std::path::Path;
use std::collections::HashMap;
use std::mem::transmute;

use self::AnyLuaValue::*;
use super::entities::*;

#[derive(Clone,Debug)]
pub enum ScriptValue {
    Number(f64),
    Bool(bool),
    String(String),
    Null,
}

impl ScriptValue {
    pub fn unwrap(&self) -> AnyLuaValue {
        match self {
            &ScriptValue::Number(n) => LuaNumber(n as f64),
            &ScriptValue::Bool(b) => LuaBoolean(b),
            &ScriptValue::String(ref s) => LuaString(s.clone()),
            &ScriptValue::Null => LuaNil
        }
    }

    pub fn extract_number(&self) -> f64 {
        match self {
            &ScriptValue::Number(n) => n,
            _ => panic!("Exptected number but found something else"),
        }
    }

    pub fn extract_string(&self) -> String {
        match self {
            &ScriptValue::String(ref n) => n.clone(),
            _ => panic!("Expected string but found something else"),
        }
    }

    pub fn from_lua_value(v: AnyLuaValue) -> ScriptValue {
        match v {
            LuaString(s) => ScriptValue::String(s),
            LuaAnyString(_as) => panic!("anystring not handled"),
            LuaNumber(np) => ScriptValue::Number(np),
            LuaBoolean(b) => ScriptValue::Bool(b),
            LuaArray(a) => panic!("Array not implemented in this context"),
            LuaNil => ScriptValue::Null,
            LuaOther => {
                println!("Got \"other\" object");
                ScriptValue::Null
            },
        }
    }
}

pub struct ScriptEngine<'a> {
    pub lua: Lua<'a>,
    script_watcher: ScriptWatcher, //TODO(Rename ScriptWatcher)
}

impl<'a> ScriptEngine<'a> {
    pub fn new() -> ScriptEngine<'a> {
        let mut lua = Lua::new();
        lua.openlibs();

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

    pub fn call_function(&mut self, name: &str, args: &[ScriptValue]) -> ScriptValue {
        let mut fun: LuaFunction<_> = self.lua.get(name).unwrap();

        let result = match args.len() {
            0 => fun.call_with_args(()).unwrap(),
            1 => fun.call_with_args((args[0].unwrap())).unwrap(),
            2 => fun.call_with_args((args[0].unwrap(),args[1].unwrap())).unwrap(),
            3 => fun.call_with_args((args[0].unwrap(),args[1].unwrap(),args[2].unwrap())).unwrap(),
            4 => fun.call_with_args((args[0].unwrap(),args[1].unwrap(),args[2].unwrap(),args[3].unwrap())).unwrap(),
            _ => panic!("Unsupported number of arguments")
        };

        ScriptValue::from_lua_value(result)
    }

    pub fn update_input(&mut self, left_down: bool,up_down: bool,right_down: bool,down_down: bool) {
        self.call_function("update_input", &[
            ScriptValue::Bool(left_down),
            ScriptValue::Bool(up_down),
            ScriptValue::Bool(right_down),
            ScriptValue::Bool(down_down)
        ]);
    }

    fn create_entity_from_lua_table<T: AsMutLua<'a>>(t: &mut LuaTable<T>) -> Entity {
        let mut x: f64 = 0.0;
        let mut y: f64 = 0.0;
        {
            let mut pos: LuaTable<_> = t.get("position").unwrap();
            x = pos.get("x").unwrap();
            y = pos.get("y").unwrap();
        }
        let name: String = t.get("name").unwrap();

        Entity {
            pos: Vector3::new(x as f32,y as f32,0.0),
            name: name,
        }
    }

    pub fn get_entity(&mut self, id: u32) -> Entity {
        let mut fun: LuaFunction<_> = self.lua.get("get_entity").unwrap();
        let mut table: LuaTable<_> = fun.call_with_args((id as f64)).unwrap();
        ScriptEngine::create_entity_from_lua_table(&mut table)
    }

    pub fn get_entities(&mut self) -> Vec<Entity> {
        println!("Get me some entities");
        let ret = Vec::new();
        let mut entities: LuaTable<_> = self.lua.get("entities").unwrap();
        for e in entities.iter::<i32,_>() {
            println!("Getting entitites: {:?}", e);
            if let Some(e) = e {
                let (k,v) : (i32, LuaTable<Lua<'a>>) = e;

            }
            /*if let Some(e) = v {
                let entity = ScriptEngine::create_entity_from_lua_table(&e.1);
                ret.push(entity);
            }*/
        }
        ret
    }

    pub fn update_entities(&mut self, dt: f64) {
        self.call_function("update_entities", &[ScriptValue::Number(dt)]);
    }

    pub fn add_entity(&mut self, name: &str) -> f64 {
        println!("ADding entity");
        let r = self.call_function("create_entity", &[ScriptValue::String(name.to_string())]);
        match r {
            ScriptValue::Number(n) => n,
            _ => panic!("Add entity function didn't return a number"),
        }
    }
}
