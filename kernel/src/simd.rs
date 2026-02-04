use crate::BootRuntime;
use core::alloc::Layout;
use core::sync::atomic::{AtomicUsize, Ordering};

pub struct SimdGuard<'a, R: BootRuntime> {
    _rt: &'a R,
}

impl<'a, R: BootRuntime> SimdGuard<'a, R> {
    pub fn enter(rt: &'a R) -> Self {
        rt.simd_init_cpu();
        SimdGuard { _rt: rt }
    }
}

impl<'a, R: BootRuntime> Drop for SimdGuard<'a, R> {
    fn drop(&mut self) {}
}

pub fn with_simd<R: BootRuntime, T>(rt: &R, f: impl FnOnce() -> T) -> T {
    let _g = SimdGuard::enter(rt);
    f()
}

const HEAP_SIZE: usize = 65536;
static mut SIMD_HEAP: [u8; HEAP_SIZE] = [0; HEAP_SIZE];
static HEAP_TOP: AtomicUsize = AtomicUsize::new(0);

fn internal_alloc(layout: Layout) -> *mut u8 {
    loop {
        let top = HEAP_TOP.load(Ordering::Relaxed);
        let base = core::ptr::addr_of_mut!(SIMD_HEAP) as usize;
        let current_ptr = base + top;

        let align_offset = (layout.align() - (current_ptr % layout.align())) % layout.align();
        let new_top = top + align_offset + layout.size();

        if new_top > HEAP_SIZE {
            return core::ptr::null_mut();
        }

        if HEAP_TOP
            .compare_exchange(top, new_top, Ordering::Relaxed, Ordering::Relaxed)
            .is_ok()
        {
            return (base + top + align_offset) as *mut u8;
        }
    }
}

pub struct SimdState {
    buffer: *mut u8,
    #[allow(dead_code)]
    layout: Layout,
    valid: bool,
}

unsafe impl Send for SimdState {}
unsafe impl Sync for SimdState {}

impl SimdState {
    pub fn new<R: BootRuntime>(rt: &R) -> Self {
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
            panic!("OOM allocating SimdState");
        }

        unsafe { core::ptr::write_bytes(buffer, 0, size) };

        Self {
            buffer,
            layout,
            valid: true,
        }
    }

    pub fn save<R: BootRuntime>(&mut self, rt: &R) {
        if !self.buffer.is_null() {
            unsafe { rt.simd_save(self.buffer) };
            self.valid = true;
        }
    }

    pub fn restore<R: BootRuntime>(&self, rt: &R) {
        if !self.buffer.is_null() && self.valid {
            unsafe { rt.simd_restore(self.buffer) };
        }
    }
}

pub fn self_test<R: BootRuntime>(rt: &R) {
    use crate::kinfo;

    let (size, align) = rt.simd_state_layout();
    if size == 0 {
        kinfo!("SIMD self-test skipped (no SIMD support)");
        return;
    }

    with_simd(rt, || {
        #[repr(align(16))]
        struct AlignedStorage([u8; 1024]);

        let mut storage = AlignedStorage([0; 1024]);
        let buffer = storage.0.as_mut_ptr();

        if size > 1024 || align > 16 {
            kinfo!("SIMD self-test skipped (size/align too large for stack buffer)");
            return;
        }

        unsafe { rt.simd_save(buffer) };
        unsafe { rt.simd_restore(buffer) };

        kinfo!("SIMD self-test passed (save/restore cycle)");
    });
}
