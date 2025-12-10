use core::sync::atomic::{AtomicU64, Ordering};

static HHDM_OFFSET: AtomicU64 = AtomicU64::new(0);

pub fn set_hhdm_offset(offset: u64) {
    HHDM_OFFSET.store(offset, Ordering::Relaxed);
}

pub fn get_hhdm_offset() -> u64 {
    HHDM_OFFSET.load(Ordering::Relaxed)
}

pub fn phys_to_virt(phys: u64) -> u64 {
    phys + get_hhdm_offset()
}
