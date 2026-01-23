pub use alloc::sync::{Arc, Weak};
pub use spin::{Mutex, MutexGuard};

pub mod atomic {
    pub use core::sync::atomic::*;
}
