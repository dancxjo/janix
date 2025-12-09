.section .text
.global exception_vector_table

.macro SAVE_REGS
    sub sp, sp, #34 * 8
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
    mrs x21, sp_el0
    stp x30, x21, [sp, #16 * 15] 
    mrs x22, elr_el1
    mrs x23, spsr_el1
    stp x22, x23, [sp, #16 * 16]
.endm

.macro RESTORE_REGS
    ldp x22, x23, [sp, #16 * 16]
    msr elr_el1, x22
    msr spsr_el1, x23
    ldp x30, x21, [sp, #16 * 15]
    msr sp_el0, x21
    ldp x28, x29, [sp, #16 * 14]
    ldp x26, x27, [sp, #16 * 13]
    ldp x24, x25, [sp, #16 * 12]
    ldp x22, x23, [sp, #16 * 11]
    ldp x20, x21, [sp, #16 * 10]
    ldp x18, x19, [sp, #16 * 9]
    ldp x16, x17, [sp, #16 * 8]
    ldp x14, x15, [sp, #16 * 7]
    ldp x12, x13, [sp, #16 * 6]
    ldp x10, x11, [sp, #16 * 5]
    ldp x8, x9, [sp, #16 * 4]
    ldp x6, x7, [sp, #16 * 3]
    ldp x4, x5, [sp, #16 * 2]
    ldp x2, x3, [sp, #16 * 1]
    ldp x0, x1, [sp, #16 * 0]
    add sp, sp, #34 * 8
.endm

.macro HANDLER source, kind
    .align 7
    SAVE_REGS
    mov x0, sp
    mov x1, \kind
    mov x2, \source
    bl invalid_exception
    RESTORE_REGS
    eret
.endm

.align 11
exception_vector_table:
    // Current EL with SP0
    HANDLER 0, 0  // Synchronous
    HANDLER 0, 1  // IRQ
    HANDLER 0, 2  // FIQ
    HANDLER 0, 3  // SError

    // Current EL with SPx
    HANDLER 1, 0
    HANDLER 1, 1
    HANDLER 1, 2
    HANDLER 1, 3

    // Lower EL using AArch64
    HANDLER 2, 0
    HANDLER 2, 1
    HANDLER 2, 2
    HANDLER 2, 3

    // Lower EL using AArch32
    HANDLER 3, 0
    HANDLER 3, 1
    HANDLER 3, 2
    HANDLER 3, 3
