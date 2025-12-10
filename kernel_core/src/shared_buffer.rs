use crate::{graph, graph_kinds, memory::PhysFrame};
use abi::{MapFlags, PixelFormat, PropValue, SharedBufferInfo, ThingId};
use alloc::boxed::Box;
use alloc::collections::BTreeMap;
use alloc::vec::Vec as AllocVec;
use core::sync::atomic::{AtomicU64, Ordering};
use heapless::Vec;
use spin::Mutex;

pub const MAX_FRAMES_PER_BUFFER: usize = 4096;
const PAGE_SIZE: u64 = 4096;

#[derive(Clone, Debug)]
pub struct SharedBuffer {
    pub width: u32,
    pub height: u32,
    pub stride: u32,
    pub pixel_format: PixelFormat,
    pub frames: Vec<PhysFrame, { MAX_FRAMES_PER_BUFFER }>,
}

impl SharedBuffer {
    pub fn size_bytes(&self) -> u64 {
        self.stride as u64 * self.height as u64
    }

    pub fn info(&self) -> SharedBufferInfo {
        SharedBufferInfo {
            width: self.width,
            height: self.height,
            stride: self.stride,
            pixel_format: self.pixel_format,
        }
    }
}

pub struct SharedBufferManager {
    buffers: BTreeMap<ThingId, SharedBuffer>,
}

impl SharedBufferManager {
    pub const fn new() -> Self {
        Self {
            buffers: BTreeMap::new(),
        }
    }

    pub fn insert(&mut self, id: ThingId, sb: SharedBuffer) {
        self.buffers.insert(id, sb);
    }

    pub fn get(&self, id: &ThingId) -> Option<&SharedBuffer> {
        self.buffers.get(id)
    }

    pub fn get_cloned(&self, id: &ThingId) -> Option<SharedBuffer> {
        self.buffers.get(id).cloned()
    }
}

static SHARED_BUFFERS: Mutex<SharedBufferManager> = Mutex::new(SharedBufferManager::new());
static HHDM_OFFSET: AtomicU64 = AtomicU64::new(0);

pub fn manager() -> &'static Mutex<SharedBufferManager> {
    &SHARED_BUFFERS
}

pub fn set_hhdm_offset(offset: u64) {
    HHDM_OFFSET.store(offset, Ordering::SeqCst);
}

pub fn hhdm_offset() -> Option<u64> {
    let val = HHDM_OFFSET.load(Ordering::SeqCst);
    (val != 0).then_some(val)
}

pub fn create_shared_buffer_thing(
    width: u32,
    height: u32,
    stride: u32,
    pixel_format: PixelFormat,
) -> Option<ThingId> {
    let pf_str: &'static str = match pixel_format {
        PixelFormat::Rgba8888 => "Rgba8888",
        PixelFormat::Bgra8888 => "Bgra8888",
    };

    let mut props = AllocVec::new();
    props.push((graph_kinds::PROP_WIDTH, PropValue::U64(width as u64)));
    props.push((graph_kinds::PROP_HEIGHT, PropValue::U64(height as u64)));
    props.push((graph_kinds::PROP_STRIDE, PropValue::U64(stride as u64)));
    props.push((
        graph_kinds::PROP_PIXEL_FORMAT,
        PropValue::Str(pf_str.into()),
    ));
    let boxed = Box::leak(props.into_boxed_slice());
    graph::create_thing(graph_kinds::KIND_SHARED_BUFFER, boxed)
}

pub fn register_shared_buffer(
    width: u32,
    height: u32,
    stride: u32,
    pixel_format: PixelFormat,
    frames: Vec<PhysFrame, { MAX_FRAMES_PER_BUFFER }>,
) -> Result<ThingId, &'static str> {
    let buffer = SharedBuffer {
        width,
        height,
        stride,
        pixel_format,
        frames,
    };

    let buffer_id = create_shared_buffer_thing(width, height, stride, pixel_format)
        .ok_or("Failed to create SharedBuffer Thing")?;

    SHARED_BUFFERS.lock().insert(buffer_id, buffer);
    Ok(buffer_id)
}

