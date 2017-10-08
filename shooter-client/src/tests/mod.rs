use std::path::Path;
use super::lua::{Lua};

fn get_lua() -> Lua {
    let mut lua = Lua::new();
    lua.open_libs();
    lua.load(&Path::new("scripts/tests/functionality.lua"));
    lua
}

#[test]
fn test_lua_return_string() {
    let lua = get_lua();
    let result = lua.call_global("foo", &[LuaType::String("my_string".to_string())]).unwrap();
    assert!(result.unwrap_string() == "my_stringroflmy_string");
}

#[test]
fn test_lua_return_number() {
    let lua = get_lua();
    let result = lua.call_global("num", &[]).unwrap();
    assert!(result.unwrap_number() == 123123.0);
}

#[test]
fn test_lua_field() {
    let lua = get_lua();
    let myObj = lua.get_global("myObject").unwrap();
    println!("X: {:?}, Y: {:?}", myObj.get("x"), myObj.get("y"));
}

