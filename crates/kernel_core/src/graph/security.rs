use abi::ThingId;
use abi::wire::graph::GraphOp;
use abi::ids::ProcessId;

pub enum Error {
    Denied,
    // LimitExceeded, // for rate limits later
}

pub fn check_write(_pid: ProcessId, op: &GraphOp) -> Result<(), Error> {
    // Phase 2 Policy:
    // "One module: kernel_core::graph::security"
    // "All mutation entry points call a single check_write(pid, op) before committing."
    
    // For now, allow everything.
    // The task is to "Centralize" it, so we stub it here and call it.
    match op {
        // Reads are always allowed in this model? 
        // Or check_write only checks mutations?
        // Implementation plan says "check_write(pid, op, target)".
        // Ops contain targets.
        
        GraphOp::CreateThing { .. } => Ok(()),
        GraphOp::UpdateThing { .. } => Ok(()),
        GraphOp::DeleteThing { .. } => Ok(()),
        GraphOp::AddLink { .. } => Ok(()),
        GraphOp::Batch(ops) => {
            for sub in ops {
                check_write(_pid, sub)?;
            }
            Ok(())
        },
        _ => Ok(()), // Reads/Queries
    }
}
