//! ThingOS Syscall Constants
//!
//!
pub mod asm;
pub mod conv;

mod numbers {
    include!("numbers.rs");
}
pub use numbers::*;


// pollfds entry and other types remain here because they might need serde
/// Entry in the `pollfds` array passed to [`SYS_FS_POLL`].
///
/// Layout mirrors POSIX `struct pollfd` so that future libc ports can
/// alias this directly.
#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct PollFd {
    /// File descriptor to watch.
    pub fd: i32,
    /// Events to wait for (input, using [`poll_flags`]).
    pub events: u16,
    /// Events that occurred (output, filled by the kernel).
    pub revents: u16,
}
