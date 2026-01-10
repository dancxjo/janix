use crate::boot::{BootRuntime, BootModuleDesc};
use crate::arch::{ArchContext, ArchTrapFrame};

static mut RUNTIME: Option<&'static dyn BootRuntime<ArchContext, ArchTrapFrame>> = None;
static mut MODULES: &'static [BootModuleDesc] = &[];

pub fn runtime() -> &'static dyn BootRuntime<ArchContext, ArchTrapFrame> {
    unsafe { RUNTIME.expect("Kernel runtime not initialized") }
}

pub fn boot_modules() -> &'static [BootModuleDesc] {
    unsafe { MODULES }
}

pub unsafe fn set_runtime(rt: &'static dyn BootRuntime<ArchContext, ArchTrapFrame>) {
    RUNTIME = Some(rt);
}

pub unsafe fn set_boot_modules(modules: &'static [BootModuleDesc]) {
    MODULES = modules;
}