pub fn create_display_for_buffer(
    buffer_id: ThingId,
    name: &'static str,
    info: &SharedBufferInfo,
) -> Option<ThingId> {
    let pf_str: &'static str = match info.pixel_format {
        PixelFormat::Rgba8888 => "Rgba8888",
        PixelFormat::Bgra8888 => "Bgra8888",
    };

    let props = &[
        (graph_kinds::PROP_NAME, PropValue::Str(name.into())),
        (graph_kinds::PROP_WIDTH, PropValue::U64(info.width as u64)),
        (graph_kinds::PROP_HEIGHT, PropValue::U64(info.height as u64)),
        (graph_kinds::PROP_STRIDE, PropValue::U64(info.stride as u64)),
        (
            graph_kinds::PROP_PIXEL_FORMAT,
            PropValue::Str(pf_str.into()),
        ),
    ];
    let display_id = graph::create_thing(graph_kinds::KIND_DISPLAY, props)?;
    let _ = graph::add_edge(display_id, graph_kinds::EDGE_DISPLAY_SCANOUT, buffer_id);
    Some(display_id)
}

pub fn map_frames_into_current_as(
    vaddr: u64,
    frames: &[PhysFrame],
    flags: MapFlags,
) -> Result<(), &'static str> {
    #[cfg(target_arch = "x86_64")]
    {
        use crate::memory::allocate_frame;
        use x86_64::registers::control::Cr3;
        use x86_64::structures::paging::mapper::MapToError;
        use x86_64::structures::paging::{
            FrameAllocator, Mapper, OffsetPageTable, Page, PageTable, PageTableFlags,
            PhysFrame as XPhysFrame, Size4KiB,
        };
        use x86_64::{PhysAddr, VirtAddr};

        struct TableFrameAllocator;
        unsafe impl FrameAllocator<Size4KiB> for TableFrameAllocator {
            fn allocate_frame(&mut self) -> Option<XPhysFrame> {
                allocate_frame().and_then(|frame| {
                    XPhysFrame::from_start_address(PhysAddr::new(frame.start_address)).ok()
                })
            }
        }

        let hhdm = hhdm_offset().ok_or("HHDM offset unknown for mapping")?;
        let current_cr3 = Cr3::read();
        let l4_ptr = (current_cr3.0.start_address().as_u64() + hhdm) as *mut PageTable;
        let l4_table = unsafe { &mut *l4_ptr };
        let mut mapper = unsafe { OffsetPageTable::new(l4_table, VirtAddr::new(hhdm)) };
        let mut table_alloc = TableFrameAllocator;

        let mut page_addr = vaddr;
        for frame in frames {
            let page = Page::<Size4KiB>::containing_address(VirtAddr::new(page_addr));
            let phys = XPhysFrame::from_start_address(PhysAddr::new(frame.start_address))
                .map_err(|_| "Invalid frame address")?;
            let mut page_flags = PageTableFlags::PRESENT | PageTableFlags::USER_ACCESSIBLE;
            if flags.contains(MapFlags::WRITE) {
                page_flags |= PageTableFlags::WRITABLE;
            }
            if !flags.contains(MapFlags::EXECUTE) {
                page_flags |= PageTableFlags::NO_EXECUTE;
            }

            unsafe {
                match mapper.map_to(page, phys, page_flags, &mut table_alloc) {
                    Ok(mapping) => mapping.flush(),
                    Err(MapToError::PageAlreadyMapped(_)) => {
                        // Update flags if already mapped
                        if let Ok(flush) = mapper.update_flags(page, page_flags) {
                            flush.flush();
                        }
                    }
                    Err(_) => return Err("Failed to map shared buffer frame"),
                }
            }

            page_addr = page_addr.saturating_add(PAGE_SIZE);
        }

        Ok(())
    }

    #[cfg(target_arch = "aarch64")]
    {
        return aarch64_map::map_frames(vaddr, frames, flags);
    }

    #[cfg(all(not(target_arch = "x86_64"), not(target_arch = "aarch64")))]
    {
        let _ = (vaddr, frames, flags);
        return Err("SharedBuffer mapping not implemented for this architecture");
    }
}

#[cfg(target_arch = "aarch64")]
mod aarch64_map {
    use super::{PAGE_SIZE, hhdm_offset};
    use crate::memory::{self, PhysFrame};
    use abi::MapFlags;
    use core::ptr;

    const DESC_VALID: u64 = 1 << 0;
    const DESC_TABLE_OR_PAGE: u64 = 1 << 1;
    const ATTR_INDEX0: u64 = 0;
    const AP_RW_EL0: u64 = 0b01 << 6;
    const AP_RO_EL0: u64 = 0b11 << 6;
    const SH_INNER: u64 = 0b11 << 8;
    const AF: u64 = 1 << 10;
    const PXN: u64 = 1 << 53;
    const UXN: u64 = 1 << 54;
    const ADDR_MASK: u64 = 0x0000_FFFF_FFFF_F000;

