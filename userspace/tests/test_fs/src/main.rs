//! test_fs — ThingOS filesystem smoke tests.
//!
//! Exercises the VFS syscall layer directly (no `std::fs`) to verify that
//! the kernel-side primitives used by the std PAL implementation are correct.
//!
//! Each test prints "PASS: <name>" on success or "FAIL: <name>: <reason>" on
//! failure and exits with a non-zero code if any test fails.

#![feature(restricted_std)]
#![no_main]

extern crate alloc;
extern crate std;

use std::io::{Read, Seek, SeekFrom, Write};

// ── Harness ──────────────────────────────────────────────────────────────────

static mut FAILURES: usize = 0;

macro_rules! check {
    ($name:expr, $result:expr) => {
        match $result {
            Ok(()) => std::println!("PASS: {}", $name),
            Err(e) => {
                std::println!("FAIL: {}: {}", $name, e);
                unsafe { FAILURES += 1 };
            }
        }
    };
}

// ── Test: fs_create_write_read ────────────────────────────────────────────────

fn test_create_write_read() -> Result<(), std::string::String> {
    let path = "/tmp/test_fs_rw.txt";
    let content = b"hello, ThingOS filesystem!";

    // Create and write.
    {
        let mut f = std::fs::File::create(path)
            .map_err(|e| std::format!("create: {}", e))?;
        f.write_all(content)
            .map_err(|e| std::format!("write_all: {}", e))?;
    }

    // Reopen and read back.
    {
        let mut f = std::fs::File::open(path)
            .map_err(|e| std::format!("open: {}", e))?;
        let mut buf = std::vec::Vec::new();
        f.read_to_end(&mut buf)
            .map_err(|e| std::format!("read_to_end: {}", e))?;
        if buf != content {
            return Err(std::format!(
                "content mismatch: got {:?}, expected {:?}",
                buf, content
            ));
        }
    }

    // Cleanup.
    std::fs::remove_file(path)
        .map_err(|e| std::format!("remove_file: {}", e))?;

    Ok(())
}

// ── Test: fs_seek ─────────────────────────────────────────────────────────────

fn test_seek() -> Result<(), std::string::String> {
    let path = "/tmp/test_fs_seek.txt";
    let content = b"0123456789abcdef";

    {
        let mut f = std::fs::File::create(path)
            .map_err(|e| std::format!("create: {}", e))?;
        f.write_all(content)
            .map_err(|e| std::format!("write_all: {}", e))?;
    }

    {
        let mut f = std::fs::File::open(path)
            .map_err(|e| std::format!("open: {}", e))?;

        // Seek to offset 4 and read 4 bytes.
        f.seek(SeekFrom::Start(4))
            .map_err(|e| std::format!("seek(Start,4): {}", e))?;
        let mut buf = [0u8; 4];
        f.read_exact(&mut buf)
            .map_err(|e| std::format!("read_exact after seek: {}", e))?;
        if &buf != b"4567" {
            return Err(std::format!(
                "seek+read mismatch: got {:?}, expected b\"4567\"", buf
            ));
        }

        // Seek from current: go back 2 bytes, read 2.
        f.seek(SeekFrom::Current(-2))
            .map_err(|e| std::format!("seek(Current,-2): {}", e))?;
        let mut buf2 = [0u8; 2];
        f.read_exact(&mut buf2)
            .map_err(|e| std::format!("read_exact after seek(Current): {}", e))?;
        if &buf2 != b"67" {
            return Err(std::format!(
                "seek(Current)+read mismatch: got {:?}, expected b\"67\"", buf2
            ));
        }

        // Seek from end: last 4 bytes.
        f.seek(SeekFrom::End(-4))
            .map_err(|e| std::format!("seek(End,-4): {}", e))?;
        let mut buf3 = [0u8; 4];
        f.read_exact(&mut buf3)
            .map_err(|e| std::format!("read_exact after seek(End): {}", e))?;
        if &buf3 != b"cdef" {
            return Err(std::format!(
                "seek(End)+read mismatch: got {:?}, expected b\"cdef\"", buf3
            ));
        }
    }

    std::fs::remove_file(path)
        .map_err(|e| std::format!("remove_file: {}", e))?;

    Ok(())
}

