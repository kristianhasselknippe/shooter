extern crate hlua;

use self::hlua::{Lua,LuaFunction,LuaRead,Push,AsMutLua,AnyLuaValue,LuaFunctionCallError};
use std::fs::File;
use std::path::Path;
use std::collections::HashMap;

use self::AnyLuaValue::*;

pub enum ScriptValue {
    Number(f64),
    Bool(bool),
    String(String),
    Object(HashMap<String,ScriptValue>),
    Null,
}

impl ScriptValue {
    pub fn unwrap(&self) -> AnyLuaValue {
        match self {
            &ScriptValue::Number(n) => LuaNumber(n as f64),
            &ScriptValue::Bool(b) => LuaBoolean(b),
            &ScriptValue::String(ref s) => LuaString(s.clone()),
            &ScriptValue::Object(ref hm) => {
                panic!("Lua object not implemented");
            },
            &ScriptValue::Null => LuaNil
        }
    }
}

pub struct ScriptEngine<'a> {
    lua: Lua<'a>,
}

impl<'a> ScriptEngine<'a> {
    pub fn new() -> ScriptEngine<'a> {
        let mut lua = Lua::new();
        lua.openlibs();
        let foo = lua.execute_from_reader::<(),_>(File::open(&Path::new("scripts/main.lua")).unwrap());
        println!("{:?}", foo);
        {
            let mut main_fun: LuaFunction<_> = lua.get("main").unwrap();
            main_fun.call::<()>();
        }

        {
            let mut create_game_obj: LuaFunction<_> = lua.get("create_game_object").unwrap();
            let go_id: i32 = create_game_obj.call_with_args(("gameObject1")).unwrap();

            println!("Got id back: {}", go_id);
        }

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

        match result {
            LuaString(s) => ScriptValue::String(s),
            LuaAnyString(_as) => panic!("anystring not handled"),
            LuaNumber(n) => ScriptValue::Number(n),
            LuaBoolean(b) => ScriptValue::Bool(b),
            LuaArray(v) => {
                //   ScriptValue::Object()
                panic!("Object as return value not implemented");
            },
            LuaNil => ScriptValue::Null,
            LuaOther => panic!("Other as return value not implemented"),
        }
    }
}
