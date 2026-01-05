use abi::ids::{SymbolId, ThingId};
use graph::store;
use graph::symbols::sym;

fn create_val_u64(s: &mut store::GraphStore, val: u64) -> ThingId {
    let id = s.create_thing(sym::KIND_VALUE_U64).unwrap();
    let _ = s.set_payload(id, &val.to_le_bytes());
    id
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BytespaceKind {
    Ram,
    Device,
    Module,
    Framebuffer,
    KernelHeap,
    Dma,
}

impl BytespaceKind {
    pub fn to_symbol(&self) -> SymbolId {
        match self {
            BytespaceKind::Ram => sym::KIND_BYTESPACE_RAM,
            BytespaceKind::Device => sym::KIND_BYTESPACE_DEVICE,
            BytespaceKind::Module => sym::KIND_BYTESPACE_MODULE,
            BytespaceKind::Framebuffer => sym::KIND_BYTESPACE_FRAMEBUFFER,
            BytespaceKind::KernelHeap => sym::KIND_BYTESPACE_RAM,
            BytespaceKind::Dma => sym::KIND_BYTESPACE_DMA,
        }
    }
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
    pub fn new_ram(size: usize) -> Result<Self, ()> {
        let layout = core::alloc::Layout::from_size_align(size, 4096).map_err(|_| ())?;
        let ptr = unsafe { alloc::alloc::alloc_zeroed(layout) };
        if ptr.is_null() {
            return Err(());
        }

        let phys_base = crate::machine::machine().virt_to_phys(ptr as u64);

        let thing = store::with_store(|s| {
            let t = s
                .create_thing(sym::KIND_BYTE_SPACE)
                .expect("create bytespace");

            if let Ok(k) = s.create_thing(sym::KIND_BYTESPACE_RAM) {
                let _ = s.create_relationship(sym::PRED_HAS_KIND, t, k);
            }

            let size_val = create_val_u64(s, size as u64);
            let _ = s.create_relationship(sym::PRED_SIZE, t, size_val);

            let phys_val = create_val_u64(s, phys_base);
            let _ = s.create_relationship(sym::PRED_BASE_PHYS, t, phys_val);

            if let Some(mem) = s.find_by_name(sym::GRAPH_MEMORY) {
                let _ = s.create_relationship(sym::PRED_CONTAINS, mem, t);
            }
            t
        });

        Ok(Self {
            id: thing,
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

        let thing = store::with_store(|s| {
            let t = s
                .create_thing(sym::KIND_BYTE_SPACE)
                .expect("create DMA bytespace");

            if let Ok(k) = s.create_thing(sym::KIND_BYTESPACE_DMA) {
                let _ = s.create_relationship(sym::PRED_HAS_KIND, t, k);
            }

            let size_val = create_val_u64(s, size as u64);
            let _ = s.create_relationship(sym::PRED_SIZE, t, size_val);

            let phys_val = create_val_u64(s, phys_base);
            let _ = s.create_relationship(sym::PRED_BASE_PHYS, t, phys_val);

            if let Some(mem) = s.find_by_name(sym::GRAPH_MEMORY) {
                let _ = s.create_relationship(sym::PRED_CONTAINS, mem, t);
            }
            t
        });

        Ok((
            Self {
                id: thing,
                kind: BytespaceKind::Dma,
                size,
                phys_base: Some(phys_base),
                ram_backing: Some(ptr),
            },
            phys_base,
        ))
    }

    pub fn new_device(phys: u64, size: usize) -> Self {
        Self::create_external(BytespaceKind::Device, phys, size, sym::GRAPH_DEVICES)
    }

    pub fn new_framebuffer(phys: u64, size: usize) -> Self {
        Self::create_external(BytespaceKind::Framebuffer, phys, size, sym::GRAPH_DEVICES)
    }

    pub fn new_module(phys: u64, size: usize) -> Self {
        Self::create_external(BytespaceKind::Module, phys, size, sym::GRAPH_MEMORY)
    }

    pub fn new_kernel_heap(phys: u64, size: usize) -> Self {
        Self::create_external(BytespaceKind::KernelHeap, phys, size, sym::GRAPH_MEMORY)
    }

    fn create_external(kind: BytespaceKind, phys: u64, size: usize, graph_sym: SymbolId) -> Self {
        let thing = store::with_store(|s| {
            let t = s
                .create_thing(sym::KIND_BYTE_SPACE)
                .expect("create bytespace");

            if let Ok(k) = s.create_thing(kind.to_symbol()) {
                let _ = s.create_relationship(sym::PRED_HAS_KIND, t, k);
            }

            let size_val = create_val_u64(s, size as u64);
            let _ = s.create_relationship(sym::PRED_SIZE, t, size_val);

            let phys_val = create_val_u64(s, phys);
            let _ = s.create_relationship(sym::PRED_BASE_PHYS, t, phys_val);

            if let Some(graph) = s.find_by_name(graph_sym) {
                let _ = s.create_relationship(sym::PRED_CONTAINS, graph, t);
            }
            t
        });

        Self {
            id: thing,
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
