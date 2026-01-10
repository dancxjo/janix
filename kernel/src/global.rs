use crate::boot::{BootRuntime, BootModuleDesc};

use crate::root::Root;

static mut RUNTIME: Option<&'static dyn BootRuntime> = None;
static mut MODULES: &'static [BootModuleDesc] = &[];
static ROOT: Root = Root::new();

pub fn runtime() -> &'static dyn BootRuntime {
    unsafe { RUNTIME.expect("Kernel runtime not initialized") }
}

pub fn boot_modules() -> &'static [BootModuleDesc] {
    unsafe { MODULES }
}

pub fn root() -> &'static Root {
    &ROOT
}

pub unsafe fn set_runtime(rt: &'static dyn BootRuntime) {
    unsafe {
        RUNTIME = Some(rt);
    }
}

pub unsafe fn set_boot_modules(modules: &'static [BootModuleDesc]) {
    unsafe {
        MODULES = modules;
    }
}
