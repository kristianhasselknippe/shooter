use libc::{c_void,c_int,c_char, int32_t, c_schar, uint32_t};
use std::ptr;
use std::mem::{uninitialized};
use std::path::Path;
use std::ffi::{CString,CStr};

use std::env;


enum vm { }

type neko_vm = *mut c_void;

enum val_type {
    VAL_INT			= 0xFF,
	VAL_NULL		= 0,
	VAL_FLOAT		= 1,
	VAL_BOOL		= 2,
	VAL_STRING		= 3,
	VAL_OBJECT		= 4,
	VAL_ARRAY		= 5,
	VAL_FUNCTION	= 6,
	VAL_ABSTRACT	= 7,
	VAL_INT32		= 8,
	VAL_PRIMITIVE	= 6 | 16,
	VAL_JITFUN		= 6 | 32,
	VAL_32_BITS		= 0xFFFFFFFF,
}

type value = *mut _value;

type buffer = *mut c_void;

struct _value {
    t: val_type
}
type field = int32_t;


struct vstring {
    t: val_type,
    c: c_char,
}

#[link(name = "neko")]
extern "C" {
    fn neko_global_init();
    fn neko_vm_alloc(unused: *mut c_void) -> neko_vm;
    fn neko_vm_select(vm: *mut c_void);

    fn neko_default_loader(argv: *mut *mut c_char, argc: c_int) -> value;
    fn neko_alloc_string( str: *const c_char ) -> value;
    fn neko_val_callEx(vthis: value, f: value, args: *mut value, nargs: c_int, exc: *mut value) -> value;

    fn neko_alloc_buffer(init: *const c_char) -> buffer;

    fn neko_val_buffer(b: buffer, v: value);
    fn neko_val_field(o: value, f: field) -> value;
    fn neko_val_id(string: *const c_char) -> field;

    fn neko_val_call0(f: value) -> value;
    fn neko_val_call1(f: value, arg: value) -> value;
    fn neko_val_call2(f: value, arg1: value, arg2: value) -> value;
    fn neko_val_call3(f: value, arg1: value, arg2: value, arg3: value) -> value;
    fn neko_val_callN(f: value, args: *const value, nargs: c_int) -> value;

    fn neko_buffer_to_string(b: buffer) -> *mut vstring;
}

pub struct NekoVM {
    vm_handle: neko_vm
}

pub struct NekoModule {
    module_handle: value,
}

unsafe fn load_module(path: &str) -> NekoModule {
    println!("module Path: {}", path);

    let loader = neko_default_loader(ptr::null_mut(), 0);

    let mut args = [neko_alloc_string(CString::new(path).unwrap().as_ptr() as _),
                    loader];

    let id = CString::new("loadmodule").unwrap();
    let mut exc: value = ptr::null_mut();

    let module = neko_val_callEx(loader, neko_val_field(loader, neko_val_id(id.as_ptr() as _)), args.as_ptr() as _, 2, &mut exc);

    if !exc.is_null() {
        let b = neko_alloc_buffer(ptr::null_mut());
        neko_val_buffer(b.clone(), exc);
        let neko_exc_string = neko_buffer_to_string(b);
        println!("Uncaught exception - {:?}", CStr::from_ptr(&(*neko_exc_string).c));
    }

    println!("Module loaded");
    println!("Trying to call fun");
    let hello = neko_val_field(module, neko_val_id(CString::new("hello").unwrap().as_ptr() as _));
    neko_val_call0(hello);

    println!("Done call fun");

    NekoModule {
        module_handle: module
    }
}

pub fn init_new_neko_vm() -> NekoVM {

    unsafe {

        neko_global_init();
        let mut vm = neko_vm_alloc(ptr::null_mut());
        neko_vm_select(vm as _);

        NekoVM {
            vm_handle: vm,
        }
    }
}

impl NekoVM {
    pub fn load_module(&self, path: &str) -> NekoModule {
        unsafe {
            load_module(path)
        }
    }
}

pub enum ArgumentValue {
    Number(i32),
    String(String),
    Bool(bool),
}

impl ArgumentValue {
    pub fn as_neko_value(&self) -> value {
        match self {
            &ArgumentValue::Number(n) => {
                panic!("Number arg not implemented");
            },
            &ArgumentValue::String(ref s) => {
                panic!("String arg not implemented");
            },
            &ArgumentValue::Bool(b) => {
                match b {
                    true => 1 as _,
                    false => 0 as _,
                }
            },
        }
    }
}

impl NekoModule {
    pub fn call_function(&self, name: &str, args: &[ArgumentValue]) {
        unsafe {
            let function = neko_val_field(self.module_handle, neko_val_id(CString::new(name).unwrap().as_ptr() as _));

            let hello = neko_val_field(self.module_handle, neko_val_id(CString::new("hello").unwrap().as_ptr() as _));
            neko_val_call0(hello);

            println!("Calling function: {}", name);
            println!("Func: {:?}", function);

            let len = args.len();

            let args: Vec<value> = args.iter().map(|i|i.as_neko_value()).collect();
            println!("ArgLen: {}", len);
            match len {
                0 => { println!("0000000"); neko_val_call0(function); },
                1 => { neko_val_call1(function, args[0]); },
                2 => { neko_val_call2(function, args[0], args[1]); },
                3 => { neko_val_call3(function, args[0], args[1], args[2]); },
                _ => { neko_val_callN(function, args.as_ptr(), args.len() as _); },
            };
            println!("Done calling function");
        }


        /*let x = neko_val_field(module, neko_val_id(CString::new("x").unwrap().as_ptr() as _));
        println!("foobar2");
        let f = neko_val_field(module, neko_val_id(CString::new("f").unwrap().as_ptr() as _));
        println!("foobar3");
        let ret = neko_val_call1(f, x);
        }*/
    }
}

/*pub struct ScriptEngine {
    module: NekoModule
}

impl ScriptEngine {

}*/
