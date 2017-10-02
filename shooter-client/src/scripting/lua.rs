extern crate lua52_sys;

use std::io::Read;
use self::lua52_sys::*;
use std::ptr::{null,null_mut};
use libc::{c_void,size_t,c_char,free,realloc};
use std::ffi::{CString,CStr};

#[derive(Clone,Debug)]
pub enum LuaType {
    LuaObject,
    LuaArray,
    String(String),
    Number(f64),
    Bool(bool),
    Function
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

            println!("State: {:?}", state);
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


        println!("CODELEN: {}", &buffer.len);

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

            println!("Result: {:?}", result);
            if result == LUA_OK {
                let mut results = 0;
                lua_call(self.handle as _, 0, 0);
                println!("NResults {}", results);
            } else {
                panic!("Error loading script");
            }
        }
    }

    pub fn open_libs(&self) {
        println!("Opening libs");
        unsafe {
            luaL_openlibs(self.handle);
        }
    }

    pub fn call(&self, name: &str, args: &[LuaType]) -> Result<LuaType, ()> {
        unsafe {
            let name = CString::new(name).unwrap();
            lua_getglobal(self.handle as _, name.as_ptr() as _);

            for a in args {
                match a {
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

            let stack_size = lua_gettop(self.handle as _);

            println!("Calling stuff: {}", stack_size);
            let mut results = 0;
            lua_call(self.handle as _, args.len() as i32, results);
            println!("Done calling stuff");
            //let result = lua_tonumberx(self.handle as _, -1, null_mut());
            println!("Results: {}", results);
        }
        Err(())
    }

    pub fn get(&self, name: &str) -> Option<LuaType> {
        let name_c = CString::new(name).unwrap();
        unsafe {
            //lua_getglobal(self.handle as _, name_c.as_ptr() as _);

        }
        None
    }
}
