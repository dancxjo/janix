use xmas_elf::{ElfFile, program::{ProgramHeader, Type}};
use alloc::vec::Vec;
use core::slice;

pub struct LoadedImage {
    pub entry_point: u64,
}

pub fn load_elf(elf_data: &[u8], mut phys_write: impl FnMut(u64, &[u8])) -> Option<LoadedImage> {
    let elf = ElfFile::new(elf_data).ok()?;

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
                 // Use a zero buffer or call write multiple times
                 // For now, just a loop of zeros? 
                 // Or we construct a zero vec.
                 // Ideally `phys_write` handles zeros efficiently or we just pass a chunk of zeros.
                 let zeros = alloc::vec![0u8; zero_len as usize];
                 phys_write(vaddr + file_size, &zeros);
            }
        }
    }
    
    Some(LoadedImage { entry_point: elf.header.pt2.entry_point() })
}
