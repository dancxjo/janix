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
pub extern "C" fn _start() -> ! {
    main();
    loop {}
}

#[cfg(target_os = "none")]
fn main() {
    // We don't need UserlandSys for basic logging if we use a different way?
    // Wait, existing driver used `println(sys, ...)`
    // We can use `println!`? Does runtime support it?
    // Let's assume we can use `runtime::println!` or similar if we import it?
    // Or we create a dummy UserlandSys to access `println`.
    
    // Actually, ps2_keyboard_driver crate logic is in lib.rs?
    // The previous main.rs called `ps2_keyboard_driver::run(&mut sys)`.
    // I should rewrite `run` in `lib.rs`? Or just put everything in `main.rs` for simplicity as user requested "Loop: dev_read".
    
    // I'll rewrite `main.rs` to contain the logic directly.
    
    // We need logging. `thing_os::println`?
    // Let's try to use `runtime` facilities if available.
    
    let mut sys = UserlandSys::new();
    
    // Register schema for KeyScanEvent if we want to emit events?
    // The user requirement says: "Emit higher-level events (graph Things, logs, etc.)"
    // "Driver logs scancodes when keys are pressed"
    // So distinct from just printing.
    
    // But Step 3 says: "Replace with dev_open... Loop: dev_read... Decode and act"
    // "Acceptance: Driver logs scancodes"
    
    // I need `keyboard_decoder`. I can reuse the one in `lib.rs` if I modify `lib.rs`, or copy it.
    // Modifying `lib.rs` is cleaner.
    
    ps2_keyboard_driver::run_new_ABI(&mut sys);
}

#[cfg(not(target_os = "none"))]
fn main() {}

#[cfg(target_os = "none")]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
