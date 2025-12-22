extern crate alloc;
use abi::syscall_defs::{
    SymbolId, SymbolInternReq, SymbolInternResp, SymbolResolveReq, SymbolResolveResp, SysRet,
};
use abi::syscalls::*;
use abi::wire::common::{UserPtr, UserSlice};
use abi::wire::graph::{
    BatchUpdateEntry, BatchUpdateReq, WireProp, WirePropValue, WireSchemaProp, WireValueTag,
};
use abi::{MapFlags, ProcessId, ThingId, TransactionId};
use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;
use core::slice;
use kernel::graph;
use kernel::memory;
use kernel::shared_buffer;
use kernel::symbols;
use kernel::transaction;
use thing_models::{PropType, PropValue};

use core::arch::global_asm;

global_asm!(
    r#"
.global syscall_handler_asm
syscall_handler_asm:
    // 1. Swap to Kernel GS to access PerCpu
    swapgs
    
    // 2. Save User RSP to PerCpu.user_rsp (offset 8)
    mov gs:[8], rsp
    
    // 3. Load Kernel RSP from PerCpu.kernel_rsp (offset 0)
    mov rsp, gs:[0]

    // 4. Push User RSP onto Kernel Stack immediately (Preserve against preemption)
    push qword ptr gs:[8]
    
    // 5. Save User context (RIP, RFLAGS are in RCX, R11)
    push rcx // User RIP
    push r11 // User RFLAGS
    
    // 6. Save Callee-Saved Registers (RbP, RBX, R12-R15)
    push rbp
    push rbx
    push r12
    push r13
    push r14
    push r15
    
    // 7. Shuffle arguments for Rust ABI (System V AMD64)
    // Syscall ABI: RAX(num), RDI(a1), RSI(a2), RDX(a3), R10(a4), R8(a5), R9(a6)
    // Rust Call:   RDI(num), RSI(a1), RDX(a2), RCX(a3), R8 (a4), R9(a5), Stack(a6)
    
    // Argument 6 (R9) -> Stack
    push r9
    
    // Shuffle Registers
    mov r9, r8   // a5 -> r9
    mov r8, r10  // a4 -> r8
    mov rcx, rdx // a3 -> rcx
    mov rdx, rsi // a2 -> rdx
    mov rsi, rdi // a1 -> rsi
    mov rdi, rax // num -> rdi
    
    // Alignment (Total pushes: 1(RSP)+2(RIP/FL)+6(Callee)+1(Arg6) = 10 qwords.
    // RSP aligned (16-byte) at start? 
    // Wait. PerCpu.kernel_rsp is top of stack. Aligned 16.
    // 10 pushes = 80 bytes. Aligned 16.
    // So NO sub needed?
    // Let's verify. 80 is divisible by 16.
    // So RSP is aligned.
    
    call syscall_handler_rust
    
    // Cleanup Stack Arg (Arg6)
    add rsp, 8 
    
    // Result in RAX.
    
    // 8. Restore Callee-Saved
    pop r15
    pop r14
    pop r13
    pop r12
    pop rbx
    pop rbp
    
    // 9. Restore User RFLAGS, RIP
    pop r11 // User RFLAGS
    pop rcx // User RIP
    
    // 10. Restore User RSP from Stack
    pop rsp
    
    // 11. Swap GS back to User
    swapgs
    
    // 12. Return
    sysretq
"#
);

unsafe extern "C" {
    pub fn syscall_handler_asm();
}

unsafe fn user_slice<'a, T>(ptr: u64, len: u64) -> &'a [T] {
    if ptr == 0 || len == 0 {
        return &[];
    }
    slice::from_raw_parts(ptr as *const T, len as usize)
}

unsafe fn user_ptr_val<'a, T>(ptr: u64) -> Option<&'a T> {
    if ptr == 0 {
        None
    } else {
        Some(&*(ptr as *const T))
    }
}

unsafe fn user_ptr_mut<'a, T>(ptr: u64) -> Option<&'a mut T> {
    if ptr == 0 {
        None
    } else {
        Some(&mut *(ptr as *mut T))
    }
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
        }
        t if t == WireValueTag::Blob as u8 => {
            let slice = unsafe { user_slice::<u8>(wire.value.data_0, wire.value.data_1) };
            PropValue::Blob(Vec::from(slice))
        }
        _ => PropValue::U64(0),
    };
    (key, val)
}

