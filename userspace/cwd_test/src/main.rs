//! Smoke test: current_dir() and set_current_dir() via SYS_FS_GETCWD / SYS_FS_CHDIR.
//!
//! Acceptance criteria:
//!   - `std::env::current_dir()` returns a non-empty path
//!   - `std::env::set_current_dir("/tmp")` changes the cwd
//!   - a subsequent `current_dir()` reflects the change
#![feature(restricted_std)]

fn main() {
    // Initial cwd
    let initial = match std::env::current_dir() {
        Ok(p) => {
            println!("[cwd_test] initial cwd: {:?}", p);
            p
        }
        Err(e) => {
            eprintln!("[cwd_test] FAIL: current_dir() error: {}", e);
            std::process::exit(1);
        }
    };

    if initial.as_os_str().is_empty() {
        eprintln!("[cwd_test] FAIL: initial cwd is empty");
        std::process::exit(1);
    }

    // Change to /tmp (always exists as a ramfs mount on ThingOS)
    match std::env::set_current_dir("/tmp") {
        Ok(()) => println!("[cwd_test] chdir /tmp OK"),
        Err(e) => {
            eprintln!("[cwd_test] FAIL: set_current_dir(/tmp): {}", e);
            std::process::exit(1);
        }
    }

    // Verify cwd changed
    match std::env::current_dir() {
        Ok(p) => {
            println!("[cwd_test] new cwd: {:?}", p);
            if p.as_os_str() != "/tmp" {
                eprintln!("[cwd_test] FAIL: expected /tmp, got {:?}", p);
                std::process::exit(1);
            }
        }
        Err(e) => {
            eprintln!("[cwd_test] FAIL: current_dir() after chdir: {}", e);
            std::process::exit(1);
        }
    }

    println!("[cwd_test] PASS");
    std::process::exit(0);
}
