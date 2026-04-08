#![feature(restricted_std)]
#![no_main]

extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use stem::syscall::{argv_get, vfs_write, vfs_read};

fn prompt() {
    let _ = vfs_write(1, b"petals> ");
}

fn read_line() -> String {
    let mut buf = [0u8; 1024];
    let mut bytes = Vec::new();
    loop {
        match vfs_read(0, &mut buf) {
            Ok(0) => break,
            Ok(n) => {
                for &b in &buf[..n] {
                    if b == 0x03 {
                        bytes.clear();
                        return String::new(); // Ctrl-C
                    }
                    bytes.push(b);
                    if b == b'\n' {
                        let s = String::from_utf8(bytes).unwrap_or_default();
                        return s;
                    }
                }
            }
            Err(_) => break,
        }
    }
    String::from_utf8(bytes).unwrap_or_default()
}

fn spawn_program(cmd: &str, args: &[&str]) -> abi::errors::SysResult<()> {
    // try to resolve path
    let path = if cmd.starts_with('/') {
        String::from(cmd)
    } else {
        alloc::format!("/{}", cmd)
    };
    
    // Convert args to Vec<Vec<u8>>
    let mut argv = Vec::new();
    argv.push(path.as_bytes().to_vec());
    for arg in args {
        argv.push(arg.as_bytes().to_vec());
    }
    let argv_slices: Vec<&[u8]> = argv.iter().map(|v| v.as_slice()).collect();
    
    let env = alloc::collections::BTreeMap::new();
    
    match stem::syscall::spawn_process_ex(
        &path,
        &argv_slices,
        &env,
        0, // Inherit
        0, // Inherit
        0, // Inherit
    ) {
        Ok(resp) => {
            // wait for task completion using task_wait
            let _ = stem::syscall::task_wait(resp.child_tid);
            Ok(())
        }
        Err(e) => {
            let out = alloc::format!("sh: command not found: {}\n", cmd);
            let _ = vfs_write(1, out.as_bytes());
            Err(e)
        }
    }
}

#[stem::main]
fn main(_arg: usize) -> ! {
    let _ = vfs_write(1, b"janix sh\n");
    
    loop {
        prompt();
        let line = read_line();
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        
        let mut parts = trimmed.split_whitespace();
        let cmd = parts.next().unwrap_or("");
        let args: Vec<&str> = parts.collect();
        
        if cmd == "exit" {
            break;
        } else {
            let _ = spawn_program(cmd, &args);
        }
    }
    
    stem::syscall::exit(0)
}
