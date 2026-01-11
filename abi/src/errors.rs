//! Error codes and helpers.

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Errno {
    Success = 0,
    Perm = 1,
    NoEnt = 2,
    IO = 5,
    BadF = 9,
    Again = 11,
    Nomem = 12,
    Fault = 14,
    Inval = 22,
    Nospc = 28,
    Nosys = 38,
    NotSupported = 95,
    TimedOut = 110,
}

impl Errno {
    pub fn from_isize(val: isize) -> Errno {
        match val {
            0 => Errno::Success,
            -1 => Errno::Perm,
            -2 => Errno::NoEnt,
            -5 => Errno::IO,
            -9 => Errno::BadF,
            -11 => Errno::Again,
            -12 => Errno::Nomem,
            -14 => Errno::Fault,
            -22 => Errno::Inval,
            -28 => Errno::Nospc,
            -38 => Errno::Nosys,
            -95 => Errno::NotSupported,
            -110 => Errno::TimedOut,
            _ => Errno::Inval, // Fallback
        }
    }
}

/// Convert a system call return value (isize) into a Result.
///
/// Post-condition:
/// - If `res` < 0: returns `Err(Errno::from_isize(res))`
/// - If `res` >= 0: returns `Ok(res as usize)`
pub fn errno(res: isize) -> Result<usize, Errno> {
    if res < 0 {
        Err(Errno::from_isize(res))
    } else {
        Ok(res as usize)
    }
}
