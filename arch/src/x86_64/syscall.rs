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
        SYSCALL_THING_GET => {
             let id = ThingId(arg1);
             // arg2 is pointer to ThingGetSyscallResult
             if let Some(out) = unsafe { user_ptr_mut::<abi::ThingGetSyscallResult>(arg2) } {
                 let found = kernel::graph::with_thing(id, |node| {
                     // 1. Fill kind
                     if let Some(k) = kernel::symbols::resolve(node.kind) {
                         let bytes = k.as_bytes();
                         let len = core::cmp::min(bytes.len(), abi::THING_GET_MAX_KIND_LEN);
                         out.kind[..len].copy_from_slice(&bytes[..len]);
                         out.kind_len = len;
                     }

                     // 2. Fill props (limit to MAX_PROPS)
                     let mut count = 0;
                     for (key_sym, val) in &node.props {
                         if count >= abi::THING_GET_MAX_PROPS { break; }
                         if let Some(key_str) = kernel::symbols::resolve(*key_sym) {
                             let mut ent = abi::ThingPropData::default();
                             
                             // Key
                             let kbytes = key_str.as_bytes();
                             let klen = core::cmp::min(kbytes.len(), abi::THING_GET_MAX_STR_LEN);
                             ent.key[..klen].copy_from_slice(&kbytes[..klen]);
                             ent.key_len = klen;
                             ent.present = 1;

                             // Value
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
                                 _ => continue, // Skip unsupported types for now
                             }
                             out.props[count] = ent;
                             count += 1;
                         }
                     }
                     out.prop_count = count;
                     0 // Success
                 });
                 found.unwrap_or(1) // 1 if thing not found
             } else {
                 1 // Invalid pointer
             }
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
