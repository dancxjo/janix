extern crate alloc;

use crate::Sys;
use abi::{
    KernelRequest, KernelResponse, SyscallNumber, ThingGetSyscallResult, ThingPropScalarType,
};
use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

#[cfg(target_arch = "x86_64")]
mod x86_64;
#[cfg(target_arch = "x86_64")]
use x86_64::syscall_stub;

#[cfg(target_arch = "aarch64")]
mod aarch64;
#[cfg(target_arch = "aarch64")]
use aarch64::syscall_stub;

#[cfg(target_arch = "riscv64")]
mod riscv64;
#[cfg(target_arch = "riscv64")]
use riscv64::syscall_stub;

#[cfg(target_arch = "loongarch64")]
mod loongarch64;
#[cfg(target_arch = "loongarch64")]
use loongarch64::syscall_stub;

/// Sys implementation used by userland apps to invoke syscalls with an
/// architecture-specific trampoline. The struct itself stays portable;
/// only the underlying trap mechanism changes per target.
pub struct UserlandSys;

impl Sys for UserlandSys {
    fn syscall(&self, request: KernelRequest) -> KernelResponse {
        match request {
            KernelRequest::Log { message } => {
                let ptr = message.as_ptr() as u64;
                let len = message.len() as u64;
                unsafe { syscall_stub(SyscallNumber::Log, ptr, len, 0, 0, 0, 0) };
                KernelResponse::Success { data: None }
            }
            KernelRequest::ExitThread => self.exit_thread(),
            KernelRequest::SchedulerTick => {
                unsafe { syscall_stub(SyscallNumber::Yield, 0, 0, 0, 0, 0, 0) };
                KernelResponse::Success { data: None }
            }
            KernelRequest::AllocFrame { pool_index } => {
                let mut frame = abi::FrameInfo {
                    id: abi::FrameId(0),
                    base: 0,
                    size: 0,
                };
                let ret = unsafe {
                    syscall_stub(
                        SyscallNumber::AllocFrame,
                        pool_index as u64,
                        &mut frame as *mut _ as u64,
                        0,
                        0,
                        0,
                        0,
                    )
                };
                if ret == 0 {
                    KernelResponse::FrameAllocated { frame }
                } else {
                    KernelResponse::Error {
                        message: "AllocFrame failed",
                    }
                }
            }
            KernelRequest::FreeFrame { frame_id } => {
                let ret =
                    unsafe { syscall_stub(SyscallNumber::FreeFrame, frame_id.0, 0, 0, 0, 0, 0) };
                if ret == 0 {
                    KernelResponse::FrameFreed { frame_id }
                } else {
                    KernelResponse::Error {
                        message: "FreeFrame failed",
                    }
                }
            }
            KernelRequest::CreateProcess { name } => {
                let ptr = name.as_ptr() as u64;
                let len = name.len() as u64;
                let ret = unsafe {
                    syscall_stub(SyscallNumber::CreateProcess, ptr, len, 0, 0, 0, 0)
                };
                if ret == 0 {
                    KernelResponse::Error {
                        message: "CreateProcess failed",
                    }
                } else {
                    KernelResponse::ProcessCreated { pid: ret }
                }
            }
            KernelRequest::CreateThread {
                pid,
                name,
                app_id,
                priority,
            } => {
                let ptr = name.as_ptr() as u64;
                let len = name.len() as u64;
                let ret = unsafe {
                    syscall_stub(
                        SyscallNumber::CreateThread,
                        pid,
                        app_id,
                        priority,
                        ptr,
                        len,
                        0,
                    )
                };
                if ret == 0 {
                    KernelResponse::Error {
                        message: "CreateThread failed",
                    }
                } else {
                    KernelResponse::ThreadCreated { tid: ret }
                }
            }
            KernelRequest::ThingCreate { kind, props } => {
                let kind_ptr = kind.as_ptr() as u64;
                let kind_len = kind.len() as u64;
                let props_ptr = props.as_ptr() as u64;
                let props_len = props.len() as u64;
                let ret = unsafe {
                    syscall_stub(
                        SyscallNumber::ThingCreate,
                        kind_ptr,
                        kind_len,
                        props_ptr,
                        props_len,
                        0,
                        0,
                    )
                };
                KernelResponse::ThingCreated {
                    id: abi::ThingId(ret),
                }
            }
            KernelRequest::ThingGet { id } => {
                let mut raw = ThingGetSyscallResult::default();
                let ret = unsafe {
                    syscall_stub(
                        SyscallNumber::ThingGet,
                        id.0,
                        &mut raw as *mut _ as u64,
                        0,
                        0,
                        0,
                        0,
                    )
                };
                if ret != 0 {
                    KernelResponse::Error {
                        message: "ThingGet failed",
                    }
                } else {
                    let kind_bytes = &raw.kind[..raw.kind_len];
                    let kind_str = core::str::from_utf8(kind_bytes).unwrap_or("");
                    let kind_static: &'static str =
                        Box::leak(kind_str.to_string().into_boxed_str());

                    let mut copied: Vec<Option<(abi::PropKey, abi::PropValue)>> =
                        Vec::with_capacity(raw.prop_count);
                    for entry in raw.props.iter().take(raw.prop_count) {
                        if entry.present == 0 {
                            copied.push(None);
                            continue;
                        }

                        let key_bytes = &entry.key[..entry.key_len];
                        let key_str = core::str::from_utf8(key_bytes).unwrap_or("");
                        let key_static: &'static str =
                            Box::leak(key_str.to_string().into_boxed_str());

                        let value = match entry.value_type {
                            ThingPropScalarType::U64 => abi::PropValue::U64(entry.value_u64),
                            ThingPropScalarType::I64 => abi::PropValue::I64(entry.value_i64),
                            ThingPropScalarType::Bool => {
                                abi::PropValue::Bool(entry.value_bool != 0)
                            }
                            ThingPropScalarType::Str => {
                                let str_bytes = &entry.value_str[..entry.value_str_len];
                                let string =
                                    String::from_utf8(str_bytes.to_vec()).unwrap_or_else(|_| {
                                        String::from(core::str::from_utf8(str_bytes).unwrap_or(""))
                                    });
                                abi::PropValue::Str(string)
                            }
                        };

                        copied.push(Some((key_static, value)));
                    }

                    let props_static: &'static [Option<(abi::PropKey, abi::PropValue)>] =
                        Box::leak(copied.into_boxed_slice());

                    KernelResponse::ThingData {
                        id,
                        kind: kind_static,
                        props: props_static,
                    }
                }
            }
            KernelRequest::ThingList { kind, start_after } => {
                let kind_ptr = kind.as_ptr() as u64;
                let kind_len = kind.len() as u64;
                let ret = unsafe {
                    syscall_stub(
                        SyscallNumber::ThingList,
                        kind_ptr,
                        kind_len,
                        start_after.0,
                        0,
                        0,
                        0,
                    )
                };
                if ret == u64::MAX {
                    KernelResponse::ThingListEntry { id: None }
                } else {
                    KernelResponse::ThingListEntry {
                        id: Some(abi::ThingId(ret)),
                    }
                }
            }
            KernelRequest::AddEdge {
                from,
                edge_kind,
                to,
            } => {
                let kind_ptr = edge_kind.as_ptr() as u64;
                let kind_len = edge_kind.len() as u64;
                let ret = unsafe {
                    syscall_stub(
                        SyscallNumber::AddEdge,
                        from.0,
                        to.0,
                        kind_ptr,
                        kind_len,
                        0,
                        0,
                    )
                };
                if ret == 0 {
                    KernelResponse::Success { data: None }
                } else {
                    KernelResponse::Error {
                        message: "AddEdge failed",
                    }
                }
            }
            KernelRequest::EdgeAt {
                from,
                edge_kind,
                index,
            } => {
                let kind_ptr = edge_kind.as_ptr() as u64;
                let kind_len = edge_kind.len() as u64;
                let ret = unsafe {
                    syscall_stub(
                        SyscallNumber::EdgeAt,
                        from.0,
                        index,
                        kind_ptr,
                        kind_len,
                        0,
                        0,
                    )
                };
                if ret == u64::MAX {
                    KernelResponse::EdgeTarget { target: None }
                } else {
                    KernelResponse::EdgeTarget {
                        target: Some(abi::ThingId(ret)),
                    }
                }
            }
            KernelRequest::SpawnProgram { boot_program_id } => {
                let mut result = abi::SpawnProgramResult {
                    process_id: abi::ThingId(0),
                    thread_id: abi::ThingId(0),
                };
                let ret = unsafe {
                    syscall_stub(
                        SyscallNumber::SpawnProgram,
                        boot_program_id.0,
                        &mut result as *mut _ as u64,
                        0,
                        0,
                        0,
                        0,
                    )
                };
                if ret == 0 {
                    KernelResponse::ProgramSpawned {
                        process_id: result.process_id,
                        thread_id: result.thread_id,
                    }
                } else {
                    KernelResponse::Error {
                        message: "SpawnProgram failed",
                    }
                }
            }
            KernelRequest::SchemaRegister {
                kind,
                description,
                props,
            } => {
                let kind_ptr = kind.as_ptr() as u64;
                let kind_len = kind.len() as u64;
                let desc_ptr = description.as_ptr() as u64;
                let desc_len = description.len() as u64;
                let props_ptr = props.as_ptr() as u64;
                let props_len = props.len() as u64;
                let ret = unsafe {
                    syscall_stub(
                        SyscallNumber::SchemaRegister,
                        kind_ptr,
                        kind_len,
                        desc_ptr,
                        desc_len,
                        props_ptr,
                        props_len,
                    )
                };
                if ret == 0 {
                    KernelResponse::SchemaRegistered { kind }
                } else {
                    KernelResponse::Error {
                        message: "SchemaRegister failed",
                    }
                }
            }
            _ => KernelResponse::Error {
                message: "Syscall not implemented yet",
            },
        }
    }

