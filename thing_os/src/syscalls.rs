use abi::{
    SharedBufferInfo, ThingGetSyscallResult,
    ThingPropScalarType, resident::{ResidentAllocResp, ResidentError, ResidentMapResp, RestResp},
    syscalls::*, syscall_defs::{SymbolId, SymbolInternReq, WireStr},
};
use abi::{KernelRequest, KernelResponse};
use thing_models::PropType;
use thing_models::SchemaRegistryOutcome;
use crate::sys::raw_syscall;
use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

pub fn sys_symbol_intern(s: &str) -> SymbolId {
    let req = SymbolInternReq {
        s: WireStr {
            ptr: s.as_ptr() as u64,
            len: s.len() as u64,
        },
    };
    let mut resp = abi::syscall_defs::SymbolInternResp { id: SymbolId(0) };
    let ret = unsafe {
        raw_syscall(
            SYSCALL_SYMBOL_INTERN,
            &req as *const _ as u64,
            &mut resp as *mut _ as u64,
            0,
            0,
            0,
            0
        )
    };
    if ret == 0 {
        resp.id
    } else {
        SymbolId(0) // Error fallback
    }
}

pub fn syscall(request: KernelRequest) -> KernelResponse {
    match request {
        KernelRequest::Log { message } => {
            let ptr = message.ptr;
            let len = message.len;
            unsafe { raw_syscall(SYSCALL_LOG, ptr, len, 0, 0, 0, 0) };
            KernelResponse::Success { data: None }
        }

        KernelRequest::ExitThread => {
             unsafe { raw_syscall(SYSCALL_EXIT_THREAD, 0, 0, 0, 0, 0, 0) };
             loop {}
        },
        KernelRequest::SchedulerTick => {
            unsafe { raw_syscall(SYSCALL_YIELD, 0, 0, 0, 0, 0, 0) };
            KernelResponse::Success { data: None }
        }
        KernelRequest::AllocFrame { pool_index } => {
            let mut frame = abi::FrameInfo {
                id: abi::FrameId(0),
                base: 0,
                size: 0,
            };
            let ret = unsafe {
                raw_syscall(
                    SYSCALL_ALLOC_FRAME,
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
                    err: abi::syscall_defs::SysError { code: 1, detail: 0 },
                }
            }
        }
        KernelRequest::FreeFrame { frame_id } => {
            let ret =
                unsafe { raw_syscall(SYSCALL_FREE_FRAME, frame_id.0, 0, 0, 0, 0, 0) };
            if ret == 0 {
                KernelResponse::FrameFreed { frame_id }
            } else {
                KernelResponse::Error {
                    err: abi::syscall_defs::SysError { code: 1, detail: 0 },
                }
            }
        }
        KernelRequest::CreateProcess { name } => {
            let ptr = name.ptr;
            let len = name.len;
            let ret =
                unsafe { raw_syscall(SYSCALL_CREATE_PROCESS, ptr, len, 0, 0, 0, 0) };
            if ret == 0 {
                KernelResponse::Error {
                    err: abi::syscall_defs::SysError { code: 1, detail: 0 },
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
            let ptr = name.ptr;
            let len = name.len;
            let ret = unsafe {
                raw_syscall(
                    SYSCALL_CREATE_THREAD,
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
                    err: abi::syscall_defs::SysError { code: 1, detail: 0 },
                }
            } else {
                KernelResponse::ThreadCreated { tid: ret }
            }
        }
        KernelRequest::ThingCreate { kind, props } => {
            let ret = unsafe {
                raw_syscall(
                    SYSCALL_THING_CREATE,
                    kind.0 as u64,
                    props.ptr,
                    props.len,
                    0,
                    0,
                    0,
                )
            };
            if ret == 0 {
                KernelResponse::Error {
                    err: abi::syscall_defs::SysError { code: 1, detail: 0 },
                }
            } else {
                KernelResponse::ThingCreated {
                    id: abi::ThingId(ret),
                }
            }
        }
        KernelRequest::ThingUpdate { id, props } => {
            // arg1=id, arg2=props_ptr, arg3=props_len
            let ret = unsafe {
                raw_syscall(
                    SYSCALL_THING_UPDATE,
                    id.0,
                    props.ptr,
                    props.len,
                    0,
                    0,
                    0,
                )
            };
            if ret == 0 {
                KernelResponse::Success { data: None }
            } else {
                KernelResponse::Error {
                    err: abi::syscall_defs::SysError { code: 1, detail: 0 },
                }
            }
        }
        KernelRequest::ThingGet { id, out: _ } => {
            // Disabled ThingGet shim
            KernelResponse::Error {
                 err: abi::syscall_defs::SysError { code: 1, detail: 0 },
            }
        }
        KernelRequest::ThingList { kind, start_after } => {
            // arg1=kind(SymbolId), arg2=start_after
            let ret = unsafe {
                raw_syscall(
                    SYSCALL_THING_LIST,
                    kind.0 as u64,
                    start_after.0,
                    0,
                    0,
                    0,
                    0,
                )
            };
            if ret == u64::MAX {
                KernelResponse::ThingListEntry { id: abi::ThingId(0), valid: 0 }
            } else {
                KernelResponse::ThingListEntry {
                    id: abi::ThingId(ret),
                    valid: 1,
                }
            }
        }
        KernelRequest::AddLink { src, pred, dst } => {
            let ret =
                unsafe { raw_syscall(SYSCALL_ADD_LINK, src.0, pred.0, dst.0, 0, 0, 0) };
            if ret == 0 {
                KernelResponse::Success { data: None }
            } else {
                KernelResponse::Error {
                    err: abi::syscall_defs::SysError { code: 1, detail: 0 },
                }
            }
        }
        KernelRequest::LinkAt { src, pred, idx } => {
            let ret = unsafe {
                raw_syscall(SYSCALL_LINK_AT, src.0, idx as u64, pred.0, 0, 0, 0)
            };
            if ret == u64::MAX {
                KernelResponse::LinkTarget { target: abi::ThingId(0), found: 0 }
            } else {
                KernelResponse::LinkTarget {
                    target: abi::ThingId(ret),
                    found: 1,
                }
            }
        }
        KernelRequest::SpawnProgram { boot_program_id } => {
use abi::wire::process::SpawnProgramResult;
            let mut result = SpawnProgramResult {
                process_id: abi::ThingId(0),
                thread_id: abi::ThingId(0),
            };
            let ret = unsafe {
                raw_syscall(
                    SYSCALL_SPAWN_PROGRAM,
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
                    err: abi::syscall_defs::SysError { code: 1, detail: 0 },
                }
            }
        }
        KernelRequest::SchemaRegisterPackage {
            kind,
            description,
            props,
        } => {
            let ret = unsafe {
                raw_syscall(
                    SYSCALL_SCHEMA_REGISTER_PACKAGE,
                    kind.0 as u64,
                    description.0 as u64,
                    props.ptr,
                    props.len,
                    0,
                    0,
                )
            };
            if ret <= 2 {
                let outcome = unsafe { core::mem::transmute(ret as u8) };
                KernelResponse::SchemaRegistered { kind, outcome }
            } else {
                KernelResponse::Error {
                    err: abi::syscall_defs::SysError { code: 1, detail: 0 },
                }
            }
        }
        KernelRequest::MapSharedBuffer { buffer_id, flags } => {
            let mut vaddr = 0_u64;
            let mut size = 0_u64;
            let ret = unsafe {
                raw_syscall(
                    SYSCALL_MAP_SHARED_BUFFER,
                    buffer_id.0,
                    flags.bits(),
                    &mut vaddr as *mut _ as u64,
                    &mut size as *mut _ as u64,
                    0,
                    0,
                )
            };
            if ret == 0 {
                KernelResponse::SharedBufferMapped { vaddr, size }
            } else {
                KernelResponse::Error {
                    err: abi::syscall_defs::SysError { code: 1, detail: 0 },
                }
            }
        }
        KernelRequest::CreateSharedBuffer {
            width,
            height,
            pixel_format,
        } => {
            let ret = unsafe {
                raw_syscall(
                    SYSCALL_CREATE_SHARED_BUFFER,
                    width as u64,
                    height as u64,
                    pixel_format as u8 as u64,
                    0,
                    0,
                    0,
                )
            };
            if ret != 0 {
                KernelResponse::SharedBufferCreated {
                    buffer_id: abi::ThingId(ret),
                }
            } else {
                KernelResponse::Error {
                    err: abi::syscall_defs::SysError { code: 1, detail: 0 },
                }
            }
        }
        KernelRequest::GetSharedBufferInfo { buffer_id } => {
            let mut info = SharedBufferInfo {
                width: 0,
                height: 0,
                stride: 0,
                pixel_format: abi::PixelFormat::Rgba8888,
            };
            let ret = unsafe {
                raw_syscall(
                    SYSCALL_GET_SHARED_BUFFER_INFO,
                    buffer_id.0,
                    &mut info as *mut _ as u64,
                    0,
                    0,
                    0,
                    0,
                )
            };
            if ret == 0 {
                KernelResponse::SharedBufferInfoResponse { info }
            } else {
                KernelResponse::Error {
                    err: abi::syscall_defs::SysError { code: 1, detail: 0 },
                }
            }
        }
        KernelRequest::ResidentAlloc { kind, byte_len, flags: _ } => {
            // kind is SymbolId now
            let mut resp = ResidentAllocResp::default();
            let mut err = ResidentError::default();
            
            let ret = unsafe {
                raw_syscall(
                    SYSCALL_RESIDENT_ALLOC,
                    kind.0 as u64,
                    byte_len as u64,
                    &mut resp as *mut _ as u64,
                    &mut err as *mut _ as u64,
                    0,
                    0,
                )
            };
            
            if ret == 0 {
                KernelResponse::ResidentAllocated { resp }
            } else {
                KernelResponse::ResidentError(err)
            }
        }
        KernelRequest::ResidentMap { id, perms } => {
            let mut resp = ResidentMapResp::default();
            let mut err = ResidentError::default();
            
            let ret = unsafe {
                raw_syscall(
                    SYSCALL_RESIDENT_MAP,
                    id.0,
                    perms.0 as u64,
                    &mut resp as *mut _ as u64,
                    &mut err as *mut _ as u64,
                    0,
                    0,
                )
            };
            
            if ret == 0 {
                KernelResponse::ResidentMapped { resp }
            } else {
                KernelResponse::ResidentError(err)
            }
        }
        KernelRequest::ResidentUnmap { thing_id } => {
            let mut err = ResidentError::default();
            let ret = unsafe {
                raw_syscall(
                    SYSCALL_RESIDENT_UNMAP,
                    thing_id.0,
                    &mut err as *mut _ as u64,
                    0,
                    0,
                    0,
                    0,
                )
            };
            
            if ret == 0 {
                KernelResponse::Success { data: None }
            } else {
                KernelResponse::ResidentError(err)
            }
        }
        KernelRequest::ThingRest { thing_id, policy } => {
            let mut resp = RestResp::default();
            let mut err = ResidentError::default();
            
            let ret = unsafe {
                raw_syscall(
                    SYSCALL_THING_REST,
                    thing_id.0,
                    policy as u64,
                    &mut resp as *mut _ as u64,
                    &mut err as *mut _ as u64,
                    0,
                    0,
                )
            };
            
            if ret == 0 {
                KernelResponse::ThingRested { resp }
            } else {
                KernelResponse::ResidentError(err)
            }
        }
        _ => KernelResponse::Error {
            err: abi::syscall_defs::SysError { code: 1, detail: 0 },
        },
    }
}
