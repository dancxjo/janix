extern crate alloc;
use abi::syscalls::*;
use abi::wire::common::{UserPtr, UserSlice};
use abi::wire::graph::{WireProp, WireSchemaProp, WirePropValue, WireValueTag, BatchUpdateReq};
use abi::syscall_defs::{SymbolId, SysRet, SymbolInternReq, SymbolInternResp, SymbolResolveReq, SymbolResolveResp};
use abi::{PropValue, PropType, ThingId, ProcessId, MapFlags};
use kernel::graph;
use kernel::symbols;
use alloc::vec::Vec;
use alloc::string::String;
use alloc::boxed::Box;
use core::slice;

use core::arch::global_asm;

global_asm!(r#"
.global syscall_handler_asm
syscall_handler_asm:
    // We are on Kernel Stack (via TSS/IDT).
    // Push order matching SyscallRegs (reversed)
    
    // 1. Saved regs (Deepest)
    push rax // rax_saved
    push rdi // rdi_saved
    push rsi // rsi_saved
    push rdx // rdx_saved
    push rcx // rcx_saved
    push r8  // r8_saved
    push r9  // r9_saved
    push r10 // r10_saved
    push r11 // r11_saved
    
    // 2. Callee-saved
    push rbx
    push rbp
    push r12
    push r13
    push r14
    push r15
    
    // 3. Scratch/Dup regs (Shallowest)
    push r11
    push r10
    push r9
    push r8
    push rcx
    push rdx
    push rsi
    push rdi
    push rax
    
    // rsp points to SyscallRegs.rax
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
    
    // Cleanup saved args (9 regs * 8 bytes = 72)
    add rsp, 72
    
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

    match num {
        SYSCALL_LOG => {
            let ptr = arg1;
            let len = arg2;
            let slice = unsafe { user_slice::<u8>(ptr, len) };
            if let Ok(s) = core::str::from_utf8(slice) {
                kernel::log(s);
            }
            0
        }
        SYSCALL_YIELD => {
            {
               kernel::sched::with_scheduler(|sched| {
                    if let Some(tid) = sched.current_id() {
                        if let Some(thread) = sched.thread_mut(tid) {
                             thread.started = true;
                        }
                    }
               });
            }
            kernel::sched::yield_current_thread();
            crate::user::schedule_next();
            0 
        }
        SYSCALL_EXIT_THREAD => {
            kernel::sched::exit_current_thread("syscall", arg1);
            crate::user::schedule_next();
            0
        }
        SYSCALL_SLEEP_FOR_NS => { 
             crate::user::sys_sleep_for_ns(arg1);
             0 
        }
        SYSCALL_SPAWN_PROGRAM => {
             let boot_program_id = ThingId(arg1);
             let req = abi::KernelRequest::SpawnProgram { boot_program_id };
             match kernel::handle_request(req) {
                 abi::KernelResponse::ProgramSpawned { process_id, thread_id } => {
                     if let Some(res) = unsafe { user_ptr_mut::<abi::wire::process::SpawnProgramResult>(arg2) } {
                         res.process_id = process_id;
                         res.thread_id = thread_id;
                     }
                     0
                 },
                 _ => 1
             }
        }
        SYSCALL_SYMBOL_INTERN => {
             unsafe {
                 if let Some(req) = user_ptr_val::<SymbolInternReq>(arg1) {
                     let slice = user_slice::<u8>(req.s.ptr, req.s.len);
                     if let Ok(s) = core::str::from_utf8(slice) {
                         let id = symbols::intern(s);
                         if let Some(resp) = user_ptr_mut::<SymbolInternResp>(arg2) {
                             resp.id = id;
                             return 0;
                         }
                     }
                 }
             }
             1 
        }
        SYSCALL_SYMBOL_RESOLVE => {
             unsafe {
                 if let Some(req) = user_ptr_val::<SymbolResolveReq>(arg1) {
                     if let Some(resp) = user_ptr_mut::<SymbolResolveResp>(arg2) {
                         if let Some(s) = symbols::resolve(req.id) {
                              let out_slice = slice::from_raw_parts_mut(req.out_ptr as *mut u8, req.out_cap as usize);
                              let len = core::cmp::min(s.len(), out_slice.len());
                              out_slice[..len].copy_from_slice(&s.as_bytes()[..len]);
                              resp.written = len as u64;
                              return 0;
                         }
                     }
                 }
             }
             1
        }
        SYSCALL_THING_CREATE => {
             let kind = SymbolId(arg1 as u32);
             let props_ptr = arg2;
             let props_len = arg3;
             
             let wire_props = unsafe { user_slice::<WireProp>(props_ptr, props_len) };
             let mut kernel_props = Vec::with_capacity(wire_props.len());
             for wp in wire_props {
                 kernel_props.push(convert_prop(wp));
             }
             
             let id = graph::create_thing(kind, kernel_props);
             id.0
        }
        SYSCALL_THING_UPDATE => {
            let id = ThingId(arg1);
            let props_ptr = arg2;
            let props_len = arg3;
             
             let wire_props = unsafe { user_slice::<WireProp>(props_ptr, props_len) };
             let mut kernel_props = Vec::with_capacity(wire_props.len());
             for wp in wire_props {
                 kernel_props.push(convert_prop(wp));
             }
             
             graph::update_thing(id, kernel_props);
             0
        }
        SYSCALL_THING_LIST => {
             let kind = SymbolId(arg1 as u32);
             let start_after = ThingId(arg2);
             if let Some(id) = graph::next_thing_of_kind_sym(kind, start_after) {
                  return id.0;
             }
             u64::MAX
        }
        SYSCALL_DEV_OPEN => {
             if let Some(args) = unsafe { user_ptr_val::<abi::syscall_defs::DevOpenArgs>(arg1) } {
                 let res = kernel::bridge::ps2::dev_open(unsafe { core::mem::transmute(args.kind) }, args.index);
                  if let Some(ret_ref) = unsafe { user_ptr_mut(arg2) } {
                      *ret_ref = match res {
                          Ok(handle) => abi::syscall_defs::SysRet::ok(abi::syscall_defs::DevOpenRet { handle }),
                          Err(e) => abi::syscall_defs::SysRet::err(e.code, e.detail),
                      };
                      return 0;
                  }
             }
             1
        }
        SYSCALL_DEV_READ => {
            if let Some(args) = unsafe { user_ptr_val::<abi::syscall_defs::DevReadArgs>(arg1) } {
                if let Some(ret_ref) = unsafe { user_ptr_mut::<abi::syscall_defs::SysRet<abi::syscall_defs::DevReadRet>>(arg2) } {
                     // Get buffer
                     let buffer_ptr = args.out.ptr.addr as *mut u8;
                     let buffer_len = args.out.len as usize;
                     if buffer_ptr.is_null() {
                          unsafe { *ret_ref = abi::syscall_defs::SysRet::err(abi::syscall_defs::SysError::INVALID_ARG, 0); }
                          return 0;
                     }
                     let buffer = unsafe { core::slice::from_raw_parts_mut(buffer_ptr, buffer_len) };
                     let res = kernel::bridge::ps2::dev_read(args.handle, buffer);
                     
                     match res {
                          Ok(bytes_read) => {
                               unsafe { *ret_ref = abi::syscall_defs::SysRet::ok(abi::syscall_defs::DevReadRet { bytes_read: bytes_read as u32 }); }
                               0
                          },
                          Err(e) if e.code == abi::syscall_defs::SysError::WOULD_BLOCK => {
                               // Block logic:
                               {
                                   kernel::sched::with_scheduler(|sched| {
                                        if let Some(tid) = sched.current_id() {
                                             if let Some(thread) = sched.thread_mut(tid) {
                                                  thread.started = true;
                                                  if sched.mark_blocked(tid) {
                                                      // Blocked
                                                  } else {
                                                      // Race condition won, return 0 bytes read for now (loop in userland)
                                                  }
                                             }
                                        }
                                   });
                               }
                               crate::user::schedule_next();
                               // After resume, we still need to return something? 
                               // Or does schedule_next not return until we are woken?
                               // If we were woken, it means data is available or signal.
                               // We should probably just return WOULD_BLOCK again or 0 if we assume read retry.
                               // Returning 0 for bytes_read works if logic handles it.
                               // But typically we should just restart the syscall or return success.
                               // For simplicity, we return "Ok(0 bytes)" to indicate wake-up-and-retry.
                               // Actually, let's just return WOULD_BLOCK if we failed to block?
                               // If mark_blocked returns false, it means we consumed a wake event.
                               // So we should retry immediately.
                               // But we can't retry kernel call from here easily.
                               // Return 0 bytes read.
                               unsafe { *ret_ref = abi::syscall_defs::SysRet::ok(abi::syscall_defs::DevReadRet { bytes_read: 0 }); }
                               0
                          },
                          Err(e) => {
                               unsafe { *ret_ref = abi::syscall_defs::SysRet::err(e.code, e.detail); }
                               0
                          }
                     }
                } else { 1 }
            } else { 1 }
        }
        _ => {
            u64::MAX 
        }
    }
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
    // SYSRET: CS = base+16 (UserCode), SS = base+8 (UserData).
    // GDT Order: ..., UData, UCode.
    // So base = UData - 8.
    // UData selector (u16).
    let udata_val = selectors.udata.0;
    let user_base_sel = SegmentSelector(udata_val - 8);

    unsafe {
        Star::write(
            user_base_sel,
            user_base_sel,
            kernel_code_sel,
            kernel_data_sel
        ).unwrap();
    }
}
