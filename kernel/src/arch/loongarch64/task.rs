#[repr(C)]
#[derive(Debug, Default)]
pub struct ArchContext {
    _dummy: u64,
}

pub unsafe fn context_switch(_old: *mut ArchContext, _new: *const ArchContext) {
    unimplemented!("context_switch not implemented for loongarch64");
}

pub fn context_init(
    _ctx: &mut ArchContext,
    _kstack_top: u64,
    _entry: extern "C" fn(arg: usize) -> !,
    _arg: usize,
) {
    unimplemented!("context_init not implemented for loongarch64");
}
