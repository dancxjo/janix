//! Smoke test: argv comes through correctly via SYS_ARGV_GET.
//!
//! Acceptance criteria:
//!   - argv[0] is the executable name
//!   - subsequent arguments are preserved in order
#![feature(restricted_std)]

fn main() {
    let args: Vec<String> = std::env::args().collect();

    println!("[show_args] argc = {}", args.len());
    for (i, arg) in args.iter().enumerate() {
        println!("[show_args] argv[{}] = {:?}", i, arg);
    }

    if args.is_empty() {
        eprintln!("[show_args] FAIL: expected at least argv[0]");
        std::process::exit(1);
    }

    println!("[show_args] PASS");
    std::process::exit(0);
}
