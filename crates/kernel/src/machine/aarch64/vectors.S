.section .text
.global aarch64_vectors
.global aarch64_return_from_trap

.macro VECTOR name char
.align 7
    b \name
.endm

.align 11
aarch64_vectors:
    // Current EL with SP0
    VECTOR handle_sync_el1, 83 // 'S'
    VECTOR handle_irq_el1, 73  // 'I'
    VECTOR handle_fiq_invalid, 70 // 'F'
    VECTOR handle_serror_invalid, 69 // 'E'

    // Current EL with SPx
    VECTOR handle_sync_el1, 115 // 's'
    VECTOR handle_irq_el1, 105  // 'i'
    VECTOR handle_fiq_invalid, 102 // 'f'
    VECTOR handle_serror_invalid, 101 // 'e'

    // Lower EL using AArch64
    VECTOR handle_sync_el0, 88 // 'X'
    VECTOR handle_irq_el0, 89 // 'Y'
    VECTOR handle_fiq_el0, 90 // 'Z'
    VECTOR handle_serror_el0, 87 // 'W'

    // Lower EL using AArch32
    VECTOR handle_sync_invalid, 48 // '0'
    VECTOR handle_irq_invalid, 48
    VECTOR handle_fiq_invalid, 48
    VECTOR handle_serror_invalid, 48

.macro SAVE_CONTEXT
    sub sp, sp, #288
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
    str x30, [sp, #240]

    // mrs x9, sp_el0  // We use the provided x18 instead
    mrs x10, elr_el1
    mrs x11, spsr_el1
    mrs x12, esr_el1
    mrs x13, far_el1

    str x18, [sp, #248] // Save provided SP
    stp x10, x11, [sp, #256]
    stp x12, x13, [sp, #272]
.endm

.macro RESTORE_CONTEXT
    ldr x18, [sp, #248]
    ldp x10, x11, [sp, #256]
    ldp x12, x13, [sp, #272]

    msr elr_el1, x10
    msr spsr_el1, x11

    ldp x0, x1, [sp, #0]
    ldp x2, x3, [sp, #16]
    ldp x4, x5, [sp, #32]
    ldp x6, x7, [sp, #48]
    ldp x8, x9, [sp, #64]
    ldp x10, x11, [sp, #80]
    ldp x12, x13, [sp, #96]
    ldp x14, x15, [sp, #112]
    ldp x16, x17, [sp, #128]
    // x18, x19 restored last
    ldp x20, x21, [sp, #160]
    ldp x22, x23, [sp, #176]
    ldp x24, x25, [sp, #192]
    ldp x26, x27, [sp, #208]
    ldp x28, x29, [sp, #224]
    ldr x30, [sp, #240]

    ldp x18, x19, [sp, #144]
    
    // Restore SP from our saved copy in x18
    // If x18 was sp_el0, we need to decide if we restore sp_el0 or sp_el1
    // Actually, we just want to restore SP to what it was.
    mov sp, x18
    
    eret
.endm

handle_sync_el1:
    msr tpidrro_el0, x0
    mrs x0, tpidr_el1
    ldr x0, [x0, #72] // exception_stack_ptr
    sub x0, x0, #288
    mov x18, sp
    mov sp, x0
    mrs x0, tpidrro_el0
    
    SAVE_CONTEXT
    mov x0, sp
    mov x1, #4
    bl aarch64_handle_exception
    cbz x0, 1f
    mov sp, x0
1:  RESTORE_CONTEXT
    // eret is in RESTORE_CONTEXT

// Old global exception stack removed. Now per-CPU.

.section .text

handle_irq_el1:
    msr tpidrro_el0, x0
    mrs x0, tpidr_el1
    
    // Increment irq_depth
    ldr w1, [x0, #64] // irq_depth
    add w1, w1, #1
    str w1, [x0, #64]

    ldr x0, [x0, #72] // exception_stack_ptr
    sub x0, x0, #288
    mov x18, sp
    mov sp, x0
    mrs x0, tpidrro_el0

    SAVE_CONTEXT
    mov x0, sp
    mov x1, #5
    bl aarch64_handle_exception
    
    // Decrement irq_depth
    msr tpidrro_el0, x0
    mrs x0, tpidr_el1
    ldr w1, [x0, #64]
    sub w1, w1, #1
    str w1, [x0, #64]
    mrs x0, tpidrro_el0
    
    cbz x0, 1f
    mov sp, x0
1:  RESTORE_CONTEXT

handle_sync_el0:
    // EL0 -> EL1: Use TPIDR_EL1 + 88 (kernel_stack_top)
    msr tpidrro_el0, x0
    mrs x0, tpidr_el1
    ldr x0, [x0, #88] // kernel_stack_top
    sub x0, x0, #288
    mov x18, sp_el0  // Incoming SP is SP_EL0
    mov sp, x0
    mrs x0, tpidrro_el0
    
    SAVE_CONTEXT
    mov x0, sp
    mov x1, #8
    bl aarch64_handle_exception
    cbz x0, 1f
    mov sp, x0
1:  RESTORE_CONTEXT

handle_irq_el0:
    msr tpidrro_el0, x0
    mrs x0, tpidr_el1
    
    // Increment irq_depth
    ldr w1, [x0, #64]
    add w1, w1, #1
    str w1, [x0, #64]

    ldr x0, [x0, #88] // kernel_stack_top
    sub x0, x0, #288
    mov x18, sp_el0
    mov sp, x0
    mrs x0, tpidrro_el0

    SAVE_CONTEXT
    mov x0, sp
    mov x1, #9
    bl aarch64_handle_exception

    // Decrement irq_depth
    msr tpidrro_el0, x0
    mrs x0, tpidr_el1
    ldr w1, [x0, #64]
    sub w1, w1, #1
    str w1, [x0, #64]
    mrs x0, tpidrro_el0

    cbz x0, 1f
    mov sp, x0
1:  RESTORE_CONTEXT

handle_fiq_el1: 
    mov x18, sp
    ldr x19, =exception_stack_top
    mov sp, x19
    SAVE_CONTEXT
    wfi; b handle_fiq_el1

handle_serror_el1: 
    mov x18, sp
    ldr x19, =exception_stack_top
    mov sp, x19
    SAVE_CONTEXT
    wfi; b handle_serror_el1
handle_fiq_el0: wfi; b handle_fiq_el0
handle_serror_el0: wfi; b handle_serror_el0
handle_sync_invalid: wfi; b handle_sync_invalid
handle_irq_invalid: wfi; b handle_irq_invalid
handle_fiq_invalid: wfi; b handle_fiq_invalid
handle_serror_invalid: wfi; b handle_serror_invalid

aarch64_return_from_trap:
    mov sp, x0
    RESTORE_CONTEXT
