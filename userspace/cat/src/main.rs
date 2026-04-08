#![feature(restricted_std)]
#![no_main]

extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use stem::syscall::{argv_get, vfs_close, vfs_open, vfs_read, vfs_write};

fn get_args() -> Vec<String> {
    let mut len = 0;
    if let Ok(l) = argv_get(&mut []) {
        len = l;
    }
    if len == 0 {
        return Vec::new();
    }
    let mut buf = alloc::vec![0u8; len];
    if argv_get(&mut buf).is_err() {
        return Vec::new();
    }

    let mut args = Vec::new();
    if buf.len() >= 4 {
        let count = u32::from_le_bytes(buf[0..4].try_into().unwrap()) as usize;
        let mut offset = 4;
        for _ in 0..count {
            if offset + 4 > buf.len() {
                break;
            }
            let str_len = u32::from_le_bytes(buf[offset..offset + 4].try_into().unwrap()) as usize;
            offset += 4;
            if offset + str_len > buf.len() {
                break;
            }
            if let Ok(s) = core::str::from_utf8(&buf[offset..offset + str_len]) {
                args.push(String::from(s));
            }
            offset += str_len;
        }
    }
    args
}

fn print_error(msg: &str) {
    let out = alloc::format!("cat: {}\n", msg);
    let _ = vfs_write(1, out.as_bytes());
}

#[stem::main]
fn main(_arg: usize) -> ! {
    let args = get_args();
    if args.len() < 2 {
        print_error("no file specified");
        stem::syscall::exit(1);
    }

    for path in args.iter().skip(1) {
        match vfs_open(path, 0) {
            // O_RDONLY = 0
            Ok(fd) => {
                let mut buf = [0u8; 1024];
                loop {
                    match vfs_read(fd, &mut buf) {
                        Ok(0) => break,
                        Ok(n) => {
                            let _ = vfs_write(1, &buf[..n]);
                        }
                        Err(_) => {
                            print_error("read error");
                            break;
                        }
                    }
                }
                let _ = vfs_close(fd);
            }
            Err(_) => {
                print_error(&alloc::format!("failed to open {}", path));
            }
        }
    }

    stem::syscall::exit(0)
}
