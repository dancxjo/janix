#![no_std]
#![no_main]

extern crate alloc;
use alloc::collections::BTreeMap;
use stem::println;
use stem::syscall::{execv, execve, getpid};

#[stem::main]
fn main(_arg0: usize) -> ! {
    let pid = getpid();
    println!("TEST_EXEC: Starting. PID={}", pid);

    // ── Test 1: execve with a non-existent path should return ENOENT ─────────
    {
        let res = execve("/does/not/exist", &[], &BTreeMap::new());
        match res {
            Err(e) => println!("TEST_EXEC: [PASS] non-existent path returned error: {:?}", e),
            Ok(()) => {
                println!("TEST_EXEC: [FAIL] execve of non-existent path succeeded unexpectedly");
                stem::syscall::exit(-1);
            }
        }
    }

    // ── Test 2: execv with a non-existent path should return ENOENT ──────────
    {
        let res = execv("/does/not/exist", &[b"/does/not/exist"]);
        match res {
            Err(e) => println!("TEST_EXEC: [PASS] execv non-existent path returned error: {:?}", e),
            Ok(()) => {
                println!("TEST_EXEC: [FAIL] execv of non-existent path succeeded unexpectedly");
                stem::syscall::exit(-1);
            }
        }
    }

    // ── Test 3: execve a valid ELF binary (replaces this process) ────────────
    // We assume /bin/echo exists on the system.
    let path = "/bin/echo";
    let args: &[&[u8]] = &[
        b"echo", b"Hello", b"from", b"execve!", b"(PID", b"should", b"be", b"the", b"same)",
    ];
    let env = BTreeMap::new();

    println!("TEST_EXEC: Executing {} with args...", path);
    let res = execve(path, args, &env);

    // If we reach here, execve failed.
    println!("TEST_EXEC: execve FAILED with error: {:?}", res);
    stem::syscall::exit(-1);
}