// ── Test: fs_readdir ──────────────────────────────────────────────────────────

fn test_readdir() -> Result<(), std::string::String> {
    let dir = "/tmp/test_fs_dir";

    // Ensure a clean slate.
    let _ = std::fs::remove_dir(dir);

    std::fs::create_dir(dir)
        .map_err(|e| std::format!("create_dir: {}", e))?;

    // Create two files inside.
    let file_a = std::format!("{}/alpha.txt", dir);
    let file_b = std::format!("{}/beta.txt", dir);
    std::fs::File::create(&file_a)
        .map_err(|e| std::format!("create alpha.txt: {}", e))?;
    std::fs::File::create(&file_b)
        .map_err(|e| std::format!("create beta.txt: {}", e))?;

    // Collect directory entries.
    let entries: std::vec::Vec<std::string::String> = std::fs::read_dir(dir)
        .map_err(|e| std::format!("read_dir: {}", e))?
        .filter_map(|e| e.ok())
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .collect();

    if !entries.iter().any(|n| n == "alpha.txt") {
        return Err(std::format!("alpha.txt not found in {:?}", entries));
    }
    if !entries.iter().any(|n| n == "beta.txt") {
        return Err(std::format!("beta.txt not found in {:?}", entries));
    }

    // Cleanup.
    std::fs::remove_file(&file_a)
        .map_err(|e| std::format!("remove alpha.txt: {}", e))?;
    std::fs::remove_file(&file_b)
        .map_err(|e| std::format!("remove beta.txt: {}", e))?;
    std::fs::remove_dir(dir)
        .map_err(|e| std::format!("remove_dir: {}", e))?;

    Ok(())
}

// ── Test: fs_metadata ─────────────────────────────────────────────────────────

fn test_metadata() -> Result<(), std::string::String> {
    let file_path = "/tmp/test_fs_meta.txt";
    let dir_path = "/tmp/test_fs_meta_dir";
    let content = b"metadata test content";

    // Create a file.
    {
        let mut f = std::fs::File::create(file_path)
            .map_err(|e| std::format!("create: {}", e))?;
        f.write_all(content)
            .map_err(|e| std::format!("write_all: {}", e))?;
    }

    // Verify file metadata.
    let meta = std::fs::metadata(file_path)
        .map_err(|e| std::format!("metadata(file): {}", e))?;
    if !meta.is_file() {
        return Err("is_file() returned false for a regular file".into());
    }
    if meta.is_dir() {
        return Err("is_dir() returned true for a regular file".into());
    }
    if meta.len() != content.len() as u64 {
        return Err(std::format!(
            "len mismatch: got {}, expected {}", meta.len(), content.len()
        ));
    }

    // Create a directory.
    std::fs::create_dir(dir_path)
        .map_err(|e| std::format!("create_dir: {}", e))?;

    // Verify directory metadata.
    let dir_meta = std::fs::metadata(dir_path)
        .map_err(|e| std::format!("metadata(dir): {}", e))?;
    if !dir_meta.is_dir() {
        return Err("is_dir() returned false for a directory".into());
    }
    if dir_meta.is_file() {
        return Err("is_file() returned true for a directory".into());
    }

    // Cleanup.
    std::fs::remove_file(file_path)
        .map_err(|e| std::format!("remove_file: {}", e))?;
    std::fs::remove_dir(dir_path)
        .map_err(|e| std::format!("remove_dir: {}", e))?;

    Ok(())
}

// ── Test: fs_rename_remove ───────────────────────────────────────────────────

