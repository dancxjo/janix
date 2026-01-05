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

// SAVE_CONTEXT_WITH_SP: Save all registers including the incoming SP passed in x9
// Caller must set x9 = incoming SP before invoking this macro
// x9 will be saved as part of the regular register save, then extracted
// and stored in the SP slot at offset 248.
.macro SAVE_CONTEXT_WITH_SP
    sub sp, sp, #288
    stp x0, x1, [sp, #0]
    stp x2, x3, [sp, #16]
    stp x4, x5, [sp, #32]
    stp x6, x7, [sp, #48]
    stp x8, x9, [sp, #64]      // x9 contains incoming SP, saved here
    stp x10, x11, [sp, #80]
    stp x12, x13, [sp, #96]
    stp x14, x15, [sp, #112]
    stp x16, x17, [sp, #128]
    stp x18, x19, [sp, #144]   // Original x18, x19 preserved
    stp x20, x21, [sp, #160]
    stp x22, x23, [sp, #176]
    stp x24, x25, [sp, #192]
    stp x26, x27, [sp, #208]
    stp x28, x29, [sp, #224]
    str x30, [sp, #240]

    // Read system registers
    mrs x10, elr_el1
    mrs x11, spsr_el1
    mrs x12, esr_el1
    mrs x13, far_el1

    // Copy incoming SP from x9's save slot to the SP field
    ldr x14, [sp, #72]         // Load saved x9 value (offset 64 + 8)
    str x14, [sp, #248]        // Save it as the SP field
    stp x10, x11, [sp, #256]
    stp x12, x13, [sp, #272]
.endm

.macro RESTORE_CONTEXT
    ldp x10, x11, [sp, #256]
    ldp x12, x13, [sp, #272]

    msr elr_el1, x10
    msr spsr_el1, x11

    // For EL0 returns, set sp_el0 BEFORE restoring registers
    // SPSR_EL1 bits 3:0: 0x0 = EL0, 0x4/0x5 = EL1
    // We do this early so we can use scratch registers
    mrs x10, spsr_el1
    and x10, x10, #0xf
    ldr x11, [sp, #248]        // Load saved SP into x11
    cbnz x10, 1f               // If returning to EL1, skip sp_el0 write
    msr sp_el0, x11            // Restore user stack to SP_EL0
1:

    // Now restore all general-purpose registers
    ldp x0, x1, [sp, #0]
    ldp x2, x3, [sp, #16]
    ldp x4, x5, [sp, #32]
    ldp x6, x7, [sp, #48]
    ldp x8, x9, [sp, #64]
    ldp x10, x11, [sp, #80]
    ldp x12, x13, [sp, #96]
    ldp x14, x15, [sp, #112]
    ldp x16, x17, [sp, #128]
    ldp x18, x19, [sp, #144]   // x18, x19 properly restored
    ldp x20, x21, [sp, #160]
    ldp x22, x23, [sp, #176]
    ldp x24, x25, [sp, #192]
    ldp x26, x27, [sp, #208]
    ldp x28, x29, [sp, #224]
    ldr x30, [sp, #240]

    // Check if we need to set kernel SP (for EL1 returns only)
    // We need a scratch register - borrow x9 temporarily
    mrs x9, spsr_el1
    and x9, x9, #0xf
    cbz x9, 2f                 // If returning to EL0, skip SP restore
    ldr x9, [sp, #248]         // Load saved kernel SP
    mov sp, x9                 // Restore kernel stack
    // x9 was clobbered, but we're going to EL1 so it will be reloaded
    // from the kernel stack when needed
    eret
2:
    // Returning to EL0 - sp_el0 was already set above
    // Re-restore x9 since we clobbered it
    ldr x9, [sp, #72]          // Re-load x9 from its save slot
    add sp, sp, #288           // Deallocate frame (for EL0 case only)
    eret
.endm

handle_sync_el1:
    msr tpidrro_el0, x0        // Save x0 temporarily
    mov x9, sp                 // Save incoming SP in x9 (will be saved by macro)
    // Switch to exception stack
    ldr x0, =exception_stack_top
    ldr x0, [x0]
    mov sp, x0
    mrs x0, tpidrro_el0        // Restore x0 before saving
    
    SAVE_CONTEXT_WITH_SP
    
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
    ldr w1, [x0, #72] // irq_depth
    add w1, w1, #1
    str w1, [x0, #72]

    mov x9, sp                 // Save incoming SP in x9
    // Switch to exception stack
    ldr x1, =exception_stack_top
    ldr x1, [x1]
    mov sp, x1
    mrs x0, tpidrro_el0

    SAVE_CONTEXT_WITH_SP
    mov x0, sp
    mov x1, #5
    bl aarch64_handle_exception
    
    // Decrement irq_depth
    msr tpidrro_el0, x0
    mrs x0, tpidr_el1
    ldr w1, [x0, #72]
    sub w1, w1, #1
    str w1, [x0, #72]
    mrs x0, tpidrro_el0
    
    cbz x0, 1f
    mov sp, x0
1:  RESTORE_CONTEXT

handle_sync_el0:
    // EL0 -> EL1: Use TPIDR_EL1 + 104 (kernel_stack_top)
    msr tpidrro_el0, x0
    mrs x0, tpidr_el1
    ldr x0, [x0, #104] // kernel_stack_top
    sub x0, x0, #288
    mrs x9, sp_el0             // Get user SP into x9 (will be saved by macro)
    mov sp, x0
    mrs x0, tpidrro_el0
    
    SAVE_CONTEXT_WITH_SP
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
    ldr w1, [x0, #72]
    add w1, w1, #1
    str w1, [x0, #72]

    ldr x0, [x0, #104] // kernel_stack_top
    sub x0, x0, #288
    mrs x9, sp_el0             // Get user SP into x9
    mov sp, x0
    mrs x0, tpidrro_el0

    SAVE_CONTEXT_WITH_SP
    mov x0, sp
    mov x1, #9
    bl aarch64_handle_exception

    // Decrement irq_depth
    msr tpidrro_el0, x0
    mrs x0, tpidr_el1
    ldr w1, [x0, #72]
    sub w1, w1, #1
    str w1, [x0, #72]
    mrs x0, tpidrro_el0

    cbz x0, 1f
    mov sp, x0
1:  RESTORE_CONTEXT

handle_fiq_el1: 
    mov x9, sp
    ldr x19, =exception_stack_top
    ldr x19, [x19]
    mov sp, x19
    SAVE_CONTEXT_WITH_SP
    wfi; b handle_fiq_el1

handle_serror_el1: 
    mov x9, sp
    ldr x19, =exception_stack_top
    ldr x19, [x19]
    mov sp, x19
    SAVE_CONTEXT_WITH_SP
    wfi; b handle_serror_el1
handle_fiq_el0: wfi; b handle_fiq_el0
handle_serror_el0: wfi; b handle_serror_el0
handle_sync_invalid: wfi; b handle_sync_invalid
handle_irq_invalid: wfi; b handle_irq_invalid
handle_fiq_invalid: wfi; b handle_fiq_invalid
handle_serror_invalid: wfi; b handle_serror_invalid

.global aarch64_restore_context
aarch64_return_from_trap:
    mov sp, x0
aarch64_restore_context:
    RESTORE_CONTEXT
