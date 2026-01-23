
use crate::fmt;
use crate::ffi::OsString;

pub unsafe fn init(_argc: isize, _argv: *const *const u8) {}
pub unsafe fn cleanup() {}

pub struct Args {
    iter: crate::iter::Empty<OsString>,
}

pub fn args() -> Args {
    Args { iter: crate::iter::empty() }
}

impl Iterator for Args {
    type Item = OsString;
    fn next(&mut self) -> Option<OsString> {
        self.iter.next()
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl ExactSizeIterator for Args {
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl DoubleEndedIterator for Args {
    fn next_back(&mut self) -> Option<OsString> {
        self.iter.next_back()
    }
}

impl fmt::Debug for Args {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().finish()
    }
}