fn test_rename_remove() -> Result<(), std::string::String> {
    let old_path = "/tmp/test_fs_old.txt";
    let new_path = "/tmp/test_fs_new.txt";
    let content = b"rename test";

    // Create the original file.
    {
        let mut f = std::fs::File::create(old_path)
            .map_err(|e| std::format!("create: {}", e))?;
        f.write_all(content)
            .map_err(|e| std::format!("write_all: {}", e))?;
    }

    // Rename.
    std::fs::rename(old_path, new_path)
        .map_err(|e| std::format!("rename: {}", e))?;

    // Old path should not exist.
    if std::path::Path::new(old_path).exists() {
        return Err(std::format!("old path '{}' still exists after rename", old_path));
    }

    // New path should exist with correct content.
    {
        let mut f = std::fs::File::open(new_path)
            .map_err(|e| std::format!("open after rename: {}", e))?;
        let mut buf = std::vec::Vec::new();
        f.read_to_end(&mut buf)
            .map_err(|e| std::format!("read after rename: {}", e))?;
        if buf != content {
            return Err(std::format!(
                "content after rename mismatch: got {:?}", buf
            ));
        }
    }

    // Delete the new file.
    std::fs::remove_file(new_path)
        .map_err(|e| std::format!("remove_file: {}", e))?;

    // Verify it's gone.
    if std::path::Path::new(new_path).exists() {
        return Err(std::format!("'{}' still exists after remove_file", new_path));
    }

    Ok(())
}

// ── Test: fs_create_new (O_EXCL) ─────────────────────────────────────────────

fn test_create_new() -> Result<(), std::string::String> {
    use std::io::ErrorKind;

    let path = "/tmp/test_fs_excl.txt";
    let _ = std::fs::remove_file(path);

    // First create should succeed.
    std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|e| std::format!("create_new (first): {}", e))?;

    // Second create_new on the same path should fail with AlreadyExists.
    match std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
    {
        Err(e) if e.kind() == ErrorKind::AlreadyExists => {}
        Err(e) => return Err(std::format!("create_new (second): unexpected error: {}", e)),
        Ok(_) => return Err("create_new (second): should have failed but succeeded".into()),
    }

    std::fs::remove_file(path)
        .map_err(|e| std::format!("remove_file: {}", e))?;

    Ok(())
}

// ── Test: fs_truncate ─────────────────────────────────────────────────────────

fn test_truncate() -> Result<(), std::string::String> {
    let path = "/tmp/test_fs_trunc.txt";

    // Write 16 bytes.
    {
        let mut f = std::fs::File::create(path)
            .map_err(|e| std::format!("create: {}", e))?;
        f.write_all(b"Hello, truncate!")
            .map_err(|e| std::format!("write_all: {}", e))?;
    }

    // Open for writing and truncate to 5 bytes.
    {
        let f = std::fs::OpenOptions::new()
            .write(true)
            .open(path)
            .map_err(|e| std::format!("open for truncate: {}", e))?;
        f.set_len(5)
            .map_err(|e| std::format!("set_len(5): {}", e))?;
    }

    // Read back and verify.
    {
        let mut f = std::fs::File::open(path)
            .map_err(|e| std::format!("open after truncate: {}", e))?;
        let mut buf = std::vec::Vec::new();
        f.read_to_end(&mut buf)
            .map_err(|e| std::format!("read_to_end: {}", e))?;
        if buf != b"Hello" {
            return Err(std::format!(
                "truncate: got {:?}, expected b\"Hello\"", buf
            ));
        }
    }

    std::fs::remove_file(path)
        .map_err(|e| std::format!("remove_file: {}", e))?;

    Ok(())
}

// ── Entry point ───────────────────────────────────────────────────────────────

#[stem::main]
fn main(_arg: usize) -> ! {
    std::println!("=== test_fs: ThingOS filesystem smoke tests ===");

    check!("fs_create_write_read", test_create_write_read());
    check!("fs_seek", test_seek());
    check!("fs_readdir", test_readdir());
    check!("fs_metadata", test_metadata());
    check!("fs_rename_remove", test_rename_remove());
    check!("fs_create_new", test_create_new());
    check!("fs_truncate", test_truncate());

    let failures = unsafe { FAILURES };
    if failures == 0 {
        std::println!("=== test_fs: ALL TESTS PASSED ===");
        stem::syscall::exit(0);
    } else {
        std::println!("=== test_fs: {} TESTS FAILED ===", failures);
        stem::syscall::exit(1);
    }
}
