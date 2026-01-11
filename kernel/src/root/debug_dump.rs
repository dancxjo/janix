use super::{enqueue, RootOp};
use core::sync::atomic::Ordering;

pub fn dump_all_to_console() {
    let reply = enqueue(RootOp::DumpGraph { limit: 4096 });
    loop {
        let done = reply.done.load(Ordering::Acquire);
        if done != 0 {
            break;
        }
        unsafe { crate::task::scheduler::yield_now_current(); }
    }
}
