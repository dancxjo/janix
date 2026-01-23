
use crate::io;

#[derive(Copy, Clone)]
pub struct IoSlice<'a>(&'a [u8]);

impl<'a> IoSlice<'a> {
    #[inline]
    pub fn new(buf: &'a [u8]) -> IoSlice<'a> {
        IoSlice(buf)
    }

    #[inline]
    pub fn advance(&mut self, n: usize) {
        if self.0.len() < n {
            panic!("advancing IoSlice beyond its length");
        }
        self.0 = &self.0[n..];
    }

    #[inline]
    pub fn as_slice(&self) -> &[u8] {
        self.0
    }
}

pub struct IoSliceMut<'a>(&'a mut [u8]);

impl<'a> IoSliceMut<'a> {
    #[inline]
    pub fn new(buf: &'a mut [u8]) -> IoSliceMut<'a> {
        IoSliceMut(buf)
    }

    #[inline]
    pub fn advance(&mut self, n: usize) {
        if self.0.len() < n {
            panic!("advancing IoSliceMut beyond its length");
        }
        let len = self.0.len();
        let ptr = self.0.as_mut_ptr();
        // SAFETY: The pointer arithmetic is valid because we check n <= len
        unsafe {
            self.0 = crate::slice::from_raw_parts_mut(ptr.add(n), len - n);
        }
    }

    #[inline]
    pub fn as_slice(&self) -> &[u8] {
        self.0
    }

    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        self.0
    }
}

pub fn is_terminal(_: &dyn io::IsTerminal) -> bool {
    false
}
