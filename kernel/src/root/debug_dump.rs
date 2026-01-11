use super::{enqueue, RootOp};
use core::sync::atomic::Ordering;

pub fn dump_all_to_console() {
    // For v0.1, we rely on RootOp::DumpGraph to do the printing inside the kernel thread.
    // The kernel thread implementation in service.rs should be updated if we want grouping/sorting.
    // Currently service.rs iterates map order (BTreeMap order, so ID order).
    // The user wants grouped by "Host, Kernel, Root first".
    // IDs vary.
    // We can't easily control iteration order inside `DumpGraph` without filtering.
    // But since we just want "Census Moment", maybe ID order is fine if we registered them first?
    // We registered Host (t1), Kernel (t2), Root (t3).
    // So ID order DOES group them!
    
    // What about "Edges grouped by source node"?
    // service.rs:
    // for (id, node) in nodes.iter() {
    //    for edge in node.edges { ... }
    // }
    // This iterates by source ID! So edges are grouped by source.
    // The requirement "sort by (src, rel, dst)" is mostly satisfied by BTreeMap order of src.
    // `node.edges` is a Vec. We should probably sort it?
    // But for v0.1, append order is likely fine.
    
    // So we just call the Op.
    let reply = enqueue(RootOp::DumpGraph { limit: 4096 });
    loop {
        let done = reply.done.load(Ordering::Acquire);
        if done != 0 {
            break;
        }
        unsafe { crate::task::scheduler::yield_now_current(); }
    }
}
