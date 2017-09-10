use libc::{c_void,c_int,c_char, int32_t, c_schar, uint32_t};
use std::ptr;
use std::mem::{uninitialized};
use std::path::Path;
use std::ffi::{CString,CStr};

use std::env;

use std::fmt;

type neko_vm = c_void;

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

impl fmt::Display for val_type {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            VAL_INT => { write!(fmt, "INT") },
	        VAL_NULL => { write!(fmt, "NULL") },
	        VAL_FLOAT => { write!(fmt, "FLOAT") },
	        VAL_BOOL => { write!(fmt, "BOOL") },
	        VAL_STRING => { write!(fmt, "STRING") },
	        VAL_OBJECT => { write!(fmt, "OBJECT") },
	        VAL_ARRAY => { write!(fmt, "ARRAY") },
	        VAL_FUNCTION => { write!(fmt, "FUNCTION") },
	        VAL_ABSTRACT => { write!(fmt, "ABSTRACT") },
	        VAL_INT32 => { write!(fmt, "INT32") },
	        VAL_PRIMITIVE => { write!(fmt, "PRIMITIVE") },
	        VAL_JITFUN => { write!(fmt, "JITFUN") },
	        VAL_32_BITS => { write!(fmt, "32_BITS") },
        }
    }
}

type value = *mut c_void;
type buffer = *mut c_void;

type field = int32_t;

struct _value {
    t: val_type,
}

struct vstring {
    t: val_type,
    c: c_char,
}

#[link(name = "neko")]
extern "C" {
    static val_true: value;
    static val_false: value;


    fn neko_global_init();
    fn neko_vm_alloc(unused: *mut c_void) -> *mut neko_vm;
    fn neko_vm_select(vm: *mut neko_vm);

    fn neko_default_loader(argv: *mut *mut c_char, argc: c_int) -> value;
    fn neko_alloc_string( str: *const c_char ) -> value;
    fn neko_val_callEx(vthis: value, f: value, args: *mut value, nargs: c_int, exc: *mut value) -> value;

    fn neko_alloc_buffer(init: *const c_char) -> buffer;

    fn neko_val_buffer(b: buffer, v: value);
    fn neko_val_field(o: value, f: field) -> value;
    fn neko_val_id(string: *const c_char) -> field;
    fn neko_val_field_name(f: field) -> value;

    fn neko_val_call0(f: value) -> value;
    fn neko_val_call1(f: value, arg: value) -> value;
    fn neko_val_call2(f: value, arg1: value, arg2: value) -> value;
    fn neko_val_call3(f: value, arg1: value, arg2: value, arg3: value) -> value;
    fn neko_val_callN(f: value, args: *const value, nargs: c_int) -> value;

    fn neko_buffer_to_string(b: buffer) -> *mut vstring;

    fn neko_call_stack(vm: *mut neko_vm) -> value;

    //EXTERN void val_iter_fields( value obj, void f( value v, field f, void * ), void *p );
    fn neko_val_iter_fields(obj: value, f: extern fn(value, field, *mut c_void), p: *mut c_void);

    fn neko_val_this() -> value;
    fn neko_val_print(s: value);

    fn neko_vm_dump_stack(vm: *mut neko_vm);
}

#[no_mangle]
extern "C" fn iter_fields_callback(v: value, f: field, _: *mut c_void) {
    println!("Callback::::");
    unsafe {
        let vv = v as *mut _value;
        let field_name = neko_val_field_name(f);
        neko_val_print(field_name);
        println!("::::::::::Value: {}", (*vv).t);


    }
}

pub struct NekoVM {
    vm_handle: *mut neko_vm
}

pub struct NekoModule {
    module_handle: value,
}

unsafe fn load_module(path: &str) -> NekoModule {
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

    //let this = neko_val_this() as *mut _value;
    neko_val_print(neko_val_this());
    /*println!("THIS: {}", (*this).t);
    neko_val_iter_fields(neko_val_this(), iter_fields_callback, ptr::null());
    println!("DOne");*/

    let mymodule = neko_val_field_name(neko_val_id(CString::new("MyModule").unwrap().as_ptr()));
    neko_val_print(mymodule);

    //neko_val_iter_fields(module, iter_fields_callback, ptr::null_mut());

    let _classes = neko_val_field(module, neko_val_id(CString::new("__classes").unwrap().as_ptr()));
    println!("==================");
    println!("==================");
    println!("==================");
    println!("==================");
    neko_val_iter_fields(_classes, iter_fields_callback, ptr::null_mut());



    println!("==================");
    println!("==================");
    println!("==================");
    println!("==================");

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
                    true => val_true as _,
                    false => val_false as _,
                }
            },
        }
    }
}

impl NekoModule {
    fn get_field(&self, name: &str) -> value {
        unsafe {
            neko_val_field(self.module_handle, neko_val_id(CString::new(name).unwrap().as_ptr() as _))
        }
    }

    pub fn call_function(&self, vm: &NekoVM, name: &str, args: &[ArgumentValue]) {
        unsafe {
            let function = neko_val_field(self.module_handle, neko_val_id(CString::new(name).unwrap().as_ptr() as _));

            println!("========");
            neko_val_print(neko_call_stack(vm.vm_handle));
            println!("========");

            println!("Calling function: {}", name);

            println!("Function {:?}", function);

            let len = args.len();

            let _val = function as *mut _value;

            match (*_val).t {
                val_type::VAL_FUNCTION => {
                    let args: Vec<value> = args.iter().map(|i|i.as_neko_value()).collect();
                    println!("ArgLen: {}", len);
                    match len {
                        0 => { neko_val_call0(function); },
                        1 => { neko_val_call1(function, args[0]); },
                        2 => { neko_val_call2(function, args[0], args[1]); },
                        3 => { neko_val_call3(function, args[0], args[1], args[2]); },
                        _ => { neko_val_callN(function, args.as_slice().as_ptr(), len as _); },
                    };
                },
                val_type::VAL_NULL => {
                    eprintln!("Value of name {} was NULL", name);
                }
                _ => {
                    eprintln!("Value of name {} was not a function", name);
                }
            }


        }
    }
}

/*pub struct ScriptEngine {
    module: NekoModule
}

impl ScriptEngine {

}*/
