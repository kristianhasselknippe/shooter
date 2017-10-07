extern crate lua52_sys;

use std::io::Read;
use self::lua52_sys::*;
use std::ptr::{null,null_mut};
use libc::{c_void,size_t,c_char,free,realloc};
use std::ffi::{CString,CStr};
use super::ScriptValue;

#[derive(Clone,Debug)]
pub enum LuaType {
    Object,
    Array,
    String(String),
    Number(f64),
    Bool(bool),
    Function,
    Null
}

impl From<ScriptValue> for LuaType
{
    fn from(sv: ScriptValue) -> Self {
        match sv {
            ScriptValue::Object => { LuaType::Object },
            ScriptValue::Array => { LuaType::Array },
            ScriptValue::String(s) => { LuaType::String(s) },
            ScriptValue::Number(n) => { LuaType::Number(n) },
            ScriptValue::Bool(b) => { LuaType::Bool(b) },
            ScriptValue::Function => { LuaType::Function },
            ScriptValue::Null => { LuaType::Null },
        }
    }
}

impl LuaType {

    pub fn call(&self, args: &[LuaType]) -> Option<LuaType> {
        None
    }

    pub fn get(&self, name: &str) -> Option<LuaType> {
        None
    }

    pub fn iter(&self) -> Result<Vec<LuaType>, ()> {
        Err(())
    }
}

pub struct Lua {
    handle: *mut lua_State,
}

impl Lua {
    pub fn new() -> Lua {
        extern "C" fn alloc(_ud: *mut c_void,
                            ptr: *mut c_void,
                            _osize: size_t,
                            nsize: size_t)
                            -> *mut c_void {
            unsafe {
                if nsize == 0 {
                    free(ptr as *mut c_void);
                    null_mut()
                } else {
                    realloc(ptr, nsize)
                }
            }
        }
        unsafe {
            let state = lua_newstate(alloc, null_mut());

            //println!("State: {:?}", state);
            Lua {
                handle: state
            }
        }

    }



    pub fn execute_from_reader(&self, reader: &mut Read) {
        let mut code = Vec::new();
        reader.read_to_end(&mut code);
        let len = code.len();

        struct Buffer {
            pos: i32,
            buffer: Vec<u8>,
            len: i32,
        }

        let mut buffer = Buffer {
            pos: 0,
            buffer: code,
            len: len as i32,
        };

        extern "C" fn read(_: *mut lua_State, data: *mut c_void, size: *mut size_t) -> *const c_char {
            unsafe {
                let mut buffer: *mut Buffer = data as _;


                if (*buffer).pos == (*buffer).len {
                    (*size) = 0;
                    return 0 as *const c_char;
                } else {
                    (*buffer).pos += (*buffer).len;
                    (*size) = (*buffer).len as size_t;
                    return (*buffer).buffer.as_ptr() as *const c_char
                }
            }

        }

        unsafe {
            let result = lua_load(self.handle, read, &mut buffer as *mut _ as *mut c_void,  b"chunk\0".as_ptr() as *const _, null());

            if result == LUA_OK {
                let mut results = 0;
                lua_call(self.handle as _, 0, 0);
            } else {
                panic!("Error loading script");
            }
        }
    }

    pub fn open_libs(&self) {
        unsafe {
            luaL_openlibs(self.handle);
        }
    }

    fn push_value(&self, val: &LuaType) {
        unsafe {
            match val {
                &LuaType::String(ref s) => {
                    let c_str = CString::new(s.clone()).unwrap();
                    lua_pushstring(self.handle as _, c_str.as_ptr() as _);
                },
                &LuaType::Number(n) => {
                    lua_pushnumber(self.handle as _, n);
                },
                &LuaType::Bool(b) => {
                    lua_pushboolean(self.handle as _, if b { 1 } else { 0 });
                },
                _ => {
                    panic!("Unsupported argument type");
                }
            }
        }
    }

    fn pop_value(&self) -> Option<LuaType> {
        unsafe {
            let t = lua_type(self.handle as _, -1);
            match t {
                LUA_TNIL => { println!("Got null"); Some(LuaType::Null) },
                LUA_TNUMBER => {
                    let number = lua_tonumberx(self.handle as _, -1, null_mut());
                    println!("Got number {}", number);
                    Some(LuaType::Number(number))
                },
                LUA_TBOOLEAN => { println!("Got bool"); None },
                LUA_TSTRING => {
                    println!("Got string");
                    let mut chars = lua_tostring(self.handle as _, -1);
                    let c_string = CString::from_raw(chars as _);
                    let ret = c_string.into_string().unwrap();
                    println!("Returning string: {}", ret);
                    Some(LuaType::String(ret))
                },
                LUA_TTABLE => { println!("Got ttable"); None },
                LUA_TFUNCTION => { println!("Got funciton"); None },
                LUA_TUSERDATA => { println!("Got user data"); None },
                LUA_TTHREAD => { println!("Got thread"); None },
                LUA_TLIGHTUSERDATA => { println!("Got light userdata"); None },
                _ => { panic!("Unrecognized type"); }
            }
        }
    }

    pub fn call(&self, n: &str, args: &[LuaType]) -> Result<LuaType, ()> {
        unsafe {
            let name = CString::new(n).unwrap();
            lua_getglobal(self.handle as _, name.as_ptr() as _);

            for a in args {
                self.push_value(&a);
            }

            let mut results = 1;
            let mut msgh = 0;
            println!("Calling function: {}", n);
            let error_status = lua_pcall(self.handle as _, args.len() as i32, results, msgh);
            match error_status {
                LUA_ERRRUN => {
                    let mut error_message = lua_tostring(self.handle as _, -1) as *mut i8;
                    let err_msg_c_str = CString::from_raw(error_message);
                    let error_message_s = err_msg_c_str.to_str().unwrap();
                    panic!("Runtime error calling function {}", error_message_s);
                },
                LUA_OK => {
                    println!("Function call was ok");
                },
                _ => {
                    panic!("Not ok");
                }
            }

            let stack_size = lua_gettop(self.handle as _);
            println!("Popping value by call: {}", n);
            let result = self.pop_value();
            return match result {
                Some(res) => Ok(res),
                None => Err(()),
            }
        }
        Err(())
    }

    pub fn get(&self, name: &str) -> Option<LuaType> {
        let name_c = CString::new(name).unwrap();
        unsafe {
            lua_getglobal(self.handle as _, name_c.as_ptr() as _);

        }
        None
    }
}
