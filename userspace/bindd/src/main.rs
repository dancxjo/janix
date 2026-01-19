#![no_std]
#![no_main]

extern crate alloc;

use abi::ids::HandleId; // Import trait for from_u64/to_u64_lossy methods
use abi::schema::{keys, kinds};
use abi::types::RootWatchEvent;
use alloc::vec::Vec;
use core::time::Duration;
use stem::info;
use stem::thing::sys::{find, intern, prop_get, prop_set, stream_poll, watch_subscribe};
use stem::thing::ThingId;

struct ActiveBinding {
    _source: ThingId, // kept for ref?
    target: ThingId,
    stream: ThingId,
}

#[stem::main]
fn main() -> ! {
    info!("bindd starting...");

    let clock_now_text_key = intern(keys::CLOCK_NOW_TEXT).expect("intern key");
    let mut bindings = Vec::new();

    // Initial scan for bindings (in a real daemon this would be dynamic/watched)
    // We retry a few times to wait for the clock publisher
    let mut binding_ids = [ThingId::default(); 16];

    // Simple retry loop for finding bindings
    for _ in 0..120 {
        if let Ok(count) = find(kinds::BINDING, &mut binding_ids) {
            if count > 0 {
                info!("Found {} bindings", count);
                for i in 0..count {
                    let b_id = binding_ids[i];

                    // Read properties
                    let src_id = prop_get(b_id, keys::BINDING_SOURCE)
                        .map(ThingId::from_u64)
                        .unwrap_or(ThingId::default());
                    let dst_id = prop_get(b_id, keys::BINDING_TARGET)
                        .map(ThingId::from_u64)
                        .unwrap_or(ThingId::default());

                    if src_id.to_u64_lossy() == 0 || dst_id.to_u64_lossy() == 0 {
                        continue;
                    }

                    // Subscribe to source
                    // Mask 0xFF... for all events or specific?
                    // Usually we want changes.
                    // Let's assume mask 1 is CHANGE. Or just pass !0 for everything.
                    if let Ok(stream) = watch_subscribe(src_id, !0) {
                        info!(
                            "Subscribed to source {} for binding {}",
                            src_id.to_u64_lossy(),
                            b_id.to_u64_lossy()
                        );
                        bindings.push(ActiveBinding {
                            _source: src_id,
                            target: dst_id,
                            stream,
                        });
                    } else {
                        info!("Failed to subscribe to source {}", src_id.to_u64_lossy());
                    }
                }
                break;
            }
        }
        stem::sleep(Duration::from_secs(1));
    }

    if bindings.is_empty() {
        info!("No bindings found after retries. Exiting.");
        loop {
            stem::sleep(Duration::from_secs(10));
        }
    }

    info!("Entering event loop with {} bindings", bindings.len());

    let mut event = RootWatchEvent::default();
    loop {
        let mut did_work = false;

        for binding in &bindings {
            // Poll stream
            // stream_poll returns number of events read (or 0)
            // It modifies `event`.
            // Non-blocking? The syscall implementation might be non-blocking or blocking.
            // Usually stream_poll is non-blocking if empty.
            if let Ok(n) = stream_poll(binding.stream, &mut event) {
                if n > 0 {
                    did_work = true;
                    // Check if the changed key matches what we care about
                    // For v0, we only map clock.now_text -> ui.text
                    if event.key == clock_now_text_key as u64 {
                        // event.value is the new value (BytespaceID)
                        // Write to target ui.text
                        // Note: prop_set expects a u64 value.
                        if prop_set(binding.target, keys::UI_TEXT, event.value).is_ok() {
                            // info!("Updated target {} with val {}", binding.target.0, event.value);
                        }
                    }
                }
            }
        }

        if !did_work {
            stem::sleep(Duration::from_millis(50));
        }
    }
}
