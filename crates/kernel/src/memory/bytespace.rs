use abi::ids::ThingId;
use alloc::collections::BTreeMap;
use spin::Mutex;

use crate::memory::id::next_thing_id;
use crate::memory::journal;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BytespaceKind {
    Ram,
    Device,
    Module,
    Framebuffer,
    KernelHeap,
    Dma,
}

#[derive(Clone, Copy, Debug)]
pub struct BytespaceInfo {
    pub kind: BytespaceKind,
    pub size: usize,
    pub phys_base: Option<u64>,
}

static BYTESPACE_REGISTRY: Mutex<BTreeMap<ThingId, BytespaceInfo>> = Mutex::new(BTreeMap::new());

fn register_bytespace(id: ThingId, info: BytespaceInfo) {
    BYTESPACE_REGISTRY.lock().insert(id, info);
}

pub fn lookup_bytespace(id: ThingId) -> Option<BytespaceInfo> {
    BYTESPACE_REGISTRY.lock().get(&id).copied()
}

pub struct Bytespace {
    pub id: ThingId,
    pub kind: BytespaceKind,
    pub size: usize,
    pub phys_base: Option<u64>,
    pub ram_backing: Option<*mut u8>,
}

unsafe impl Send for Bytespace {}
unsafe impl Sync for Bytespace {}

impl Bytespace {
    pub fn register_existing(id: ThingId, kind: BytespaceKind, size: usize, phys: u64) {
        let info = BytespaceInfo {
            kind,
            size,
            phys_base: Some(phys),
        };
        register_bytespace(id, info);

        let event_kind = match kind {
            BytespaceKind::Ram => journal::EVENT_BYTESPACE_CREATED_RAM,
            BytespaceKind::Device => journal::EVENT_BYTESPACE_CREATED_DEVICE,
            BytespaceKind::Module => journal::EVENT_BYTESPACE_CREATED_MODULE,
            BytespaceKind::Framebuffer => journal::EVENT_BYTESPACE_CREATED_FRAMEBUFFER,
            BytespaceKind::KernelHeap => journal::EVENT_BYTESPACE_CREATED_KERNEL_HEAP,
            BytespaceKind::Dma => journal::EVENT_BYTESPACE_CREATED_DMA,
        };
        journal::emit_bytespace_created(event_kind, id, size, phys);
    }

    pub fn lookup(id: ThingId) -> Option<BytespaceInfo> {
        lookup_bytespace(id)
    }

    pub fn new_ram(size: usize) -> Result<Self, ()> {
        let layout = core::alloc::Layout::from_size_align(size, 4096).map_err(|_| ())?;
        let ptr = unsafe { alloc::alloc::alloc_zeroed(layout) };
        if ptr.is_null() {
            return Err(());
        }

        let phys_base = crate::machine::machine().virt_to_phys(ptr as u64);
        let id = next_thing_id();

        let info = BytespaceInfo {
            kind: BytespaceKind::Ram,
            size,
            phys_base: Some(phys_base),
        };
        register_bytespace(id, info);
        journal::emit_bytespace_created(
            journal::EVENT_BYTESPACE_CREATED_RAM,
            id,
            size,
            phys_base,
        );

        Ok(Self {
            id,
            kind: BytespaceKind::Ram,
            size,
            phys_base: Some(phys_base),
            ram_backing: Some(ptr),
        })
    }

    /// Create a DMA-safe bytespace with physically contiguous memory.
    /// Returns the bytespace and its physical base address.
    pub fn new_dma(size: usize) -> Result<(Self, u64), ()> {
        // Validate: must be page-aligned and at least 4K
        if size == 0 || size % 4096 != 0 {
            return Err(());
        }
        // Cap at 16MB to prevent OOM
        if size > 16 * 1024 * 1024 {
            return Err(());
        }

        // Allocate with 64-byte alignment (xHCI TRB alignment requirement)
        let layout = core::alloc::Layout::from_size_align(size, 64).map_err(|_| ())?;
        let ptr = unsafe { alloc::alloc::alloc_zeroed(layout) };
        if ptr.is_null() {
            return Err(());
        }

        let phys_base = crate::machine::machine().virt_to_phys(ptr as u64);
        let id = next_thing_id();

        let info = BytespaceInfo {
            kind: BytespaceKind::Dma,
            size,
            phys_base: Some(phys_base),
        };
        register_bytespace(id, info);
        journal::emit_bytespace_created(
            journal::EVENT_BYTESPACE_CREATED_DMA,
            id,
            size,
            phys_base,
        );

        Ok((
            Self {
                id,
                kind: BytespaceKind::Dma,
                size,
                phys_base: Some(phys_base),
                ram_backing: Some(ptr),
            },
            phys_base,
        ))
    }

    pub fn new_device(phys: u64, size: usize) -> Self {
        Self::create_external(BytespaceKind::Device, phys, size)
    }

    pub fn new_framebuffer(phys: u64, size: usize) -> Self {
        Self::create_external(BytespaceKind::Framebuffer, phys, size)
    }

    pub fn new_module(phys: u64, size: usize) -> Self {
        Self::create_external(BytespaceKind::Module, phys, size)
    }

    pub fn new_kernel_heap(phys: u64, size: usize) -> Self {
        Self::create_external(BytespaceKind::KernelHeap, phys, size)
    }

    fn create_external(kind: BytespaceKind, phys: u64, size: usize) -> Self {
        let id = next_thing_id();
        let info = BytespaceInfo {
            kind,
            size,
            phys_base: Some(phys),
        };
        register_bytespace(id, info);

        let event_kind = match kind {
            BytespaceKind::Ram => journal::EVENT_BYTESPACE_CREATED_RAM,
            BytespaceKind::Device => journal::EVENT_BYTESPACE_CREATED_DEVICE,
            BytespaceKind::Module => journal::EVENT_BYTESPACE_CREATED_MODULE,
            BytespaceKind::Framebuffer => journal::EVENT_BYTESPACE_CREATED_FRAMEBUFFER,
            BytespaceKind::KernelHeap => journal::EVENT_BYTESPACE_CREATED_KERNEL_HEAP,
            BytespaceKind::Dma => journal::EVENT_BYTESPACE_CREATED_DMA,
        };
        journal::emit_bytespace_created(event_kind, id, size, phys);

        Self {
            id,
            kind,
            size,
            phys_base: Some(phys),
            ram_backing: None,
        }
    }

    pub fn backing_ptr(&self) -> Option<*mut u8> {
        if let Some(ptr) = self.ram_backing {
            Some(ptr)
        } else if let Some(phys) = self.phys_base {
            use crate::boot::get_boot_ctx;
            let offset = get_boot_ctx().hhdm_offset;
            Some(phys.wrapping_add(offset) as *mut u8)
        } else {
            None
        }
    }
}