macro_rules! dispatch_syscall {
    (SYSCALL_LOG, $regs:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr) => {{
        let ptr = $a1;
        let len = $a2;
        let slice = unsafe { user_slice::<u8>(ptr, len) };
        if let Ok(s) = core::str::from_utf8(slice) {
            kernel::log(s);
        }
        0
    }};
    (SYSCALL_YIELD, $regs:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr) => {{
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
    }};
    (SYSCALL_EXIT_THREAD, $regs:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr) => {{
        kernel::sched::exit_current_thread("syscall", $a1);
        crate::user::schedule_next();
        0
    }};
    (SYSCALL_SLEEP_FOR_NS, $regs:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr) => {{
        crate::user::sys_sleep_for_ns($a1);
        0
    }};
    (SYSCALL_SPAWN_PROGRAM, $regs:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr) => {{
        let boot_program_id = ThingId($a1);
        let req = abi::KernelRequest::SpawnProgram { boot_program_id };
        match kernel::handle_request(req) {
            abi::KernelResponse::ProgramSpawned {
                process_id,
                thread_id,
            } => {
                if let Some(res) =
                    unsafe { user_ptr_mut::<abi::wire::process::SpawnProgramResult>($a2) }
                {
                    res.process_id = process_id;
                    res.thread_id = thread_id;
                }
                0
            }
            _ => 1,
        }
    }};
    (SYSCALL_SYMBOL_INTERN, $regs:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr) => {{
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
    }};
    (SYSCALL_SYMBOL_RESOLVE, $regs:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr) => {{
        let mut ret: u64 = 1;
        unsafe {
            if let Some(req) = user_ptr_val::<SymbolResolveReq>($a1) {
                if let Some(resp) = user_ptr_mut::<SymbolResolveResp>($a2) {
                    if let Some(s) = symbols::resolve(req.id) {
                        let out_slice =
                            slice::from_raw_parts_mut(req.out_ptr as *mut u8, req.out_cap as usize);
                        let len = core::cmp::min(s.len(), out_slice.len());
                        out_slice[..len].copy_from_slice(&s.as_bytes()[..len]);
                        resp.written = len as u64;
                        ret = 0;
                    }
                }
            }
        }
        ret
    }};
    (SYSCALL_THING_CREATE, $regs:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr) => {{
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
    }};
    (SYSCALL_THING_UPDATE, $regs:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr) => {{
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
    }};
    (SYSCALL_THING_LIST, $regs:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr) => {{
        let kind = SymbolId($a1 as u32);
        let start_after = ThingId($a2);
        if let Some(id) = graph::next_thing_of_kind_sym(kind, start_after) {
            id.0
        } else {
            u64::MAX
        }
    }};
    (SYSCALL_SCHEMA_GET, $regs:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr) => {{
        let kind = SymbolId($a1 as u32);
        let out_ptr = $a2;
        let out_len = $a3;

        let req = abi::KernelRequest::SchemaGet {
            kind,
            out: UserSlice {
                ptr: out_ptr,
                len: out_len,
                _phantom: core::marker::PhantomData,
            },
        };

        match kernel::handle_request(req) {
            abi::KernelResponse::SchemaData {
                written,
                fingerprint,
            } => {
                // Write back outputs
                if let Some(w_ptr) = unsafe { user_ptr_mut::<u64>($a4) } {
                    *w_ptr = written;
                }
                if let Some(f_ptr) = unsafe { user_ptr_mut::<u64>($a5) } {
                    *f_ptr = fingerprint;
                }
                0
            }
            _ => 1,
        }
    }};
    (SYSCALL_THING_GET, $regs:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr) => {{
        let id = ThingId($a1);
        let out_ptr = $a2;
        let out_len = $a3;

        let found = kernel::graph::with_thing(id, |node| {
            let mut count = 0;
            if let Some(out_slice) = unsafe { user_ptr_mut::<WireProp>(out_ptr) }
                .map(|p| unsafe { slice::from_raw_parts_mut(p, out_len as usize) })
            {
                for (key_sym, val) in &node.props {
                    if count >= out_slice.len() {
                        break;
                    }

                    let wire_val = match val {
                        PropValue::U64(v) => WirePropValue::u64(*v),
                        PropValue::I64(v) => WirePropValue::i64(*v),
                        PropValue::Bool(v) => WirePropValue::bool(*v),
                        PropValue::Str(s) => WirePropValue::sym(kernel::symbols::intern(s)),
                        PropValue::Symbol(id) => WirePropValue::sym(*id),
                        PropValue::Blob(b) => {
                            WirePropValue::blob(b.as_ptr() as u64, b.len() as u64)
                        }
                    };

                    out_slice[count] = WireProp {
                        key: *key_sym,
                        value: wire_val,
                        _pad: 0,
                    };
                    count += 1;
                }
            }
            count as u64
        });
        found.unwrap_or(u64::MAX)
    }};
    (SYSCALL_SCHEMA_REGISTER_PACKAGE, $regs:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr) => {{
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
                _phantom: core::marker::PhantomData,
            },
        };

        match kernel::handle_request(req) {
            abi::KernelResponse::SchemaRegistered { outcome, .. } => outcome as u64,
            _ => 3,
        }
    }};
    (SYSCALL_ADD_LINK, $regs:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr) => {{
        let src = ThingId($a1);
        let pred = abi::Predicate($a2);
        let dst = ThingId($a3);

        if kernel::graph::add_link(src, pred, dst) {
            0
        } else {
            1
        }
    }};
    (SYSCALL_LINK_AT, $regs:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr) => {{
        let src = ThingId($a1);
        let idx = $a2 as usize;
        let pred = abi::Predicate($a3);

        if let Some(target) = kernel::graph::link_target_at(src, pred, idx) {
            target.0
        } else {
            u64::MAX
        }
    }};
    (SYSCALL_MAP_SHARED_BUFFER, $regs:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr) => {{
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

            if let Some(vaddr) = kernel::sched::SCHEDULER.lock().reserve_resident_region(
                pid,
                size_aligned as usize,
                4096,
            ) {
                if let Ok(_) =
                    kernel::shared_buffer::map_frames_into_current_as(vaddr as u64, &frames, flags)
                {
                    if let Some(v_out) = unsafe { user_ptr_mut::<u64>($a3) } {
                        *v_out = vaddr as u64;
                    }
                    if let Some(s_out) = unsafe { user_ptr_mut::<u64>($a4) } {
                        *s_out = size_bytes;
                    }
                    0
                } else {
                    1
                }
            } else {
                1
            }
        } else {
            1
        }
    }};
    (SYSCALL_CREATE_SHARED_BUFFER, $regs:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr) => {{
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
                    // Frame allocation succeeded but list is full.
                    // Free the current frame and abort to cleanup existing frames.
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
    }};
    (SYSCALL_GET_SHARED_BUFFER_INFO, $regs:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr) => {{
        let id = ThingId($a1);
        let manager = kernel::shared_buffer::manager().lock();
        if let Some(sb) = manager.get(&id) {
            let info = sb.info();
            drop(manager);

            if let Some(out) = unsafe { user_ptr_mut::<abi::SharedBufferInfo>($a2) } {
                *out = info;
                0
            } else {
                1
            }
        } else {
            1
        }
    }};
    (SYSCALL_RESIDENT_ALLOC, $regs:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr) => {{
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
                } else {
                    1
                }
            }
            Err(e) => {
                if let Some(out) = unsafe { user_ptr_mut($a4) } {
                    *out = e;
                }
                1
            }
        }
    }};
    (SYSCALL_RESIDENT_MAP, $regs:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr) => {{
        let id = ThingId($a1);
        let perms = abi::resident::ResidentMapPerms($a2 as u32);

        let args = abi::resident::ResidentMapArgs { id, perms };

        match kernel::resident::manager::sys_resident_map(args) {
            Ok(resp) => {
                if let Some(out) = unsafe { user_ptr_mut($a3) } {
                    *out = resp;
                    0
                } else {
                    1
                }
            }
            Err(e) => {
                if let Some(out) = unsafe { user_ptr_mut($a4) } {
                    *out = e;
                }
                1
            }
        }
    }};
    (SYSCALL_RESIDENT_UNMAP, $regs:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr) => {{
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
    }};
    (SYSCALL_THING_REST, $regs:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr) => {{
        let id = ThingId($a1);
        let policy = unsafe { core::mem::transmute($a2 as u32) };
        match kernel::resident::manager::sys_thing_rest(id, policy) {
            Ok(resp) => {
                if let Some(out) = unsafe { user_ptr_mut($a3) } {
                    *out = resp;
                    0
                } else {
                    1
                }
            }
            Err(e) => {
                if let Some(out) = unsafe { user_ptr_mut($a4) } {
                    *out = e;
                }
                1
            }
        }
    }};
    (SYSCALL_DEV_OPEN, $regs:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr) => {{
        if let Some(args) = unsafe { user_ptr_val::<abi::syscall_defs::DevOpenArgs>($a1) } {
            let res = kernel::bridge::ps2::dev_open(
                unsafe { core::mem::transmute(args.kind) },
                args.index,
            );
            if let Some(ret_ref) = unsafe { user_ptr_mut($a2) } {
                *ret_ref = match res {
                    Ok(handle) => {
                        abi::syscall_defs::SysRet::ok(abi::syscall_defs::DevOpenRet { handle })
                    }
                    Err(e) => abi::syscall_defs::SysRet::err(e.code, e.detail),
                };
                0
            } else {
                1
            }
        } else {
            1
        }
    }};
    (SYSCALL_DEV_READ, $regs:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr) => {{
        if let Some(args) = unsafe { user_ptr_val::<abi::syscall_defs::DevReadArgs>($a1) } {
            if let Some(ret_ref) = unsafe {
                user_ptr_mut::<abi::syscall_defs::SysRet<abi::syscall_defs::DevReadRet>>($a2)
            } {
                let buffer_ptr = args.out.ptr as *mut u8;
                let buffer_len = args.out.len as usize;
                if buffer_ptr.is_null() {
                    unsafe {
                        *ret_ref = abi::syscall_defs::SysRet::err(
                            abi::syscall_defs::SysError::INVALID_ARG,
                            0,
                        );
                    }
                    0
                } else {
                    let buffer = unsafe { core::slice::from_raw_parts_mut(buffer_ptr, buffer_len) };
                    let res = kernel::bridge::ps2::dev_read(args.handle, buffer);

                    match res {
                        Ok(bytes_read) => {
                            unsafe {
                                *ret_ref =
                                    abi::syscall_defs::SysRet::ok(abi::syscall_defs::DevReadRet {
                                        bytes_read: bytes_read as u32,
                                    });
                            }
                            0
                        }
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
                            unsafe {
                                *ret_ref =
                                    abi::syscall_defs::SysRet::ok(abi::syscall_defs::DevReadRet {
                                        bytes_read: 0,
                                    });
                            }
                            0
                        }
                        Err(e) => {
                            unsafe {
                                *ret_ref = abi::syscall_defs::SysRet::err(e.code, e.detail);
                            }
                            0
                        }
                    }
                }
            } else {
                1
            }
        } else {
            1
        }
    }};
    (SYSCALL_PCI_READ_CONFIG, $regs:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr) => {{
        if let Some(args) = unsafe { user_ptr_val::<abi::syscall_defs::PciReadConfigArgs>($a1) } {
            if let Some(ret_ref) =
                unsafe { user_ptr_mut::<abi::syscall_defs::PciReadConfigRet>($a2) }
            {
                let val = match args.width {
                    1 => crate::pci::read_config_u8(args.bus, args.slot, args.func, args.offset, 0)
                        .map(|v| v as u32),
                    2 => {
                        crate::pci::read_config_u16(args.bus, args.slot, args.func, args.offset, 0)
                            .map(|v| v as u32)
                    }
                    4 => {
                        crate::pci::read_config_u32(args.bus, args.slot, args.func, args.offset, 0)
                    }
                    _ => None,
                };

                if let Some(v) = val {
                    unsafe {
                        *ret_ref = abi::syscall_defs::PciReadConfigRet { value: v };
                    }
                    0
                } else {
                    1 // Error
                }
            } else {
                1
            }
        } else {
            1
        }
    }};
    (SYSCALL_SLEEP_UNTIL, $regs:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr) => {{
        kernel::sched::with_scheduler(|sched| {
            sched.sleep_current_thread($a1);
        });
        crate::user::schedule_next();
        0
    }};
    (SYSCALL_TIME_MONOTONIC_NS, $regs:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr) => {{
        kernel::time::monotonic_now_ns()
    }};
    (SYSCALL_TIME_SYSTEM_NS, $regs:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr) => {{
        kernel::time::system_time_ns().unwrap_or(0)
    }};
    (SYSCALL_TIME_NOW, $regs:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr) => {{
        let (sec, nanos) = kernel::time::now_unix_from_rtc();
        if let Some(sec_ptr) = unsafe { user_ptr_mut::<i64>($a1) } {
            *sec_ptr = sec;
        }
        if let Some(nanos_ptr) = unsafe { user_ptr_mut::<u32>($a2) } {
            *nanos_ptr = nanos;
        }
        0
    }};
    (SYSCALL_ALLOC_FRAME, $regs:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr) => {{
        if let Some(frame) = kernel::memory::allocate_frame() {
            if let Some(out) = unsafe { user_ptr_mut::<abi::wire::memory::FrameInfo>($a2) } {
                out.id = abi::FrameId(frame.start_address >> 12);
                out.base = frame.start_address;
                out.size = frame.size;
                0
            } else {
                1
            }
        } else {
            1
        }
    }};
    (SYSCALL_FREE_FRAME, $regs:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr) => {{
        let addr = $a1 << 12;
        let frame = unsafe {
             kernel::memory::PhysFrame { start_address: addr, size: 4096 }
        };
        kernel::memory::free_frame(frame);
        0
    }};
    (SYSCALL_CREATE_PROCESS, $regs:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr) => {{
        let ptr = $a1;
        let len = $a2;
        let slice = unsafe { user_slice::<u8>(ptr, len) };
        if let Ok(s) = core::str::from_utf8(slice) {
             let name = alloc::string::String::from(s);
             let leaked: &'static str = Box::leak(name.into_boxed_str());
             kernel::sched::SCHEDULER.lock().add_process(leaked).0
        } else {
             u64::MAX
        }
    }};
    (SYSCALL_CREATE_THREAD, $regs:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr) => {{
        let pid = abi::ProcessId($a1);
        let entry = $a2;
        let prio = $a3;
        let name_ptr = $a4;
        let name_len = $a5;

        let slice = unsafe { user_slice::<u8>(name_ptr, name_len) };
        let name_str = core::str::from_utf8(slice).unwrap_or("unknown");
        let name = alloc::string::String::from(name_str);
        let leaked_name: &'static str = Box::leak(name.into_boxed_str());

        let stack_size = 64 * 1024; // 64KB
        let pid_check = kernel::sched::SCHEDULER.lock().current_process_id();
        if pid_check != Some(pid) {
             u64::MAX
        } else {
             if let Some(stack_base) = kernel::sched::SCHEDULER.lock().reserve_user_region(pid, stack_size, 4096) {
                 let page_count = stack_size / 4096;
                 let mut frames = alloc::vec::Vec::new();
                 let mut success = true;
                 for _ in 0..page_count {
                     if let Some(f) = kernel::memory::allocate_frame() {
                         frames.push(f);
                     } else {
                         success = false;
                         break;
                     }
                 }

                 if success {
                     if kernel::shared_buffer::map_frames_into_current_as(stack_base, &frames, abi::MapFlags::READ | abi::MapFlags::WRITE).is_ok() {
                         let stack_top = stack_base + stack_size as u64;
                         kernel::sched::SCHEDULER.lock().add_thread(pid, leaked_name, unsafe { core::mem::transmute(entry) }, 0, stack_top, prio).0
                     } else {
                         for f in frames { kernel::memory::free_frame(f); }
                         u64::MAX
                     }
                 } else {
                     for f in frames { kernel::memory::free_frame(f); }
                     u64::MAX
                 }
             } else {
                 u64::MAX
             }
        }
    }};
    (SYSCALL_CREATE_TRANSACTION, $regs:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr) => {{
        kernel::transaction::create_transaction().0
    }};
    (SYSCALL_COMMIT_TRANSACTION, $regs:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr) => {{
        match kernel::transaction::commit_transaction(abi::TransactionId($a1)) {
            Ok(_) => 0,
            Err(_) => 1,
        }
    }};
    (SYSCALL_GRAPH_QUERY, $regs:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr) => {{
        let id = abi::ThingId($a1);
        let out_ptr = $a2;
        let out_len = $a3;
        if let Some(data) = graph::query_node(id) {
            let slice = unsafe { slice::from_raw_parts_mut(out_ptr as *mut u8, out_len as usize) };
            let len = core::cmp::min(data.len(), slice.len());
            slice[..len].copy_from_slice(&data[..len]);
            0
        } else {
            1
        }
    }};
    (SYSCALL_THING_BATCH_UPDATE, $regs:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr) => {{
        let ptr = $a1;
        let len = $a2;
        let slice = unsafe { user_slice::<BatchUpdateEntry>(ptr, len) };
        for entry in slice {
            let props_ptr = entry.props_ptr.ptr;
            let props_len = entry.props_len;
            let wire_props = unsafe { user_slice::<WireProp>(props_ptr, props_len) };
            let mut kernel_props = Vec::with_capacity(wire_props.len());
            for wp in wire_props {
                kernel_props.push(convert_prop(wp));
            }
            graph::update_thing(entry.id, kernel_props);
        }
        0
    }};

    // Fallback for missing syscalls (Stubs)
    ($name:ident, $regs:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr) => {
        u64::MAX
    };
}

