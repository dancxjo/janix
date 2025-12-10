//! x86_64 syscall handling.
//!
//! This module provides the syscall entry point for x86_64. It defines
//! `SyscallRegs` layout matching the stack-saved registers, an assembly
//! wrapper `syscall_handler_asm` which saves registers and calls the Rust
//! handler `syscall_handler_rust`, and the Rust-side dispatcher that decodes
//! `SyscallNumber` values and forwards requests to `kernel_core` and `user`
//! helpers. The handler also saves/restores thread contexts and implements
//! basic syscalls such as yield, sleep, logging, process/thread management,
//! frame allocation, and time queries.
use abi::{
    KernelRequest, KernelResponse, MapFlags, ProcessId, SyscallNumber, THING_GET_MAX_KIND_LEN,
    THING_GET_MAX_PROPS, THING_GET_MAX_STR_LEN, ThingGetSyscallResult, ThingId, ThingPropData,
    ThingPropScalarType,
};
use core::arch::global_asm;
use core::cmp;
extern crate alloc;
use crate::user;
use alloc::boxed::Box;
use alloc::string::ToString;

#[repr(C)]
pub struct SyscallRegs {
    pub rax: u64,
    pub rdi: u64,
    pub rsi: u64,
    pub rdx: u64,
    pub rcx: u64,
    pub r8: u64,
    pub r9: u64,
    pub r10: u64,
    pub r11: u64,
    pub rbx: u64,
    pub rbp: u64,
    pub r12: u64,
    pub r13: u64,
    pub r14: u64,
    pub r15: u64,
    pub rip: u64,
    pub cs: u64,
    pub rflags: u64,
    pub rsp: u64,
    pub ss: u64,
}

const _: () = {
    use core::mem::offset_of;
    assert!(core::mem::size_of::<SyscallRegs>() == 160);
    assert!(offset_of!(SyscallRegs, rax) == 0);
    assert!(offset_of!(SyscallRegs, rdi) == 8);
    assert!(offset_of!(SyscallRegs, rsi) == 16);
    assert!(offset_of!(SyscallRegs, rdx) == 24);
    assert!(offset_of!(SyscallRegs, rcx) == 32);
    assert!(offset_of!(SyscallRegs, r8) == 40);
    assert!(offset_of!(SyscallRegs, r9) == 48);
    assert!(offset_of!(SyscallRegs, r10) == 56);
    assert!(offset_of!(SyscallRegs, r11) == 64);
    assert!(offset_of!(SyscallRegs, rbx) == 72);
    assert!(offset_of!(SyscallRegs, rbp) == 80);
    assert!(offset_of!(SyscallRegs, r12) == 88);
    assert!(offset_of!(SyscallRegs, r13) == 96);
    assert!(offset_of!(SyscallRegs, r14) == 104);
    assert!(offset_of!(SyscallRegs, r15) == 112);
    assert!(offset_of!(SyscallRegs, rip) == 120);
    assert!(offset_of!(SyscallRegs, cs) == 128);
    assert!(offset_of!(SyscallRegs, rflags) == 136);
    assert!(offset_of!(SyscallRegs, rsp) == 144);
    assert!(offset_of!(SyscallRegs, ss) == 152);
};

global_asm!(
    r#"
.global syscall_handler_asm
syscall_handler_asm:
    push r15
    push r14
    push r13
    push r12
    push rbp
    push rbx
    push r11
    push r10
    push r9
    push r8
    push rcx
    push rdx
    push rsi
    push rdi
    push rax

    mov rdi, rsp
    call syscall_handler_rust
    // The return address is popped by `ret`, so don't mutate `rsp` here.
    // Overwrite the saved RAX (at [rsp]) with the return value from Rust.
    mov [rsp], rax

    // Debug: snapshot the pending iret frame and saved regs.
    // mov rdi, rsp
    // call log_syscall_iret_frame

    pop rax
    pop rdi
    pop rsi
    pop rdx
    pop rcx
    pop r8
    pop r9
    pop r10
    pop r11
    pop rbx
    pop rbp
    pop r12
    pop r13
    pop r14
    pop r15

    iretq
"#
);

