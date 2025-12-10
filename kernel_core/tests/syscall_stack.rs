// These tests model the x86_64 int 0x80 entry/exit path used by the boot
// crate. They don't execute any real interrupts; instead they simulate the
// CPU-pushed iret frame plus the manual register pushes/pops in
// `boot/src/arch/x86_64/syscall.rs`. The goal is to catch stack discipline
// regressions such as missing pops or extra adjustments that would corrupt
// the iret frame.

fn cpu_pushes(stack: &mut Vec<&'static str>) {
    // On privilege change the CPU pushes SS, RSP, RFLAGS, CS, RIP (in that order)
    // so that RIP sits at the top of the frame for iret to pop first.
    for reg in ["ss", "rsp", "rflags", "cs", "rip"] {
        stack.push(reg);
    }
}

fn syscall_pushes(stack: &mut Vec<&'static str>) {
    // Must mirror the assembly push order in boot/src/arch/x86_64/syscall.rs
    for reg in [
        "r15", "r14", "r13", "r12", "rbp", "rbx", "r11", "r10", "r9", "r8", "rcx", "rdx", "rsi",
        "rdi", "rax",
    ] {
        stack.push(reg);
    }
}

fn syscall_pops(stack: &mut Vec<&'static str>) {
    // Must mirror the assembly pop order in boot/src/arch/x86_64/syscall.rs
    for expected in [
        "rax", "rdi", "rsi", "rdx", "rcx", "r8", "r9", "r10", "r11", "rbx", "rbp", "r12", "r13",
        "r14", "r15",
    ] {
        let got = stack.pop().expect("stack underflow while popping GPRs");
        assert_eq!(got, expected, "pop order mismatch");
    }
}

fn cpu_iret_pops(stack: &mut Vec<&'static str>) {
    // iret pops RIP, CS, RFLAGS, RSP, SS in that order.
    for expected in ["rip", "cs", "rflags", "rsp", "ss"] {
        let got = stack
            .pop()
            .expect("stack underflow while popping iret frame");
        assert_eq!(got, expected, "iret frame order mismatch");
    }
}

#[test]
fn syscall_stack_frame_matches_expected_layout() {
    let mut stack = Vec::new();
    cpu_pushes(&mut stack);
    syscall_pushes(&mut stack);

    // Snapshot the layout from the top of the stack downward as seen by the handler.
    let actual_top_to_bottom: Vec<_> = stack.iter().rev().copied().collect();
    let expected_top_to_bottom = vec![
        // Manual pushes (top first)
        "rax", "rdi", "rsi", "rdx", "rcx", "r8", "r9", "r10", "r11", "rbx", "rbp", "r12", "r13",
        "r14", "r15", // CPU frame (bottom)
        "rip", "cs", "rflags", "rsp", "ss",
    ];

    assert_eq!(
        actual_top_to_bottom, expected_top_to_bottom,
        "stack layout no longer matches SyscallRegs expectation; update the struct/assembly together"
    );
}

#[test]
fn syscall_push_pop_disciplined() {
    let mut stack = Vec::new();
    cpu_pushes(&mut stack);
    let base_depth = stack.len();
    syscall_pushes(&mut stack);
    // call pushes a return address; model it, then drop it like the asm does (add rsp,8).
    stack.push("retaddr");
    assert_eq!(stack.pop(), Some("retaddr"));

    // After dropping retaddr, we should have CPU frame + 15 registers.
    assert_eq!(stack.len(), base_depth + 15);

    syscall_pops(&mut stack);
    // iret frame should remain intact before iret executes.
    assert_eq!(stack.len(), base_depth);
    cpu_iret_pops(&mut stack);
    assert!(
        stack.is_empty(),
        "iret should balance the initial CPU pushes; extra adjustments would corrupt the return frame"
    );
}
