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

.macro HANDLER_IMPL source, kind, label
\label:
    SAVE_REGS
    mov x0, sp
    mov x1, \kind
    mov x2, \source
    bl invalid_exception
    RESTORE_REGS
    eret
.endm

.macro SYSCALL_HANDLER_IMPL label
\label:
    SAVE_REGS
    mov x0, sp
    bl syscall_handler_rust
    str x0, [sp, #0]
    RESTORE_REGS
    eret
.endm

.macro IRQ_HANDLER_IMPL label
\label:
    SAVE_REGS
    mov x0, sp
    bl irq_handler
    RESTORE_REGS
    eret
.endm

.macro TRAMPOLINE label
    .align 7
    b \label
.endm

.align 11
exception_vector_table:
    // Current EL with SP0
    TRAMPOLINE handler_0_0
    TRAMPOLINE handler_0_1
    TRAMPOLINE handler_0_2
    TRAMPOLINE handler_0_3

    // Current EL with SPx
    TRAMPOLINE handler_1_0
    TRAMPOLINE handler_1_1
    TRAMPOLINE handler_1_2
    TRAMPOLINE handler_1_3

    // Lower EL using AArch64
    TRAMPOLINE handler_syscall
    TRAMPOLINE handler_2_1
    TRAMPOLINE handler_2_2
    TRAMPOLINE handler_2_3

    // Lower EL using AArch32
    TRAMPOLINE handler_3_0
    TRAMPOLINE handler_3_1
    TRAMPOLINE handler_3_2
    TRAMPOLINE handler_3_3



// Implementations
HANDLER_IMPL 0, 0, handler_0_0
IRQ_HANDLER_IMPL handler_0_1
HANDLER_IMPL 0, 2, handler_0_2
HANDLER_IMPL 0, 3, handler_0_3

HANDLER_IMPL 1, 0, handler_1_0
IRQ_HANDLER_IMPL handler_1_1
HANDLER_IMPL 1, 2, handler_1_2
HANDLER_IMPL 1, 3, handler_1_3

SYSCALL_HANDLER_IMPL handler_syscall
IRQ_HANDLER_IMPL handler_2_1
HANDLER_IMPL 2, 2, handler_2_2
HANDLER_IMPL 2, 3, handler_2_3

HANDLER_IMPL 3, 0, handler_3_0
HANDLER_IMPL 3, 1, handler_3_1
HANDLER_IMPL 3, 2, handler_3_2
HANDLER_IMPL 3, 3, handler_3_3
