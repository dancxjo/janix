#![no_std]
#![feature(prelude_import)]

extern crate alloc;

#[cfg(feature = "rt")]
pub use stem::main;

pub mod fs;
pub mod io;
pub mod path;
pub mod process;
pub mod sync;
pub mod thread;
pub mod time;

pub mod prelude {
    pub mod v1 {
        pub use alloc::boxed::Box;
        pub use alloc::borrow::ToOwned;
        pub use alloc::string::String;
        pub use alloc::vec::Vec;
        pub use core::prelude::v1::*;
        pub use core::format_args;
    }

    pub mod rust_2024 {
        pub use alloc::boxed::Box;
        pub use alloc::borrow::ToOwned;
        pub use alloc::string::String;
        pub use alloc::vec::Vec;
        pub use core::prelude::v1::*;
        pub use core::format_args;
    }
}

#[prelude_import]
#[allow(unused)]
use prelude::v1::*;

pub mod collections {
    pub use alloc::collections::*;
}

pub mod string {
    pub use alloc::string::*;
}

pub mod vec {
    pub use alloc::vec::*;
}

pub mod boxed {
    pub use alloc::boxed::*;
}

pub mod borrow {
    pub use alloc::borrow::*;
}

pub mod error {
    pub use core::error::Error;
}

pub use core::any;
pub use core::cell;
pub use core::cmp;
pub use core::convert;
pub use core::default;
pub use core::fmt;
pub use core::hash;
pub use core::iter;
pub use core::mem;
pub use core::num;
pub use core::ops;
pub use core::ptr;
pub use core::result;
pub use core::slice;
pub use core::str;

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {{
        $crate::io::_print(core::format_args!($($arg)*));
    }};
}

#[macro_export]
macro_rules! println {
    () => {{
        $crate::print!("\n");
    }};
    ($($arg:tt)*) => {{
        $crate::print!("{}\n", core::format_args!($($arg)*));
    }};
}

#[macro_export]
macro_rules! eprint {
    ($($arg:tt)*) => {{
        $crate::io::_eprint(core::format_args!($($arg)*));
    }};
}

#[macro_export]
macro_rules! eprintln {
    () => {{
        $crate::eprint!("\n");
    }};
    ($($arg:tt)*) => {{
        $crate::eprint!("{}\n", core::format_args!($($arg)*));
    }};
}
