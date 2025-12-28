#![no_std]
#![no_main]

extern crate alloc;
use alloc::vec::Vec;
use alloc::format;
use core::fmt::Write;

use thing_std as std;
use thing_std::{GraphClient, StdoutConsole, Console};
use thing_models::abi::wire::graph::{GraphOp, GraphReply};
use thing_models::builtins::ids::*;
use thing_models::core::fs::{FileBody, MountBody};
use thing_models::prelude::ThingId; // Import ThingId from prelude

// Helper for sleep
fn sleep_ms(ms: u64) {
    // 1ms = 1_000_000 ns
    let _ = unsafe {
        // defined in syscall wrappers usually, but let's just make a quick syscall wrapper
        // SYSCALL_SLEEP is 12 defined in abi
        // But let's use ABI constant for clarity if we could import it, but it's fine to hardcode or bind
        // sys_sleep(ms * 1_000_000);
        
        let ns = ms * 1_000_000;
        let mut _ret: isize;
        core::arch::asm!(
            "syscall",
            in("rax") 12, // SYSCALL_SLEEP
            in("rdi") ns,
            lateout("rax") _ret,
            out("rcx") _,
            out("r11") _,
        );
    };
}

// Syscall wrapper
#[inline(always)]
unsafe fn sys_spawn(data: &[u8], name: &str) -> isize {
    let mut ret: isize;
    let data_ptr = data.as_ptr();
    let data_len = data.len();
    let name_ptr = name.as_ptr();
    let name_len = name.len();
    
    core::arch::asm!(
        "syscall",
        in("rax") 20, // SYSCALL_SPAWN
        in("rdi") data_ptr,
        in("rsi") data_len,
        in("rdx") name_ptr,
        in("r10") name_len,
        lateout("rax") ret,
        out("rcx") _,
        out("r11") _,
    );
    ret
}

#[no_mangle]
pub extern "C" fn _start(heap_start: u64) -> ! {
    unsafe { std::rt::init_heap(heap_start as usize, 32 * 1024 * 1024); }
    std::init();
    
    let g = GraphClient::new();
    let c = StdoutConsole;
    let mut buf = [0u8; 4096]; // Ensure large enough for replies

    let _ = c.write_str("LOADED: Starting...\n");

    // 1. Wait for /boot
    let mut mount_id = None;
    
    loop {
        // Scan for MOUNTS attached to BOOT_ROOT? No, scan for MOUNTS linked to HAS_MOUNT_KIND?
        // BootFS scanner creates a MountBody thing and links it to BOOT_ROOT with HAS_MOUNT_KIND.
        
        let op = GraphOp::ScanLinks { from: Some(THING_BOOT_ROOT), to: None, kind: Some(THING_HAS_MOUNT_KIND) };
        if let Ok(GraphReply::Links(links)) = g.call_op(&op, &mut buf) {
            for (_from, to, _pd) in links {
                let op = GraphOp::GetThing { id: to };
                if let Ok(GraphReply::TypedValue(tb)) = g.call_op(&op, &mut buf) {
                    if let Ok(mnt) = postcard::from_bytes::<MountBody>(&tb.bytes) {
                        if mnt.path == "/boot" {
                            mount_id = Some(to);
                            let _ = c.write_str("LOADED: Found /boot!\n");
                            break;
                        }
                    }
                }
            }
        }
        
        if mount_id.is_some() { break; }
        
        let _ = c.write_str("LOADED: Waiting for /boot...\n");
        sleep_ms(100);
    }
    
    let mount_id = mount_id.unwrap();

    // 2. Scan recursively
    let mut files_to_spawn = Vec::new();
    loop {
        // Clear previous results
        files_to_spawn.clear();

        // Step 2a: Find Root Dir of Mount
        let op = GraphOp::ScanLinks { from: Some(mount_id), to: None, kind: Some(THING_MOUNTS_KIND) };
        if let Ok(GraphReply::Links(links)) = g.call_op(&op, &mut buf) {
            if !links.is_empty() {
                 let root_dir_id = links[0].1;
                 scan_dir(&g, &c, root_dir_id, &mut files_to_spawn, &mut buf);
            }
        }

        let _ = c.write_str(&format!("LOADED: Found {} candidates.\n", files_to_spawn.len()));
        
        if !files_to_spawn.is_empty() {
            break;
        }

        let _ = c.write_str("LOADED: No candidates yet. Retrying...\n");
        sleep_ms(1000);
    }

    // 3. Spawn
    for (id, name, size) in files_to_spawn {
        let name: alloc::string::String = name; // Type hint
        if name.ends_with(".elf") && !name.ends_with("loaded.elf") && name != "kernel" {
             let _ = c.write_str(&format!("LOADED: Spawning {}...\n", name));
             
             // Allocate buffer for file content
             let mut file_buf = alloc::vec![0u8; size as usize];
             
             // Read in chunks? System call read_bytes likely supports full read if buf is large enough?
             // GraphClient::read_bytes uses SYSCALL_GRAPH with ReadBytes op.
             // If we rely on graph.rs read helper...
             
             // Let's try reading in one go.
             // Wait, GraphClient::read_bytes takes a &mut [u8] scratch buffer for reply header? No it takes out_buf for data?
             // No, `call_op` takes scratch buffer.
             // `read_bytes` implementation:
             /*
                pub fn read_bytes(&self, thing: ThingId, offset: u64, len: usize, buf: &mut [u8]) -> Result<&[u8], SysRet> {
                    // This copies into `buf`.
                }
             */
             
             if let Ok(_) = g.read_bytes(id, 0, size as u32, &mut file_buf) {
                 // Spawn
                 let _ = unsafe { sys_spawn(&file_buf, &name) };
             } else {
                 let _ = c.write_str("LOADED: Failed to read file!\n");
             }
        }
    }

    let _ = c.write_str("LOADED: All done. Sleeping.\n");
    loop {
        sleep_ms(1000);
    }
}

fn scan_dir(g: &GraphClient, c: &StdoutConsole, dir_id: ThingId, files: &mut Vec<(ThingId, alloc::string::String, u64)>, buf: &mut [u8]) {
    use thing_models::core::fs::DirBody;
    
    // Scan contents (Entries)
    let op = GraphOp::ScanLinks { from: Some(dir_id), to: None, kind: Some(THING_HAS_ENTRY_KIND) };
    if let Ok(GraphReply::Links(entries)) = g.call_op(&op, buf) {
        // Collect IDs first to avoid buffer borrowing conflict if recursive? 
        // We need to copy the entry list because `buf` is re-used in recursion.
        let mut child_ids = Vec::new();
        for (_src, dst, _pd) in entries {
            child_ids.push(dst);
        }
        
        for child_id in child_ids {
            let op = GraphOp::GetThing { id: child_id };
            // Temporarily use stack buf or re-use `buf`?
            // `g.call_op` needs `buf` for reply.
            // If we are careful... 
            // We can just create a new scratch buffer on stack for `GetThing`
            let mut scratch = [0u8; 1024];
            
            if let Ok(GraphReply::TypedValue(tb)) = g.call_op(&op, &mut scratch) {
                 if tb.type_id.0 == THING_FILE_KIND.0 as u128 {
                     if let Ok(f) = postcard::from_bytes::<FileBody>(&tb.bytes) {
                         files.push((child_id, f.name.into(), f.size));
                     }
                 } else if tb.type_id.0 == THING_DIR_KIND.0 as u128 {
                     // Recurse
                     scan_dir(g, c, child_id, files, buf);
                 }
            }
        }
    }
}
