use crate::Kernel;
use hw::HardwareBridge;
use alloc::string::ToString;
use abi::{ThingId, SymbolId};
use crate::diag::{LogRing};
use thing_models::builtins::ids::*;

use thing_models::diag::{LogEntryBody, ErrorBody, FaultBody};
use thing_models::thing::Thing;
use thing_models::value::ThingBody;
use core::sync::atomic::{AtomicU64, AtomicBool, Ordering};

// State for the flusher
static GRAPH_HEALTHY: AtomicBool = AtomicBool::new(true);
static LAST_SEQ: AtomicU64 = AtomicU64::new(0);

// Boot Session ID
static BOOT_SESSION_ID: AtomicU64 = AtomicU64::new(0);
static BOOT_SESSION_CREATED: AtomicBool = AtomicBool::new(false);

fn ensure_boot_session<B: HardwareBridge>(_kernel: &mut Kernel<B>) {
    if !BOOT_SESSION_CREATED.load(Ordering::Relaxed) {
        // Create BootSession Thing
        // TODO: use timestamp or random
        let boot_id = 12345; // Placeholder
        BOOT_SESSION_ID.store(boot_id, Ordering::Relaxed);
        BOOT_SESSION_CREATED.store(true, Ordering::Relaxed);

        // Actually insert Thing?
        // Skipped for brevity, but we should create a Thing for the boot session here.
    }
}

pub fn flush_diagnostics<B: HardwareBridge>(kernel: &mut Kernel<B>) {
    if !GRAPH_HEALTHY.load(Ordering::Relaxed) {
        return;
    }

    ensure_boot_session(kernel);

    let ring = LogRing::global();

    // We limit processing to avoid starving the system
    let max_entries = 50;
    let mut processed = 0;

    ring.drain(|entry| {
        if processed >= max_entries {
            return;
        }
        processed += 1;

        // Convert entry to Thing
        // Note: entry is internal. We copy out data.

        let msg = {
             let len = entry.msg_len as usize;
             let slice = &entry.msg_bytes[..len];
             core::str::from_utf8(slice).unwrap_or("<invalid utf8>").to_string()
        };

        // Use a static seq counter for now if entry.seq is not used
        let seq = LAST_SEQ.fetch_add(1, Ordering::Relaxed);
        let boot_id = BOOT_SESSION_ID.load(Ordering::Relaxed);

        // Generate a deterministic ID based on boot + seq
        // Simple hash: boot_id << 32 | seq (if seq < 32 bits)
        // Or just some mix.
        let unique_id = boot_id.wrapping_add(seq).wrapping_add(2000000); // Placeholder mix
        let tid = ThingId(unique_id);

        let thing_res = match entry.kind {
            0 => { // Log
                // Safety: internal log ring levels must match model enums
                let level: thing_models::diag::LogLevel = unsafe { core::mem::transmute(entry.level) };
                let body = LogEntryBody {
                    timestamp_ns: entry.timestamp_or_ticks, // TODO: real time
                    level,
                    message: msg,
                    subsystem: SymbolId(0), // unknown
                    cpu_id: entry.cpu,
                    thread_id: 0,
                    process_id: 0,
                    seq,
                };

                Some(Thing {
                    id: tid,
                    kind: THING_LOG_ENTRY_KIND,
                    body: ThingBody::from(&body).unwrap(),
                })
            }
            1 => { // Error
                let body = ErrorBody {
                    code: SymbolId(0),
                    message: msg,
                    severity: entry.level,
                    recoverable: false,
                };
                Some(Thing {
                    id: tid,
                    kind: THING_ERROR_KIND,
                    body: ThingBody::from(&body).unwrap(),
                })
            }
            2 => { // Fault
                 // Safety: assuming level holds FaultKind discriminant
                let fault_kind: thing_models::diag::FaultKind = unsafe { core::mem::transmute(entry.level) };
                let body = FaultBody {
                    fault_kind,
                    rip: entry.payload_a,
                    error_code: entry.payload_b,
                    cr2: entry.payload_c,
                    rflags: entry.payload_d,
                    rsp: 0, // lost for now
                    access: thing_models::diag::Access::Read, // placeholder
                    address_space: thing_models::diag::AddressSpace::Kernel, // placeholder
                    kill_action: thing_models::diag::KillAction::Panic, // placeholder
                };
                Some(Thing {
                    id: tid,
                    kind: THING_FAULT_KIND,
                    body: ThingBody::from(&body).unwrap(),
                })
            }
            _ => None,
        };

        if let Some(thing) = thing_res {
             match kernel.graph.insert_thing(thing) {
                 Ok(_) => {},
                 Err(_) => {
                     // If graph full or error, mark unhealthy
                     GRAPH_HEALTHY.store(false, Ordering::Relaxed);
                     // Fallback logging?
                 }
             }
        }
    });
}
