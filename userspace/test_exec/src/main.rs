#![no_std]
#![no_main]

extern crate alloc;
use stem::println;
use stem::syscall::{execve, getpid};
use alloc::collections::BTreeMap;

#[stem::main]
fn main(_arg0: usize) -> ! {
    let pid = getpid();
    println!("TEST_EXEC: Starting. PID={}", pid);

    // We assume /bin/echo exists on the system.
    let path = "/bin/echo";
    let args: &[&[u8]] = &[b"echo", b"Hello", b"from", b"execve!", b"(PID", b"should", b"be", b"the", b"same)"];
    let env = BTreeMap::new();

    println!("TEST_EXEC: Executing {} with args...", path);
    let res = execve(path, args, &env);

    // If we reach here, execve failed.
    println!("TEST_EXEC: execve FAILED with error: {:?}", res);
    stem::syscall::exit(-1);
}