macro_rules! collect_dispatched_numbers {
    ($($name:ident => $num:expr),* $(,)?) => {
        pub const DISPATCHED_SYSCALL_NUMBERS: &[u64] = &[ $($num),* ];
    };
}

abi::for_each_syscall!(collect_dispatched_numbers);

#[unsafe(no_mangle)]
pub extern "C" fn syscall_handler_rust(
    num: u64,
    arg1: u64,
    arg2: u64,
    arg3: u64,
    arg4: u64,
    arg5: u64,
    arg6: u64,
) -> u64 {
    macro_rules! dispatch_helper {
        ($($name:ident => $num:expr),* $(,)?) => {
            match num {
                $($name => dispatch_syscall!($name, 0, arg1, arg2, arg3, arg4, arg5, arg6),)*
                _ => u64::MAX
            }
        };
    }

    abi::for_each_syscall!(dispatch_helper)
}

pub fn install_handler() {
    use crate::gdt;
    use x86_64::VirtAddr;
    use x86_64::registers::model_specific::{LStar, SFMask, Star};
    use x86_64::structures::gdt::SegmentSelector;

    // Init per-cpu GS first
    unsafe {
        gdt::init_per_cpu();
    }

    // Set LStar to handler
    let handler_addr = VirtAddr::new(syscall_handler_asm as *const () as u64);
    unsafe {
        LStar::write(handler_addr);
    }

    use x86_64::registers::rflags::RFlags;
    // Mask interrupts (and Direction/Trap) when entering syscall
    unsafe {
        SFMask::write(RFlags::INTERRUPT_FLAG | RFlags::TRAP_FLAG | RFlags::DIRECTION_FLAG);
    }

    let selectors = gdt::get_selectors();
    let kernel_code_sel = selectors.kcode;
    let kernel_data_sel = selectors.kdata;
    let user_code_sel = selectors.ucode;
    let user_data_sel = selectors.udata;

    unsafe {
        // STAR MSR expects:
        // Bits 63:48 - Sysret CS (User Code) + 16 (User Data)
        // Bits 47:32 - Syscall CS (Kernel Code) + 0 (Kernel Data)
        // Note: Sysret loads CS with selector+16, SS with selector+8.
        // We pass user_code_sel_base in bits 63:48.
        // Need to ensure selectors are ordered: User 32=Code, 40=Data.

        Star::write(
            user_code_sel,
            user_data_sel,
            kernel_code_sel,
            kernel_data_sel,
        )
        .unwrap();

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

        assert_eq!(
            abi_nums.len(),
            dispatched_nums.len(),
            "Mismatch in syscall count between ABI and Dispatch"
        );

        for (i, &num) in abi_nums.iter().enumerate() {
            assert_eq!(num, dispatched_nums[i], "Mismatch at index {}", i);
        }
    }
}
