
.section .text
.global loongarch64_trap_vector
.extern KERNEL_STACK_TOP
.align 12 // LoongArch exception entry alignment

loongarch64_trap_vector:
    // Swap $sp with CSR_KS0 (0x30)
    csrwr $sp, 0x30
    
    // If $sp != 0, we came from User Mode (KS0 had KernelSP)
    bnez $sp, 1f
    
    // We came from Kernel Mode: restore original SP from KS0
    csrrd $sp, 0x30
    
1:
    // Now $sp is the kernel stack
    addi.d $sp, $sp, -288
    
    st.d $r1, $sp, 0
    st.d $r2, $sp, 8
    
    // Save original SP (now in KS0)
    csrrd $t0, 0x30
    st.d $t0, $sp, 16

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

    // Save CSRs
    csrrd $t0, 0x6    // ERA
    csrrd $t1, 0x5    // ESTAT
    csrrd $t2, 0x7    // BADV
    csrrd $t3, 0x1    // PRMD
    
    st.d $t0, $sp, 248
    st.d $t1, $sp, 256
    st.d $t2, $sp, 264
    st.d $t3, $sp, 272
    
    move $a0, $sp

    bl loongarch64_handle_trap
    
.global loongarch64_trap_restore
loongarch64_trap_restore:
    // a0 = new sp (TrapFrame pointer)
    // Check for context switch
    beqz $a0, 1f
    move $sp, $a0
1:
    // Check return mode: PRMD.PPLV (bits 0-1)
    ld.d $t1, $sp, 272
    andi $t1, $t1, 0x3
    beqz $t1, 2f // PRMD.PPLV == 0, return to kernel
    
    // Returning to User Mode:
    // 1. Store KERNEL_STACK_TOP back to KS0 for the NEXT trap
    la.global $t0, KERNEL_STACK_TOP
    ld.d $t0, $t0, 0
    csrwr $t0, 0x30
    
    // 2. Load UserSP into t0 for later restoration
    ld.d $t0, $sp, 16
    
    // 3. Restore CSRs
    ld.d $t1, $sp, 248 // ERA
    ld.d $t2, $sp, 272 // PRMD
    csrwr $t1, 0x6
    csrwr $t2, 0x1
    
    // 4. Restore all regs
    ld.d $r1, $sp, 0
    ld.d $r2, $sp, 8
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
    
    // 5. Restore UserSP and return
    move $sp, $t0
    ertn

2:
    // Returning to Kernel Mode:
    // 1. Clear KS0 (marks as kernel mode)
    csrwr $r0, 0x30
    
    // 2. Restore CSRs
    ld.d $t1, $sp, 248
    ld.d $t2, $sp, 272
    csrwr $t1, 0x6
    csrwr $t2, 0x1
    
    // 3. Restore all regs
    ld.d $r1, $sp, 0
    ld.d $r2, $sp, 8
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
    
    // 4. Restore $sp and return
    addi.d $sp, $sp, 288
    ertn

.global loongarch64_switch_to
loongarch64_switch_to:
    // a0 = &mut old_sp, a1 = new_sp
    addi.d $sp, $sp, -88
    st.d $r1, $sp, 0
    st.d $r22, $sp, 8
    st.d $r23, $sp, 16
    st.d $r24, $sp, 24
    st.d $r25, $sp, 32
    st.d $r26, $sp, 40
    st.d $r27, $sp, 48
    st.d $r28, $sp, 56
    st.d $r29, $sp, 64
    st.d $r30, $sp, 72
    st.d $r31, $sp, 80

    st.d $sp, $a0, 0
    move $sp, $a1

    ld.d $r1, $sp, 0
    ld.d $r22, $sp, 8
    ld.d $r23, $sp, 16
    ld.d $r24, $sp, 24
    ld.d $r25, $sp, 32
    ld.d $r26, $sp, 40
    ld.d $r27, $sp, 48
    ld.d $r28, $sp, 56
    ld.d $r29, $sp, 64
    ld.d $r30, $sp, 72
    ld.d $r31, $sp, 80
    addi.d $sp, $sp, 88
    jirl $r0, $r1, 0

.global loongarch64_task_entry_stub
loongarch64_task_entry_stub:
    ld.d $a0, $sp, 0 // dispatch_ptr
    ld.d $t0, $sp, 8 // entry_point
    addi.d $sp, $sp, 16
    jirl $r0, $t0, 0
