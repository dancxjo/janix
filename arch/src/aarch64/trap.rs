use crate::user;
use abi::{
    KernelRequest, KernelResponse, MapFlags, ProcessId, SyscallNumber, THING_GET_MAX_KIND_LEN,
    THING_GET_MAX_PROPS, THING_GET_MAX_STR_LEN, ThingGetSyscallResult, ThingId, ThingPropData,
    ThingPropScalarType,
};
use core::arch::global_asm;
use core::cmp;
extern crate alloc;
use alloc::boxed::Box;
use alloc::string::ToString;

global_asm!(include_str!("trap.S"));

#[unsafe(no_mangle)]
pub extern "C" fn syscall_handler_rust(tf: &mut TrapFrame) -> u64 {
    let esr: u64;
    unsafe { core::arch::asm!("mrs {}, esr_el1", out(reg) esr) };
    let ec = (esr >> 26) & 0x3F;

    if ec != 0x15 {
        let far: u64;
        unsafe { core::arch::asm!("mrs {}, far_el1", out(reg) far) };
        kernel::println!("EXCEPTION: AArch64 Trap (Not SVC)");
        kernel::println!("ESR: {:#x}", esr);
        kernel::println!("FAR: {:#x}", far);
        kernel::println!("{:#?}", tf);
        loop {}
    }

    let num = tf.x8;
    let arg1 = tf.x0;
    let arg2 = tf.x1;
    let arg3 = tf.x2;
    let arg4 = tf.x3;
    let arg5 = tf.x4;
    let arg6 = tf.x5;

    static mut UNKNOWN_SYSCALLS_LOGGED: usize = 0;

    if num == SyscallNumber::Yield as u64 {
        save_current_thread_context(tf);
        kernel::sched::yield_current_thread();
        return user::schedule_next();
    } else if num == SyscallNumber::SleepForNs as u64 {
        save_current_thread_context(tf);
        return user::sys_sleep_for_ns(arg1);
    } else if num == SyscallNumber::Log as u64 {
        let ptr = arg1 as *const u8;
        let len = arg2 as usize;
        if let Ok(s) = unsafe { core::str::from_utf8(user_slice(ptr, len)) } {
            kernel::log(s);
        }
        0
    } else if num == SyscallNumber::ExitThread as u64 {
        kernel::log("Thread exited via syscall");
        kernel::sched::exit_current_thread();
        return user::schedule_next();
    } else if num == SyscallNumber::AllocFrame as u64 {
        let pool_index = arg1;
        let frame_info_ptr = arg2 as *mut abi::FrameInfo;

        let req = KernelRequest::AllocFrame {
            pool_index: pool_index,
        };
        match kernel::handle_request(req) {
            KernelResponse::FrameAllocated { frame } => {
                unsafe { *frame_info_ptr = frame };
                0
            }
            _ => 1,
        }
    } else if num == SyscallNumber::FreeFrame as u64 {
        let frame_id = abi::FrameId(arg1);
        let req = KernelRequest::FreeFrame { frame_id };
        match kernel::handle_request(req) {
            KernelResponse::FrameFreed { .. } => 0,
            _ => 1,
        }
    } else if num == SyscallNumber::CreateProcess as u64 {
        let Some(name) = leak_user_str(arg1, arg2 as usize) else {
            return 0;
        };
        let pid = {
            let mut sched = kernel::sched::SCHEDULER.lock();
            let pid = sched.add_process(name);
            pid.0
        };
        pid
    } else if num == SyscallNumber::CreateThread as u64 {
        let process_id = ProcessId(arg1);
        let app_id = arg2;
        let priority = arg3;
        let Some(name) = leak_user_str(arg4, arg5 as usize) else {
            return 0;
        };
        let stack = user::alloc_user_stack();
        let tid = {
            let mut sched = kernel::sched::SCHEDULER.lock();
            sched.add_thread(
                process_id,
                name,
                user::user_thread_main,
                app_id,
                stack,
                priority,
            )
        };
        tid.0
    } else if num == SyscallNumber::AddEdge as u64 {
        let from = ThingId(arg1);
        let pred = abi::EdgePred(arg2);
        let to = ThingId(arg3);
        let req = KernelRequest::AddEdge { from, pred, to };
        match kernel::handle_request(req) {
            KernelResponse::Success { .. } => 0,
            _ => 1,
        }
    } else if num == SyscallNumber::EdgeAt as u64 {
        let from = ThingId(arg1);
        let index = arg2;
        let pred = abi::EdgePred(arg3);
        match kernel::handle_request(KernelRequest::EdgeAt { from, pred, index }) {
            KernelResponse::EdgeTarget { target } => target.map_or(u64::MAX, |id| id.0),
            _ => u64::MAX,
        }
    } else if num == SyscallNumber::CreateSharedBuffer as u64 {
        let width = arg1 as u32;
        let height = arg2 as u32;
        let pixel_format = match arg3 as u8 {
            1 => abi::PixelFormat::Bgra8888,
            _ => abi::PixelFormat::Rgba8888,
        };
        match kernel::handle_request(KernelRequest::CreateSharedBuffer {
            width,
            height,
            pixel_format,
        }) {
            KernelResponse::SharedBufferCreated { buffer_id } => buffer_id.0,
            _ => 0,
        }
    } else if num == SyscallNumber::MapSharedBuffer as u64 {
        let buffer_id = ThingId(arg1);
        let flags = MapFlags(arg2);
        let vaddr_out = arg3 as *mut u64;
        let size_out = arg4 as *mut u64;
        match kernel::handle_request(KernelRequest::MapSharedBuffer { buffer_id, flags }) {
            KernelResponse::SharedBufferMapped { vaddr, size } => {
                if !vaddr_out.is_null() {
                    unsafe { *vaddr_out = vaddr };
                }
                if !size_out.is_null() {
                    unsafe { *size_out = size };
                }
                0
            }
            _ => 1,
        }
    } else if num == SyscallNumber::GetSharedBufferInfo as u64 {
        let buffer_id = ThingId(arg1);
        let info_out = arg2 as *mut abi::SharedBufferInfo;
        match kernel::handle_request(KernelRequest::GetSharedBufferInfo { buffer_id }) {
            KernelResponse::SharedBufferInfoResponse { info } => {
                if !info_out.is_null() {
                    unsafe { *info_out = info };
                }
                0
            }
            _ => 1,
        }
    } else if num == SyscallNumber::SpawnProgram as u64 {
        let boot_program_id = ThingId(arg1);
        let result_ptr = arg2 as *mut abi::SpawnProgramResult;
        if result_ptr.is_null() {
            return 1;
        }
        match kernel::handle_request(KernelRequest::SpawnProgram { boot_program_id }) {
            KernelResponse::ProgramSpawned {
                process_id,
                thread_id,
            } => {
                unsafe {
                    (*result_ptr).process_id = process_id;
                    (*result_ptr).thread_id = thread_id;
                }
                0
            }
            KernelResponse::Error { message } => {
                kernel::log(message);
                1
            }
            _ => 1,
        }
    } else if num == SyscallNumber::ThingGet as u64 {
        let id = ThingId(arg1);
        let result_ptr = arg2 as *mut ThingGetSyscallResult;
        if result_ptr.is_null() {
            return 1;
        }
        let req = KernelRequest::ThingGet { id };
        match kernel::handle_request(req) {
            KernelResponse::ThingData { kind, props, .. } => {
                unsafe {
                    let result = &mut *result_ptr;
                    *result = ThingGetSyscallResult::default();
                    let kind_bytes = kind.as_bytes();
                    let kind_len = cmp::min(kind_bytes.len(), THING_GET_MAX_KIND_LEN);
                    result.kind[..kind_len].copy_from_slice(&kind_bytes[..kind_len]);
                    result.kind_len = kind_len;

                    let mut count = 0;
                    for entry in props.iter() {
                        if count >= THING_GET_MAX_PROPS {
                            break;
                        }
                        let slot: &mut ThingPropData = &mut result.props[count];
                        if let Some((key, value)) = entry {
                            slot.present = 1;
                            let key_bytes = key.as_bytes();
                            let key_len = cmp::min(key_bytes.len(), THING_GET_MAX_STR_LEN);
                            slot.key[..key_len].copy_from_slice(&key_bytes[..key_len]);
                            slot.key_len = key_len;
                            match value {
                                abi::PropValue::U64(v) => {
                                    slot.value_type = ThingPropScalarType::U64;
                                    slot.value_u64 = *v;
                                }
                                abi::PropValue::I64(v) => {
                                    slot.value_type = ThingPropScalarType::I64;
                                    slot.value_i64 = *v;
                                }
                                abi::PropValue::Bool(v) => {
                                    slot.value_type = ThingPropScalarType::Bool;
                                    slot.value_bool = if *v { 1 } else { 0 };
                                }
                                abi::PropValue::Str(s) => {
                                    slot.value_type = ThingPropScalarType::Str;
                                    let bytes = s.as_bytes();
                                    let str_len = cmp::min(bytes.len(), THING_GET_MAX_STR_LEN);
                                    slot.value_str[..str_len].copy_from_slice(&bytes[..str_len]);
                                    slot.value_str_len = str_len;
                                }
                            }
                            count += 1;
                        } else {
                            slot.present = 0;
                        }
                    }
                    result.prop_count = count;
                }
                0
            }
            KernelResponse::Error { message } => {
                kernel::log(message);
                1
            }
            other => {
                let msg = alloc::format!("ThingGet unexpected response {:?}", other);
                let leaked: &'static str = Box::leak(msg.into_boxed_str());
                kernel::log(leaked);
                1
            }
        }
    } else if num == SyscallNumber::ThingList as u64 {
        let kind_ptr = arg1 as *const u8;
        let kind_len = arg2 as usize;
        let start_after = ThingId(arg3);
        let kind = unsafe { core::str::from_utf8(user_slice(kind_ptr, kind_len)).unwrap_or("") };
        let kind_static: &'static str = Box::leak(kind.to_string().into_boxed_str());
        match kernel::handle_request(KernelRequest::ThingList {
            kind: kind_static,
            start_after,
        }) {
            KernelResponse::ThingListEntry { id } => id.map_or(u64::MAX, |tid| tid.0),
            KernelResponse::Error { .. } => u64::MAX,
            other => {
                let msg = alloc::format!("ThingList unexpected response {:?}", other);
                let leaked: &'static str = Box::leak(msg.into_boxed_str());
                kernel::log(leaked);
                u64::MAX
            }
        }
    } else if num == SyscallNumber::ThingCreate as u64 {
        let kind_ptr = arg1 as *const u8;
        let kind_len = arg2 as usize;
        let props_ptr = arg3 as *const (abi::PropKey, abi::PropValue);
        let props_len = arg4 as usize;

        let kind = unsafe { core::str::from_utf8(user_slice(kind_ptr, kind_len)).unwrap_or("") };
        let props = unsafe { user_slice(props_ptr, props_len) };

        let kind_static: &'static str = Box::leak(kind.to_string().into_boxed_str());
        let props_vec: alloc::vec::Vec<(abi::PropKey, abi::PropValue)> = props.to_vec();
        let props_static: &'static [(abi::PropKey, abi::PropValue)] =
            Box::leak(props_vec.into_boxed_slice());

        let req = KernelRequest::ThingCreate {
            kind: kind_static,
            props: props_static,
        };
        match kernel::handle_request(req) {
            KernelResponse::ThingCreated { id } => id.0,
            _ => 0,
        }
    } else if num == SyscallNumber::ThingUpdate as u64 {
        let id = ThingId(arg1);
        let props_ptr = arg2 as *const (abi::PropKey, abi::PropValue);
        let props_len = arg3 as usize;

        let props = unsafe { user_slice(props_ptr, props_len) };
        let props_vec: alloc::vec::Vec<(abi::PropKey, abi::PropValue)> = props.to_vec();
        let props_static: &'static [(abi::PropKey, abi::PropValue)] =
            Box::leak(props_vec.into_boxed_slice());

        let req = KernelRequest::ThingUpdate {
            id,
            props: props_static,
        };
        match kernel::handle_request(req) {
            KernelResponse::Success { .. } => 0,
            _ => 1,
        }
    } else if num == SyscallNumber::SchemaRegister as u64 {
        let kind_ptr = arg1 as *const u8;
        let kind_len = arg2 as usize;
        let props_ptr = arg3 as *const (&'static str, abi::PropType);
        let props_len = arg4 as usize;

        let kind = unsafe { core::str::from_utf8(user_slice(kind_ptr, kind_len)).unwrap_or("") };
        let props = unsafe { user_slice(props_ptr, props_len) };

        let kind_static: &'static str = Box::leak(kind.to_string().into_boxed_str());
        let props_vec = props.to_vec();
        let props_static = Box::leak(props_vec.into_boxed_slice());

        // Provide an empty description for schema registrations originating
        // from userland syscalls (no description argument is passed over
        // the syscall ABI). Leak to `'static` like `kind` and `props`.
        let description_static: &'static str = Box::leak("".to_string().into_boxed_str());

        let req = KernelRequest::SchemaRegister {
            kind: kind_static,
            description: description_static,
            props: props_static,
        };
        match kernel::handle_request(req) {
            KernelResponse::SchemaRegistered { .. } => 0,
            _ => 1,
        }
    } else if num == SyscallNumber::TimeNow as u64 {
        kernel::time::monotonic_now_ns()
    } else if num == SyscallNumber::TimeMonotonicNs as u64 {
        kernel::time::monotonic_now_ns()
    } else if num == SyscallNumber::TimeSystemNs as u64 {
        if let Some(ns) = kernel::time::system_time_ns() {
            ns
        } else {
            0
        }
    } else if num == SyscallNumber::SleepUntil as u64 {
        let deadline_ns = arg1;
        while kernel::time::monotonic_now_ns() < deadline_ns {
            user::schedule_next();
        }
        0
    } else {
        unsafe {
            if UNKNOWN_SYSCALLS_LOGGED < 5 {
                kernel::log("Unknown syscall");
                UNKNOWN_SYSCALLS_LOGGED += 1;
            }
        }
        1
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn invalid_exception(tf: &TrapFrame, kind: usize, source: usize) {
    let esr: u64;
    let far: u64;
    unsafe {
        core::arch::asm!("mrs {}, esr_el1", out(reg) esr);
        core::arch::asm!("mrs {}, far_el1", out(reg) far);
    }
    kernel::println!("EXCEPTION: AArch64 Trap");
    kernel::println!("Kind: {}, Source: {}", kind, source);
    kernel::println!("ESR: {:#x}, FAR: {:#x}", esr, far);
    kernel::println!("{:#?}", tf);
    loop {}
}

#[repr(C)]
#[derive(Debug)]
pub struct TrapFrame {
    pub x0: u64,
    pub x1: u64,
    pub x2: u64,
    pub x3: u64,
    pub x4: u64,
    pub x5: u64,
    pub x6: u64,
    pub x7: u64,
    pub x8: u64,
    pub x9: u64,
    pub x10: u64,
    pub x11: u64,
    pub x12: u64,
    pub x13: u64,
    pub x14: u64,
    pub x15: u64,
    pub x16: u64,
    pub x17: u64,
    pub x18: u64,
    pub x19: u64,
    pub x20: u64,
    pub x21: u64,
    pub x22: u64,
    pub x23: u64,
    pub x24: u64,
    pub x25: u64,
    pub x26: u64,
    pub x27: u64,
    pub x28: u64,
    pub x29: u64,
    pub x30: u64,
    pub sp_el0: u64,
    pub elr: u64,
    pub spsr: u64,
}

pub fn init() {
    unsafe extern "C" {
        static exception_vector_table: u8;
    }
    unsafe {
        core::arch::asm!(
            "msr vbar_el1, {}",
            in(reg) &exception_vector_table,
        );
    }
}

/// Switches to a dedicated EL1 kernel stack and jumps to the given entry point.
/// This is necessary because we cannot return to the caller after switching stacks
/// (the return address would be on the old stack).
pub unsafe fn jump_to_el1_stack(stack_top: u64, entry: unsafe extern "C" fn() -> !) -> ! {
    // Ensure stack is 16-byte aligned
    let stack_top = stack_top & !0xf;

    // kernel::println!("Switching to SP_EL1. Stack: {:#x}, Entry: {:#x}", stack_top, entry as usize);
    unsafe {
        core::arch::asm!(
            "msr spsel, #1",
            "mov sp, {stack}",
            "mov x29, xzr", // Clear FP
            "mov x30, xzr", // Clear LR
            "isb",
            "br {entry}",
            "b .",
            stack = in(reg) stack_top,
            entry = in(reg) entry,
            options(noreturn)
        );
    }
}

fn save_current_thread_context(tf: &TrapFrame) {
    let mut sched = kernel::sched::SCHEDULER.lock();
    if let Some(tid) = sched.current_id() {
        if let Some(thread) = sched.thread_mut(tid) {
            let regs_ptr = tf as *const TrapFrame as *const u64;
            let regs_slice = unsafe { core::slice::from_raw_parts(regs_ptr, 34) };
            thread.context.copy_from_slice(regs_slice);
            thread.started = true;
        }
    }
}

fn leak_user_str(ptr: u64, len: usize) -> Option<&'static str> {
    if len == 0 {
        return Some("");
    }
    let bytes = unsafe { user_slice(ptr as *const u8, len) };
    core::str::from_utf8(bytes).ok().map(|s| {
        let leaked: &'static mut str = Box::leak(s.to_string().into_boxed_str());
        leaked as &'static str
    })
}

unsafe fn user_slice<'a, T>(ptr: *const T, len: usize) -> &'a [T] {
    if len == 0 || ptr.is_null() {
        return &[];
    }
    let align = core::mem::align_of::<T>();
    if align > 1 && (ptr as usize) % align != 0 {
        &[]
    } else {
        core::slice::from_raw_parts(ptr, len)
    }
}
