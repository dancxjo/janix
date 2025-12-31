use crate::qemu::QemuProcess;
use std::sync::Arc;
use tokio::sync::Mutex;

// We use a global mutex to allow the synchronous Cucumber Writer to access
// the QEMU process spawned by the async steps.
lazy_static::lazy_static! {
    pub static ref GLOBAL_QEMU: Arc<Mutex<Option<QemuProcess>>> = Arc::new(Mutex::new(None));
    pub static ref GLOBAL_LAST_ERROR: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(None));
}

pub static ANY_FAILURE: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
