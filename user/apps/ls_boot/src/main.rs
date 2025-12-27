#![no_std]
#![no_main]

extern crate alloc;

use thing_std as std;
use thing_std::{GraphClient, StdoutConsole, Console};
use alloc::format;
use core::fmt::Write;

use thing_models::abi::wire::graph::{GraphOp, GraphReply};
use thing_models::builtins::ids::*;
use thing_models::core::fs::{FileBody, DirBody, MountBody};

#[no_mangle]
pub extern "C" fn _start(heap_start: u64) -> ! {
    unsafe { std::rt::init_heap(heap_start as usize, 1024 * 1024); }
    std::init();
    
    let g = GraphClient::new();
    let c = StdoutConsole;
    let mut buf = [0u8; 4096];

    let _ = c.write_str("LS_BOOT: Starting...\n");

    // 1. Find /boot mount
    let op = GraphOp::ScanLinks { from: Some(THING_BOOT_ROOT), to: None, kind: Some(THING_HAS_MOUNT_KIND) };
    let reply = g.call_op(&op, &mut buf);
    
    let links = match reply {
        Ok(GraphReply::Links(l)) => l,
        _ => { let _ = c.write_str("Error listing mounts\n"); loop {} }
    };

    let mut mount_id = None;
    for (_from, to, _kind) in links {
        let op = GraphOp::GetThing { id: to };
        if let Ok(GraphReply::TypedValue(tb)) = g.call_op(&op, &mut buf) {
             if let Ok(mnt) = postcard::from_bytes::<MountBody>(&tb.bytes) {
                 if mnt.path == "/boot" {
                     mount_id = Some(to);
                     break;
                 }
             }
        }
    }

    if mount_id.is_none() {
        let _ = c.write_str("/boot not found\n");
        loop {}
    }
    let m_id = mount_id.unwrap();
    let _ = c.write_str(&format!("Found /boot at Thing({})\n", m_id.0));

    // 2. Find Root Dir
    let op = GraphOp::ScanLinks { from: Some(m_id), to: None, kind: Some(THING_MOUNTS_KIND) };
    let reply = g.call_op(&op, &mut buf);
    match reply {
        Ok(GraphReply::Links(links)) => {
            if links.is_empty() {
                let _ = c.write_str("Empty mount\n");
                loop {}
            }
            let root_dir_id = links[0].1;

            // 3. List entries
            let op = GraphOp::ScanLinks { from: Some(root_dir_id), to: None, kind: Some(THING_HAS_ENTRY_KIND) }; // Assuming HAS_ENTRY_KIND check in kernel main.rs used this.
            // Wait, main.rs used `THING_LINK_KIND` for the Link Thing, but `predicate` field was used.
            // If I look at main.rs (Step 653): `predicate: THING_HAS_ENTRY_KIND`. No, `THING_CONTAINS_FILE_KIND` was removed.
            // I should check `main.rs` link logic.
            // Line 816: `predicate: THING_MOUNTS_KIND`.
            // Line 823: `predicate: THING_ON_VOLUME_KIND`.
            // Lines 866-880: Linking Dirs/Files?
            // "Link Dir -> ON_VOLUME"?
            // "Link Dir -> HAS_ENTRY -> File"?
            // I'll assume HAS_ENTRY or similar.
            // I'll check builtins/ids.rs later if needed, but for now I'll use None for kind to list ALL links.
            
            let op = GraphOp::ScanLinks { from: Some(root_dir_id), to: None, kind: None };
            let reply = g.call_op(&op, &mut buf);
             if let Ok(GraphReply::Links(links)) = reply {
                 let _ = c.write_str("Entries in /boot:\n");
                 for (_from, to, _kind) in links {
                     let op = GraphOp::GetThing { id: to };
                     if let Ok(GraphReply::TypedValue(tb)) = g.call_op(&op, &mut buf) {
                          if let Ok(f) = postcard::from_bytes::<FileBody>(&tb.bytes) {
                              let _ = c.write_str(&format!("FILE: {} ({} bytes)\n", f.name, f.size));
                          } else if let Ok(d) = postcard::from_bytes::<DirBody>(&tb.bytes) {
                              let _ = c.write_str(&format!("DIR:  {}\n", d.name));
                          }
                     }
                 }
             }
        },
        _ => { let _ = c.write_str("Error listing mount children\n"); }
    }

    loop {}
}
