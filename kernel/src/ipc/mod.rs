//! Kernel IPC: Port-based inter-process communication
//!
//! Provides capability-gated byte pipes for userspace communication.

mod handles;
mod port;

pub use handles::{Handle, HandleEntry, HandleMode, HandleTable, MAX_HANDLES};
pub use port::{Port, PortId, Sender, Receiver};

use alloc::sync::Arc;
use alloc::vec::Vec;
use spin::Mutex;

/// Global port registry
static PORTS: Mutex<Vec<Option<Arc<Port>>>> = Mutex::new(Vec::new());

/// Global Handle Table (Single Process Model for v0)
pub static GLOBAL_HANDLE_TABLE: Mutex<HandleTable> = Mutex::new(HandleTable::new());

/// Create a new port and return its ID
pub fn create_port(capacity: usize) -> PortId {
    let port = Arc::new(Port::new(capacity));
    let mut ports = PORTS.lock();
    
    // Find a free slot or append
    for (i, slot) in ports.iter_mut().enumerate() {
        if slot.is_none() {
            *slot = Some(port);
            return PortId(i as u32);
        }
    }
    
    // No free slot, append
    let id = ports.len() as u32;
    ports.push(Some(port));
    PortId(id)
}

/// Get a port by ID
pub fn get_port(id: PortId) -> Option<Arc<Port>> {
    let ports = PORTS.lock();
    ports.get(id.0 as usize).and_then(|opt| opt.clone())
}

/// Close a port (for cleanup)
pub fn close_port(id: PortId) {
    let mut ports = PORTS.lock();
    if let Some(slot) = ports.get_mut(id.0 as usize) {
        *slot = None;
    }
}

/// Get statistics about the port registry (for debugging)
pub fn port_count() -> usize {
    let ports = PORTS.lock();
    ports.iter().filter(|s| s.is_some()).count()
}
