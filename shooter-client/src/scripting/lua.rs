extern crate lua52_sys;

use std::io::Read;
use std::path::Path;
use std::fs::File;
use self::lua52_sys::*;
use std::ptr::{null,null_mut};
use libc::{c_void,size_t,c_char,free,realloc};
use std::ffi::{CString,CStr};
use of::OrderedFloat;


#[derive(Clone,Debug)]
pub enum LuaType {
    Table(Vec<(LuaType, LuaType)>),
    Array(Vec<LuaType>),
    String(String),
    Number(OrderedFloat<f64>),
    Bool(bool),
    Function,
    Thread,
    UserData,
    Null
}

impl LuaType {
    pub fn unwrap_string<'a>(&'a self) -> &'a str {
        match self {
            &LuaType::String(ref s) => s,
            _ => panic!("Lua type was not a string"),
        }   
    }
    
    pub fn unwrap_number(&self) -> f64 {
        match self {
            &LuaType::Number(n) => n.0,
            _ => panic!("Lua type was not a number"),
        }   
    }
    
    pub fn unwrap_bool(&self) -> bool {
        match self {
            &LuaType::Bool(b) => b,
            _ => panic!("Lua type was not a boolean"),
        }   
    }

    pub fn unwrap_array<'a>(&'a self) -> &'a Vec<LuaType> {
        match self {
            &LuaType::Array(ref arr) => arr,
            _ => panic!("Lua type was not a table"),
        }
    }

    pub fn get<'a>(&'a self, name: &str) -> Result<&'a LuaType,()> {
        match self {
            &LuaType::Table(ref fields) => {
                for &(ref k,ref v) in fields {
                    if let &LuaType::String(ref k) = k {
                        if k == name {
                            return Ok(v);
                        }
                    }
                }
                println!("{:?}", self);
                panic!("Could not find field {} on lua table", name);
            }
            _ => panic!("LuaType was not a table; can't get field {}", name),
        }
    }
}

pub struct Lua {
    handle: *mut lua_State,
}

impl Drop for Lua {
    fn drop(&mut self) {
    }
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

    pub fn print_stack_dump(&self) {
        unsafe {
            let top = lua_gettop(self.handle as _);
            println!("StackSize: {}", top);
            for i in 1 .. top + 1 {
                let t = lua_type(self.handle as _, i);
                match t {
                    LUA_TSTRING => {
                        let lua_str = lua_tostring(self.handle as _, i);
                        let c_str = CStr::from_ptr(lua_str);
                        /* strings */println!("\t{} => {}", i, c_str.to_str().unwrap());
                    },
                    LUA_TBOOLEAN => {
                        /* booleans */
                        println!("\t{} => {}", i, if lua_toboolean(self.handle as _, i) == 1  { "true" } else { "false" });
                    },
                    LUA_TNUMBER => {
                        /* numbers */
                        println!("\t{} => {}", i, lua_tonumberx(self.handle as _, i, null_mut()));
                    },
                    _ => {
                        let lua_str = lua_typename(self.handle as _, t);
                        let c_str = CStr::from_ptr(lua_str);
                        println!("\t{} => {}",i, c_str.to_str().unwrap());
                    }   
                }
                println!("  ");  /* put a separator */
            }
            if (top > 0) {
                println!("");  /* end the listing */
            }
        }
        
    }

    pub fn load(&self, path: &Path) {
        let mut file = File::open(path).unwrap();
        let filename = path.file_stem().unwrap().to_str().unwrap().to_string();
        self.execute_from_reader(&mut file, &filename);
    }