unsafe extern "C" {
    pub fn syscall_handler_asm();
}

#[unsafe(no_mangle)]
extern "C" fn log_syscall_iret_frame(rsp: *const u64) {
    // Read a few qwords from the stack to see what iret will consume.
    let mut words = [0u64; 8];
    for (i, slot) in words.iter_mut().enumerate() {
        // SAFETY: best-effort diagnostic read; stack pointer is expected to be valid here.
        unsafe {
            *slot = core::ptr::read_volatile(rsp.add(i));
        }
    }
    kernel_core::println!(
        "syscall iret frame: rsp={:#x} top=[{:#x}, {:#x}, {:#x}, {:#x}, {:#x}, {:#x}, {:#x}, {:#x}]",
        rsp as u64,
        words[0],
        words[1],
        words[2],
        words[3],
        words[4],
        words[5],
        words[6],
        words[7],
    );
}

#[allow(unreachable_code, unsafe_op_in_unsafe_fn)]
#[unsafe(no_mangle)]
pub extern "C" fn syscall_handler_rust(regs: *mut SyscallRegs) -> u64 {
    let regs = unsafe { &mut *regs };
    let num = regs.rax;
    let arg1 = regs.rdi;
    let arg2 = regs.rsi;
    let arg3 = regs.rdx;
    let arg4 = regs.rcx;
    let arg5 = regs.r8;
    let _arg6 = regs.r9;

    if num == SyscallNumber::Yield as u64 {
        {
            let mut sched = kernel_core::sched::SCHEDULER.lock();
            if let Some(tid) = sched.current_id() {
                if let Some(thread) = sched.thread_mut(tid) {
                    let regs_ptr = regs as *const SyscallRegs as *const u64;
                    let regs_slice = unsafe { core::slice::from_raw_parts(regs_ptr, 20) };
                    thread.context[..20].copy_from_slice(regs_slice);
                    thread.started = true;
                }
            }
        }
        kernel_core::sched::yield_current_thread();
        return user::schedule_next();
    } else if num == SyscallNumber::SleepForNs as u64 {
        {
            let mut sched = kernel_core::sched::SCHEDULER.lock();
            if let Some(tid) = sched.current_id() {
                if let Some(thread) = sched.thread_mut(tid) {
                    let regs_ptr = regs as *const SyscallRegs as *const u64;
                    let regs_slice = unsafe { core::slice::from_raw_parts(regs_ptr, 20) };
                    thread.context[..20].copy_from_slice(regs_slice);
                    thread.started = true;
                }
            }
        }
        return user::sys_sleep_for_ns(arg1);
    } else if num == SyscallNumber::Log as u64 {
        let ptr = arg1 as *const u8;
        let len = arg2 as usize;
        if let Ok(s) = unsafe { core::str::from_utf8(user_slice(ptr, len)) } {
            let leaked: &'static str = Box::leak(s.to_string().into_boxed_str());
            kernel_core::log(leaked);
        }
        0
    } else if num == SyscallNumber::ExitThread as u64 {
        kernel_core::log("Thread exited via syscall");
        kernel_core::sched::exit_current_thread();
        return user::schedule_next();
    } else if num == SyscallNumber::AllocFrame as u64 {
        let pool_index = arg1;
        let frame_info_ptr = arg2 as *mut abi::FrameInfo;

        let req = KernelRequest::AllocFrame {
            pool_index: pool_index,
        };
        match kernel_core::handle_request(req) {
            KernelResponse::FrameAllocated { frame } => {
                unsafe { *frame_info_ptr = frame };
                0
            }
            _ => 1,
        }
    } else if num == SyscallNumber::FreeFrame as u64 {
        let frame_id = abi::FrameId(arg1);
        let req = KernelRequest::FreeFrame { frame_id };
        match kernel_core::handle_request(req) {
            KernelResponse::FrameFreed { .. } => 0,
            _ => 1,
        }
    } else if num == SyscallNumber::CreateProcess as u64 {
        let Some(name) = leak_user_str(arg1, arg2 as usize) else {
            return 0;
        };
        let pid = {
            let mut sched = kernel_core::sched::SCHEDULER.lock();
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
            let mut sched = kernel_core::sched::SCHEDULER.lock();
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
        match kernel_core::handle_request(req) {
            KernelResponse::Success { .. } => 0,
            _ => 1,
        }
    } else if num == SyscallNumber::EdgeAt as u64 {
        let from = ThingId(arg1);
        let index = arg2;
        let pred = abi::EdgePred(arg3);
        match kernel_core::handle_request(KernelRequest::EdgeAt { from, pred, index }) {
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
        match kernel_core::handle_request(KernelRequest::CreateSharedBuffer {
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
        match kernel_core::handle_request(KernelRequest::MapSharedBuffer { buffer_id, flags }) {
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
        match kernel_core::handle_request(KernelRequest::GetSharedBufferInfo { buffer_id }) {
            KernelResponse::SharedBufferInfoResponse { info } => {
                if !info_out.is_null() {
                    unsafe { *info_out = info };
                }
                0
            }
            _ => 1,
        }
    } else if num == SyscallNumber::SpawnProgram as u64 {
        let boot_program_id = abi::ThingId(arg1);
        let result_ptr = arg2 as *mut abi::SpawnProgramResult;
        if result_ptr.is_null() {
            return 1;
        }
        match crate::program::spawn_program(boot_program_id) {
            Ok((process_id, thread_id)) => {
                unsafe {
                    (*result_ptr).process_id = process_id;
                    (*result_ptr).thread_id = thread_id;
                }
                0
            }
            Err(msg) => {
                kernel_core::log(msg);
                1
            }
        }
    } else if num == SyscallNumber::ThingGet as u64 {
        let id = ThingId(arg1);
        let result_ptr = arg2 as *mut ThingGetSyscallResult;
        if result_ptr.is_null() {
            return 1;
        }
        let req = KernelRequest::ThingGet { id };
        match kernel_core::handle_request(req) {
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
                kernel_core::log(message);
                1
            }
            other => {
                let msg = alloc::format!("ThingGet unexpected response {:?}", other);
                let leaked: &'static str = Box::leak(msg.into_boxed_str());
                kernel_core::log(leaked);
                1
            }
        }
    } else if num == SyscallNumber::ThingList as u64 {
        let kind_ptr = arg1 as *const u8;
        let kind_len = arg2 as usize;
        let start_after = ThingId(arg3);
        let kind = unsafe { core::str::from_utf8(user_slice(kind_ptr, kind_len)).unwrap_or("") };
        let kind_static: &'static str = Box::leak(kind.to_string().into_boxed_str());
        match kernel_core::handle_request(KernelRequest::ThingList {
            kind: kind_static,
            start_after,
        }) {
            KernelResponse::ThingListEntry { id } => id.map_or(u64::MAX, |tid| tid.0),
            KernelResponse::Error { .. } => u64::MAX,
            other => {
                let msg = alloc::format!("ThingList unexpected response {:?}", other);
                let leaked: &'static str = Box::leak(msg.into_boxed_str());
                kernel_core::log(leaked);
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
        match kernel_core::handle_request(req) {
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
        match kernel_core::handle_request(req) {
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
        match kernel_core::handle_request(req) {
            KernelResponse::SchemaRegistered { .. } => 0,
            _ => 1,
        }
    } else if num == SyscallNumber::TimeNow as u64 {
        kernel_core::time::monotonic_now_ns()
    } else if num == SyscallNumber::TimeMonotonicNs as u64 {
        kernel_core::time::monotonic_now_ns()
    } else if num == SyscallNumber::TimeSystemNs as u64 {
        if let Some(ns) = kernel_core::time::system_time_ns() {
            ns
        } else {
            0
        }
    } else if num == SyscallNumber::SleepUntil as u64 {
        let deadline_ns = arg1;
        while kernel_core::time::monotonic_now_ns() < deadline_ns {
            user::schedule_next();
        }
        0
    } else {
        1 // SYS_ENOSYS or error
    }
}

pub fn install_handler() {
    super::trap::init();
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
        unsafe { core::slice::from_raw_parts(ptr, len) }
    }
}
