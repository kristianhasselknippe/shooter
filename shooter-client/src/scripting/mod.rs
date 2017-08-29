use libc::{c_void};
use std::ptr;

#[link(name = "neko")]
extern {
    fn neko_global_init();
    fn neko_vm_alloc(unused: *const c_void);
}

pub fn init_neko() {
    unsafe {
        neko_global_init();

        let vm = neko_vm_alloc(ptr::null());
    }
}