    pub fn execute_from_reader(&self, reader: &mut Read, module_name: &str) {
        let mut code = Vec::new();
        reader.read_to_end(&mut code).unwrap();
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
            lua_getglobal(self.handle as _, CString::new("package".to_string()).unwrap().as_ptr() as _);
            lua_getfield(self.handle as _, -1, CString::new("loaded".to_string()).unwrap().as_ptr() as _);

            let cn = format!("{}\0", module_name);
            let chunk_name = cn.as_bytes();
            let result = lua_load(self.handle, read, &mut buffer as *mut _ as *mut c_void,  chunk_name.as_ptr() as *const _, null());
            if result == LUA_OK {
                lua_call(self.handle as _, 0, 1); //should return a module
                lua_setfield(self.handle as _, -2, CString::new(module_name.to_string()).unwrap().as_ptr() as _);
                self.pop();
                self.pop();
            } else {
                panic!("Error loading script");
            }
        }
    }

    fn call(&self, args: &[LuaType], n: &str) -> Result<LuaType, ()> {
        unsafe {
            for a in args {
                self.push_value(&a);
            }

            let results = 1;
            let msgh = 0;
            let error_status = lua_pcall(self.handle as _, args.len() as i32, results, msgh);
            match error_status {
                LUA_ERRRUN => {
                    let error_message = lua_tostring(self.handle as _, -1) as *mut i8;
                    let err_msg_c_str = CString::from_raw(error_message);
                    let error_message_s = err_msg_c_str.to_str().unwrap();
                    panic!("Runtime error calling function {}: {}", n, error_message_s);
                },
                LUA_OK => { },
                _ => {
                    panic!("Not ok");
                }
            }
            let result = self.get_top_value(0);
            self.pop();
            Ok(result)
        }
    }

    pub fn call_method(&self, n: &str, args: &[LuaType]) -> Result<LuaType, ()> {
        unsafe {
            let name = CString::new(n).unwrap();
            lua_getfield(self.handle as _, -1, name.as_ptr() as _);
            self.call(args, n)
        }
    }

    pub fn call_global(&self, n: &str, args: &[LuaType]) -> Result<LuaType, ()> {
        unsafe {
            let name = CString::new(n).unwrap();
            lua_getglobal(self.handle as _, name.as_ptr() as _);
            self.call(args, n)
        }
    }

    pub fn get_field(&self, name: &str) -> Option<LuaType> {
        let name_c = CString::new(name).unwrap();
        unsafe {
            lua_getfield(self.handle as _, -1, name_c.as_ptr() as _);
            let ret = Some(self.get_top_value(0));
            self.pop();
            ret
        }
    }

    pub fn get_global(&self, name: &str) -> Option<LuaType> {
        let name_c = CString::new(name).unwrap();
        unsafe {
            lua_getglobal(self.handle as _, name_c.as_ptr() as _);
            let ret = Some(self.get_top_value(0));
            self.pop();
            ret
        }
    }

    pub fn get_stack_size(&self) -> u32 {
        let ret = unsafe {
            lua_gettop(self.handle as _)
        };
        ret as u32
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
                    lua_pushnumber(self.handle as _, n.0);
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

    fn top_as_string(&self) -> LuaType {
        let ret = unsafe {
            let chars = lua_tostring(self.handle as _, -1);
            let c_str = CStr::from_ptr(chars as _);
            let ret = c_str.to_str().unwrap().to_string();
            ret
        };
        //println!("Got string. {}", ret);
        LuaType::String(ret)
    }

    fn top_as_bool(&self) -> LuaType {
        let ret = unsafe {
            lua_toboolean(self.handle as _, -1)
        };
        //println!("Got bool: {}", ret);
        LuaType::Bool(if ret == 0 { false } else { true })
    }

    fn top_as_number(&self) -> LuaType {
        let number = unsafe {
            let ret = lua_tonumberx(self.handle as _, -1, null_mut());
            ret
        };
        //println!("Got number {}", number);
        LuaType::Number(OrderedFloat(number))
    }

    fn top_as_table(&self, stack_size: i32) -> LuaType {
        let mut keys = Vec::new();
        let mut values = Vec::new();
        let mut is_array = true;
        unsafe {
            lua_pushnil(self.handle as _);
            while lua_next(self.handle as _, -2) != 0 {
                let val = self.get_top_value(stack_size + 1);
                self.pop();
                let key = self.get_top_value(stack_size + 1);
                match &key {
                    &LuaType::Number(_) => (),
                    _ => is_array = false,
                }
                keys.push(key);
                values.push(val);
            }
        }

        if is_array {
            LuaType::Array(values)
        } else {
            let mut table = Vec::new();
            for i in 0..keys.len() {
                table.push((keys[i].clone(), values[i].clone()));
            }
            LuaType::Table(table)
        }
    }

    pub fn get_top_value(&self, stack_size: i32) -> LuaType {
        let t = unsafe {
            lua_type(self.handle as _, -1)
        };

        match t {
            LUA_TNIL => { LuaType::Null },
            LUA_TNUMBER => { self.top_as_number() },
            LUA_TBOOLEAN => { self.top_as_bool() },
            LUA_TSTRING => { self.top_as_string() },
            LUA_TTABLE => { self.top_as_table(stack_size) },
            LUA_TFUNCTION => { LuaType::Function },
            LUA_TUSERDATA => { LuaType::UserData },
            LUA_TTHREAD => { LuaType::Thread },
            LUA_TLIGHTUSERDATA => { LuaType::UserData },
            _ => { panic!("Unrecognized type"); }
        }
    }

    fn pop(&self) {
        unsafe { lua_pop(self.handle as _, 1); }
    }

}
