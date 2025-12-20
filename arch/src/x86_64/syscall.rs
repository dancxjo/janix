extern crate alloc;
use abi::syscalls::*;
use abi::wire::common::{UserPtr, UserSlice};
use abi::wire::graph::{WireProp, WireSchemaProp, WirePropValue, WireValueTag, BatchUpdateReq};
use abi::syscall_defs::{SymbolId, SysRet, SymbolInternReq, SymbolInternResp, SymbolResolveReq, SymbolResolveResp};
use abi::{ThingId, ProcessId, MapFlags};
use thing_models::{PropValue, PropType};
use kernel::graph;
use kernel::symbols;
use alloc::vec::Vec;
use alloc::string::String;
use alloc::boxed::Box;
use core::slice;

use core::arch::global_asm;

// Global variables for syscall stack switching (used by ASM)
#[unsafe(no_mangle)]
static mut SYSCALL_KERNEL_RSP: u64 = 0;
#[unsafe(no_mangle)]
static mut SYSCALL_USER_RSP_SCRATCH: u64 = 0;
#[unsafe(no_mangle)]
static mut SYSCALL_USER_CS: u64 = 0;
#[unsafe(no_mangle)]
static mut SYSCALL_USER_SS: u64 = 0;

global_asm!(r#"
.global syscall_handler_asm
syscall_handler_asm:
    // Interrupts are disabled (SFMASK).
    // User Stack is currently active.
    
    // 1. Save User RSP to scratch
    mov [rip + SYSCALL_USER_RSP_SCRATCH], rsp
    
    // 2. Load Kernel RSP
    mov rsp, [rip + SYSCALL_KERNEL_RSP]
    
    // 3. Construct IRETQ frame on Kernel Stack
    // Frame: SS, RSP, RFLAGS, CS, RIP
    
    // Push SS (User Data)
    push qword ptr [rip + SYSCALL_USER_SS]
    
    // Push RSP (User RSP)
    push qword ptr [rip + SYSCALL_USER_RSP_SCRATCH]
    
    // Push RFLAGS (r11 saved by syscall)
    push r11
    
    // Push CS (User Code)
    push qword ptr [rip + SYSCALL_USER_CS]
    
    // Push RIP (rcx saved by syscall)
    push rcx
    
    // 4. Push SyscallRegs (reversed order)
    
    // Saved Regs
    push rax // rax_saved
    push rdi // rdi_saved
    push rsi
    push rdx
    push rcx // rcx_saved (RIP, but also general arg)
    push r8
    push r9
    push r10
    push r11 // r11_saved (RFlAGS, but also general arg)
    
    // Callee-saved
    push rbx
    push rbp
    push r12
    push r13
    push r14
    push r15
    
    // Scratch (Syscall Arguments passed in registers)
    // Syscall ABI: RDI, RSI, RDX, R10, R8, R9.
    // SyscallRegs Layout (top down):
    // r11, r10, r9, r8, rcx, rdx, rsi, rdi, rax
    
    push r11
    push r10
    push r9
    push r8
    push rcx
    push rdx
    push rsi
    push rdi
    push rax
    
    // rsp points to SyscallRegs.
    mov rdi, rsp
    
    call syscall_handler_rust
    
    // Return value is in rax. Write it to stack slot for pop rax.
    mov [rsp], rax
    
    // Restore scratch
    pop rax
    pop rdi
    pop rsi
    pop rdx
    pop rcx
    pop r8
    pop r9
    pop r10
    pop r11
    
    // Restore callee-saved
    pop r15
    pop r14
    pop r13
    pop r12
    pop rbp
    pop rbx
    
    // Cleanup saved args (9 regs * 8 = 72 bytes)
    add rsp, 72
    
    // Now stack points to IRETQ frame compatible with user mode return
    iretq
"#);

unsafe extern "C" {
    pub fn syscall_handler_asm();
}


// ... (SyscallRegs struct and helpers as defined before) ...
#[repr(C)]
#[derive(Debug)]
pub struct SyscallRegs {
    pub rax: u64,
    pub rdi: u64, // Arg 1
    pub rsi: u64, // Arg 2
    pub rdx: u64, // Arg 3
    pub rcx: u64, // Arg 4
    pub r8: u64,  // Arg 5
    pub r9: u64,  // Arg 6
    pub r10: u64, // Used by linux syscalls as 4th arg
    pub r11: u64, // RFLAGS
    // ... saved regs ...
    pub r15: u64,
    pub r14: u64,
    pub r13: u64,
    pub r12: u64,
    pub rbp: u64,
    pub rbx: u64,
    pub r11_saved: u64,
    pub r10_saved: u64,
    pub r9_saved: u64,
    pub r8_saved: u64,
    pub rcx_saved: u64,
    pub rdx_saved: u64,
    pub rsi_saved: u64,
    pub rdi_saved: u64,
    pub rax_saved: u64,
}

unsafe fn user_slice<'a, T>(ptr: u64, len: u64) -> &'a [T] {
    if ptr == 0 || len == 0 { return &[]; }
    slice::from_raw_parts(ptr as *const T, len as usize)
}

unsafe fn user_ptr_val<'a, T>(ptr: u64) -> Option<&'a T> {
    if ptr == 0 { None } else { Some(&*(ptr as *const T)) }
}

unsafe fn user_ptr_mut<'a, T>(ptr: u64) -> Option<&'a mut T> {
    if ptr == 0 { None } else { Some(&mut *(ptr as *mut T)) }
}

fn convert_prop(wire: &WireProp) -> (SymbolId, PropValue) {
    let key = wire.key;
    let val = match wire.value.tag {
        t if t == WireValueTag::U64 as u8 => PropValue::U64(wire.value.data_0),
        t if t == WireValueTag::I64 as u8 => PropValue::I64(wire.value.data_0 as i64),
        t if t == WireValueTag::Bool as u8 => PropValue::Bool(wire.value.data_0 != 0),
        t if t == WireValueTag::Str as u8 => {
            let sym_id = SymbolId(wire.value.data_0 as u32);
            if let Some(s) = symbols::resolve(sym_id) {
                PropValue::Str(s)
            } else {
                PropValue::Str(String::from("???"))
            }
        },
        t if t == WireValueTag::Blob as u8 => {
            let slice = unsafe { user_slice::<u8>(wire.value.data_0, wire.value.data_1) };
            PropValue::Blob(Vec::from(slice))
        },
        _ => PropValue::U64(0),
    };
    (key, val)
}

macro_rules! dispatch_syscall {
    (SYSCALL_LOG, $regs:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr) => {
        {
            let ptr = $a1;
            let len = $a2;
            let slice = unsafe { user_slice::<u8>(ptr, len) };
            if let Ok(s) = core::str::from_utf8(slice) {
                kernel::log(s);
            }
            0
        }
    };
    (SYSCALL_YIELD, $regs:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr) => {
        {
           kernel::sched::with_scheduler(|sched| {
                if let Some(tid) = sched.current_id() {
                    if let Some(thread) = sched.thread_mut(tid) {
                         thread.started = true;
                    }
                }
           });
           kernel::sched::yield_current_thread();
           crate::user::schedule_next();
           0
        }
    };
    (SYSCALL_EXIT_THREAD, $regs:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr) => {
        {
            kernel::sched::exit_current_thread("syscall", $a1);
            crate::user::schedule_next();
            0
        }
    };
    (SYSCALL_SLEEP_FOR_NS, $regs:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr) => {
        {
             crate::user::sys_sleep_for_ns($a1);
             0 
        }
    };
    (SYSCALL_SPAWN_PROGRAM, $regs:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr) => {
        {
             let boot_program_id = ThingId($a1);
             let req = abi::KernelRequest::SpawnProgram { boot_program_id };
             match kernel::handle_request(req) {
                 abi::KernelResponse::ProgramSpawned { process_id, thread_id } => {
                     if let Some(res) = unsafe { user_ptr_mut::<abi::wire::process::SpawnProgramResult>($a2) } {
                         res.process_id = process_id;
                         res.thread_id = thread_id;
                     }
                     0
                 },
                 _ => 1
             }
        }
    };
    (SYSCALL_SYMBOL_INTERN, $regs:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr) => {
        {
             let mut ret = 1;
             unsafe {
                 if let Some(req) = user_ptr_val::<SymbolInternReq>($a1) {
                     let slice = user_slice::<u8>(req.s.ptr, req.s.len);
                     if let Ok(s) = core::str::from_utf8(slice) {
                         let id = symbols::intern(s);
                         if let Some(resp) = user_ptr_mut::<SymbolInternResp>($a2) {
                             resp.id = id;
                             ret = 0;
                         }
                     }
                 }
             }
             ret
        }
    };
    (SYSCALL_SYMBOL_RESOLVE, $regs:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr) => {
        {
             let mut ret: u64 = 1;
             unsafe {
                 if let Some(req) = user_ptr_val::<SymbolResolveReq>($a1) {
                     if let Some(resp) = user_ptr_mut::<SymbolResolveResp>($a2) {
                         if let Some(s) = symbols::resolve(req.id) {
                              let out_slice = slice::from_raw_parts_mut(req.out_ptr as *mut u8, req.out_cap as usize);
                              let len = core::cmp::min(s.len(), out_slice.len());
                              out_slice[..len].copy_from_slice(&s.as_bytes()[..len]);
                              resp.written = len as u64;
                              ret = 0;
                         }
                     }
                 }
             }
             ret
        }
    };
    (SYSCALL_THING_CREATE, $regs:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr) => {
        {
             let kind = SymbolId($a1 as u32);
             let props_ptr = $a2;
             let props_len = $a3;
             
             let wire_props = unsafe { user_slice::<WireProp>(props_ptr, props_len) };
             let mut kernel_props = Vec::with_capacity(wire_props.len());
             for wp in wire_props {
                 kernel_props.push(convert_prop(wp));
             }
             
             let id = graph::create_thing(kind, kernel_props);
             id.0
        }
    };
    (SYSCALL_THING_UPDATE, $regs:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr) => {
        {
            let id = ThingId($a1);
            let props_ptr = $a2;
            let props_len = $a3;
             
             let wire_props = unsafe { user_slice::<WireProp>(props_ptr, props_len) };
             let mut kernel_props = Vec::with_capacity(wire_props.len());
             for wp in wire_props {
                 kernel_props.push(convert_prop(wp));
             }
             
             graph::update_thing(id, kernel_props);
             0
        }
    };
    (SYSCALL_THING_LIST, $regs:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr) => {
        {
             let kind = SymbolId($a1 as u32);
             let start_after = ThingId($a2);
             if let Some(id) = graph::next_thing_of_kind_sym(kind, start_after) {
                  id.0
             } else {
                  u64::MAX
             }
        }
    };
    (SYSCALL_THING_GET, $regs:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr) => {
        {
             let id = ThingId($a1);
             if let Some(out) = unsafe { user_ptr_mut::<abi::ThingGetSyscallResult>($a2) } {
                 let found = kernel::graph::with_thing(id, |node| {
                     if let Some(k) = kernel::symbols::resolve(node.kind) {
                         let bytes = k.as_bytes();
                         let len = core::cmp::min(bytes.len(), abi::THING_GET_MAX_KIND_LEN);
                         out.kind[..len].copy_from_slice(&bytes[..len]);
                         out.kind_len = len;
                     }

                     let mut count = 0;
                     for (key_sym, val) in &node.props {
                         if count >= abi::THING_GET_MAX_PROPS { break; }
                         if let Some(key_str) = kernel::symbols::resolve(*key_sym) {
                             let mut ent = abi::ThingPropData::default();
                             
                             let kbytes = key_str.as_bytes();
                             let klen = core::cmp::min(kbytes.len(), abi::THING_GET_MAX_STR_LEN);
                             ent.key[..klen].copy_from_slice(&kbytes[..klen]);
                             ent.key_len = klen;
                             ent.present = 1;

                             match val {
                                 PropValue::U64(v) => {
                                     ent.value_type = abi::ThingPropScalarType::U64;
                                     ent.value_u64 = *v;
                                 },
                                 PropValue::I64(v) => {
                                     ent.value_type = abi::ThingPropScalarType::I64;
                                     ent.value_i64 = *v;
                                 },
                                 PropValue::Bool(v) => {
                                     ent.value_type = abi::ThingPropScalarType::Bool;
                                     ent.value_bool = if *v { 1 } else { 0 };
                                 },
                                 PropValue::Str(s) => {
                                     ent.value_type = abi::ThingPropScalarType::Str;
                                     let sbytes = s.as_bytes();
                                     let slen = core::cmp::min(sbytes.len(), abi::THING_GET_MAX_STR_LEN);
                                     ent.value_str[..slen].copy_from_slice(&sbytes[..slen]);
                                     ent.value_str_len = slen;
                                 },
                                 _ => continue,
                             }
                             out.props[count] = ent;
                             count += 1;
                         }
                     }
                     out.prop_count = count;
                     0
                 });
                 found.unwrap_or(1)
             } else {
                 1
             }
        }
    };
    (SYSCALL_SCHEMA_REGISTER_PACKAGE, $regs:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr) => {
        {
            let kind = SymbolId($a1 as u32);
            let desc = SymbolId($a2 as u32);
            let props_ptr = $a3;
            let props_len = $a4;

            let req = abi::KernelRequest::SchemaRegisterPackage {
                kind,
                description: desc,
                props: UserSlice { 
                    ptr: props_ptr, 
                    len: props_len,
                    _phantom: core::marker::PhantomData
                },
            };

            match kernel::handle_request(req) {
                abi::KernelResponse::SchemaRegistered { outcome, .. } => outcome as u64,
                _ => 3,
            }
        }
    };
    (SYSCALL_ADD_LINK, $regs:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr) => {
        {
             let src = ThingId($a1);
             let pred = abi::Predicate($a2);
             let dst = ThingId($a3);
             
             if kernel::graph::add_link(src, pred, dst) {
                 0
             } else {
                 1
             }
        }
    };
    (SYSCALL_LINK_AT, $regs:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr) => {
        {
             let src = ThingId($a1);
             let idx = $a2 as usize;
             let pred = abi::Predicate($a3);
             
             if let Some(target) = kernel::graph::link_target_at(src, pred, idx) {
                 target.0
             } else {
                 u64::MAX
             }
        }
    };
    (SYSCALL_MAP_SHARED_BUFFER, $regs:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr) => {
        {
             let id = ThingId($a1);
             let flags = MapFlags($a2);
             
             let pid = match kernel::sched::SCHEDULER.lock().current_process_id() {
                 Some(p) => p,
                 None => return 1,
             };

             let manager = kernel::shared_buffer::manager().lock();
             if let Some(sb) = manager.get(&id) {
                 let size_bytes = sb.size_bytes();
                 let size_aligned = kernel::shared_buffer::align_up(size_bytes, 4096);
                 let frames = sb.frames.clone();
                 drop(manager);

                 if let Some(vaddr) = kernel::sched::SCHEDULER.lock().reserve_resident_region(pid, size_aligned as usize, 4096) {
                      if let Ok(_) = kernel::shared_buffer::map_frames_into_current_as(vaddr as u64, &frames, flags) {
                           if let Some(v_out) = unsafe { user_ptr_mut::<u64>($a3) } { *v_out = vaddr as u64; }
                           if let Some(s_out) = unsafe { user_ptr_mut::<u64>($a4) } { *s_out = size_bytes; }
                           0
                      } else { 1 }
                 } else { 1 }
             } else { 1 }
        }
    };
    (SYSCALL_CREATE_SHARED_BUFFER, $regs:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr) => {
        {
             let width = $a1 as u32;
             let height = $a2 as u32;
             let format = unsafe { core::mem::transmute($a3 as u8) };
             let stride = width * 4;
             let size = (stride * height) as u64;
             let pages = kernel::shared_buffer::page_count_for_size(size);
             
             let mut frames = heapless::Vec::new();
             let mut success = true;
             for _ in 0..pages {
                 if let Some(f) = kernel::memory::allocate_frame() {
                     if frames.push(f).is_err() {
                         kernel::memory::free_frame(f);
                         success = false;
                         break;
                     }
                 } else {
                     success = false;
                     break;
                 }
             }

             if !success {
                 for frame in frames {
                     kernel::memory::free_frame(frame);
                 }
                 return u64::MAX;
             }
             
             match kernel::shared_buffer::register_shared_buffer(width, height, stride, format, frames) {
                 Ok(id) => id.0,
                 Err(_) => u64::MAX,
             }
        }
    };
    (SYSCALL_GET_SHARED_BUFFER_INFO, $regs:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr) => {
        {
              let id = ThingId($a1);
              let manager = kernel::shared_buffer::manager().lock();
              if let Some(sb) = manager.get(&id) {
                  let info = sb.info();
                  drop(manager);
                  
                  if let Some(out) = unsafe { user_ptr_mut::<abi::SharedBufferInfo>($a2) } {
                      *out = info;
                      0
                  } else { 1 }
              } else { 1 }
        }
    };
    (SYSCALL_RESIDENT_ALLOC, $regs:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr) => {
        {
             let kind = SymbolId($a1 as u32);
             let byte_len = $a2;
             
             let args = abi::resident::ResidentAllocArgs {
                 kind_id: ThingId(kind.0 as u64),
                 byte_len: byte_len as u32,
                 flags: 0,
             };
             
             match kernel::resident::manager::sys_resident_alloc(args) {
                 Ok(resp) => {
                      if let Some(out) = unsafe { user_ptr_mut($a3) } {
                          *out = resp;
                          0
                      } else { 1 }
                 },
                 Err(e) => {
                      if let Some(out) = unsafe { user_ptr_mut($a4) } {
                          *out = e;
                      }
                      1
                 }
             }
        }
    };
    (SYSCALL_RESIDENT_MAP, $regs:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr) => {
        {
             let id = ThingId($a1);
             let perms = abi::resident::ResidentMapPerms($a2 as u32);
             
             let args = abi::resident::ResidentMapArgs {
                 id,
                 perms,
             };
             
             match kernel::resident::manager::sys_resident_map(args) {
                 Ok(resp) => {
                      if let Some(out) = unsafe { user_ptr_mut($a3) } {
                          *out = resp;
                          0
                      } else { 1 }
                 },
                 Err(e) => {
                      if let Some(out) = unsafe { user_ptr_mut($a4) } {
                          *out = e;
                      }
                      1
                 }
             }
        }
    };
    (SYSCALL_RESIDENT_UNMAP, $regs:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr) => {
        {
             let id = ThingId($a1);
             match kernel::resident::manager::sys_resident_unmap(id) {
                 Ok(_) => 0,
                 Err(e) => {
                      if let Some(out) = unsafe { user_ptr_mut($a2) } {
                          *out = e;
                      }
                      1
                 }
             }
        }
    };
    (SYSCALL_THING_REST, $regs:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr) => {
        {
             let id = ThingId($a1);
             let policy = unsafe { core::mem::transmute($a2 as u32) };
             match kernel::resident::manager::sys_thing_rest(id, policy) {
                 Ok(resp) => {
                      if let Some(out) = unsafe { user_ptr_mut($a3) } {
                          *out = resp;
                          0
                      } else { 1 }
                 },
                 Err(e) => {
                      if let Some(out) = unsafe { user_ptr_mut($a4) } {
                          *out = e;
                      }
                      1
                 }
             }
        }
    };
    (SYSCALL_DEV_OPEN, $regs:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr) => {
        {
             if let Some(args) = unsafe { user_ptr_val::<abi::syscall_defs::DevOpenArgs>($a1) } {
                 let res = kernel::bridge::ps2::dev_open(unsafe { core::mem::transmute(args.kind) }, args.index);
                  if let Some(ret_ref) = unsafe { user_ptr_mut($a2) } {
                      *ret_ref = match res {
                          Ok(handle) => abi::syscall_defs::SysRet::ok(abi::syscall_defs::DevOpenRet { handle }),
                          Err(e) => abi::syscall_defs::SysRet::err(e.code, e.detail),
                      };
                      0
                  } else { 1 }
             } else {
                 1
             }
        }
    };
    (SYSCALL_DEV_READ, $regs:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr) => {
        {
            if let Some(args) = unsafe { user_ptr_val::<abi::syscall_defs::DevReadArgs>($a1) } {
                if let Some(ret_ref) = unsafe { user_ptr_mut::<abi::syscall_defs::SysRet<abi::syscall_defs::DevReadRet>>($a2) } {
                     let buffer_ptr = args.out.ptr as *mut u8;
                     let buffer_len = args.out.len as usize;
                     if buffer_ptr.is_null() {
                          unsafe { *ret_ref = abi::syscall_defs::SysRet::err(abi::syscall_defs::SysError::INVALID_ARG, 0); }
                          0
                     } else {
                         let buffer = unsafe { core::slice::from_raw_parts_mut(buffer_ptr, buffer_len) };
                         let res = kernel::bridge::ps2::dev_read(args.handle, buffer);

                         match res {
                              Ok(bytes_read) => {
                                   unsafe { *ret_ref = abi::syscall_defs::SysRet::ok(abi::syscall_defs::DevReadRet { bytes_read: bytes_read as u32 }); }
                                   0
                              },
                              Err(e) if e.code == abi::syscall_defs::SysError::WOULD_BLOCK => {
                                   {
                                       kernel::sched::with_scheduler(|sched| {
                                            if let Some(tid) = sched.current_id() {
                                                 if let Some(thread) = sched.thread_mut(tid) {
                                                      thread.started = true;
                                                      if sched.mark_blocked(tid) {
                                                      } else {
                                                      }
                                                 }
                                            }
                                       });
                                   }
                                   crate::user::schedule_next();
                                   unsafe { *ret_ref = abi::syscall_defs::SysRet::ok(abi::syscall_defs::DevReadRet { bytes_read: 0 }); }
                                   0
                              },
                              Err(e) => {
                                   unsafe { *ret_ref = abi::syscall_defs::SysRet::err(e.code, e.detail); }
                                   0
                              }
                         }
                     }
                } else { 1 }
            } else { 1 }
        }
    };

    // Fallback for missing syscalls (Stubs)
    ($name:ident, $regs:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr) => {
        u64::MAX
    }
}


macro_rules! collect_dispatched_numbers {
    ($($name:ident => $num:expr),* $(,)?) => {
        pub const DISPATCHED_SYSCALL_NUMBERS: &[u64] = &[ $($num),* ];
    };
}

abi::for_each_syscall!(collect_dispatched_numbers);

#[unsafe(no_mangle)]
pub extern "C" fn syscall_handler_rust(regs: *mut SyscallRegs) -> u64 {
    let regs = unsafe { &mut *regs };
    let num = regs.rax_saved;

    let arg1 = regs.rdi_saved;
    let arg2 = regs.rsi_saved;
    let arg3 = regs.rdx_saved;
    let arg4 = regs.r10_saved;
    let arg5 = regs.r8_saved;
    let arg6 = regs.r9_saved;

    macro_rules! dispatch_helper {
        ($($name:ident => $num:expr),* $(,)?) => {
            match num {
                $($name => dispatch_syscall!($name, regs, arg1, arg2, arg3, arg4, arg5, arg6),)*
                _ => u64::MAX
            }
        };
    }

    abi::for_each_syscall!(dispatch_helper)
}

pub fn install_handler() {
    use x86_64::registers::model_specific::{LStar, SFMask, Star};
    use x86_64::structures::gdt::SegmentSelector;
    use x86_64::VirtAddr;
    use crate::gdt;

    // Set LStar to handler
    let handler_addr = VirtAddr::new(syscall_handler_asm as *const () as u64);
    unsafe { LStar::write(handler_addr); }

    use x86_64::registers::rflags::RFlags;
    unsafe { SFMask::write(RFlags::INTERRUPT_FLAG | RFlags::TRAP_FLAG); }

    let selectors = gdt::get_selectors();
    let kernel_code_sel = selectors.kcode;
    let kernel_data_sel = selectors.kdata;
    let user_code_sel = selectors.ucode;
    let user_data_sel = selectors.udata;

    unsafe {
        // Init global statics
        SYSCALL_KERNEL_RSP = gdt::kernel_stack_top();
        SYSCALL_USER_CS = user_code_sel.0 as u64 | 3;
        SYSCALL_USER_SS = user_data_sel.0 as u64 | 3;
        
        let krsp = SYSCALL_KERNEL_RSP;
        let ucs = SYSCALL_USER_CS;
        let uss = SYSCALL_USER_SS;
        
        kernel::println!(
            "Syscall setup: KRSP={:#x} CS={:#x} SS={:#x}",
            krsp,
            ucs,
            uss
        );

        Star::write(
            user_code_sel,
            user_data_sel,
            kernel_code_sel,
            kernel_data_sel
        ).unwrap();

        // Enable Syscall Extensions (EFER.SCE)
        use x86_64::registers::model_specific::{Efer, EferFlags};
        Efer::update(|f| f.insert(EferFlags::SYSTEM_CALL_EXTENSIONS));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_syscall_dispatch_coverage() {
        let abi_nums = abi::syscalls::ABI_SYSCALL_NUMBERS;
        let dispatched_nums = DISPATCHED_SYSCALL_NUMBERS;

        assert_eq!(abi_nums.len(), dispatched_nums.len(), "Mismatch in syscall count between ABI and Dispatch");

        for (i, &num) in abi_nums.iter().enumerate() {
            assert_eq!(num, dispatched_nums[i], "Mismatch at index {}", i);
        }
    }
}
