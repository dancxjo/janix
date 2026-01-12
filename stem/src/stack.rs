use crate::errors::Errno;

#[derive(Clone, Copy, Debug)]
pub struct Stack {
    top: usize,
}

impl Stack {
    pub const fn from_top(top: usize) -> Self {
        Self { top }
    }

    pub fn top(&self) -> usize {
        self.top
    }

    pub fn alloc_default() -> Result<Self, Errno> {
        Self::alloc_pages(0)
    }

    pub fn alloc_pages(pages: usize) -> Result<Self, Errno> {
        crate::syscall::alloc_stack(pages).map(Stack::from_top)
    }
}
