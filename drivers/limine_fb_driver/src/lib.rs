#![no_std]
#![no_main] // Added no_main
#![feature(alloc_error_handler)]

// Zero dependencies
// We implement minimal allocator and panic handler

extern crate alloc;

use core::alloc::{GlobalAlloc, Layout};

// Local definitions to break dependency loop
mod defs;
use defs::*;

mod postcard_min;
use postcard_min::*;

// --- Allocator ---

struct DummyAllocator;

unsafe impl GlobalAlloc for DummyAllocator {
    unsafe fn alloc(&self, _layout: Layout) -> *mut u8 {
        core::ptr::null_mut()
    }
    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        // no-op
    }
}

#[global_allocator]
static ALLOCATOR: DummyAllocator = DummyAllocator;

#[alloc_error_handler]
fn alloc_error_handler(_layout: Layout) -> ! {
    panic!("allocation error")
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}

// --- Driver Data ---

static mut FB_INFO: Option<DriverBootInfo> = None;

// --- Descriptor ---

#[no_mangle]
pub static PROVIDES: [DriverProvides; 1] = [
    DriverProvides {
        iface: sym(IFACE_FRAMEBUFFER),
        ver: 1,
        instance: sym("fb0"),
    },
];

#[link_section = ".thingos.machine_driver"]
#[no_mangle]
pub static DRIVER_DESCRIPTOR: DriverDescriptor = DriverDescriptor {
    name: sym("driver.limine_fb"),
    lane: sym("module_rpc"),
    provides_len: 1,
    provides_ptr: unsafe { &PROVIDES as *const _ }, // Use reference as pointer
    requires_len: 0,
    requires_ptr: 0,
    claims_len: 0,
    claims_ptr: 0,
    abi_version: 1,
};

// --- Entry Points ---

#[no_mangle]
pub unsafe extern "C" fn thingos_driver_init(ctx: *mut DriverContext) -> i32 {
    let ctx = &*ctx;

    // 1. Get Boot Info
    let mut info = DriverBootInfo {
        fb_addr: 0,
        fb_width: 0,
        fb_height: 0,
        fb_stride: 0,
        fb_format: 0,
        fb_size: 0,
    };

    if (ctx.get_boot_info)(&mut info) == 0 {
        FB_INFO = Some(info);
        (ctx.log)(b"limine_fb_driver: Got Boot Info\n".as_ptr(), 28);
    } else {
        (ctx.log)(b"limine_fb_driver: Failed to get Boot Info\n".as_ptr(), 40);
        return -1;
    }

    // 2. Register
    let ret = (ctx.register)(sym(IFACE_FRAMEBUFFER), 1, sym("fb0"));
    if ret < 0 {
        (ctx.log)(b"limine_fb_driver: Register failed\n".as_ptr(), 34);
        return ret;
    }

    (ctx.log)(b"limine_fb_driver: Init OK\n".as_ptr(), 26);
    0
}

#[no_mangle]
pub unsafe extern "C" fn thingos_driver_rpc(
    op: u32,
    _req_ptr: *const u8,
    _req_len: u32,
    out_ptr: *mut u8,
    out_cap: u32,
) -> i32 {
    let info = match FB_INFO {
        Some(i) => i,
        None => return -1,
    };

    // OP_FB_GET_INFO = 1

    match op {
        1 => {
            let resp = FbGetInfoResp {
                width: info.fb_width,
                height: info.fb_height,
                stride: info.fb_stride,
                format: info.fb_format,
                addr: 0x1_0000_0000,
                size: info.fb_size,
            };

            // Serialize
            let out_slice = core::slice::from_raw_parts_mut(out_ptr, out_cap as usize);

            let len = encode_fb_info(&resp, out_slice);
            if len > 0 {
                len as i32
            } else {
                -1 // Buffer too small?
            }
        }
        _ => -1,
    }
}
