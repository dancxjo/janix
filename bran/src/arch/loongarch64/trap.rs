use kernel::boot::ArchTrapFrame;

#[derive(Default, Debug, Clone, Copy)]
#[repr(C)]
pub struct TrapFrame {
    pub dummy: u64,
}

impl ArchTrapFrame for TrapFrame {
    fn as_any(&self) -> &dyn core::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn core::any::Any {
        self
    }

    fn syscall_arg(&self, _idx: usize) -> u64 { 0 }
    fn syscall_ret(&mut self, _val: u64) {}
    fn syscall_num(&self) -> u64 { 0 }
    fn set_user_stack(&mut self, _val: u64) {}
    fn user_stack(&self) -> u64 { 0 }
    fn set_user_ip(&mut self, _val: u64) {}
    fn user_ip(&self) -> u64 { 0 }
}
