use graph::store;
use graph::symbols::sym;
use abi::ids::{ThingId, SymbolId};

fn create_val_u64(s: &mut store::PlaceStore, val: u64) -> ThingId {
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
}

impl BytespaceKind {
    pub fn to_symbol(&self) -> SymbolId {
        match self {
            BytespaceKind::Ram => sym::KIND_BYTESPACE_RAM,
            BytespaceKind::Device => sym::KIND_BYTESPACE_DEVICE,
            BytespaceKind::Module => sym::KIND_BYTESPACE_MODULE,
            BytespaceKind::Framebuffer => sym::KIND_BYTESPACE_FRAMEBUFFER,
            BytespaceKind::KernelHeap => sym::KIND_BYTESPACE_RAM, // Reuse RAM kind or new? Let's use RAM for now as it IS RAM.
        }
    }
}

pub struct Bytespace {
    pub id: ThingId,
    pub kind: BytespaceKind,
    pub size: usize,
    pub phys_base: Option<u64>,
    // For RAM, we might hold the allocation. For others, it's just a handle/view.
    // In V0.3, "RAM" bytespaces own their memory (Allocated Frames).
    // For now, using a leaked Vec for RAM to simplify proof of concept.
    pub ram_backing: Option<*mut u8>, 
}

unsafe impl Send for Bytespace {}
unsafe impl Sync for Bytespace {}

impl Bytespace {
    pub fn new_ram(size: usize) -> Result<Self, ()> {
        // Allocate backing memory
        // Check size? core::alloc::Layout handles it. 
        // We use 4096 alignment.
        let layout = core::alloc::Layout::from_size_align(size, 4096).map_err(|_| ())?;
        let ptr = unsafe { alloc::alloc::alloc_zeroed(layout) };
        if ptr.is_null() {
            return Err(());
        }
        
        let phys_base = crate::machine::machine().virt_to_phys(ptr as u64);

        let thing = store::with_store(|s| {
            let t = s.create_thing(sym::KIND_BYTE_SPACE).expect("create bytespace");
            
            if let Ok(k) = s.create_thing(sym::KIND_BYTESPACE_RAM) {
                 let _ = s.create_relationship(sym::PRED_HAS_KIND, t, k);
            }
            
            // Size
            let size_val = create_val_u64(s, size as u64);
            let _ = s.create_relationship(sym::PRED_SIZE, t, size_val);
            
            // Phys Base (for Mapping)
            let phys_val = create_val_u64(s, phys_base);
            let _ = s.create_relationship(sym::PRED_BASE_PHYS, t, phys_val);
            
            // Link to memory
            if let Some(mem) = s.find_by_name(sym::PLACE_MEMORY) {
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

    pub fn new_device(phys: u64, size: usize) -> Self {
        Self::create_external(BytespaceKind::Device, phys, size, sym::PLACE_DEVICES)
    }

    pub fn new_framebuffer(phys: u64, size: usize) -> Self {
        Self::create_external(BytespaceKind::Framebuffer, phys, size, sym::PLACE_DEVICES)
    }

    pub fn new_module(phys: u64, size: usize) -> Self {
        Self::create_external(BytespaceKind::Module, phys, size, sym::PLACE_MEMORY)
    }

    pub fn new_kernel_heap(phys: u64, size: usize) -> Self {
         Self::create_external(BytespaceKind::KernelHeap, phys, size, sym::PLACE_MEMORY)
    }

    fn create_external(kind: BytespaceKind, phys: u64, size: usize, place_sym: SymbolId) -> Self {
        let thing = store::with_store(|s| {
            let t = s.create_thing(sym::KIND_BYTE_SPACE).expect("create bytespace");
            
            if let Ok(k) = s.create_thing(kind.to_symbol()) {
                 let _ = s.create_relationship(sym::PRED_HAS_KIND, t, k);
            }
            
            let size_val = create_val_u64(s, size as u64);
            let _ = s.create_relationship(sym::PRED_SIZE, t, size_val);

            let phys_val = create_val_u64(s, phys);
            let _ = s.create_relationship(sym::PRED_BASE_PHYS, t, phys_val);

            if let Some(place) = s.find_by_name(place_sym) {
                 let _ = s.create_relationship(sym::PRED_CONTAINS, place, t);
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
             // If we have a physical base, we assume it's mapped in HHDM or we need to map it.
             // For modules (bootloader loaded), they are in HHDM.
             // For devices, they might not be.
             // For this task, we assume generic HHDM access for modules.
             // Devices might fail if not mapped.
             // This is a specialized kernel unsafe read.
             use crate::boot::get_boot_ctx;
             let offset = get_boot_ctx().hhdm_offset;
             Some((phys + offset) as *mut u8)
        } else {
            None
        }
    }
}