    fn time_now_ns(&mut self) -> u64 {
        unsafe { syscall_stub(SyscallNumber::TimeNow, 0, 0, 0, 0, 0, 0) }
    }

    fn time_monotonic_ns(&mut self) -> u64 {
        unsafe { syscall_stub(SyscallNumber::TimeMonotonicNs, 0, 0, 0, 0, 0, 0) }
    }

    fn time_system_ns(&mut self) -> u64 {
        unsafe { syscall_stub(SyscallNumber::TimeSystemNs, 0, 0, 0, 0, 0, 0) }
    }

    fn sleep_for_ns(&mut self, delta_ns: u64) {
        unsafe { syscall_stub(SyscallNumber::SleepForNs, delta_ns, 0, 0, 0, 0, 0) };
    }

    fn sleep_until_ns(&mut self, deadline_ns: u64) {
        unsafe { syscall_stub(SyscallNumber::SleepUntil, deadline_ns, 0, 0, 0, 0, 0) };
    }

    fn yield_now(&mut self) {
        unsafe { syscall_stub(SyscallNumber::Yield, 0, 0, 0, 0, 0, 0) };
    }

    fn exit_thread(&mut self) -> ! {
        unsafe { syscall_stub(SyscallNumber::ExitThread, 0, 0, 0, 0, 0, 0) };
        loop {}
    }
}

impl UserlandSys {
    /// Construct a portable syscall client suitable for user-mode code.
    pub fn new() -> Self {
        #[cfg(all(target_os = "none", not(feature = "kernel")))]
        {
            crate::init_user_heap();
        }
        UserlandSys
    }

    /// Exit the current thread via the platform syscall interface. The
    /// implementation never returns.
    pub fn exit_thread(&self) -> ! {
        unsafe {
            syscall_stub(SyscallNumber::ExitThread, 0, 0, 0, 0, 0, 0);
        }
        loop {}
    }
}
