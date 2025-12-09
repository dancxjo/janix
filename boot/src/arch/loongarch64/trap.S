.section .text
.global trap_vector
.align 12
trap_vector:
    // Save registers
    addi.d $sp, $sp, -32 * 8
    st.d $r1, $sp, 0 * 8
    st.d $r2, $sp, 1 * 8
    st.d $r3, $sp, 2 * 8
    st.d $r4, $sp, 3 * 8
    st.d $r5, $sp, 4 * 8
    st.d $r6, $sp, 5 * 8
    st.d $r7, $sp, 6 * 8
    st.d $r8, $sp, 7 * 8
    st.d $r9, $sp, 8 * 8
    st.d $r10, $sp, 9 * 8
    st.d $r11, $sp, 10 * 8
    st.d $r12, $sp, 11 * 8
    st.d $r13, $sp, 12 * 8
    st.d $r14, $sp, 13 * 8
    st.d $r15, $sp, 14 * 8
    st.d $r16, $sp, 15 * 8
    st.d $r17, $sp, 16 * 8
    st.d $r18, $sp, 17 * 8
    st.d $r19, $sp, 18 * 8
    st.d $r20, $sp, 19 * 8
    st.d $r21, $sp, 20 * 8
    st.d $r22, $sp, 21 * 8
    st.d $r23, $sp, 22 * 8
    st.d $r24, $sp, 23 * 8
    st.d $r25, $sp, 24 * 8
    st.d $r26, $sp, 25 * 8
    st.d $r27, $sp, 26 * 8
    st.d $r28, $sp, 27 * 8
    st.d $r29, $sp, 28 * 8
    st.d $r30, $sp, 29 * 8
    st.d $r31, $sp, 30 * 8

    move $a0, $sp
    csrrd $a1, 0x6 // ESTAT
    csrrd $a2, 0x7 // ERA
    csrrd $a3, 0x5 // BADV
    bl trap_handler

    // Restore registers
    ld.d $r1, $sp, 0 * 8
    ld.d $r2, $sp, 1 * 8
    ld.d $r3, $sp, 2 * 8
    ld.d $r4, $sp, 3 * 8
    ld.d $r5, $sp, 4 * 8
    ld.d $r6, $sp, 5 * 8
    ld.d $r7, $sp, 6 * 8
    ld.d $r8, $sp, 7 * 8
    ld.d $r9, $sp, 8 * 8
    ld.d $r10, $sp, 9 * 8
    ld.d $r11, $sp, 10 * 8
    ld.d $r12, $sp, 11 * 8
    ld.d $r13, $sp, 12 * 8
    ld.d $r14, $sp, 13 * 8
    ld.d $r15, $sp, 14 * 8
    ld.d $r16, $sp, 15 * 8
    ld.d $r17, $sp, 16 * 8
    ld.d $r18, $sp, 17 * 8
    ld.d $r19, $sp, 18 * 8
    ld.d $r20, $sp, 19 * 8
    ld.d $r21, $sp, 20 * 8
    ld.d $r22, $sp, 21 * 8
    ld.d $r23, $sp, 22 * 8
    ld.d $r24, $sp, 23 * 8
    ld.d $r25, $sp, 24 * 8
    ld.d $r26, $sp, 25 * 8
    ld.d $r27, $sp, 26 * 8
    ld.d $r28, $sp, 27 * 8
    ld.d $r29, $sp, 28 * 8
    ld.d $r30, $sp, 29 * 8
    ld.d $r31, $sp, 30 * 8
    addi.d $sp, $sp, 32 * 8
    ertn
