use abi::display::DisplayInfo;
use abi::wire::SyscallResult;
use abi::syscall::err;

pub fn sys_display_primary(out_ptr: *mut DisplayInfo, out_len: usize) -> SyscallResult {
    use crate::log::{klog, Level};

    if out_ptr.is_null() {
        klog(Level::Error, "DISPLAY", "sys_display_primary: null pointer");
        return SyscallResult::new(err::EFAULT, 0, 0);
    }
    
    if out_len < core::mem::size_of::<DisplayInfo>() {
        klog(Level::Error, "DISPLAY", "sys_display_primary: buffer too small");
        return SyscallResult::new(err::EINVAL, 0, 0);
    }

    if let Some(info) = crate::display::get_primary() {
        // Safe Copy
        let src_bytes = unsafe {
             core::slice::from_raw_parts(&info as *const DisplayInfo as *const u8, core::mem::size_of::<DisplayInfo>())
        };
        
        match crate::memory::copy_to_user(out_ptr as u64, src_bytes) {
            Ok(_) => {
                 klog(Level::Info, "DISPLAY", "sys_display_primary: success");
                 SyscallResult::new(0, 0, 0)
            },
            Err(e) => {
                 klog(Level::Error, "DISPLAY", "sys_display_primary: copy failed");
                 SyscallResult::new(e, 0, 0)
            }
        }
    } else {
        klog(Level::Warn, "DISPLAY", "sys_display_primary: no primary display");
        SyscallResult::new(err::ENOENT, 0, 0)
    }
}
