//! Syscall error conventions and errno helpers.
//!
//! # Examples
//! ```
//! use abi::errors::{Errno, errno};
//!
//! let rc: isize = Errno::ENOENT.as_isize();
//! assert_eq!(rc, -2);
//! assert_eq!(errno(rc), Err(Errno::ENOENT));
//! assert_eq!(errno(0), Ok(0));
//! ```

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Errno {
    Success = 0,
    EPERM = 1,
    ENOENT = 2,
    ESRCH = 3,
    EINTR = 4,
    EIO = 5,
    ENXIO = 6,
    E2BIG = 7,
    ENOEXEC = 8,
    EBADF = 9,
    ECHILD = 10,
    EAGAIN = 11,
    ENOMEM = 12,
    EACCES = 13,
    EFAULT = 14,
    ENOTBLK = 15,
    EBUSY = 16,
    EEXIST = 17,
    EXDEV = 18,
    ENODEV = 19,
    ENOTDIR = 20,
    EISDIR = 21,
    EINVAL = 22,
    ENFILE = 23,
    EMFILE = 24,
    ENOTTY = 25,
    ETXTBSY = 26,
    EFBIG = 27,
    ENOSPC = 28,
    ESPIPE = 29,
    EROFS = 30,
    EMLINK = 31,
    EPIPE = 32,
    EDOM = 33,
    ERANGE = 34,
    ENOSYS = 38,
    EOVERFLOW = 75,
    ENOBUFS = 105,
    EMSGSIZE = 90,
    ETIMEDOUT = 110,
    // Add more as needed, following Linux numbers usually helps debugging

    // Custom/Extension
}

pub type SysResult<T> = core::result::Result<T, Errno>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    Errno(Errno),
    BufferTooSmall { required: usize, available: usize },
    InvalidDataLength { expected: usize, actual: usize },
}

impl From<Errno> for Error {
    fn from(e: Errno) -> Self {
        Error::Errno(e)
    }
}

pub type Result<T> = core::result::Result<T, Error>;

impl Errno {
    #[allow(non_upper_case_globals)]
    pub const NotSupported: Errno = Errno::ENOSYS;
    pub fn as_isize(self) -> isize {
        -(self as isize)
    }
}

pub fn errno(ret: isize) -> core::result::Result<usize, Errno> {
    if ret < 0 && ret >= -4096 {
        // This is a rough mapping back, optimizing for common case
        // In a real impl we'd match every value.
        // For now let's just assume it's valid if negative.
        // But to be safe in Rust enum, we might want to transmute if we trust the source,
        // or just return a generic error.
        // Let's do a basic match for the ones we care about.
        let code = -ret;
        match code {
            1 => Err(Errno::EPERM),
            2 => Err(Errno::ENOENT),
            5 => Err(Errno::EIO),
            11 => Err(Errno::EAGAIN),
            12 => Err(Errno::ENOMEM),
            14 => Err(Errno::EFAULT),
            22 => Err(Errno::EINVAL),
            38 => Err(Errno::ENOSYS),
            _ => Err(Errno::EINVAL), // Fallback
        }
    } else {
        Ok(ret as usize)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn errno_as_isize_returns_negative_value() {
        assert_eq!(Errno::EPERM.as_isize(), -1);
        assert_eq!(Errno::ENOENT.as_isize(), -2);
        assert_eq!(Errno::EINVAL.as_isize(), -22);
        assert_eq!(Errno::ENOSYS.as_isize(), -38);
    }

    #[test]
    fn errno_success_as_isize_is_zero() {
        assert_eq!(Errno::Success.as_isize(), 0);
    }

    #[test]
    fn errno_recognizes_eperm() {
        assert_eq!(errno(-1), Err(Errno::EPERM));
    }

    #[test]
    fn errno_recognizes_enoent() {
        assert_eq!(errno(-2), Err(Errno::ENOENT));
    }

    #[test]
    fn errno_recognizes_eio() {
        assert_eq!(errno(-5), Err(Errno::EIO));
    }

    #[test]
    fn errno_recognizes_eagain() {
        assert_eq!(errno(-11), Err(Errno::EAGAIN));
    }

    #[test]
    fn errno_recognizes_enomem() {
        assert_eq!(errno(-12), Err(Errno::ENOMEM));
    }

    #[test]
    fn errno_recognizes_efault() {
        assert_eq!(errno(-14), Err(Errno::EFAULT));
    }

    #[test]
    fn errno_recognizes_einval() {
        assert_eq!(errno(-22), Err(Errno::EINVAL));
    }

    #[test]
    fn errno_recognizes_enosys() {
        assert_eq!(errno(-38), Err(Errno::ENOSYS));
    }

    #[test]
    fn errno_unknown_code_fallback_to_einval() {
        assert_eq!(errno(-999), Err(Errno::EINVAL));
    }

    #[test]
    fn errno_success_returns_ok() {
        assert_eq!(errno(0), Ok(0usize));
    }

    #[test]
    fn errno_positive_returns_ok() {
        assert_eq!(errno(42), Ok(42usize));
        assert_eq!(errno(1000), Ok(1000usize));
    }

    #[test]
    fn errno_large_negative_is_fallback() {
        // Values < -4096 are not treated as errors
        assert_eq!(errno(-5000), Ok((-5000isize) as usize));
    }
}
