// library/std/src/sys/pal/thingos/env.rs
pub mod os {
    pub const FAMILY: &str = "thingos";
    pub const OS: &str = "thingos";
    pub const DLL_PREFIX: &str = "lib";
    pub const DLL_SUFFIX: &str = ".so";
    pub const DLL_EXTENSION: &str = "so";
    pub const EXE_SUFFIX: &str = "";
    pub const EXE_EXTENSION: &str = "";
}

pub fn args() -> crate::sys::args::Args {
    crate::sys::args::Args::default()
}

pub fn var(_key: &crate::ffi::OsStr) -> crate::io::Result<crate::ffi::OsString> {
    Err(crate::io::Error::new(crate::io::ErrorKind::NotFound, "env vars not supported"))
}

pub fn var_os(_key: &crate::ffi::OsStr) -> Option<crate::ffi::OsString> {
    None
}

pub fn set_var(_key: &crate::ffi::OsStr, _value: &crate::ffi::OsStr) {
}

pub fn remove_var(_key: &crate::ffi::OsStr) {
}

pub fn vars() -> crate::env::Vars {
    // Return empty iterator
    // This requires implementing the iterator, which is internal to std.
    // Usually we construct it from environment pointers.
    // For now, let's panic or return empty.
    // Since we can't easily construct `Vars` (it wraps `sys::os::Env`), we need `sys::os::Env`.
    // Let's rely on `unsupported` via stub.
    panic!("vars not supported")
}

pub fn vars_os() -> crate::env::VarsOs {
    panic!("vars_os not supported")
}

pub fn current_exe() -> crate::io::Result<crate::path::PathBuf> {
    Err(crate::io::Error::new(crate::io::ErrorKind::Unsupported, "current_exe not supported"))
}

pub fn temp_dir() -> crate::path::PathBuf {
    crate::path::PathBuf::from("/tmp")
}

pub fn home_dir() -> Option<crate::path::PathBuf> {
    None
}

pub fn exit(code: i32) -> ! {
    // SYSCALL_EXIT? We don't have it. Loop forever or panic.
    // Or write to log and loop.
    crate::eprintln!("Process exited with code {}", code);
    loop {}
}

pub fn page_size() -> usize {
    4096
}
