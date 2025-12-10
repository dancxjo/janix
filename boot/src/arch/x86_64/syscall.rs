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
use abi::{KernelRequest, KernelResponse, ProcessId, SyscallNumber, ThingId};
use core::arch::global_asm;
extern crate alloc;
use crate::user;
use alloc::boxed::Box;
use alloc::string::ToString;

#[repr(C)]
pub struct SyscallRegs {
    pub r15: u64,
    pub r14: u64,
    pub r13: u64,
    pub r12: u64,
    pub rbp: u64,
    pub rbx: u64,
    pub r11: u64,
    pub r10: u64,
    pub r9: u64,
    pub r8: u64,
    pub rcx: u64,
    pub rdx: u64,
    pub rsi: u64,
    pub rdi: u64,
    pub rax: u64,
    pub rip: u64,
    pub cs: u64,
    pub rflags: u64,
    pub rsp: u64,
    pub ss: u64,
}

global_asm!(
    r#"
.global syscall_handler_asm
syscall_handler_asm:
    push rax
    push rdi
    push rsi
    push rdx
    push rcx
    push r8
    push r9
    push r10
    push r11
    push rbx
    push rbp
    push r12
    push r13
    push r14
    push r15

    mov rdi, rsp
    call syscall_handler_rust
    
    pop r15
    pop r14
    pop r13
    pop r12
    pop rbp
    pop rbx
    pop r11
    pop r10
    pop r9
    pop r8
    pop rcx
    pop rdx
    pop rsi
    pop rdi
    pop rax
    
    iretq
"#
);

unsafe extern "C" {
    pub fn syscall_handler_asm();
}

#[unsafe(no_mangle)]
pub extern "C" fn syscall_handler_rust(regs: *mut SyscallRegs) -> u64 {
    let regs = unsafe { &mut *regs };
    let num = regs.rax;
    let arg1 = regs.rdi;
    let arg2 = regs.rsi;
    let arg3 = regs.rdx;
    let arg4 = regs.rcx;
    let arg5 = regs.r8;
    let arg6 = regs.r9;

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
        if let Ok(s) = unsafe { core::str::from_utf8(core::slice::from_raw_parts(ptr, len)) } {
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
        let to = ThingId(arg2);
        let Some(edge_kind) = leak_user_str(arg3, arg4 as usize) else {
            return 1;
        };
        let req = KernelRequest::AddEdge {
            from,
            edge_kind,
            to,
        };
        match kernel_core::handle_request(req) {
            KernelResponse::Success { .. } => 0,
            _ => 1,
        }
    } else if num == SyscallNumber::EdgeAt as u64 {
        let from = ThingId(arg1);
        let index = arg2;
        let Some(edge_kind) = leak_user_str(arg3, arg4 as usize) else {
            return u64::MAX;
        };
        match kernel_core::handle_request(KernelRequest::EdgeAt {
            from,
            edge_kind,
            index,
        }) {
            KernelResponse::EdgeTarget { target } => target.map_or(u64::MAX, |id| id.0),
            _ => u64::MAX,
        }
    } else if num == SyscallNumber::ThingCreate as u64 {
        let kind_ptr = arg1 as *const u8;
        let kind_len = arg2 as usize;
        let props_ptr = arg3 as *const (abi::PropKey, abi::PropValue);
        let props_len = arg4 as usize;

        let kind = unsafe {
            core::str::from_utf8(core::slice::from_raw_parts(kind_ptr, kind_len)).unwrap_or("")
        };
        let props = unsafe { core::slice::from_raw_parts(props_ptr, props_len) };

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
    } else if num == SyscallNumber::SchemaRegister as u64 {
        let kind_ptr = arg1 as *const u8;
        let kind_len = arg2 as usize;
        let props_ptr = arg3 as *const (&'static str, abi::PropType);
        let props_len = arg4 as usize;

        let kind = unsafe {
            core::str::from_utf8(core::slice::from_raw_parts(kind_ptr, kind_len)).unwrap_or("")
        };
        let props = unsafe { core::slice::from_raw_parts(props_ptr, props_len) };

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
    let bytes = unsafe { core::slice::from_raw_parts(ptr as *const u8, len) };
    core::str::from_utf8(bytes).ok().map(|s| {
        let leaked: &'static mut str = Box::leak(s.to_string().into_boxed_str());
        leaked as &'static str
    })
}
