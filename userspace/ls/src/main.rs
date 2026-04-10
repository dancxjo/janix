#![feature(restricted_std)]
#![no_main]

extern crate alloc;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use stem::syscall::{argv_get, vfs_close, vfs_open, vfs_readdir, vfs_write, vfs_stat, exit};

#[derive(Debug, Default)]
struct Flags {
    long: bool,
    recursive: bool,
    all: bool,
}

fn get_args() -> (Vec<String>, Flags) {
    let mut len = 0;
    if let Ok(l) = argv_get(&mut []) {
        len = l;
    }
    if len == 0 {
        return (Vec::new(), Flags::default());
    }
    let mut buf = alloc::vec![0u8; len];
    if argv_get(&mut buf).is_err() {
        return (Vec::new(), Flags::default());
    }

    let raw_args = stem::utils::parse_argv(&buf);
    let mut paths = Vec::new();
    let mut flags = Flags::default();
    
    // Skip arg[0] (command name)
    for arg_bytes in raw_args.into_iter().skip(1) {
        if let Ok(arg) = core::str::from_utf8(arg_bytes) {
            if arg.starts_with('-') && arg.len() > 1 {
                for c in arg.chars().skip(1) {
                    match c {
                        'l' => flags.long = true,
                        'R' => flags.recursive = true,
                        'a' => flags.all = true,
                        _ => {}
                    }
                }
            } else {
                paths.push(String::from(arg));
            }
        }
    }

    if paths.is_empty() {
        paths.push(String::from("."));
    }

    (paths, flags)
}

fn print(msg: &str) {
    let _ = vfs_write(1, msg.as_bytes());
}

fn format_mode(mode: u32) -> String {
    let mut s = String::with_capacity(10);
    let kind = mode & 0o170000;
    if kind == 0o040000 { s.push('d'); }
    else if kind == 0o020000 { s.push('c'); }
    else if kind == 0o010000 { s.push('p'); }
    else { s.push('-'); }

    let perms = mode & 0o777;
    for i in (0..3).rev() {
        let p = (perms >> (i * 3)) & 0o7;
        s.push(if p & 0o4 != 0 { 'r' } else { '-' });
        s.push(if p & 0o2 != 0 { 'w' } else { '-' });
        s.push(if p & 0o1 != 0 { 'x' } else { '-' });
    }
    s
}

fn list_path(path: &str, flags: &Flags, is_nested: bool) {
    stem::info!("ls: listing path '{}'", path);
    if flags.recursive || is_nested {
        print(&format!("{}:\n", path));
    }

    let fd = match vfs_open(path, 0) { // O_RDONLY = 0
        Ok(fd) => fd,
        Err(e) => {
            stem::error!("ls: failed to open '{}': {:?}", path, e);
            print(&format!("ls: cannot access '{}': No such file or directory\n", path));
            return;
        }
    };

    let stat = match vfs_stat(fd) {
        Ok(s) => s,
        Err(_) => {
            let _ = vfs_close(fd);
            return;
        }
    };

    if (stat.mode & 0o170000) != 0o040000 {
        // Not a directory, just print the file itself
        if flags.long {
            print(&format!("{} {:8} {}\n", format_mode(stat.mode), stat.size, path));
        } else {
            print(&format!("{}\n", path));
        }
        let _ = vfs_close(fd);
        return;
    }

    let mut entries = Vec::new();
    let mut buf = [0u8; 4096];
    loop {
        match vfs_readdir(fd, &mut buf) {
            Ok(0) => break,
            Ok(n) => {
                let mut offset = 0;
                while offset < n {
                    let mut end = offset;
                    while end < n && buf[end] != 0 {
                        end += 1;
                    }
                    if let Ok(name) = core::str::from_utf8(&buf[offset..end]) {
                        if !name.is_empty() {
                            if flags.all || !name.starts_with('.') {
                                entries.push(String::from(name));
                            }
                        }
                    }
                    offset = end + 1;
                }
            }
            Err(_) => break,
        }
    }
    let _ = vfs_close(fd);

    // Sort entries for consistency
    entries.sort();

    let mut subdirs = Vec::new();

    for name in entries {
        let mut full_path = String::from(path);
        if !full_path.ends_with('/') {
            full_path.push('/');
        }
        full_path.push_str(&name);

        if flags.long {
            match vfs_open(&full_path, 0) {
                Ok(child_fd) => {
                    if let Ok(child_stat) = vfs_stat(child_fd) {
                        print(&format!("{} {:8} {}\n", format_mode(child_stat.mode), child_stat.size, name));
                        if flags.recursive && (child_stat.mode & 0o170000) == 0o040000 && name != "." && name != ".." {
                            subdirs.push(full_path);
                        }
                    }
                    let _ = vfs_close(child_fd);
                }
                Err(_) => {
                    print(&format!("?--------- ?        {}\n", name));
                }
            }
        } else {
            print(&format!("{}\n", name));
            if flags.recursive && name != "." && name != ".." {
                // We need to check if it's a directory
                if let Ok(child_fd) = vfs_open(&full_path, 0) {
                    if let Ok(child_stat) = vfs_stat(child_fd) {
                        if (child_stat.mode & 0o170000) == 0o040000 {
                            subdirs.push(full_path);
                        }
                    }
                    let _ = vfs_close(child_fd);
                }
            }
        }
    }

    if flags.recursive && !subdirs.is_empty() {
        print("\n");
        for subdir in subdirs {
            list_path(&subdir, flags, true);
        }
    }
}

#[stem::main]
fn main(_arg: usize) -> ! {
    let (paths, flags) = get_args();
    
    for (i, path) in paths.iter().enumerate() {
        if i > 0 {
            print("\n");
        }
        list_path(path, &flags, paths.len() > 1);
    }

    exit(0)
}