    #[repr(C, align(4096))]
    struct PageTable {
        entries: [u64; 512],
    }

    pub fn map_frames(
        vaddr: u64,
        frames: &[PhysFrame],
        flags: MapFlags,
    ) -> Result<(), &'static str> {
        if vaddr & (PAGE_SIZE - 1) != 0 {
            return Err("SharedBuffer base must be page aligned");
        }
        let hhdm = hhdm_offset().ok_or("HHDM offset unknown for mapping")?;
        let ttbr0 = current_ttbr0_phys();
        if ttbr0 == 0 {
            return Err("TTBR0_EL1 unavailable for mapping");
        }

        let writable = flags.contains(MapFlags::WRITE);
        let executable = flags.contains(MapFlags::EXECUTE);
        let mut addr = vaddr;
        for frame in frames {
            map_page(ttbr0, hhdm, addr, frame.start_address, writable, executable)?;
            addr = addr.saturating_add(PAGE_SIZE);
        }

        unsafe {
            core::arch::asm!("dsb ishst", "isb", options(nostack, preserves_flags));
        }

        Ok(())
    }

    fn map_page(
        l0_phys: u64,
        hhdm: u64,
        virt: u64,
        phys: u64,
        writable: bool,
        executable: bool,
    ) -> Result<(), &'static str> {
        if phys & (PAGE_SIZE - 1) != 0 {
            return Err("SharedBuffer frame is not page aligned");
        }

        let l0_table = table_mut(l0_phys, hhdm);
        let l0_idx = ((virt >> 39) & 0x1ff) as usize;
        let l1_phys = ensure_table(l0_table, l0_idx, hhdm)?;
        let l1_table = table_mut(l1_phys, hhdm);

        let l1_idx = ((virt >> 30) & 0x1ff) as usize;
        let l2_phys = ensure_table(l1_table, l1_idx, hhdm)?;
        let l2_table = table_mut(l2_phys, hhdm);

        let l2_idx = ((virt >> 21) & 0x1ff) as usize;
        let l3_phys = ensure_table(l2_table, l2_idx, hhdm)?;
        let l3_table = table_mut(l3_phys, hhdm);

        let l3_idx = ((virt >> 12) & 0x1ff) as usize;
        if l3_table.entries[l3_idx] & DESC_VALID != 0 {
            return Err("SharedBuffer VA already mapped");
        }

        let mut desc = phys | DESC_VALID | DESC_TABLE_OR_PAGE | ATTR_INDEX0 | SH_INNER | AF | PXN;
        desc |= if writable { AP_RW_EL0 } else { AP_RO_EL0 };
        if !executable {
            desc |= UXN;
        }
        l3_table.entries[l3_idx] = desc;
        Ok(())
    }

    fn ensure_table(parent: &mut PageTable, idx: usize, hhdm: u64) -> Result<u64, &'static str> {
        let entry = parent.entries[idx];
        if entry & DESC_VALID == 0 {
            let frame = memory::allocate_frame().ok_or("Out of frames for page table")?;
            zero_frame(frame.start_address, hhdm);
            parent.entries[idx] = frame.start_address | DESC_VALID | DESC_TABLE_OR_PAGE;
            Ok(frame.start_address)
        } else if entry & DESC_TABLE_OR_PAGE != 0 {
            Ok(entry & ADDR_MASK)
        } else {
            Err("Encountered block entry while walking page tables")
        }
    }

    fn table_mut(phys: u64, hhdm: u64) -> &'static mut PageTable {
        let ptr = (phys + hhdm) as *mut PageTable;
        unsafe { &mut *ptr }
    }

    fn zero_frame(phys: u64, hhdm: u64) {
        unsafe {
            ptr::write_bytes((phys + hhdm) as *mut u8, 0, PAGE_SIZE as usize);
        }
    }

    fn current_ttbr0_phys() -> u64 {
        let ttbr: u64;
        unsafe {
            core::arch::asm!("mrs {val}, ttbr0_el1", val = out(reg) ttbr);
        }
        ttbr & ADDR_MASK
    }
}

pub const fn align_up(value: u64, align: u64) -> u64 {
    if align == 0 {
        return value;
    }
    let rem = value % align;
    if rem == 0 {
        value
    } else {
        value + (align - rem)
    }
}

pub const fn page_count_for_size(size: u64) -> u64 {
    align_up(size, PAGE_SIZE) / PAGE_SIZE
}
