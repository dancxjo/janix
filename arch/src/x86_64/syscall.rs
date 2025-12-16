//! x86_64 syscall handling.
//!
//! This module provides the syscall entry point for x86_64. It defines
//! `SyscallRegs` layout matching the stack-saved registers, an assembly
//! wrapper `syscall_handler_asm` which saves registers and calls the Rust
//! handler `syscall_handler_rust`, and the Rust-side dispatcher that decodes
//! `SyscallNumber` values and forwards requests to `kernel` and `user`
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
}

const _: () = {
    use core::mem::offset_of;
    assert!(core::mem::size_of::<SyscallRegs>() == 120);
    assert!(offset_of!(SyscallRegs, r15) == 0);
    assert!(offset_of!(SyscallRegs, rax) == 112);
};

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
    // The return address is popped by `ret`, so don't mutate `rsp` here.
    // Overwrite the saved RAX (at [rsp + 14*8]) with the return value from Rust.
    // Wait, [rsp] is R15 which is offset 0. R14 is offset 8. ... RAX is offset 112.
    mov [rsp + 112], rax

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

#[unsafe(no_mangle)]
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
    kernel::println!(
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
#[allow(unreachable_code, unsafe_op_in_unsafe_fn)]
#[unsafe(no_mangle)]
pub extern "C" fn syscall_handler_rust(regs: *mut SyscallRegs) -> u64 {
    let regs = unsafe { &mut *regs };
    let num = regs.rax;
    let arg1 = regs.rdi;
    let arg2 = regs.rsi;
    let arg3 = regs.rdx;
    let arg4 = regs.rcx; // Userland stub passes arg4 in RCX (int 0x80 convention)
    let arg5 = regs.r8;
    let arg6 = regs.r9;

    if num == SyscallNumber::Yield as u64 {
        {
            let mut sched = kernel::sched::SCHEDULER.lock();
            if let Some(tid) = sched.current_id() {
                if let Some(thread) = sched.thread_mut(tid) {
                    let regs_ptr = regs as *const SyscallRegs as *const u64;
                    // Copy GPRs (15 u64s)
                    let gprs = unsafe { core::slice::from_raw_parts(regs_ptr, 15) };
                    thread.context[..15].copy_from_slice(gprs);
                    
                    // Manually read IRET frame from stack (offset 15)
                    unsafe {
                        let frame_ptr = regs_ptr.add(15);
                        let rip = *frame_ptr.add(0);
                        let cs = *frame_ptr.add(1);
                        let rflags = *frame_ptr.add(2);
                        let rsp = *frame_ptr.add(3);
                        let ss = *frame_ptr.add(4);
                        
                        thread.context[15] = rip;
                        thread.context[16] = cs;
                        thread.context[17] = rflags;
                        thread.context[18] = rsp;
                        thread.context[19] = ss;
                    }

                    thread.started = true;
                }
            }
        }
        kernel::sched::yield_current_thread();
        return user::schedule_next();
    } else if num == SyscallNumber::SleepForNs as u64 {
        {
            let mut sched = kernel::sched::SCHEDULER.lock();
            if let Some(tid) = sched.current_id() {
                if let Some(thread) = sched.thread_mut(tid) {
                    let regs_ptr = regs as *const SyscallRegs as *const u64;
                    // Copy GPRs (15 u64s)
                    let gprs = unsafe { core::slice::from_raw_parts(regs_ptr, 15) };
                    thread.context[..15].copy_from_slice(gprs);
                    
                    // Manually read IRET frame from stack (offset 15)
                    unsafe {
                        let frame_ptr = regs_ptr.add(15);
                        let rip = *frame_ptr.add(0);
                        let cs = *frame_ptr.add(1);
                        let rflags = *frame_ptr.add(2);
                        let rsp = *frame_ptr.add(3);
                        let ss = *frame_ptr.add(4);

                        thread.context[15] = rip;
                        thread.context[16] = cs;
                        thread.context[17] = rflags;
                        thread.context[18] = rsp;
                        thread.context[19] = ss;
                    }
                    thread.started = true;
                }
            }
        }
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
    } else if num == SyscallNumber::AddLink as u64 {
        let src = ThingId(arg1);
        let pred = abi::Predicate(arg2);
        let dst = ThingId(arg3);
        let req = KernelRequest::AddLink { src, pred, dst };
        match kernel::handle_request(req) {
            KernelResponse::Success { .. } => 0,
            _ => 1,
        }
    } else if num == SyscallNumber::LinkAt as u64 {
        let src = ThingId(arg1);
        let idx = arg2 as usize;
        let pred = abi::Predicate(arg3);
        match kernel::handle_request(KernelRequest::LinkAt { src, pred, idx }) {
            KernelResponse::LinkTarget { target } => target.map_or(u64::MAX, |id| id.0),
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
        let boot_program_id = abi::ThingId(arg1);
        let result_ptr = arg2 as *mut abi::SpawnProgramResult;
        if result_ptr.is_null() {
            return 1;
        }
        let req = KernelRequest::SpawnProgram { boot_program_id };
        match kernel::handle_request(req) {
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
        let mut deep_props = alloc::vec::Vec::with_capacity(props_len);
        for (k, v) in props {
            deep_props.push((k.clone(), v.clone()));
        }
        let props_static: &'static [(abi::PropKey, abi::PropValue)] =
            Box::leak(deep_props.into_boxed_slice());

        let req = KernelRequest::ThingCreate {
            kind: kind_static,
            props: props_static,
        };
        let ret = match kernel::handle_request(req) {
            KernelResponse::ThingCreated { id } => id.0,
            _ => 0,
        };

        // SAFETY: The request handling is done.
        // We clean up props because they are copied into the ThingNode (which owns its props).
        // WE DO NOT CLEAN UP kind_static, because ThingNode stores &'static str directly!
        // This causes a memory leak (one kind string per thing), but preventing UAF is priority.
        unsafe {
            let _ = alloc::boxed::Box::from_raw(props_static as *const [(abi::PropKey, abi::PropValue)] as *mut [(abi::PropKey, abi::PropValue)]);
        }

        ret
    } else if num == SyscallNumber::ThingUpdate as u64 {
        let id = ThingId(arg1);
        let props_ptr = arg2 as *const (abi::PropKey, abi::PropValue);
        let props_len = arg3 as usize;
        let props = unsafe { user_slice(props_ptr, props_len) };
        let mut deep_props = alloc::vec::Vec::with_capacity(props_len);
        for (k, v) in props {
            deep_props.push((k.clone(), v.clone()));
        }
        let props_static: &'static [(abi::PropKey, abi::PropValue)] =
            Box::leak(deep_props.into_boxed_slice());

        let req = KernelRequest::ThingUpdate {
            id,
            props: props_static,
        };
        let ret = match kernel::handle_request(req) {
            KernelResponse::Success { .. } => 0,
            _ => 1,
        };
        
        // SAFETY: Cleanup leaked props
        unsafe {
             let _ = alloc::boxed::Box::from_raw(props_static as *const [(abi::PropKey, abi::PropValue)] as *mut [(abi::PropKey, abi::PropValue)]);
        }
        
        ret
    } else if num == SyscallNumber::SchemaRegister as u64 {
        let kind_ptr = arg1 as *const u8;
        let kind_len = arg2 as usize;
        let desc_ptr = arg3 as *const u8;
        let desc_len = arg4 as usize;
        let props_ptr = arg5 as *const (&'static str, abi::PropType);
        let props_len = arg6 as usize;
        
        {
            let msg = alloc::format!("SchemaReg: kind_len={} props_len={}", kind_len, props_len);
            let leaked = Box::leak(msg.into_boxed_str());
            kernel::log(leaked);
        }

        let kind = unsafe { core::str::from_utf8(user_slice(kind_ptr, kind_len)).unwrap_or("") };
        let description = unsafe { core::str::from_utf8(user_slice(desc_ptr, desc_len)).unwrap_or("") };
        let props = unsafe { user_slice(props_ptr, props_len) };

        let kind_static: &'static str = Box::leak(kind.to_string().into_boxed_str());
        let description_static: &'static str = Box::leak(description.to_string().into_boxed_str());

        let mut deep_props = alloc::vec::Vec::with_capacity(props_len);
        for (k, t) in props {
             let k_static: &'static str = Box::leak(k.to_string().into_boxed_str());
             deep_props.push((k_static, *t));
        }
        let props_static: &'static [(&'static str, abi::PropType)] =
            Box::leak(deep_props.into_boxed_slice());

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
    } else if num == abi::syscall_numbers::SYS_DEV_OPEN as u64 {
        let args_ptr = arg1 as *const abi::syscall_defs::DevOpenArgs;
        let ret_ptr = arg2 as *mut abi::syscall_defs::SysRet<abi::syscall_defs::DevOpenRet>;

        if args_ptr.is_null() || ret_ptr.is_null() {
            return 1;
        }

        // Safety: We blindly trust user pointers here for simplicity,
        // matching existing syscall patterns in this file.
        let args = unsafe { *args_ptr };
        let res = kernel::devices::ps2_buffers::dev_open(args.kind, args.index);

        let sys_ret = match res {
            Ok(handle) => abi::syscall_defs::SysRet::ok(abi::syscall_defs::DevOpenRet { handle }),
            Err(e) => abi::syscall_defs::SysRet::err(e.code, e.detail),
        };
        unsafe { *ret_ptr = sys_ret };
        0
    } else if num == abi::syscall_numbers::SYS_DEV_READ as u64 {
        let args_ptr = arg1 as *const abi::syscall_defs::DevReadArgs;
        let ret_ptr = arg2 as *mut abi::syscall_defs::SysRet<abi::syscall_defs::DevReadRet>;

        if args_ptr.is_null() || ret_ptr.is_null() {
            return 1;
        }

        let args = unsafe { *args_ptr };
        let buffer_ptr = args.out.ptr.addr as *mut u8;
        let buffer_len = args.out.len as usize;

        // Validation similar to user_slice but strictly mutable
        if buffer_ptr.is_null() {
             unsafe { *ret_ptr = abi::syscall_defs::SysRet::err(abi::syscall_defs::SysError::INVALID_ARG, 0) };
             return 0;
        }

        let buffer = unsafe { core::slice::from_raw_parts_mut(buffer_ptr, buffer_len) };
        let res = kernel::devices::ps2_buffers::dev_read(args.handle, buffer);

        let sys_ret = match res {
            Ok(bytes_read) => abi::syscall_defs::SysRet::ok(abi::syscall_defs::DevReadRet {
                bytes_read: bytes_read as u32,
            }),
            Err(e) => abi::syscall_defs::SysRet::err(e.code, e.detail),
        };
        unsafe { *ret_ptr = sys_ret };
        0
    } else if num == SyscallNumber::ResidentAlloc as u64 {
        let kind_ptr = arg1 as *const u8;
        let kind_len = arg2 as usize;
        let byte_len = arg3 as u32;
        let resp_ptr = arg4 as *mut abi::resident::ResidentAllocResp;
        let err_ptr = arg5 as *mut abi::resident::ResidentError;

        let kind = unsafe { core::str::from_utf8(user_slice(kind_ptr, kind_len)).unwrap_or("") };
        let kind_static: &'static str = Box::leak(kind.to_string().into_boxed_str());

        match kernel::handle_request(KernelRequest::ResidentAlloc {
            kind: kind_static,
            byte_len,
            flags: 0,
        }) {
            KernelResponse::ResidentAllocated { resp } => {
                if !resp_ptr.is_null() {
                    unsafe { *resp_ptr = resp };
                }
                0
            }
            KernelResponse::ResidentError(e) => {
                if !err_ptr.is_null() {
                    unsafe { *err_ptr = e };
                }
                1
            }
            _ => 1,
        }
    } else if num == SyscallNumber::ResidentMap as u64 {
        let id = ThingId(arg1);
        let perms = abi::resident::ResidentMapPerms(arg2 as u32);
        let resp_ptr = arg3 as *mut abi::resident::ResidentMapResp;
        let err_ptr = arg4 as *mut abi::resident::ResidentError;

        match kernel::handle_request(KernelRequest::ResidentMap {
            id,
            perms,
        }) {
            KernelResponse::ResidentMapped { resp } => {
                if !resp_ptr.is_null() {
                    unsafe { *resp_ptr = resp };
                }
                0
            }
            KernelResponse::ResidentError(e) => {
                if !err_ptr.is_null() {
                    unsafe { *err_ptr = e };
                }
                1
            }
            _ => 1,
        }
    } else if num == SyscallNumber::ResidentUnmap as u64 {
        let id = ThingId(arg1);
        let err_ptr = arg2 as *mut abi::resident::ResidentError;
        match kernel::handle_request(KernelRequest::ResidentUnmap { thing_id: id }) {
            KernelResponse::Success { .. } => 0,
            KernelResponse::ResidentError(e) => {
                if !err_ptr.is_null() {
                    unsafe { *err_ptr = e };
                }
                1
            }
            _ => 1,
        }
    } else if num == SyscallNumber::ThingRest as u64 {
        let id = ThingId(arg1);
        let policy = match arg2 {
            0 => abi::resident::RestPolicy::SnapshotKeepResident,
            1 => abi::resident::RestPolicy::SnapshotEvictResident,
            _ => return 1,
        };
        let resp_ptr = arg3 as *mut abi::resident::RestResp;
        let err_ptr = arg4 as *mut abi::resident::ResidentError;

        match kernel::handle_request(KernelRequest::ThingRest {
            thing_id: id,
            policy,
        }) {
            KernelResponse::ThingRested { resp } => {
                if !resp_ptr.is_null() {
                    unsafe { *resp_ptr = resp };
                }
                0
            }
            KernelResponse::ResidentError(e) => {
                if !err_ptr.is_null() {
                    unsafe { *err_ptr = e };
                }
                1
            }
            _ => 1,
        }
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
