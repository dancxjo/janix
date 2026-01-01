
.section .text
.global aarch64_vectors
.align 11
aarch64_vectors:

// Current EL with SP0
.align 7
    b handle_sync_sp0
.align 7
    b handle_irq_sp0
.align 7
    b handle_fiq_sp0
.align 7
    b handle_serror_sp0

// Current EL with SPx
.align 7
    b handle_sync_spx
.align 7
    b handle_irq_spx
.align 7
    b handle_fiq_spx
.align 7
    b handle_serror_spx

// Lower EL using AArch64
.align 7
    b handle_sync_lower
.align 7
    b handle_irq_lower
.align 7
    b handle_fiq_lower
.align 7
    b handle_serror_lower

// Lower EL using AArch32
.align 7
    b handle_sync_lower_32
.align 7
    b handle_irq_lower_32
.align 7
    b handle_fiq_lower_32
.align 7
    b handle_serror_lower_32

.macro HANDLE_EXCEPTION vector
    // Save context
    sub sp, sp, #288
    stp x0, x1, [sp, #16 * 0]
    stp x2, x3, [sp, #16 * 1]
    stp x4, x5, [sp, #16 * 2]
    stp x6, x7, [sp, #16 * 3]
    stp x8, x9, [sp, #16 * 4]
    stp x10, x11, [sp, #16 * 5]
    stp x12, x13, [sp, #16 * 6]
    stp x14, x15, [sp, #16 * 7]
    stp x16, x17, [sp, #16 * 8]
    stp x18, x19, [sp, #16 * 9]
    stp x20, x21, [sp, #16 * 10]
    stp x22, x23, [sp, #16 * 11]
    stp x24, x25, [sp, #16 * 12]
    stp x26, x27, [sp, #16 * 13]
    stp x28, x29, [sp, #16 * 14]
    
    // Save LR (x30)
    str x30, [sp, #16 * 15]

    // Save System Registers
    mrs x9, sp_el0
    mrs x10, elr_el1
    mrs x11, spsr_el1
    mrs x12, esr_el1
    mrs x13, far_el1

    stp x9, x10, [sp, #248] // sp_el0, elr_el1
    stp x11, x12, [sp, #264] // spsr_el1, esr_el1
    str x13, [sp, #280]      // far_el1

    mov x0, sp      // context pointer
    mov x1, \vector // vector number
    
    bl aarch64_handle_exception

    // Restore context
    ldp x9, x10, [sp, #248]
    ldp x11, x12, [sp, #264]
    ldr x13, [sp, #280]

    msr sp_el0, x9
    msr elr_el1, x10
    msr spsr_el1, x11
    // esr_el1, far_el1 are read-only-ish / dont need restore usually

    ldp x0, x1, [sp, #16 * 0]
    ldp x2, x3, [sp, #16 * 1]
    ldp x4, x5, [sp, #16 * 2]
    ldp x6, x7, [sp, #16 * 3]
    ldp x8, x9, [sp, #16 * 4]
    ldp x10, x11, [sp, #16 * 5]
    ldp x12, x13, [sp, #16 * 6]
    ldp x14, x15, [sp, #16 * 7]
    ldp x16, x17, [sp, #16 * 8]
    ldp x18, x19, [sp, #16 * 9]
    ldp x20, x21, [sp, #16 * 10]
    ldp x22, x23, [sp, #16 * 11]
    ldp x24, x25, [sp, #16 * 12]
    ldp x26, x27, [sp, #16 * 13]
    ldp x28, x29, [sp, #16 * 14]
    ldr x30, [sp, #16 * 15] // Restore LR

    add sp, sp, #288
    eret
.endm

handle_sync_sp0:
    HANDLE_EXCEPTION 0
handle_irq_sp0:
    HANDLE_EXCEPTION 1
handle_fiq_sp0:
    HANDLE_EXCEPTION 2
handle_serror_sp0:
    HANDLE_EXCEPTION 3

handle_sync_spx:
    HANDLE_EXCEPTION 4
handle_irq_spx:
    HANDLE_EXCEPTION 5
handle_fiq_spx:
    HANDLE_EXCEPTION 6
handle_serror_spx:
    HANDLE_EXCEPTION 7

handle_sync_lower:
    HANDLE_EXCEPTION 8
handle_irq_lower:
    HANDLE_EXCEPTION 9
handle_fiq_lower:
    HANDLE_EXCEPTION 10
handle_serror_lower:
    HANDLE_EXCEPTION 11

handle_sync_lower_32:
    HANDLE_EXCEPTION 12
handle_irq_lower_32:
    HANDLE_EXCEPTION 13
handle_fiq_lower_32:
    HANDLE_EXCEPTION 14
handle_serror_lower_32:
    HANDLE_EXCEPTION 15
