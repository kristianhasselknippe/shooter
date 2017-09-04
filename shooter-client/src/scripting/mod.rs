use libc::{c_void,c_int,c_char};
use std::ptr;
use std::mem::{uninitialized};
use std::path::Path;
use std::ffi::CString;

struct vm { }

#[derive(Copy,Clone)] #[repr(C)]
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

#[derive(Copy,Clone)] #[repr(C)]
struct _value {
    t: val_type,
}

type field = c_int;

#[link(name = "neko")]
extern {
    fn neko_global_init();
    fn neko_vm_alloc(unused: *mut c_void) -> c_void;
    fn neko_vm_select(vm: *mut c_void) -> c_void;

    fn neko_default_loader(argv: *mut *mut c_char, argc: c_int) -> value;

    fn neko_val_field(o: value, f: field) -> value;
    fn neko_val_callEx(vthis: value, f: value, args: *mut value, nargs: c_int, exc: *mut value) -> value;
    fn neko_val_id(str: *const c_char) -> field;
    fn neko_alloc_string( str: *const c_char ) -> value;
}

pub fn load(path: &Path) {

}

pub fn init_neko() {
    unsafe {
        neko_global_init();

        let mut vm = neko_vm_alloc(ptr::null_mut());

        neko_vm_select(&mut vm);

        let mut exc: value = ptr::null_mut();

        let mut loader = neko_default_loader(ptr::null_mut(), 0);

        let mut val_id = neko_val_id(CString::new("loadmodule").unwrap().as_ptr());
        println!("1");
        let mut val_field = neko_val_field(loader, val_id);
        println!("2");

        let module_string: *const c_char = CString::new("mymodule.neko").unwrap().as_ptr();
        let neko_string: value = neko_alloc_string(module_string);
        let mut args = [neko_string, loader];
        println!("3");
        let ret = neko_val_callEx(loader, val_field, args.as_mut_ptr(), 2, (&mut exc) as *mut value);
println!("4");


        println!("Neko inited and selected");
    }
}
