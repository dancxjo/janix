use abi::{
    KernelRequest, KernelResponse, SharedBufferInfo, ThingGetSyscallResult,
    ThingPropScalarType, resident::{ResidentAllocResp, ResidentError, ResidentMapResp, RestResp},
    syscalls::*, syscall_defs::{SymbolId, SymbolInternReq, WireStr}
};
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
    let ret = unsafe {
        raw_syscall(
            SYSCALL_SYMBOL_INTERN,
            &req as *const _ as u64,
            0,
            0,
            0,
            0,
            0
        )
    };
    SymbolId(ret as u32)
}

pub fn syscall(request: KernelRequest) -> KernelResponse {
    match request {
        KernelRequest::Log { message } => {
            let ptr = message.as_ptr() as u64;
            let len = message.len() as u64;
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
                    message: "AllocFrame failed",
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
                    message: "FreeFrame failed",
                }
            }
        }
        KernelRequest::CreateProcess { name } => {
            let ptr = name.as_ptr() as u64;
            let len = name.len() as u64;
            let ret =
                unsafe { raw_syscall(SYSCALL_CREATE_PROCESS, ptr, len, 0, 0, 0, 0) };
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
                    message: "CreateThread failed",
                }
            } else {
                KernelResponse::ThreadCreated { tid: ret }
            }
        }
        KernelRequest::ThingCreate { kind, props } => {
            // Updated to new ABI: passes request struct by pointer?
            // No, ABI plan said ThingCreate arguments: rdi=kind(u32), rsi=props_ptr, rdx=props_len
            // Wait, kind is SymbolId (u32 wrapped). 
            // Register passing: kind.0 as u64
            // props is UserSlice<WireProp>. We pass ptr/len of the slice directly?
            // UserSlice is just POD (ptr, len).
            // Usually we pass ptr/len as registers if possible, OR pointer to UserSlice struct.
            // arch/src/x86_64/syscall.rs: 
            // let kind = SymbolId(arg1 as u32);
            // let props_ptr = UserPtr::new(arg2);
            // let props_len = arg3;
            // So we pass kind in arg1, props ptr in arg2, props len in arg3.
            
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
                    message: "ThingCreate failed",
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
                    message: "ThingUpdate failed",
                }
            }
        }
        KernelRequest::ThingGet { id } => {
            let mut raw = ThingGetSyscallResult::default();
            let ret = unsafe {
                raw_syscall(
                    SYSCALL_THING_GET,
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

                    copied.push(Some((key_static.to_string(), value)));
                }

                let props_static: &'static [Option<(abi::PropKey, abi::PropValue)>] =
                    Box::leak(copied.into_boxed_slice());

                KernelResponse::ThingData {
                    id,
                    kind: sys_symbol_intern(kind_static),
                    props: props_static,
                }
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
                KernelResponse::ThingListEntry { id: None }
            } else {
                KernelResponse::ThingListEntry {
                    id: Some(abi::ThingId(ret)),
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
                    message: "AddLink failed",
                }
            }
        }
        KernelRequest::LinkAt { src, pred, idx } => {
            let ret = unsafe {
                raw_syscall(SYSCALL_LINK_AT, src.0, idx as u64, pred.0, 0, 0, 0)
            };
            if ret == u64::MAX {
                KernelResponse::LinkTarget { target: None }
            } else {
                KernelResponse::LinkTarget {
                    target: Some(abi::ThingId(ret)),
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
                    message: "SpawnProgram failed",
                }
            }
        }
        KernelRequest::SchemaRegister {
            kind,
            description,
            props,
        } => {
            // arg1=kind(Sym), arg2=desc(Sym), arg3=props_ptr, arg4=props_len
            
            let ret = unsafe {
                raw_syscall(
                    SYSCALL_SCHEMA_REGISTER,
                    kind.0 as u64,
                    description.0 as u64,
                    props.ptr,
                    props.len,
                    0,
                    0,
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
                    message: "MapSharedBuffer failed",
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
                    message: "CreateSharedBuffer failed",
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
                    message: "GetSharedBufferInfo failed",
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
            message: "Syscall not implemented yet",
        },
    }
}
