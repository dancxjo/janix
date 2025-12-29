pub mod reloc;

use alloc::vec::Vec;
use xmas_elf::{program::Type, ElfFile};

#[derive(Debug)]
pub struct LoadedImage {
    pub entry: u64,
    pub load_base: u64,
    pub max_mapped: u64,
}

pub fn load_elf_user_image<F>(
    elf_data: &[u8],
    load_base: u64,
    mut map_write: F,
) -> Option<LoadedImage>
where
    F: FnMut(u64, &[u8]),
{
    let elf = ElfFile::new(elf_data).ok()?;
    let mut max_mapped = 0;

    // 1. Load Segments
    for ph in elf.program_iter() {
        if ph.get_type().ok()? == Type::Load {
            let file_size = ph.file_size();
            let mem_size = ph.mem_size();
            let vaddr = ph.virtual_addr();
            let offset = ph.offset();

            if file_size > 0 {
                let segment_data = &elf_data[offset as usize..(offset + file_size) as usize];
                // Map Write handles offsets relative to load_base?
                // No, load_elf usually maps to VADDR.
                // If position independent, vaddr starts at 0.
                // The caller supplied `load_base`.
                // `map_write` should expect absolute VADDR = load_base + vaddr?
                // "x86 provides map_write that maps pages and copies...".
                // In my x86 logic, I added `current_app_base + vaddr`.
                // So `map_write` receives `vaddr` relative to 0 (if PIE) or absolute?
                // The ELF `vaddr` is 0-based for PIE.
                // So I should pass `load_base + vaddr`?
                // Or I pass `vaddr` and caller adds `load_base`?
                // The previous code: `load_elf(data, base, |vaddr, seg| write(base + vaddr, ...))`.
                // So `load_elf` passed `vaddr` from PHDR.
                // The caller handled offset.
                // BUT, relocations need to know where it ended up.
                // `apply_relocations` needs `load_base` to calculate value.
                // `apply_relocations` also needs to write to `load_base + r_offset`.
                // If I pass `map_write` to `apply_relocations`, and `map_write` expects `vaddr` (PHDR based), then for relocs I need to pass `r_offset`.
                // `r_offset` IS a vaddr.
                // So `map_write` should take VADDR (from ELF).
                // The caller is responsible for mapping ELF VADDR to Physical/User Virtual.
                // Wait, if `load_base` is passed to `load_elf_user_image`, does the function use it?
                // Yes, for relocations.
                // Segments are loaded at `vaddr` (relative).

                // My implementation:
                map_write(vaddr, segment_data);
            }

            if mem_size > file_size {
                let zero_len = mem_size - file_size;
                let zeros = alloc::vec![0u8; zero_len as usize];
                map_write(vaddr + file_size, &zeros);
            }

            let end = vaddr + mem_size;
            if end > max_mapped {
                max_mapped = end;
            }
        }
    }

    // 2. Apply Relocations
    reloc::apply_relocations(elf_data, load_base, &elf, &mut map_write);

    Some(LoadedImage {
        entry: elf.header.pt2.entry_point(),
        load_base,
        max_mapped,
    })
}
