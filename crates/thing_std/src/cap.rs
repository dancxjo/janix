use crate::syscall;
use abi::syscall::nr;
use abi::ids::ThingId;
pub use abi::cap::{Cap, CapOp, CapScope};

pub fn grant(target: ThingId, cap: Cap) -> Result<(), i32> {
    unsafe {
        let res = syscall(
            nr::SYS_CAP_GRANT,
            target.low(),
            target.high(),
            &cap as *const Cap as u64,
            0,
            0,
            0,
        );
        if res.status != 0 {
            Err(res.status as i32)
        } else {
            Ok(())
        }
    }
}
