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
}

impl BytespaceKind {
    pub fn to_symbol(&self) -> SymbolId {
        match self {
            BytespaceKind::Ram => sym::KIND_BYTESPACE_RAM,
            BytespaceKind::Device => sym::KIND_BYTESPACE_DEVICE,
            BytespaceKind::Module => sym::KIND_BYTESPACE_MODULE,
            BytespaceKind::Framebuffer => sym::KIND_BYTESPACE_FRAMEBUFFER,
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
    pub fn new_ram(size: usize) -> Self {
        // Allocate backing memory
        let layout = core::alloc::Layout::from_size_align(size, 4096).unwrap();
        let ptr = unsafe { alloc::alloc::alloc_zeroed(layout) };
        if ptr.is_null() {
            panic!("Bytespace OOM");
        }

        let thing = store::with_store(|s| {
            let t = s.create_thing(sym::KIND_BYTE_SPACE).expect("create bytespace");
            let ram_kind = s.create_thing(sym::KIND_BYTESPACE_RAM).unwrap_or(ThingId(0));
            let _ = s.create_relationship(sym::PRED_HAS_KIND, t, ram_kind); 
            // Better: Thing Kind is ByteSpace. Predicate HasKind points to specific flavor if needed, 
            // or we just use Attributes.
            // Symbol table has KIND_BYTESPACE_RAM. Is that the Thing Kind or a property?
            // "bytespace --[predicate.has_kind]--> bytespace_kind.*"
            // So Thing Kind is KIND_BYTE_SPACE.
            // We link to a singleton/concept for KIND_BYTESPACE_RAM.
            
            // Actually, let's just create a thing with KIND_BYTE_SPACE and set a property or relation.
            // We need to find or create the "Ram Kind" thing.
            // Ideally these are pre-seeded. We didn't pre-seed them as Things, just symbols.
            // We'll create ephemeral "Kind" things for now or rely on the symbol ID in a value.
            // Let's use `predicate.has_kind` -> `Thing(Kind=bytespace_kind.ram)`.
            if let Ok(k) = s.create_thing(sym::KIND_BYTESPACE_RAM) {
                 let _ = s.create_relationship(sym::PRED_HAS_KIND, t, k);
            }
            
            // Size
            let size_val = create_val_u64(s, size as u64);
            let _ = s.create_relationship(sym::PRED_SIZE, t, size_val);
            
            // Link to memory
            if let Some(mem) = s.find_by_name(sym::PLACE_MEMORY) {
                 let _ = s.create_relationship(sym::PRED_CONTAINS, mem, t);
            }
            t
        });

        Self {
            id: thing,
            kind: BytespaceKind::Ram,
            size,
            phys_base: None, // It's virtual RAM.
            ram_backing: Some(ptr),
        }
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
