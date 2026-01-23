
use crate::ffi::{OsStr, OsString};
use crate::io;

pub mod os {
    pub const FAMILY: &str = "thingos";
    pub const OS: &str = "thingos";
    pub const DLL_PREFIX: &str = "";
    pub const DLL_SUFFIX: &str = "";
    pub const DLL_EXTENSION: &str = "";
    pub const EXE_SUFFIX: &str = "";
    pub const EXE_EXTENSION: &str = "";
}

pub fn init() {}

#[derive(Debug)]
pub struct Env(crate::iter::Empty<(OsString, OsString)>);

impl Iterator for Env {
    type Item = (OsString, OsString);
    fn next(&mut self) -> Option<Self::Item> { None }
}

pub fn env() -> Env { Env(crate::iter::empty()) }

pub fn getenv(k: &OsStr) -> Option<OsString> { None }
pub fn setenv(k: &OsStr, v: &OsStr) -> io::Result<()> { Err(io::Error::UNSUPPORTED_PLATFORM) }
pub fn unsetenv(k: &OsStr) -> io::Result<()> { Err(io::Error::UNSUPPORTED_PLATFORM) }

pub fn var(key: &OsStr) -> Option<OsString> { getenv(key) }
pub fn vars() -> impl Iterator<Item = (OsString, OsString)> { env() }
pub fn set_var(key: &OsStr, value: &OsStr) { let _ = setenv(key, value); }
pub fn remove_var(key: &OsStr) { let _ = unsetenv(key); }
