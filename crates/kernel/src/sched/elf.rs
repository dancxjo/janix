use alloc::vec::Vec;
use xmas_elf::{program::Type, symbol_table::Entry, ElfFile};

pub struct LoadedImage {
    pub entry_point: u64,
}

pub fn load_elf(
    elf_data: &[u8],
    load_base: u64,
    mut phys_write: impl FnMut(u64, &[u8]),
) -> Option<LoadedImage> {
    let elf = ElfFile::new(elf_data).ok()?;

    // 1. Load Segments
    for ph in elf.program_iter() {
        if ph.get_type().ok()? == Type::Load {
            let file_size = ph.file_size();
            let mem_size = ph.mem_size();
            let vaddr = ph.virtual_addr();
            let offset = ph.offset();

            if file_size > 0 {
                let segment_data = &elf_data[offset as usize..(offset + file_size) as usize];
                phys_write(vaddr, segment_data);
            }

            // Zero fill remaining memory (bss)
            if mem_size > file_size {
                let zero_len = mem_size - file_size;
                let zeros = alloc::vec![0u8; zero_len as usize];
                phys_write(vaddr + file_size, &zeros);
            }
        }
    }

    // 2. Apply Relocations (R_X86_64_RELATIVE)
    // Find Dynamic Segment
    if let Some(dyn_ph) = elf
        .program_iter()
        .find(|ph| ph.get_type().unwrap_or(Type::Null) == Type::Dynamic)
    {
        let dyn_offset = dyn_ph.offset();
        let dyn_size = dyn_ph.file_size();
        let dyn_entries = &elf_data[dyn_offset as usize..(dyn_offset + dyn_size) as usize];

        // Parse Dyn entries manually or use xmas_elf helpers if valid.
        // xmas_elf doesn't seem to expose raw iterator over bytes easily for unknown layout (32/64).
        // But ElfFile has `.dynamic_iter()?` No?
        // Let's rely on common structure: 16 bytes per entry (Tag: u64, Val: u64) for ELF64.

        // Find RELA, RELASZ, RELAENT
        let mut rela_addr = 0u64;
        let mut rela_sz = 0u64;
        let mut rela_ent = 0u64;

        for chunk in dyn_entries.chunks(16) {
            let tag = u64::from_le_bytes(chunk[0..8].try_into().unwrap());
            let val = u64::from_le_bytes(chunk[8..16].try_into().unwrap());
            match tag {
                7 => rela_addr = val, // DT_RELA
                8 => rela_sz = val,   // DT_RELASZ
                9 => rela_ent = val,  // DT_RELAENT
                0 => break,           // DT_NULL
                _ => {}
            }
        }

        if rela_addr > 0 && rela_sz > 0 {
            // Convert Virtual Address (rela_addr) to File Offset.
            // Iterate PHDRs to find which segment contains rela_addr.
            let mut file_offset = 0u64;
            for ph in elf.program_iter() {
                if ph.get_type().unwrap_or(Type::Null) == Type::Load {
                    let vaddr = ph.virtual_addr();
                    let mem_sz = ph.mem_size();
                    if rela_addr >= vaddr && rela_addr < vaddr + mem_sz {
                        file_offset = ph.offset() + (rela_addr - vaddr);
                        break;
                    }
                }
            }

            if file_offset > 0 {
                let rela_data = &elf_data[file_offset as usize..(file_offset + rela_sz) as usize];
                // Entry size is usually 24 bytes for ELF64 Rela.
                // r_offset (8), r_info (8), r_addend (8).
                let ent_size = if rela_ent > 0 { rela_ent } else { 24 };

                for chunk in rela_data.chunks(ent_size as usize) {
                    let r_offset = u64::from_le_bytes(chunk[0..8].try_into().unwrap());
                    let r_info = u64::from_le_bytes(chunk[8..16].try_into().unwrap());
                    let r_addend = i64::from_le_bytes(chunk[16..24].try_into().unwrap());

                    let r_type = r_info & 0xFFFFFFFF; // Low 32 bits

                    if r_type == 8 {
                        // R_X86_64_RELATIVE
                        let value = load_base.wrapping_add(r_addend as u64);
                        phys_write(r_offset, &value.to_le_bytes());
                    }
                }
            }
        }
    }

    Some(LoadedImage {
        entry_point: elf.header.pt2.entry_point(),
    })
}

pub fn find_symbol(elf_data: &[u8], symbol_name: &str) -> Option<u64> {
    let elf = ElfFile::new(elf_data).ok()?;

    // Iterate sections to find .symtab or .dynsym
    // Ideally we check .symtab for static linking or .dynsym for dynamic?
    // Drivers are likely statically linked ELFs (executables) but might export symbols via .symtab.
    // If they are dynamic libs, .dynsym.
    // "limine_fb_driver" is an executable.
    // Rust binaries usually preserve .symtab unless stripped.

    for section in elf.section_iter() {
        if let Ok(name) = section.get_name(&elf) {
            if name == ".symtab" {
                if let Ok(xmas_elf::sections::SectionData::SymbolTable64(entries)) =
                    section.get_data(&elf)
                {
                    for entry in entries {
                        if let Ok(sym_name) = entry.get_name(&elf) {
                            if sym_name == symbol_name {
                                return Some(entry.value());
                            }
                        }
                    }
                }
            }
        }
    }

    // Fallback to dynsym
    for section in elf.section_iter() {
        if let Ok(name) = section.get_name(&elf) {
            if name == ".dynsym" {
                if let Ok(xmas_elf::sections::SectionData::DynSymbolTable64(entries)) =
                    section.get_data(&elf)
                {
                    for entry in entries {
                        if let Ok(sym_name) = entry.get_name(&elf) {
                            if sym_name == symbol_name {
                                return Some(entry.value());
                            }
                        }
                    }
                }
            }
        }
    }

    None
}
