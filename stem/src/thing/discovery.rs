use crate::errors::Errno;
use crate::graph_wait::GraphWatch;
use crate::thing::{sys, ThingId};
use crate::wait_set::WaitSet;

/// Blocks the current task until a node of the specified `kind` is found in the System Graph.
/// Returns the `ThingId` of that node.
pub fn wait_for_kind(kind: u64) -> Result<ThingId, Errno> {
    let mut buf = [ThingId::default(); 1];

    // Initial check without setting up watches
    if let Ok(count) = sys::find(kind, &mut buf) {
        if count > 0 {
            return Ok(buf[0]);
        }
    }

    // Set up a global graph watch.
    // It is more efficient than busy-polling via `yield_now()`.
    let watch = GraphWatch::open_all()?;
    let mut ws = WaitSet::new();
    let _token = ws.add_root_watch(watch.id())?;

    loop {
        // Block until any graph mutation happens.
        // We use None for timeout to sleep indefinitely until awakened.
        let _ = ws.wait(None::<crate::time::Duration>);

        // Drain the watch events to reset the readable state and prevent QueueFull deadlocks.
        let mut ev_buf = [0u8; 512];
        let mut seq = 0;
        while let Ok(_) = watch.next(&mut seq, &mut ev_buf) {}

        // Re-check after waking up
        if let Ok(count) = sys::find(kind, &mut buf) {
            if count > 0 {
                let _ = watch.close();
                return Ok(buf[0]);
            }
        }
    }
}
