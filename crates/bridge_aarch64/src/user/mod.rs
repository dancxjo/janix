pub mod enter;
#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct UserEntryRegs {
    pub entry_point: u64,
    pub user_stack: u64,
    pub arg0: u64,
}
