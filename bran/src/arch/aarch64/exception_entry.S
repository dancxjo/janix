.section .text
.balign 2048
.global exception_vector_table
exception_vector_table:
    /* Current EL with SP0 */
    .balign 128
    b unhandled_exception
    .balign 128
    b unhandled_exception
    .balign 128
    b unhandled_exception
    .balign 128
    b unhandled_exception

    /* Current EL with SPx = Kernel */
    .balign 128
    b sync_el1_handler
    .balign 128
    b irq_el1_handler
    .balign 128
    b unhandled_exception
    .balign 128
    b unhandled_exception

    /* Lower EL using AArch64 = User */
    .balign 128
    b sync_el0_handler
    .balign 128
    b irq_el0_handler
    .balign 128
    b unhandled_exception
    .balign 128
    b unhandled_exception

    /* Lower EL using AArch32 */
    .balign 128
    b unhandled_exception
    .balign 128
    b unhandled_exception
    .balign 128
    b unhandled_exception
    .balign 128
    b unhandled_exception

/* Unhandled: Infinite Loop for now */
unhandled_exception:
    wfi
    b unhandled_exception

/* Macros for Context Save/Restore */
.macro SAVE_CONTEXT
    /* Allocate space for 30 regs + LR + ELR + SPSR + padding */
    /* 32 * 8 = 256 bytes? */
    /* Regs x0-x29 (30) */
    /* x30, ELR_EL1, SPSR_EL1 */
    /* Align to 16 */
    sub sp, sp, #272
    stp x0, x1, [sp, #0]
    stp x2, x3, [sp, #16]
    stp x4, x5, [sp, #32]
    stp x6, x7, [sp, #48]
    stp x8, x9, [sp, #64]
    stp x10, x11, [sp, #80]
    stp x12, x13, [sp, #96]
    stp x14, x15, [sp, #112]
    stp x16, x17, [sp, #128]
    stp x18, x19, [sp, #144]
    stp x20, x21, [sp, #160]
    stp x22, x23, [sp, #176]
    stp x24, x25, [sp, #192]
    stp x26, x27, [sp, #208]
    stp x28, x29, [sp, #224]
    
    mrs x9, elr_el1
    mrs x10, spsr_el1
    stp x30, x9, [sp, #240]  /* x30, ELR */
    str x10, [sp, #256]      /* SPSR */
    
    /* Save SP_EL0 -> Offset 264 */
    mrs x9, sp_el0
    str x9, [sp, #264]
.endm

/* RESTORE_CONTEXT */
.macro RESTORE_CONTEXT
    /* Restore User Stack Pointer from offset 264 */
    ldr x10, [sp, #264]
    msr sp_el0, x10

    ldr x10, [sp, #256]
    ldp x30, x9, [sp, #240]
    
    msr spsr_el1, x10
    msr elr_el1, x9
    
    ldp x28, x29, [sp, #224]
    ldp x26, x27, [sp, #208]
    ldp x24, x25, [sp, #192]
    ldp x22, x23, [sp, #176]
    ldp x20, x21, [sp, #160]
    ldp x18, x19, [sp, #144]
    ldp x16, x17, [sp, #128]
    ldp x14, x15, [sp, #112]
    ldp x12, x13, [sp, #96]
    ldp x10, x11, [sp, #80]
    ldp x8, x9, [sp, #64]
    ldp x6, x7, [sp, #48]
    ldp x4, x5, [sp, #32]
    ldp x2, x3, [sp, #16]
    ldp x0, x1, [sp, #0]
    
    add sp, sp, #272
.endm

.global aarch64_enter_user_mode
aarch64_enter_user_mode:
    /* x0 = pointer to TrapFrame */
    mov sp, x0
    RESTORE_CONTEXT
    eret

/* Sync EL1 Handler */
sync_el1_handler:
    SAVE_CONTEXT
    mov x0, sp /* Pass TrapFrame ptr */
    bl kern_sync_handler
    RESTORE_CONTEXT
    eret

/* IRQ EL1 Handler */
irq_el1_handler:
    SAVE_CONTEXT
    bl kern_irq_handler
    RESTORE_CONTEXT
    eret

/* IRQ EL0 Handler (User Preemption) */
irq_el0_handler:
    SAVE_CONTEXT
    bl kern_irq_handler
    RESTORE_CONTEXT
    eret

/* Sync EL0 Handler (Syscall) */
sync_el0_handler:
    /* Check if it is an SVC */
    mrs x9, esr_el1
    lsr x9, x9, #26
    cmp x9, #0x15 /* EC 0x15 = SVC64 */
    b.eq handle_svc
    
    /* Not SVC, unexpected user fault */
    SAVE_CONTEXT
    mov x0, sp
    bl kern_sync_handler
    RESTORE_CONTEXT
    eret

handle_svc:
    /* SVC Handler */
    /* Save callee saved regs? */
    /* Syscall conventions: Caller saves scratch. Callee saves x19-x29. */
    /* But we want to call rust function. Rust will clobber scratch. */
    /* And we need to preserve user state. */
    SAVE_CONTEXT
    
    /* Load SYSCALL_HANDLER global variable? */
    /* But wait, x86 uses `syscall::enable(entry)`. */
    /* `entry` is the address of `syscall_dispatcher(scratch_ctx)`. */
    /* The `BootRuntime` registers it. */
    
    /* We will use a global symbol that Rust sets. */
    /* We assume rust side exposes `syscall_entry` or we jump to stored ptr? */
    /* Let's follow `register_syscall_handler(u64)` pattern. */
    /* We store the address in a local var. */
    
    ldr x9, =_syscall_handler_addr
    ldr x10, [x9]
    cmp x10, #0
    b.eq no_handler
    
    /* Call handler. Arguments? */
    /* x86 passes (sp) of scratch context? */
    /* Or arguments in registers? */
    /* thing-os ABI: args in registers. */
    /* Wrapper should call handlers. */
    /* If `register_syscall_handler` provided the dispatcher, we call it. */
    /* Dispatcher usually takes (arg0..arg5, call_id). */
    /* Or it takes a context pointer? */
    /* Let's pass the TrapFrame pointer (sp) as arg 0. */
    
    mov x0, sp
    blr x10
    
    /* Upon return, x0 might be modified if it returns a value? */
    /* If the handler updates the TrapFrame on stack, we restore from it. */
    /* Standard Rust syscall handler usually returns nothing and modifies frame. */
    
    RESTORE_CONTEXT
    eret

no_handler:
    b unhandled_exception

.section .data
.global _syscall_handler_addr
_syscall_handler_addr:
    .quad 0
