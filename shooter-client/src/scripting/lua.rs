pub extern crate lua52_sys;

use std::io::Read;
use std::path::Path;
use std::fs::File;
use self::lua52_sys::*;
use std::ptr::{null,null_mut};
use libc::{c_void,size_t,c_char,c_int,free,realloc};
use std::ffi::{CString,CStr};
use of::OrderedFloat;

use entities::*;

#[macro_export]
macro_rules! cstringptr {
    ($name:expr) => {
        CString::new($name).unwrap().as_ptr() as _
    }
}

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
    LightUserdata(*mut c_void),
    Null
}

extern {
    //void luaL_traceback (lua_State *L, lua_State *L1, const char *msg, int level);
    pub fn luaL_traceback(L: *mut lua_State, L1: *mut lua_State, msg: *const c_char, level: c_int);


    //lua_Number luaL_checknumber (lua_State *L, int arg);
    pub fn luaL_checknumber(L: *mut lua_State, arg: c_int) -> lua_Number;

    //const char *luaL_checklstring (lua_State *L, int arg, size_t *l);
    pub fn luaL_checklstring(L: *mut lua_State, arg: c_int, l: *mut size_t) -> *const char;
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

    pub fn unwrap_array<'a>(&'a self) -> Option<&'a Vec<LuaType>> {
        match self {
            &LuaType::Array(ref arr) => Some(arr),
            _ => None,
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

#[derive(Debug)]
pub struct Lua {
    handle: *mut lua_State,
}

impl Drop for Lua {
    fn drop(&mut self) {
    }
}

#[derive(Debug)]
pub struct NativeLibrary {
    pub name: String,
    pub functions: Vec<(String,lua_CFunction)>,
}

pub trait NativeLibraryProvider {
    fn get_native_library() -> NativeLibrary;
}

pub fn push_value(L: *mut lua_State, val: &LuaType) {
    unsafe {
        match val {
            &LuaType::String(ref s) => {
                lua_pushstring(L, cstringptr!(s.clone()));
            },
            &LuaType::Number(n) => {
                lua_pushnumber(L, n.0);
            },
            &LuaType::Bool(b) => {
                lua_pushboolean(L, if b { 1 } else { 0 });
            },
            &LuaType::LightUserdata(p) => {
                lua_pushlightuserdata(L, p);
            },
            _ => {
                panic!("Unsupported argument type");
            }
        }
    }
}

pub struct LuaTableBuilder {
    lua: *mut lua_State
}

impl LuaTableBuilder {
    pub fn with_value(&mut self, field: &str, v: &LuaType) -> &mut LuaTableBuilder {
        unsafe {
            push_value(self.lua, v);
            lua_setfield(self.lua, -2, cstringptr!(field));
        };
        self
    }
}

pub fn push_new_table(L: *mut lua_State) -> LuaTableBuilder {
    unsafe { lua_newtable(L); };
    LuaTableBuilder { lua: L }
}

pub fn new_native_library(L: *mut lua_State, native_library: &NativeLibrary) {
    unsafe {
        println!("Creating native library for {:?}", native_library.functions);
        lua_newtable(L);
        for &(ref n,ref f) in &native_library.functions {
            //void lua_pushcfunction (lua_State *L, lua_CFunction f);
            lua_pushcfunction(L, *f);
            lua_setfield(L, -2, cstringptr!(n.clone()));
        }
        //luaL_setfuncs(L, native_library.functions.as_ptr() as _, 0);
        lua_setglobal(L, cstringptr!(native_library.name.clone()));
    }
}

pub fn print_stack_dump(L: *mut lua_State) {
    unsafe {
        let top = lua_gettop(L);
        println!("StackSize: {}", top);
        for i in 1 .. top + 1 {
            let t = lua_type(L, i);
            match t {
                LUA_TSTRING => {
                    let lua_str = lua_tostring(L, i);
                    let c_str = CStr::from_ptr(lua_str);
                    /* strings */println!("\t{} => {}", i, c_str.to_str().unwrap());
                },
                LUA_TBOOLEAN => {
                    /* booleans */
                    println!("\t{} => {}", i, if lua_toboolean(L, i) == 1  { "true" } else { "false" });
                },
                LUA_TNUMBER => {
                    /* numbers */
                    println!("\t{} => {}", i, lua_tonumberx(L, i, null_mut()));
                },
                _ => {
                    let lua_str = lua_typename(L, t);
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

pub fn pop(L: *mut lua_State) {
    unsafe { lua_pop(L, 1); }
}

pub fn set_global(L: *mut lua_State, name: &str, value: &LuaType) {
    let path: Vec<&str> = name.split(".").collect();
    println!("Path len: {}", path.len());
    unsafe {
        if path.len() > 1 {
            lua_getglobal(L, cstringptr!(path[0]));
        }
        for i in 1..(path.len() - 1) {
            lua_getfield(L, -1, cstringptr!(path[i]));
        }

        push_value(L, value);
        if path.len() == 1 {
            lua_setglobal(L, cstringptr!(name));
        } else {
            lua_setfield(L, -2, cstringptr!(path[path.len() - 1]));
            pop(L);
        }
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
                handle: state,
            }
        }

    }

    pub fn new_native_library(&self, userdata: &NativeLibrary) {
        new_native_library(self.handle as _, userdata)
    }

    pub fn set_global(&self, name: &str, value: &LuaType) {
        println!("Trying to set global");
        set_global(self.handle as _, name, value);
        println!("Done setting global");
    }
    
    pub fn print_stack_dump(&self) {
        print_stack_dump(self.handle as _);
    }

    fn call(&self, args: &[LuaType], function_name_for_debugging: &str) -> Result<LuaType, ()> {
        unsafe {

            lua_getglobal(self.handle as _, cstringptr!("main_error_handler"));
            lua_insert(self.handle as _, -2);
            for a in args {
                self.push_value(&a);
            }

            let msgh = -(args.len() as i32 + 2);
            let error_status = lua_pcall(self.handle as _, args.len() as i32, 1, msgh);
            match error_status {
                LUA_ERRRUN => {
                    let error_message = lua_tostring(self.handle as _, -1) as *mut i8;
                    let err_msg_c_str = CString::from_raw(error_message);
                    let error_message_s = err_msg_c_str.to_str().unwrap();
                    panic!("Runtime error calling function {}: {}", function_name_for_debugging, error_message_s);
                },
                LUA_OK => { },
                LUA_ERRMEM => { panic!("Memory error"); },
                LUA_ERRERR => { panic!("Error running the error handler"); }
                LUA_ERRGCMM => { panic!("Error running GC"); },
                _ => { panic!("Unknown error calling function: {}", function_name_for_debugging); }
            }
            let result = self.get_top_value(0);
            self.pop();
            self.pop();
            Ok(result)
        }
    }

    pub fn call_global(&self, n: &str, args: &[LuaType]) -> Result<LuaType, ()> {
        let path: Vec<&str> = n.split(".").collect();

        unsafe { lua_getglobal(self.handle as _, cstringptr!(path[0])); }
        for i in 1..path.len() {
            self.push_field(path[i]);
        }
        
        let ret = unsafe { self.call(args, n) };

        for i in 0..path.len()-1 {
            self.pop();
        }

        ret
    }

    pub fn push_field(&self, name: &str) {
        unsafe {
            let ret = lua_getfield(self.handle as _, -1, cstringptr!(name));
        }
    }

    pub fn get_field(&self, name: &str) -> Option<LuaType> {
        unsafe {
            let ret = lua_getfield(self.handle as _, -1, cstringptr!(name));
            let ret = Some(self.get_top_value(0));
            self.pop();
            ret
        }
    }

    pub fn push_global(&self, name: &str) {
        unsafe {
            lua_getglobal(self.handle as _, cstringptr!(name));
        }
    }

    pub fn get_global(&self, name: &str) -> Option<LuaType> {
        unsafe {
            lua_getglobal(self.handle as _, cstringptr!(name));
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
        push_value(self.handle as _, val);
    }

    fn top_as_string(&self) -> LuaType {
        let ret = unsafe {
            let chars = lua_tostring(self.handle as _, -1);
            let c_str = CStr::from_ptr(chars as _);
            if let Ok(ret)  = c_str.to_str() {
                return LuaType::String(ret.to_string())
            }
            return LuaType::String("not UTF8".to_string())
        };
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
        pop(self.handle as _);
    }


    pub fn load_as_module(&self, path: &Path) {
        let mut file = File::open(path).unwrap();
        let filename = path.file_stem().unwrap().to_str().unwrap().to_string();

        unsafe {
            lua_getglobal(self.handle as _, cstringptr!("package".to_string()));
            lua_getfield(self.handle as _, -1, cstringptr!("loaded".to_string()));
        }

        self.execute_from_reader(&mut file, &filename);

        unsafe {
            lua_setfield(self.handle as _, -2, cstringptr!(filename));
        }
        
        self.pop();
        self.pop();
    }

    pub fn load_as_script(&self, path: &Path, id: &str) {
        let mut file = File::open(path).unwrap();
        let file_name = path.file_stem().unwrap().to_str().unwrap();
         
        println!("Loading user script: {}", id);

        unsafe { lua_getglobal(self.handle as _, cstringptr!("__entity_scripts".to_string())); }
        self.print_stack_dump();
        self.execute_from_reader(&mut file, &format!("{}{}", id, file_name));
        self.print_stack_dump();
        unsafe { lua_setfield(self.handle as _, -2, cstringptr!(id)); }
        self.print_stack_dump();
        self.pop();
        println!("Done loading scritp");
        self.print_stack_dump();
    }

    fn execute_from_reader(&self, reader: &mut Read, module_name: &str) {
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
            let cn = format!("{}\0", module_name);
            let chunk_name = cn.as_bytes();
            let result = lua_load(self.handle, read, &mut buffer as *mut _ as *mut c_void, chunk_name.as_ptr() as *const _, null());
            if result == LUA_OK {
                if module_name != "debug" {
                    lua_getglobal(self.handle as _, cstringptr!("main_error_handler"));
                    lua_insert(self.handle as _, -2);
                }

                //self.print_stack_dump();
                
                lua_pcall(self.handle as _, 0, 1, -2); //should return a module

                if module_name != "debug" {
                    lua_remove(self.handle as _, -2); // remove the debug function
                }

            } else {
                panic!("Error loading script");
            }
        }
    }

}
