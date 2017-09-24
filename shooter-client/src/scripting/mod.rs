pub extern crate hlua;

use self::hlua::{Lua,LuaFunction,LuaRead,Push,AsMutLua,AnyLuaValue,LuaFunctionCallError,LuaTable};
use std::fs::File;
use std::path::Path;
use std::collections::HashMap;

use self::AnyLuaValue::*;

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
}

impl<'a> ScriptEngine<'a> {
    pub fn new() -> ScriptEngine<'a> {
        let mut lua = Lua::new();
        lua.openlibs();
        let foo = lua.execute_from_reader::<(),_>(File::open(&Path::new("scripts/main.lua")).unwrap());

        ScriptEngine {
            lua: lua
        }
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

    pub fn call_function_table<T: AsMutLua<'lua>>(&mut self, name: &str, args: &[ScriptValue]) -> LuaTable<T> {
        let mut fun: LuaFunction<_> = self.lua.get(name).unwrap();

        match args.len() {
            0 => fun.call_with_args(()).unwrap(),
            1 => fun.call_with_args((args[0].unwrap())).unwrap(),
            2 => fun.call_with_args((args[0].unwrap(),args[1].unwrap())).unwrap(),
            3 => fun.call_with_args((args[0].unwrap(),args[1].unwrap(),args[2].unwrap())).unwrap(),
            4 => fun.call_with_args((args[0].unwrap(),args[1].unwrap(),args[2].unwrap(),args[3].unwrap())).unwrap(),
            _ => panic!("Unsupported number of arguments")
        }
    }

    pub fn update_input(&mut self, left_down: bool,up_down: bool,right_down: bool,down_down: bool) {
        self.call_function("update_input", &[
            ScriptValue::Bool(left_down),
            ScriptValue::Bool(up_down),
            ScriptValue::Bool(right_down),
            ScriptValue::Bool(down_down)
        ]);
    }

    pub fn update_entities(&mut self, dt: f64) {
        self.call_function("update_entities", &[ScriptValue::Number(dt)]);
    }

    pub fn add_entity(&mut self, name: &str) -> f64 {
        let r = self.call_function("create_entity", &[ScriptValue::String(name.to_string())]);
        match r {
            ScriptValue::Number(n) => n,
            _ => panic!("Add entity function didn't return a number"),
        }
    }
}
