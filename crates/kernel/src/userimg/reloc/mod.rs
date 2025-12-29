use xmas_elf::{program::Type, ElfFile};

pub fn apply_relocations<F>(
    elf_data: &[u8],
    load_base: u64,
    elf: &ElfFile,
    map_write: &mut F,
)
where
    F: FnMut(u64, &[u8]),
{
    #[cfg(target_arch = "x86_64")]
    let rel_type = 8; // R_X86_64_RELATIVE
    #[cfg(target_arch = "aarch64")]
    let rel_type = 1027; // R_AARCH64_RELATIVE
    #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
    let rel_type = 0; // Unknown

    if rel_type != 0 {
        apply_relative_relocs_generic(elf_data, load_base, elf, map_write, rel_type);
    }
}

fn apply_relative_relocs_generic<F>(
    elf_data: &[u8],
    load_base: u64,
    elf: &ElfFile,
    map_write: &mut F,
    expected_type: u32,
)
where
    F: FnMut(u64, &[u8]),
{
    let dyn_ph = elf
        .program_iter()
        .find(|ph| ph.get_type().map(|t| t == Type::Dynamic).unwrap_or(false));

    if let Some(dyn_ph) = dyn_ph {
        let dyn_offset = dyn_ph.offset();
        let dyn_size = dyn_ph.file_size();
        let dyn_entries = &elf_data[dyn_offset as usize..(dyn_offset + dyn_size) as usize];

        let mut rela_addr = 0u64;
        let mut rela_sz = 0u64;
        let mut rela_ent = 0u64;

        for chunk in dyn_entries.chunks(16) {
            if chunk.len() < 16 {
                break;
            }
            let tag = u64::from_le_bytes(chunk[0..8].try_into().unwrap());
            let val = u64::from_le_bytes(chunk[8..16].try_into().unwrap());
            match tag {
                7 => rela_addr = val, // DT_RELA
                8 => rela_sz = val,   // DT_RELASZ
                9 => rela_ent = val,  // DT_RELAENT
                0 => break,
                _ => {}
            }
        }

        if rela_addr == 0 || rela_sz == 0 {
            return;
        }

        let mut file_offset = None;
        for ph in elf.program_iter() {
            if ph.get_type().unwrap_or(Type::Null) == Type::Load {
                let vaddr = ph.virtual_addr();
                let mem_sz = ph.mem_size();
                if rela_addr >= vaddr && rela_addr < vaddr + mem_sz {
                    file_offset = Some(ph.offset() + (rela_addr - vaddr));
                    break;
                }
            }
        }

        if let Some(file_off) = file_offset {
            let rela_data = &elf_data[file_off as usize..(file_off + rela_sz) as usize];
            let ent_size = if rela_ent > 0 { rela_ent } else { 24 };

            for chunk in rela_data.chunks(ent_size as usize) {
                if chunk.len() < 24 {
                    break;
                }
                let r_offset = u64::from_le_bytes(chunk[0..8].try_into().unwrap());
                let r_info = u64::from_le_bytes(chunk[8..16].try_into().unwrap());
                let r_addend = i64::from_le_bytes(chunk[16..24].try_into().unwrap());
                let r_type = (r_info & 0xFFFF_FFFF) as u32;

                if r_type == expected_type {
                    let value = load_base.wrapping_add(r_addend as u64);
                    map_write(r_offset, &value.to_le_bytes());
                }
            }
        }
    }
}
