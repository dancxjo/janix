pub mod ring;
pub mod flusher;

#[cfg(test)]
mod tests;

pub use ring::{LogRing, EntryKind};

// Macros for logging
#[macro_export]
macro_rules! klog {
    ($level:expr, $msg:expr) => {
        $crate::diag::LogRing::global().push(
            $crate::diag::EntryKind::Log,
            $level,
            $msg,
            0, 0, 0, 0
        );
    };
    ($level:expr, $msg:expr, $a:expr) => {
        $crate::diag::LogRing::global().push(
            $crate::diag::EntryKind::Log,
            $level,
            $msg,
            $a as u64, 0, 0, 0
        );
    };
}

// Trap-safe recording
pub fn record_fault(
    rip: u64, _rsp: u64, rflags: u64, cr2: u64,
    error_code: u64, fault_kind: u8,
    msg: &str
) {
    LogRing::global().push(
        EntryKind::Fault,
        fault_kind, // level reused as fault_kind
        msg,
        rip,
        error_code,
        cr2,
        rflags
    );
}

// Helper for panic
pub fn record_panic(msg: &str) {
    LogRing::global().push(
        EntryKind::Error,
        5, // Fatal
        msg,
        0, 0, 0, 0
    );
}
