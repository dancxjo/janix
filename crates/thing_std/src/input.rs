use super::*;

pub fn read(buf: &mut [u8]) -> usize {
    let res = unsafe {
        syscall(
            nr::SYS_INPUT_READ,
            buf.as_mut_ptr() as u64,
            buf.len() as u64,
            0,
            0,
            0,
            0,
        )
    };
    res.val0 as usize
}
