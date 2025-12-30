//! Errno definitions for system call return values.
//!
//! Syscalls return `isize` where negative values are `-errno`.

#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Errno {
    /// Invalid argument
    EINVAL = 22,
    /// No such file or directory
    ENOENT = 2,
    /// Operation not permitted
    EPERM = 1,
    /// No such process
    ESRCH = 3,
    /// I/O error
    EIO = 5,
    /// No such device or address
    ENXIO = 6,
    /// Argument list too long
    E2BIG = 7,
    /// Bad file descriptor
    EBADF = 9,
    /// Try again / Would block
    EAGAIN = 11,
    /// Out of memory
    ENOMEM = 12,
    /// Permission denied
    EACCES = 13,
    /// Bad address
    EFAULT = 14,
    /// Device or resource busy
    EBUSY = 16,
    /// File exists
    EEXIST = 17,
    /// No such device
    ENODEV = 19,
    /// Not a directory
    ENOTDIR = 20,
    /// Is a directory
    EISDIR = 21,
    /// File too large
    EFBIG = 27,
    /// No space left on device
    ENOSPC = 28,
    /// Read-only file system
    EROFS = 30,
    /// Too many links
    EMLINK = 31,
    /// Broken pipe
    EPIPE = 32,
    /// Math argument out of domain
    EDOM = 33,
    /// Math result not representable
    ERANGE = 34,
    /// Resource deadlock would occur
    EDEADLK = 35,
    /// File name too long
    ENAMETOOLONG = 36,
    /// No locks available
    ENOLCK = 37,
    /// Function not implemented
    ENOSYS = 38,
    /// Directory not empty
    ENOTEMPTY = 39,
    /// Too many symbolic links
    ELOOP = 40,
    /// No message of desired type
    ENOMSG = 42,
    /// Not supported
    ENOTSUP = 95,
}

impl Errno {
    /// Convert to negative isize for syscall return.
    #[inline]
    pub const fn neg(self) -> isize {
        -(self as i32 as isize)
    }
}

/// Convert an errno to a negative isize for syscall return.
#[inline]
pub const fn neg(e: Errno) -> isize {
    e.neg()
}
