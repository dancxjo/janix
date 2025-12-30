use abi::driver::{DriverBootInfo, DriverContext, DriverDescriptor, DriverProvides};
use abi::SymbolId;
use crate::Kernel;
use crate::bridge::HardwareBridge;

// Static storage for the active driver being initialized
// This is necessary because the callbacks are extern "C" and cannot capture environment
static mut CURRENT_DRIVER_NAME: Option<SymbolId> = None;
static mut CURRENT_RPC_FN: Option<unsafe extern "C" fn(u32, *const u8, u32, *mut u8, u32) -> i32> = None;

// Strategy: Use a trait object for the "Kernel Operations" needed by the driver context.
trait DriverOps {
    fn register(&mut self, iface: SymbolId, ver: u16, instance: SymbolId, rpc_fn: unsafe extern "C" fn(u32, *const u8, u32, *mut u8, u32) -> i32);
    fn log(&self, msg: &str);
}

impl<B: HardwareBridge> DriverOps for Kernel<B> {
    fn register(&mut self, iface: SymbolId, ver: u16, instance: SymbolId, rpc_fn: unsafe extern "C" fn(u32, *const u8, u32, *mut u8, u32) -> i32) {
        if let Some(driver_name) = unsafe { CURRENT_DRIVER_NAME } {
             self.machine.register_module(driver_name, iface, ver, instance, rpc_fn);
        }
    }

    fn log(&self, msg: &str) {
        self.bridge.log(msg);
    }
}

static mut ACTIVE_OPS: Option<*mut dyn DriverOps> = None;

unsafe extern "C" fn driver_register(iface: SymbolId, ver: u16, instance: SymbolId) -> i32 {
    let rpc_fn = match CURRENT_RPC_FN {
        Some(f) => f,
        None => return -1,
    };

    if let Some(ops) = ACTIVE_OPS {
        (*ops).register(iface, ver, instance, rpc_fn);
        0
    } else {
        -1
    }
}

unsafe extern "C" fn driver_log(ptr: *const u8, len: usize) {
    let s = core::slice::from_raw_parts(ptr, len);
    if let Ok(msg) = core::str::from_utf8(s) {
        if let Some(ops) = ACTIVE_OPS {
            (*ops).log(msg);
        }
    }
}

unsafe extern "C" fn driver_get_boot_info(ptr: *mut DriverBootInfo) -> i32 {
     if let Some(info) = &ACTIVE_BOOT_INFO {
        *ptr = *info;
        0
    } else {
        -1
    }
}

static mut ACTIVE_BOOT_INFO: Option<DriverBootInfo> = None;

pub unsafe fn load_driver<B: HardwareBridge>(
    k: &mut Kernel<B>,
    driver_name: SymbolId,
    rpc_fn: unsafe extern "C" fn(u32, *const u8, u32, *mut u8, u32) -> i32,
    init_fn: unsafe extern "C" fn(*mut DriverContext) -> i32,
    fb_info: Option<(u64, u32, u32, u32, u32, u64)>, // addr, width, height, stride, format, size
) -> i32 {

    // 1. Setup global context
    CURRENT_DRIVER_NAME = Some(driver_name);
    CURRENT_RPC_FN = Some(rpc_fn);

    // Create the ops trait object
    // We need to cast `k` to `*mut dyn DriverOps`.
    // Since k is mutable borrow, it is valid as long as we don't return.
    // The static ACTIVE_OPS lives forever, but we only set it during this call.
    // However, coercing to trait object pointer might require lifetime bounds?
    // Let's force it via raw pointer casting if needed, or better, use transmute to erase lifetime?
    // The error was "type B may not live long enough" implies `dyn DriverOps` implies `dyn DriverOps + 'static`.
    // We can use `dyn DriverOps + 'a` but we are storing in `static mut`.
    // We can transmute the pointer to `*mut dyn DriverOps` (static lifetime implied) because we promise to clear it before return.

    let ops_ptr: *mut (dyn DriverOps + '_) = k;
    let ops_static: *mut (dyn DriverOps + 'static) = core::mem::transmute(ops_ptr);

    ACTIVE_OPS = Some(ops_static);

    if let Some((addr, w, h, stride, fmt, size)) = fb_info {
        ACTIVE_BOOT_INFO = Some(DriverBootInfo {
            fb_addr: addr,
            fb_width: w,
            fb_height: h,
            fb_stride: stride,
            fb_format: fmt,
            fb_size: size,
        });
    } else {
        ACTIVE_BOOT_INFO = None;
    }

    // 2. Prepare Context
    let mut ctx = DriverContext {
        register: driver_register,
        log: driver_log,
        get_boot_info: driver_get_boot_info,
    };

    // 3. Call Init
    let res = init_fn(&mut ctx);

    // 4. Cleanup
    ACTIVE_OPS = None;
    CURRENT_DRIVER_NAME = None;
    CURRENT_RPC_FN = None;
    ACTIVE_BOOT_INFO = None;

    res
}
