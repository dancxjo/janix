use crate::syscall::syscall6;
use abi::device::{DeviceCall, DeviceKind, RtcTime, RTC_OP_READ_TIME};
use abi::errors::Errno;
use abi::syscall::SYS_DEVICE_CALL;

pub fn device_call(call: &mut DeviceCall) -> Result<usize, Errno> {
    let ret = unsafe { syscall6(SYS_DEVICE_CALL, call as *mut _ as usize, 0, 0, 0, 0, 0) };
    abi::errors::errno(ret)
}

pub fn rtc_read_time() -> Result<RtcTime, Errno> {
    let mut time = RtcTime::default();
    let mut call = DeviceCall {
        kind: DeviceKind::RtcCmos,
        op: RTC_OP_READ_TIME,
        in_ptr: 0,
        in_len: 0,
        out_ptr: &mut time as *mut _ as u64,
        out_len: core::mem::size_of::<RtcTime>() as u32,
    };

    device_call(&mut call)?;
    Ok(time)
}
