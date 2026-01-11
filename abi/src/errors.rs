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
    // Add more as needed, following Linux numbers usually helps debugging
    
    // Custom/Extension
}

pub type SysResult<T> = Result<T, Errno>;

impl Errno {
    #[allow(non_upper_case_globals)]
    pub const NotSupported: Errno = Errno::ENOSYS;
    pub fn as_isize(self) -> isize {
        -(self as isize)
    }
}

pub fn errno(ret: isize) -> Result<usize, Errno> {
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
