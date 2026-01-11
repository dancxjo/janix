use core::arch::{asm, global_asm};
use super::trap::UserTrapFrame;
use kernel::syscall::dispatch;

pub unsafe fn init() {
    unsafe extern "C" {
        fn trap_entry();
    }
    // Set stvec to trap_entry (Direct mode, bit 0 = 0)
    let addr = trap_entry as usize;
    // ensure alignment (4 bytes)
    assert!(addr & 3 == 0);
    unsafe { asm!("csrw stvec, {}", in(reg) addr); }
}

global_asm!(r#"
.section .text
.global trap_entry
.balign 4
trap_entry:
    // We are in Supervisor mode now. SP is Kernel Stack (sscratch was user sp).
    // Wait, we need to swap sscratch and sp.
    // sscratch usually holds Kernel TP or Kernel SP?
    // Let's assume sscratch holds Kernel Stack Top when in User Mode.
    // When in Kernel Mode, sscratch is 0?
    // Common pattern:
    //   csrrw sp, sscratch, sp
    //   beqz sp, .Lkernel_trap
    //   # Came from user mode
    
    // For v0.5 simple user mode:
    // We assume we always come from user mode for now (since we don't have kernel interrupts enabled yet? maybe).
    
    csrrw sp, sscratch, sp
    
    // Now SP is kernel stack. sscratch is user stack.
    
    // Allocate TrapFrame on kernel stack
    addi sp, sp, -296  // 31 regs + status fields... 
    // UserTrapFrame size: 31*8 (regs) + 4*8 (sstatus, sepc, stval, scause) = 248 + 32 = 280.
    // Align to 16? 288?
    // Let's align 288.
    
    // Save registers
    sd x1, 8(sp)  // ra
    // sd x2, ... sp is saved later
    sd x3, 24(sp) // gp
    sd x4, 32(sp) // tp
    sd x5, 40(sp) // t0
    sd x6, 48(sp) // t1
    sd x7, 56(sp) // t2
    sd x8, 64(sp) // s0/fp
    sd x9, 72(sp) // s1
    sd x10, 80(sp) // a0
    sd x11, 88(sp) // a1
    sd x12, 96(sp) // a2
    sd x13, 104(sp) // a3
    sd x14, 112(sp) // a4
    sd x15, 120(sp) // a5
    sd x16, 128(sp) // a6
    sd x17, 136(sp) // a7
    sd x18, 144(sp) // s2
    sd x19, 152(sp) // s3
    sd x20, 160(sp) // s4
    sd x21, 168(sp) // s5
    sd x22, 176(sp) // s6
    sd x23, 184(sp) // s7
    sd x24, 192(sp) // s8
    sd x25, 200(sp) // s9
    sd x26, 208(sp) // s10
    sd x27, 216(sp) // s11
    sd x28, 224(sp) // t3
    sd x29, 232(sp) // t4
    sd x30, 240(sp) // t5
    sd x31, 248(sp) // t6
    
    // Save User SP (from sscratch)
    csrr t0, sscratch
    sd t0, 16(sp) // x2/sp slot in struct (index 2 * 8 = 16? wait struct is array 0..30? No, 1..31?)
    // struct is regs: [usize; 31].
    // If x1 is index 0:
    // x1 -> 0
    // x2 -> 1
    // ...
    // x31 -> 30.
    // Map:
    // regs[0] = x1 (ra) -> 0(sp)
    // regs[1] = x2 (sp) -> 8(sp)
    // regs[2] = x3 (gp) -> 16(sp)
    // ...
    // This implies regs is [usize; 31] where index i = reg i+1?
    // Let's verify struct definition.
    // "x1-x31 (x0 is zero, not saved)"
    // "regs: [usize; 31]"
    // So regs[0] is x1.
    // So offset is i * 8.
    
    // Correct offsets:
    // x1 (ra) -> 0
    // x2 (sp) -> 8
    // x3 (gp) -> 16
    // ...
    // x31 (t6) -> 240
    
    // Regs occupy 248 bytes (31*8).
    
    // CSRs start at 248.
    // sstatus: 248
    // sepc: 256
    // stval: 264
    // scause: 272
    
    // Total used: 280.
    
    sd x1, 0(sp)
    // x2 saved from t0
    sd t0, 8(sp)
    sd x3, 16(sp)
    sd x4, 24(sp)
    sd x5, 32(sp)
    sd x6, 40(sp)
    sd x7, 48(sp)
    sd x8, 56(sp)
    sd x9, 64(sp)
    sd x10, 72(sp)
    sd x11, 80(sp)
    sd x12, 88(sp)
    sd x13, 96(sp)
    sd x14, 104(sp)
    sd x15, 112(sp)
    sd x16, 120(sp)
    sd x17, 128(sp)
    sd x18, 136(sp)
    sd x19, 144(sp)
    sd x20, 152(sp)
    sd x21, 160(sp)
    sd x22, 168(sp)
    sd x23, 176(sp)
    sd x24, 184(sp)
    sd x25, 192(sp)
    sd x26, 200(sp)
    sd x27, 208(sp)
    sd x28, 216(sp)
    sd x29, 224(sp)
    sd x30, 232(sp)
    sd x31, 240(sp)

    csrr t0, sstatus
    sd t0, 248(sp)
    
    csrr t0, sepc
    sd t0, 256(sp)
    
    csrr t0, stval
    sd t0, 264(sp)
    
    csrr t0, scause
    sd t0, 272(sp)
    
    // Call handler(tf)
    mv a0, sp
    call rust_trap_handler
    
    // Restore
    // sstatus may have changed
    ld t0, 248(sp)
    csrw sstatus, t0
    
    // sepc
    ld t0, 256(sp)
    csrw sepc, t0
    
    // regs
    ld x1, 0(sp)
    ld x3, 16(sp)
    ld x4, 24(sp)
    // ... load all ...
    ld x5, 32(sp)
    ld x6, 40(sp)
    ld x7, 48(sp)
    ld x8, 56(sp)
    ld x9, 64(sp)
    ld x10, 72(sp)
    ld x11, 80(sp)
    ld x12, 88(sp)
    ld x13, 96(sp)
    ld x14, 104(sp)
    ld x15, 112(sp)
    ld x16, 120(sp)
    ld x17, 128(sp)
    ld x18, 136(sp)
    ld x19, 144(sp)
    ld x20, 152(sp)
    ld x21, 160(sp)
    ld x22, 168(sp)
    ld x23, 176(sp)
    ld x24, 184(sp)
    ld x25, 192(sp)
    ld x26, 200(sp)
    ld x27, 208(sp)
    ld x28, 216(sp)
    ld x29, 224(sp)
    ld x30, 232(sp)
    ld x31, 240(sp)
    
    // Restore User SP (x2) to sscratch
    ld t0, 8(sp)
    csrw sscratch, t0
    
    addi sp, sp, 296
    
    // Swap sp and sscratch again to restore user stack
    csrrw sp, sscratch, sp
    
    sret
"#);

#[unsafe(no_mangle)]
pub unsafe extern "C" fn rust_trap_handler(tf: &mut UserTrapFrame) {
    let scause = tf.scause;
    let is_interrupt = (scause >> 63) != 0;
    let code = scause & 0x7FFFFFFFFFFFFFFF;
    
    if is_interrupt {
        // Ignored for now
    } else {
        match code {
             8 => { // User mode ecall
                 // Syscall
                 // A7 is syscall num.
                 // A7 is x17.
                 // regs[0]=x1 ... regs[16]=x17.
                 // So A7 is at index 16.
                 let n = tf.regs[16];
                 
                 // Args: A0..A5
                 // A0=x10 -> index 9
                 // A1=x11 -> index 10
                 // A2=x12 -> index 11
                 // A3=x13 -> index 12
                 // A4=x14 -> index 13
                 // A5=x15 -> index 14
                 
                 let a0 = tf.regs[9];
                 let a1 = tf.regs[10];
                 let a2 = tf.regs[11];
                 let a3 = tf.regs[12];
                 let a4 = tf.regs[13];
                 let a5 = tf.regs[14];
                 
                 let ret = dispatch(n, [a0, a1, a2, a3, a4, a5]);
                 
                 // Return value in A0 (x10, index 9)
                 tf.regs[9] = ret as usize;
                 
                 // Advance SEPC by 4 (size of ecall)
                 tf.sepc += 4;
             }
             _ => {
                 // Panic or loop
                 kernel::kprintln!("Unexpected trap: scause={:x} sepc={:x}", scause, tf.sepc);
                 loop {}
             }
        }
    }
}
