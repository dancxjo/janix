#![cfg_attr(target_os = "none", no_std)]
#![cfg_attr(target_os = "none", no_main)]

extern crate alloc;

#[cfg(target_os = "none")]
use core::panic::PanicInfo;

#[cfg(target_os = "none")]
use runtime::UserlandSys;

use abi::syscall_defs::{
    DevOpenArgs, DevOpenRet, DevReadArgs, DevReadRet, DeviceHandle, DeviceKind, SysRet,
    SysError, UserPtr, UserSlice,
};
use abi::syscall_numbers::{SYS_DEV_OPEN, SYS_DEV_READ};

// We need a helper for syscalls. Runtime likely has one, but we need
// to match the raw syscall interface we just built.
// Since runtime might not have these specific wrappers yet, we can
// either update runtime or inline the asm here.
// Given we are "Userland Drivers", we likely depend on `thing_os` or `runtime`.
// The driver depends on `runtime`? checked Cargo.toml for driver?
// `user/ps2_keyboard_driver/src/main.rs` depends on `runtime` (implied by `UserlandSys`).

// Let's implement the helpers here for now to avoid modifying runtime just yet.
unsafe fn syscall_dev_open(kind: u32, index: u32) -> Result<DeviceHandle, SysError> {
    let args = DevOpenArgs { kind, index };
    let mut ret = SysRet::<DevOpenRet> {
        ok: 0,
        val: DevOpenRet::default(),
        err: SysError { code: 0, detail: 0 },
    };
    
    // Inline assembly for syscall
    // rax = num, rdi = arg1, rsi = arg2
    // We pass ptr to args in rdi, ptr to ret in rsi
    unsafe {
        core::arch::asm!(
            "syscall",
            in("rax") SYS_DEV_OPEN,
            in("rdi") &args,
            in("rsi") &ret,
            lateout("rcx") _,
            lateout("r11") _,
        );
    }

    if ret.ok != 0 {
        Ok(ret.val.handle)
    } else {
        Err(ret.err)
    }
}

unsafe fn syscall_dev_read(handle: DeviceHandle, out: &mut [u8]) -> Result<usize, SysError> {
    let args = DevReadArgs {
        handle,
        out: UserSlice {
            ptr: UserPtr {
                addr: out.as_mut_ptr() as u64,
                _phantom: core::marker::PhantomData,
            },
            len: out.len() as u64,
        },
    };
    let mut ret = SysRet::<DevReadRet> {
        ok: 0,
        val: DevReadRet::default(),
        err: SysError { code: 0, detail: 0 },
    };

    unsafe {
        core::arch::asm!(
            "syscall",
            in("rax") SYS_DEV_READ,
            in("rdi") &args,
            in("rsi") &ret,
            lateout("rcx") _,
            lateout("r11") _,
        );
    }

    if ret.ok != 0 {
        Ok(ret.val.bytes_read as usize)
    } else {
        Err(ret.err)
    }
}

#[cfg(target_os = "none")]
#[unsafe(no_mangle)]
fn main() {
    let mut sys = UserlandSys::new();
    ps2_keyboard_driver::main(&mut sys);
}

#[cfg(not(target_os = "none"))]
fn main() {}

#[cfg(target_os = "none")]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
