use core::arch::{asm, global_asm};
use super::trap::UserTrapFrame;
use kernel::syscall::dispatch;

pub unsafe fn init() {
    unsafe extern "C" {
        fn trap_entry();
    }
    let addr = trap_entry as usize;
    // Set EENTRY (CSR 0xC)
    unsafe { asm!("csrwr {}, 0xC", in(reg) addr); }
}

global_asm!(r#"
.section .text
.global trap_entry
.balign 4
trap_entry:
    // Swap SP with KS0 (CSR 0x30)
    // KS0 holds kernel stack top (when in user) or 0 (when in kernel)?
    // Similar to RISC-V sscratch pattern.
    csrwr $sp, 0x30
    
    // SP is now kernel stack. KS0 is user stack.
    
    // Alloc frame
    addi.d $sp, $sp, -288 
    // Regs: 31*8 = 248.
    // CSRs: 4*8 = 32.
    // Total 280 -> 288 aligned.
    
    // Save regs (r1..r31)
    st.d $r1, $sp, 0
    st.d $r2, $sp, 8  // r2 is tp
    st.d $r3, $sp, 16 // r3 is sp... wait. User SP is in KS0.
    // We save KS0 to this slot later.
    st.d $r4, $sp, 24
    st.d $r5, $sp, 32
    st.d $r6, $sp, 40
    st.d $r7, $sp, 48
    st.d $r8, $sp, 56
    st.d $r9, $sp, 64
    st.d $r10, $sp, 72
    st.d $r11, $sp, 80
    st.d $r12, $sp, 88
    st.d $r13, $sp, 96
    st.d $r14, $sp, 104
    st.d $r15, $sp, 112
    st.d $r16, $sp, 120
    st.d $r17, $sp, 128
    st.d $r18, $sp, 136
    st.d $r19, $sp, 144
    st.d $r20, $sp, 152
    st.d $r21, $sp, 160
    st.d $r22, $sp, 168
    st.d $r23, $sp, 176
    st.d $r24, $sp, 184
    st.d $r25, $sp, 192
    st.d $r26, $sp, 200
    st.d $r27, $sp, 208
    st.d $r28, $sp, 216
    st.d $r29, $sp, 224
    st.d $r30, $sp, 232
    st.d $r31, $sp, 240
    
    // Save User SP (from KS0)
    csrrd $t0, 0x30
    st.d $t0, $sp, 16 // r3 slot (index 2 * 8 = 16)
    
    // Save CSRs
    // ERA (0x6) -> PC
    csrrd $t0, 0x6
    st.d $t0, $sp, 248
    
    // PRMD (0x1) -> Status
    csrrd $t0, 0x1
    st.d $t0, $sp, 256
    
    // BADV (0x7)
    csrrd $t0, 0x7
    st.d $t0, $sp, 264
    
    // ESTAT (0x5)
    csrrd $t0, 0x5
    st.d $t0, $sp, 272
    
    // Call Rust
    move $a0, $sp
    bl rust_trap_handler
    
    // Restore CSRs (PRMD, ERA)
    // ESTAT is read-only mostly? We don't restore exception status.
    // BADV neither.
    // ERA and PRMD matter for return.
    
    ld.d $t0, $sp, 248
    csrwr $t0, 0x6
    
    ld.d $t0, $sp, 256
    csrwr $t0, 0x1
    
    // Restore regs
    ld.d $r1, $sp, 0
    ld.d $r2, $sp, 8
    // Skip r3 (SP)
    ld.d $r4, $sp, 24
    ld.d $r5, $sp, 32
    ld.d $r6, $sp, 40
    ld.d $r7, $sp, 48
    ld.d $r8, $sp, 56
    ld.d $r9, $sp, 64
    ld.d $r10, $sp, 72
    ld.d $r11, $sp, 80
    ld.d $r12, $sp, 88
    ld.d $r13, $sp, 96
    ld.d $r14, $sp, 104
    ld.d $r15, $sp, 112
    ld.d $r16, $sp, 120
    ld.d $r17, $sp, 128
    ld.d $r18, $sp, 136
    ld.d $r19, $sp, 144
    ld.d $r20, $sp, 152
    ld.d $r21, $sp, 160
    ld.d $r22, $sp, 168
    ld.d $r23, $sp, 176
    ld.d $r24, $sp, 184
    ld.d $r25, $sp, 192
    ld.d $r26, $sp, 200
    ld.d $r27, $sp, 208
    ld.d $r28, $sp, 216
    ld.d $r29, $sp, 224
    ld.d $r30, $sp, 232
    ld.d $r31, $sp, 240
    
    // Restore User SP to KS0
    ld.d $t0, $sp, 16
    csrwr $t0, 0x30
    
    addi.d $sp, $sp, 288
    
    // Swap SP/KS0
    csrwr $sp, 0x30
    
    ertn
"#);

#[unsafe(no_mangle)]
pub unsafe extern "C" fn rust_trap_handler(tf: &mut UserTrapFrame) {
    let estat = tf.estat;
    let ecode = (estat >> 16) & 0x3F;
    let subcode = estat & 0xFFFF; // Subcode not usually used for syscall
    
    if ecode == 0xB { // SYSCALL
        // Syscall num in A7 (R11).
        // R11 is index 10 (regs[10]). (r1=index 0)
        // Regs: r1, r2, r3, r4, r5, r6, r7, r8, r9, r10, r11.
        // Index: 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10.
        let n = tf.regs[10];
        
        // Args: A0..A5
        // A0=r4 (index 3)
        // A1=r5 (index 4)
        // A2=r6 (index 5)
        // A3=r7 (index 6)
        // A4=r8 (index 7)
        // A5=r9 (index 8)
        
        let a0 = tf.regs[3];
        let a1 = tf.regs[4];
        let a2 = tf.regs[5];
        let a3 = tf.regs[6];
        let a4 = tf.regs[7];
        let a5 = tf.regs[8];
        
        let ret = dispatch(n, [a0, a1, a2, a3, a4, a5]);
        
        // Return value in A0 (r4, index 3)
        tf.regs[3] = ret as usize;
        
        // Advance ERA by 4 (instruction size)
        tf.era += 4;
        
    } else {
        kernel::kprintln!("Unexpected LoongArch trap: ESTAT={:x} ERA={:x}", estat, tf.era);
        loop {}
    }
}
