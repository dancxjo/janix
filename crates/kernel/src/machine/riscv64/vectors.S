
.section .text
.global riscv64_trap_vector
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

    // Restore CSRs
    ld t0, 248(sp)
    ld t1, 256(sp)
    
    csrw sstatus, t0
    csrw sepc, t1

    // Restore regs
    ld x1, 0(sp)
    // x2 (sp) restored at end
    ld x3, 16(sp)
    ld x4, 24(sp)
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
    
    // Load user sp into t0
    ld t0, 8(sp)
    
    // Deallocate TrapContext frame (before we lose sp reference)
    addi sp, sp, 288
    
    // Check sstatus.SPP (bit 8): if returning to U-mode, set sscratch = kernel_sp
    // and sp = user_sp. If returning to S-mode, set sscratch = 0.
    csrr t1, sstatus
    andi t1, t1, 0x100  // SPP bit
    beqz t1, return_to_user
    
return_to_kernel:
    // Returning to S-mode: sscratch should be 0
    csrw sscratch, zero
    // sp stays as is (kernel context)
    sret
    
return_to_user:
    // Returning to U-mode: save kernel sp to sscratch, restore user sp
    csrw sscratch, sp
    mv sp, t0  // User stack from saved TrapContext
    sret
