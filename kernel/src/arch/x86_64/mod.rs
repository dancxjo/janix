
// Re-export TrapFrame from trap module to ensure single source of truth
pub use crate::trap::x86_64::TrapFrame;

#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct Context(pub TrapFrame);
