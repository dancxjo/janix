use crate::{BootRuntime, BootTasking, MapKind, MapPerms, runtime};

pub struct KernelHeap {
    pub base: u64,
    pub size: usize,
    pub used: usize,
}

impl KernelHeap {
    pub const fn new() -> Self {
        Self {
            base: 0xffffffffb0000000,
            size: 0,
            used: 0,
        }
    }

    pub fn init(&mut self, size_mb: usize) {
        // This is optional if we use expand, but can be used for pre-calculation
        let _ = size_mb;
    }

    pub fn expand<R: BootRuntime>(&mut self, pages: usize) -> Result<(), ()> {
        let rt = runtime::<R>();
        let tasking = rt.tasking();
        let aspace = tasking.active_address_space();

        for _ in 0..pages {
            let virt = self.base + self.size as u64;
            let phys = super::alloc_frame().ok_or(())?;

            tasking.map_page(
                aspace,
                virt,
                phys,
                MapPerms {
                    read: true,
                    write: true,
                    user: false,
                    exec: false,
                },
                MapKind::Normal,
                &KernelFrameHook,
            )?;
            self.size += 4096;
        }
        Ok(())
    }
}

pub struct KernelFrameHook;
impl crate::FrameAllocatorHook for KernelFrameHook {
    fn alloc_frame(&self) -> Option<u64> {
        super::alloc_frame()
    }
}

pub fn kernel_heap() -> &'static spin::Mutex<KernelHeap> {
    static HEAP: spin::Mutex<KernelHeap> = spin::Mutex::new(KernelHeap::new());
    &HEAP
}
