use core::arch::global_asm;

// RISC-V Callee-Saved Registers:
// ra (x1)
// s0 (x8)
// s1 (x9)
// s2-s11 (x18-x27)
// Total 13 registers.
// Plus we need to save/restore SP (handled by changing the pointer).

unsafe extern "C" {
    pub fn context_switch(old_handle_ptr: *mut u64, new_handle: u64);
}

// Trampoline for new threads
unsafe extern "C" {
    fn trampoline();
}

global_asm!(r#"
.section .text
.global context_switch
context_switch:
    // a0 = old_handle_ptr (*mut u64)
    // a1 = new_handle (u64) - This is the new SP

    // Save callee-saved registers
    // Stack must be 16-byte aligned. 13 regs * 8 = 104 bytes.
    // round up to 112 for 16-byte alignment.
    addi sp, sp, -112
    
    sd ra, 0(sp)
    sd s0, 8(sp)
    sd s1, 16(sp)
    sd s2, 24(sp)
    sd s3, 32(sp)
    sd s4, 40(sp)
    sd s5, 48(sp)
    sd s6, 56(sp)
    sd s7, 64(sp)
    sd s8, 72(sp)
    sd s9, 80(sp)
    sd s10, 88(sp)
    sd s11, 96(sp)

    // Save old SP to *old_handle_ptr
    sd sp, 0(a0)

    // Load new SP
    mv sp, a1

    // Restore callee-saved registers
    ld ra, 0(sp)
    ld s0, 8(sp)
    ld s1, 16(sp)
    ld s2, 24(sp)
    ld s3, 32(sp)
    ld s4, 40(sp)
    ld s5, 48(sp)
    ld s6, 56(sp)
    ld s7, 64(sp)
    ld s8, 72(sp)
    ld s9, 80(sp)
    ld s10, 88(sp)
    ld s11, 96(sp)

    addi sp, sp, 112
    ret

.global trampoline
trampoline:
    // s0 = arg
    // s1 = entry
    mv a0, s0
    jalr s1
    
    // Should not return
    ebreak
"#);

pub fn context_init(
    kstack_top: u64,
    entry: extern "C" fn(usize) -> !,
    arg: usize,
) -> usize {
    // Stack layout:
    // [trampoline] (ra)
    // [s0..s11]
    
    let stack_size = 112;
    let sp = kstack_top - stack_size;
    let ptr = sp as *mut u64;

    unsafe {
        // ra = trampoline
        ptr.offset(0).write(trampoline as usize as u64);
        
        // s0 = arg
        ptr.offset(1).write(arg as u64);
        
        // s1 = entry
        ptr.offset(2).write(entry as usize as u64);
        
        // s2..s11 = 0
        core::ptr::write_bytes(ptr.offset(3), 0, 10);
    }

    sp as usize
}
