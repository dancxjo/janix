#![feature(restricted_std)]
#![no_main]

extern crate alloc;

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use stem::syscall::{argv_get, exit, vfs_close, vfs_open, vfs_stat, vfs_write};

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

    stem::utils::parse_argv(&buf)
        .into_iter()
        .skip(1)
        .filter_map(|b| core::str::from_utf8(b).ok().map(String::from))
        .collect()
}

fn print(msg: &str) {
    let _ = vfs_write(1, msg.as_bytes());
}

fn file_type(mode: u32) -> &'static str {
    match mode & 0o170000 {
        0o040000 => "directory",
        0o020000 => "character device",
        0o010000 => "fifo",
        0o100000 => "regular file",
        _ => "unknown",
    }
}

fn print_timespec(label: &str, ts: stem::abi::fs::Timespec) {
    print(&format!("{}: {}.{:09}\n", label, ts.sec, ts.nsec));
}

fn print_stat(path: &str) -> Result<(), ()> {
    let fd = vfs_open(path, 0).map_err(|_| ())?;
    let stat = match vfs_stat(fd) {
        Ok(stat) => stat,
        Err(_) => {
            let _ = vfs_close(fd);
            return Err(());
        }
    };
    let _ = vfs_close(fd);

    print(&format!("  File: {}\n", path));
    print(&format!("  Type: {}\n", file_type(stat.mode)));
    print(&format!("  Mode: {:o}\n", stat.mode));
    print(&format!("   Ino: {}\n", stat.ino));
    print(&format!("  Size: {}\n", stat.size));
    print(&format!(" Links: {}\n", stat.nlink));
    print(&format!("   UID: {}\n", stat.uid));
    print(&format!("   GID: {}\n", stat.gid));
    print(&format!("  Rdev: {}\n", stat.rdev));
    print(&format!("Blksz: {}\n", stat.blksize));
    print(&format!("Blocks: {}\n", stat.blocks));
    print_timespec(" Access", stat.atime);
    print_timespec(" Modify", stat.mtime);
    print_timespec(" Change", stat.ctime);
    Ok(())
}

#[stem::main]
fn main(_arg: usize) -> ! {
    let args = get_args();
    if args.is_empty() {
        print("usage: stat <path>...\n");
        exit(1)
    }

    let mut had_error = false;
    for (i, path) in args.iter().enumerate() {
        if i != 0 {
            print("\n");
        }
        if print_stat(path).is_err() {
            print(&format!("stat: cannot stat '{}'\n", path));
            had_error = true;
        }
    }

    if had_error {
        exit(1)
    }
    exit(0)
}
