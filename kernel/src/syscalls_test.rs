use abi::syscalls::*;

#[test]
fn test_syscall_constants_hygiene() {
    let syscalls = ABI_SYSCALL_NUMBERS;

    // Check for duplicates
    for i in 0..syscalls.len() {
        for j in (i + 1)..syscalls.len() {
            if syscalls[i] == syscalls[j] {
                panic!(
                    "Duplicate syscall number detected: {}",
                    syscalls[i]
                );
            }
        }
    }

    // Check specific required constants
    assert_eq!(SYSCALL_SCHEMA_REGISTER_PACKAGE, 19, "SYSCALL_SCHEMA_REGISTER_PACKAGE must be 19");
    assert_eq!(SYSCALL_DEV_OPEN, 64, "SYSCALL_DEV_OPEN must be 64");
}
