//! Process and Thread Management

use crate::machine::Context;
use abi::ids::ThingId;

#[derive(Debug)]
pub enum ThreadState {
    Runnable,
    Blocked,
    Exited,
}

#[derive(Debug)]
pub struct Process {
    pub pid: ThingId,
    // AddressSpace will go here
    pub heap_base: u64,
    pub heap_limit: u64,
}

#[repr(C)]
#[derive(Debug)]
pub struct Thread {
    pub tid: ThingId,
    pub process: *mut Process,

    // Kernel stack top (for TSS/SYSCALL)
    pub kernel_stack_top: u64,

    // Saved context (callee-saved regs + SP)
    pub context: Context,

    pub state: ThreadState,
}

pub fn spawn_kernel_module(module: &crate::boot::ModuleInfo) -> Result<ThingId, ()> {
    use crate::memory::map::MapPerms;
    use crate::memory::space::AddressSpace;
    use crate::sched;
    use alloc::alloc::{alloc, Layout};
    use alloc::sync::Arc;

    crate::log::kprintln("PROC: spawn_kernel_module");
    let offset = crate::boot::get_boot_ctx().hhdm_offset;
    crate::log::klog(
        crate::log::Level::Info,
        "PROC",
        &alloc::format!("module phys={:x} offset={:x}", module.phys_addr, offset),
    );

    // 1. Create Address Space
    let address_space = Arc::new(AddressSpace::new().map_err(|_| ())?);

    // 2. Map Module Image (ELF Parsing)
    let elf_data = unsafe {
        core::slice::from_raw_parts(
            (module.phys_addr + crate::boot::get_boot_ctx().hhdm_offset) as *const u8,
            module.size as usize,
        )
    };

    let entry_point = load_elf(elf_data, &address_space)?;
    crate::log::kprintln("PROC: ELF loaded, entry");

    // 3. Setup Stack BEFORE spawning
    let stack_top = 0x8000_0000u64;
    let stack_size = 32 * 4096; // 128KB
    let stack_base = stack_top - stack_size;

    let layout = Layout::from_size_align(stack_size as usize, 4096).map_err(|_| ())?;
    let stack_mem = unsafe { alloc(layout) };
    if stack_mem.is_null() {
        return Err(());
    }

    // Zero init stack
    unsafe {
        core::ptr::write_bytes(stack_mem, 0, stack_size as usize);
    }

    let stack_phys = crate::machine::machine().virt_to_phys(stack_mem as u64);

    // Map stack in the new address space
    address_space
        .map(
            stack_base,
            stack_phys,
            stack_size as usize,
            MapPerms::READ | MapPerms::WRITE | MapPerms::USER,
        )
        .map_err(|_| ())?;

    // 4. Spawn Task AND configure context atomically (with IRQs disabled)
    let name = module.path;
    let irq_token = crate::machine::irq_disable();
    let task_id = {
        let mut guard = sched::SCHEDULER.lock();
        let sched = guard.as_mut().ok_or(())?;
        let task_id = sched.spawn(name, Some(address_space.clone()));

        // Configure context immediately, while still holding the lock
        // This prevents the task from being scheduled before context is set up
        sched::configure_task_context_locked(sched, task_id, entry_point, stack_top);
        
        // Return internal TaskID to caller?
        // Wait, sched.spawn returns TaskId (which matches ThingId).
        // Check sched/mod.rs: pub struct TaskId(pub u64);
        // And Task has `thing: ThingId`.
        // We probably want to return the ThingId so we can use it in graph operations.
        // Let's check sched/mod.rs again.
        // TaskId is a wrapper around u64.
        // Task struct usually has a `thing` field.
        // `sched.spawn` returns `TaskId`.
        
        // Let's grab the ThingId from the task we just spawned.
        // We are holding the lock.
        let t = sched.tasks.iter().find(|t| t.id == task_id).ok_or(())?;
        t.thing
    };
    crate::machine::irq_restore(irq_token);

    // Announce sprout scheduling so BDD can observe userland start.
    if module.path.contains("sprout") {
        crate::serial::write(b"SPROUT: I am alive\n");
    }

    crate::log::kprintln("PROC: spawned");
    Ok(task_id)
}

fn load_elf(data: &[u8], _as: &crate::memory::space::AddressSpace) -> Result<u64, ()> {
    use crate::memory::map::MapPerms;
    use alloc::alloc::{alloc, Layout};

    if data.len() < 64 || &data[0..4] != b"\x7fELF" {
        return Err(());
    }

    if data[4] != 2 {
        return Err(());
    }

    let entry = u64::from_le_bytes(data[24..32].try_into().unwrap());
    let ph_off = u64::from_le_bytes(data[32..40].try_into().unwrap());
    let ph_ent_size = u16::from_le_bytes(data[54..56].try_into().unwrap()) as usize;
    let ph_num = u16::from_le_bytes(data[56..58].try_into().unwrap()) as usize;

    for i in 0..ph_num {
        let offset = ph_off as usize + i * ph_ent_size;
        let ph = &data[offset..offset + ph_ent_size];

        let p_type = u32::from_le_bytes(ph[0..4].try_into().unwrap());
        if p_type != 1 {
            continue;
        } // LOAD

        let p_flags = u32::from_le_bytes(ph[4..8].try_into().unwrap());
        let p_offset = u64::from_le_bytes(ph[8..16].try_into().unwrap());
        let p_vaddr = u64::from_le_bytes(ph[16..24].try_into().unwrap());
        let p_filesz = u64::from_le_bytes(ph[32..40].try_into().unwrap());
        let p_memsz = u64::from_le_bytes(ph[40..48].try_into().unwrap());

        if p_memsz == 0 {
            continue;
        }

        let mut perms = MapPerms::USER;
        if p_flags & 1 != 0 {
            perms |= MapPerms::EXEC;
        }
        if p_flags & 2 != 0 {
            perms |= MapPerms::WRITE;
        }
        if p_flags & 4 != 0 {
            perms |= MapPerms::READ;
        }

        let page_start = p_vaddr & !0xfff;
        let offset_in_page = (p_vaddr - page_start) as usize;
        let len = ((p_memsz as usize + offset_in_page) + 0xfff) & !0xfff;

        let layout = Layout::from_size_align(len, 4096).map_err(|_| ())?;
        let mem = unsafe { alloc(layout) };
        if mem.is_null() {
            return Err(());
        }

        unsafe {
            core::ptr::write_bytes(mem, 0, len);
        }

        if p_filesz > 0 {
            let src_start = p_offset as usize;
            let src_end = src_start + p_filesz as usize;
            unsafe {
                core::ptr::copy_nonoverlapping(
                    data[src_start..src_end].as_ptr(),
                    mem.add(offset_in_page),
                    p_filesz as usize,
                );
            }
        }

        let phys = crate::machine::machine().virt_to_phys(mem as u64);

        _as.map(page_start, phys, len, perms).map_err(|_| ())?;
    }

    Ok(entry)
}
