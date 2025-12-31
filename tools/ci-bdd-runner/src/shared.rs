use std::sync::Arc;
use tokio::sync::Mutex;
use crate::qemu::QemuProcess;

// We use a global mutex to allow the synchronous Cucumber Writer to access 
// the QEMU process spawned by the async steps.
lazy_static::lazy_static! {
    pub static ref GLOBAL_QEMU: Arc<Mutex<Option<QemuProcess>>> = Arc::new(Mutex::new(None));
}
