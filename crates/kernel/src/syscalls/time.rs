use crate::bridge::HardwareBridge;
use crate::Kernel;
use abi::syscall_defs::SYS_EAGAIN;
use abi::wire::time::RtcSample;

pub fn sys_rtc_read<B: HardwareBridge>(kernel: &mut Kernel<B>, out_ptr: *mut u8) -> isize {
    // Safety: User pointer validation required in real OS.
    // v0: assume valid and aligned.
    if out_ptr as usize == 0 {
        return -1;
    }

    let out = unsafe { &mut *(out_ptr as *mut RtcSample) };
    kernel.bridge.rtc_read(out);

    0
}
