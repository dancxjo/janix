// library/std/src/sys/pal/thingos/mod.rs

#[path = "../unsupported/os.rs"]
pub mod os;

#[path = "../unsupported/common.rs"]
pub mod common;
pub use common::*;

pub mod time;

pub fn init(_argc: isize, _argv: *const *const u8, _sigpipe: u8) {}

pub fn cleanup() {}

pub fn unsupported<T>() -> crate::io::Result<T> {
    Err(unsupported_err())
}

pub fn unsupported_err() -> crate::io::Error {
    crate::io::Error::new(crate::io::ErrorKind::Unsupported, "operation not supported on ThingOS")
}

pub fn decode_error_kind(_code: i32) -> crate::io::ErrorKind {
    crate::io::ErrorKind::Other
}

pub fn abort_internal() -> ! {
    unsafe { core::intrinsics::abort() }
}

pub fn hashmap_random_keys() -> (u64, u64) {
    (0, 0)
}
