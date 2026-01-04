use super::allocator::ALLOCATOR;

/// Configuration for the kernel heap
#[derive(Debug, Clone, Copy)]
pub struct HeapConfig {
    pub phys_base: u64,
    pub virt_base: u64,
    pub size: usize,
}

/// Initialize the kernel heap
///
/// # Safety
/// Caller must ensure that the memory region is valid, mapped, and unused.
pub unsafe fn init(config: HeapConfig) -> Result<(), ()> {
    // Initialize the global allocator
    ALLOCATOR
        .lock()
        .init(config.virt_base as *mut u8, config.size);

    crate::log::klog(
        crate::log::Level::Info,
        "HEAP",
        &alloc::format!(
            "init: base={:#x} size={} MiB",
            config.virt_base,
            config.size / 1024 / 1024
        ),
    );

    Ok(())
}

pub struct HeapStats {
    pub total: usize,
    pub used: usize,
    pub free: usize,
}

pub fn heap_stats() -> HeapStats {
    let size = ALLOCATOR.lock().size();
    let used = ALLOCATOR.lock().used();

    HeapStats {
        total: size,
        used,
        free: size - used,
    }
}
