#![no_std]
#![no_main]

extern crate alloc;


use thing_std as std;
use thing_std::{GraphClient, StdoutConsole, Console};
use alloc::format;


use thing_models::abi::wire::graph::{GraphOp, GraphReply};
use thing_models::builtins::ids::*;
use thing_models::core::fs::{FileBody, MountBody};

#[no_mangle]
pub extern "C" fn _start(heap_start: u64) -> ! {
    unsafe { std::rt::init_heap(heap_start as usize, 1024 * 1024); }
    std::init();
    
    let g = GraphClient::new();
    let c = StdoutConsole;
    let mut buf = [0u8; 4096];

    let _ = c.write_str("CAT_BOOT: Starting...\n");

    // 1. Find /boot
    let mut mount_id = None;
    let op = GraphOp::ScanLinks { from: Some(THING_BOOT_ROOT), to: None, kind: Some(THING_HAS_MOUNT_KIND) };
    if let Ok(GraphReply::Links(links)) = g.call_op(&op, &mut buf) {
        for (_from, to, _pd) in links {
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
    }

    if let Some(m_id) = mount_id {
        // 2. Find Root Dir
        let op = GraphOp::ScanLinks { from: Some(m_id), to: None, kind: Some(THING_MOUNTS_KIND) };
        if let Ok(GraphReply::Links(links)) = g.call_op(&op, &mut buf) {
            if !links.is_empty() {
                let root_dir_id = links[0].1;
                
                // 3. Find First File
                // Filter by HAS_ENTRY_KIND to avoid catching the Volume thing as a file
                let op = GraphOp::ScanLinks { from: Some(root_dir_id), to: None, kind: Some(THING_HAS_ENTRY_KIND) };
                if let Ok(GraphReply::Links(entries)) = g.call_op(&op, &mut buf) {
                     for (_src, dst, _pd) in entries {
                         // Check if file
                         let op = GraphOp::GetThing { id: dst };
                         if let Ok(GraphReply::TypedValue(tb)) = g.call_op(&op, &mut buf) {
                              // Ensure it is a File Kind (Safety)
                              if tb.type_id.0 == THING_FILE_KIND.0 as u128 {
                                  if let Ok(f) = postcard::from_bytes::<FileBody>(&tb.bytes) {
                                      let _ = c.write_str(&format!("Found File: {} ({} bytes). Reading...\n", f.name, f.size));
                                      
                                      // 4. READ CONTENT
                                      // Use read_bytes which returns Vec<u8>
                                      if let Ok(data) = g.read_bytes(dst, 0, 64, &mut buf) {
                                          let _ = c.write_str("--- Content Start ---\n");
                                          // Print formatted
                                          for chunk in data.chunks(16) {
                                              for b in chunk {
                                                  let _ = c.write_str(&format!("{:02X} ", b));
                                              }
                                              let _ = c.write_str(" | ");
                                              for b in chunk {
                                                  if *b >= 0x20 && *b < 0x7F {
                                                      let _ = c.write_str(&format!("{}", *b as char));
                                                  } else {
                                                      let _ = c.write_str(".");
                                                  }
                                              }
                                              let _ = c.write_str("\n");
                                          }
                                          let _ = c.write_str("--- Content End ---\n");
                                      } else {
                                          let _ = c.write_str("Failed to read file content.\n");
                                      }
                                      
                                      // Wait a bit then exit/loop
                                      loop {}
                                  }
                              }
                         }
                     }
                }
            }
        }
    } else {
        let _ = c.write_str("Could not find /boot.\n");
    }

    loop {}
}
