use crate::BootRuntime;

use core::alloc::Layout;
use core::sync::atomic::{AtomicUsize, Ordering};

/// A guard that ensures SIMD/FPU is enabled and safe to use.
pub struct SimdGuard<'a> {
    _rt: &'a dyn BootRuntime,
}

impl<'a> SimdGuard<'a> {
    pub fn enter(rt: &'a dyn BootRuntime) -> Self {
        rt.simd_init_cpu();
        SimdGuard { _rt: rt }
    }
}

impl<'a> Drop for SimdGuard<'a> {
    fn drop(&mut self) {}
}

pub fn with_simd<R>(rt: &dyn BootRuntime, f: impl FnOnce() -> R) -> R {
    let _g = SimdGuard::enter(rt);
    f()
}

// Internal Bump Allocator
// 16KB buffer for SIMD states. 
// Sufficient for ~30 x86_64 contexts (512 bytes) or ~30 aarch64 contexts.
const HEAP_SIZE: usize = 16384;
static mut SIMD_HEAP: [u8; HEAP_SIZE] = [0; HEAP_SIZE];
static HEAP_TOP: AtomicUsize = AtomicUsize::new(0);

fn internal_alloc(layout: Layout) -> *mut u8 {
        // Simple CAS loop for thread safety (though we are mostly single threaded at boot)
        loop {
            let top = HEAP_TOP.load(Ordering::Relaxed);
            // Use addr_of_mut! to avoid creating a reference to static mut
            let base = core::ptr::addr_of_mut!(SIMD_HEAP) as usize;
            let current_ptr = base + top;
            
            let align_offset = (layout.align() - (current_ptr % layout.align())) % layout.align();
            let new_top = top + align_offset + layout.size();
            
            if new_top > HEAP_SIZE {
                return core::ptr::null_mut();
            }
            
            if HEAP_TOP.compare_exchange(top, new_top, Ordering::Relaxed, Ordering::Relaxed).is_ok() {
                return (base + top + align_offset) as *mut u8;
            }
        }
}

fn internal_dealloc(_ptr: *mut u8, _layout: Layout) {
    // Leaky allocator: we don't support deallocation in this simple boot-time helper.
    // In a real system, tasks would be managed by the slab allocator.
}

/// Per-task SIMD state storage.
pub struct SimdState {
    buffer: *mut u8,
    layout: Layout,
    valid: bool,
}

unsafe impl Send for SimdState {}
unsafe impl Sync for SimdState {}

impl SimdState {
    pub fn new(rt: &dyn BootRuntime) -> Self {
        let (size, align) = rt.simd_state_layout();
        if size == 0 {
            return Self {
                buffer: core::ptr::null_mut(),
                layout: Layout::from_size_align(0, 1).unwrap(),
                valid: false,
            };
        }

        let layout = Layout::from_size_align(size, align).expect("Invalid SIMD layout");
        let buffer = internal_alloc(layout);
        
        if buffer.is_null() {
            // Panic if we run out of static SIMD heap
            panic!("OOM allocating SimdState");
        }

        // Initialize to 0
        unsafe { core::ptr::write_bytes(buffer, 0, size) };

        Self {
            buffer,
            layout,
            valid: true,
        }
    }

    pub fn save(&mut self, rt: &dyn BootRuntime) {
        if !self.buffer.is_null() {
            unsafe { rt.simd_save(self.buffer) };
            self.valid = true;
        }
    }

    pub fn restore(&self, rt: &dyn BootRuntime) {
        if !self.buffer.is_null() && self.valid {
            unsafe { rt.simd_restore(self.buffer) };
        }
    }
}

impl Drop for SimdState {
    fn drop(&mut self) {
        if !self.buffer.is_null() {
            internal_dealloc(self.buffer, self.layout);
        }
    }
}

/// Run a self-test of the SIMD save/restore mechanism.
pub fn self_test(rt: &dyn BootRuntime) {
    use crate::kinfo;

    let (size, align) = rt.simd_state_layout();
    if size == 0 {
        kinfo!("SIMD self-test skipped (no SIMD support)");
        return;
    }

    with_simd(rt, || {
        // Use stack allocation for self-test to avoid consuming heap
        // Max size expected is usually < 1KB (512 bytes for AVX/SSE/NEON)
        
        // We define a struct with high alignment to ensure we cover requirements
        #[repr(align(16))]
        struct AlignedStorage([u8; 1024]);
        
        let mut storage = AlignedStorage([0; 1024]);
        let buffer = storage.0.as_mut_ptr();
        
        if size > 1024 || align > 16 {
             kinfo!("SIMD self-test skipped (size/align too large for stack buffer)");
             return;
        }
        
        // Save current state
        unsafe { rt.simd_save(buffer) };
        
        // Restore it
        unsafe { rt.simd_restore(buffer) };
        
        kinfo!("SIMD self-test passed (save/restore cycle)");
    });
}
