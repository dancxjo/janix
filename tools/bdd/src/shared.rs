use crate::qemu::QemuProcess;
use tokio::sync::Mutex;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref GLOBAL_QEMU: Arc<Mutex<Option<QemuProcess>>> = Arc::new(Mutex::new(None));
    pub static ref GLOBAL_LAST_ERROR: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(None));
    pub static ref ANY_FAILURE: AtomicBool = AtomicBool::new(false);
}

pub fn slugify(s: &str) -> String {
    s.chars().map(|c| if c.is_alphanumeric() { c.to_ascii_lowercase() } else { '_' }).collect()
}
