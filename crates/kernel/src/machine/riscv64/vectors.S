.section .text
.global riscv64_trap_vector
.extern KERNEL_STACK_TOP
.align 4
riscv64_trap_vector:
    // On trap entry, we need to determine if we came from user or kernel mode.
    // sscratch holds kernel stack pointer when in user mode, 0 when in kernel mode.
    // If sscratch != 0, swap sp and sscratch to get kernel stack.
    
    csrrw sp, sscratch, sp  // Atomically swap sp and sscratch
    
    // Now: if we came from U-mode, sp = kernel stack, sscratch = user stack
    //      if we came from S-mode, sp = 0 (BAD!), sscratch = kernel stack
    // Check if sp == 0 (means we came from S-mode and sscratch was 0)
    bnez sp, 1f
    
    // Came from S-mode: restore sp from sscratch
    csrr sp, sscratch
    
1:  // Now sp is the kernel stack
    // Allocate space for TrapContext (288 bytes aligned)
    addi sp, sp, -288

    // Save x1 (ra)
    sd x1, 0(sp)
    
    // Save user sp (x2): read from sscratch if from U-mode, or use original sp
    // For simplicity, save sscratch (which is user sp after swap, or 0 if from kernel)
    csrr t0, sscratch
    sd t0, 8(sp)    // Save user/original sp
    
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

    // Save CSRs
    csrr t0, sstatus
    csrr t1, sepc
    csrr t2, scause
    csrr t3, stval
    
    sd t0, 248(sp)
    sd t1, 256(sp)
    sd t2, 264(sp)
    sd t3, 272(sp)

    mv a0, sp // ctx pointer
    
    call riscv64_handle_trap
    
    // Check if we need to switch stack (a0 != 0)
    beqz a0, 2f
    mv sp, a0
2:
    mv a0, sp
    j riscv64_return_from_trap

.global riscv64_return_from_trap
riscv64_return_from_trap:
    // a0 = TrapContext pointer
    mv t4, a0          // Preserve frame base in t4 (x29)
    mv sp, t4

    // Read saved control state before restoring temporaries
    ld t0, 248(t4)     // sstatus
    ld t1, 256(t4)     // sepc
    ld t2, 8(t4)       // saved user sp (x2)
    addi t5, t4, 288   // kernel sp after popping frame
    andi t3, t0, 0x100 // SPP bit

    // Restore CSRs
    csrw sstatus, t0
    csrw sepc, t1

    // Restore general registers that are not used as temps
    ld x1, 0(t4)
    ld x3, 16(t4)
    ld x4, 24(t4)
    ld x8, 56(t4)
    ld x9, 64(t4)
    ld x10, 72(t4)
    ld x11, 80(t4)
    ld x12, 88(t4)
    ld x13, 96(t4)
    ld x14, 104(t4)
    ld x15, 112(t4)
    ld x16, 120(t4)
    ld x17, 128(t4)
    ld x18, 136(t4)
    ld x19, 144(t4)
    ld x20, 152(t4)
    ld x21, 160(t4)
    ld x22, 168(t4)
    ld x23, 176(t4)
    ld x24, 184(t4)
    ld x25, 192(t4)
    ld x26, 200(t4)
    ld x27, 208(t4)

    // Select return target stack
    beqz t3, 2f
    csrw sscratch, zero
    mv sp, t5          // Kernel mode return
    j 3f

2:  // User mode return: hand kernel stack to sscratch, restore user sp
    // PRE: t5 is the kernel stack top for THIS trap.
    // However, the NEXT trap from user mode should use KERNEL_STACK_TOP.
    // Fetch KERNEL_STACK_TOP
    la t6, KERNEL_STACK_TOP
    ld t6, 0(t6)
    csrw sscratch, t6
    mv sp, t2

3:
    // Restore temporary registers last
    ld x5, 32(t4)
    ld x6, 40(t4)
    ld x7, 48(t4)
    ld x28, 216(t4)
    ld x29, 224(t4)
    ld x30, 232(t4)
    ld x31, 240(t4)
    sret
