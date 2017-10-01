use std::io::Read;

#[derive(Clone,Debug)]
pub enum LuaType {
    LuaObject,
    LuaArray,
    String,
    Number,
    Bool,
    Function
}

#[derive(Clone,Debug)]
pub struct LuaObject {
    handle: i32,
    lua_type: LuaType
}

impl LuaObject {

    pub fn call(&self, args: &[LuaObject]) -> Option<LuaObject> {
        None
    }

    pub fn get(&self, name: &str) -> Option<LuaObject> {
        None
    }

    pub fn iter(&self) -> Result<Vec<LuaObject>, ()> {
        Err(())
    }
}

pub struct Lua {
    handle: i32
}

impl Lua {
    pub fn new() -> Lua {
        Lua {
            handle: 0
        }
    }

    pub fn execute_from_reader(&self, reader: &Read) {

    }

    pub fn open_libs(&self) {
        //TODO: open libs!
    }

    pub fn get(&self, name: &str) -> Option<LuaObject> {
        None
    }
}
