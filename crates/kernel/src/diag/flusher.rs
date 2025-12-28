use crate::diag::{EntryKind, LogRing};
use crate::Kernel;
use abi::{SymbolId, ThingId};
use alloc::string::ToString;
use crate::bridge::HardwareBridge;
use thing_models::builtins::ids::*;

use abi::wire::typed::{CodecId, TypeId, TypedBytes};
use core::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use thing_models::core::serial::{LogEntryCompact, LogStreamBody};
use thing_models::thing::Thing;
use thing_models::value::ThingBody;

// Log Stream Thing ID (Singleton-ish for now)
const LOG_STREAM_ID: ThingId = ThingId(3020);

// State for the flusher
static GRAPH_HEALTHY: AtomicBool = AtomicBool::new(true);
static LAST_SEQ: AtomicU64 = AtomicU64::new(0);

static LOG_STREAM_INITIALIZED: AtomicBool = AtomicBool::new(false);

pub fn flush_diagnostics<B: HardwareBridge>(kernel: &mut Kernel<B>) {
    // 1. Ensure Stream Thing Exists
    if !LOG_STREAM_INITIALIZED.load(Ordering::Relaxed) {
        let body = LogStreamBody {
            head_seq: 0,
            capacity: 256,
            dropped: 0,
            entries: alloc::vec::Vec::new(),
        };

        let body_bytes = postcard::to_allocvec(&body).unwrap();
        let tb = ThingBody::from(&TypedBytes {
            type_id: TypeId(THING_LOG_STREAM_KIND.0 as u128),
            codec_id: CodecId::POSTCARD,
            bytes: body_bytes,
        })
        .unwrap();

        let thing = Thing {
            id: LOG_STREAM_ID,
            kind: THING_LOG_STREAM_KIND,
            body: tb,
        };
        let _ = kernel.graph.insert_thing(thing);

        // Link Root -> LogStream (EMITS? or HAS_CONSOLE?)
        // Let's us EMITS from Root for now.
        let link = thing_models::link::LinkBody {
            from: THING_BOOT_ROOT,
            to: LOG_STREAM_ID,
            predicate: THING_EMITS_KIND,
        };
        let lb = ThingBody::from(&TypedBytes {
            type_id: TypeId(THING_LINK_KIND.0 as u128),
            codec_id: CodecId::POSTCARD,
            bytes: postcard::to_allocvec(&link).unwrap(),
        })
        .unwrap();
        let _ = kernel.graph.create_thing(THING_LINK_KIND, lb);

        LOG_STREAM_INITIALIZED.store(true, Ordering::Relaxed);
    }

    // 2. Read new entries
    let ring = LogRing::global();
    let mut new_entries = alloc::vec::Vec::new();

    ring.drain(|entry| {
        let msg = {
            let len = entry.msg_len as usize;
            let slice = &entry.msg_bytes[..len];
            let s = core::str::from_utf8(slice).unwrap_or("<invalid utf8>");
            // Echo to serial/bridge (Redundant if we trust stream, but good for debug)
            kernel.bridge.log(s);
            s.to_string()
        };

        // Filter out low level noise? No, we want everything.

        let seq = LAST_SEQ.fetch_add(1, Ordering::Relaxed);
        let compact = LogEntryCompact {
            seq,
            timestamp_ns: entry.timestamp_or_ticks, // Approximate
            level: entry.level,
            message: msg,
        };
        new_entries.push(compact);
    });

    if new_entries.is_empty() {
        return;
    }

    // 3. Update Thing
    // We need to read existing body, append, and write back.
    // GraphStore doesn't support partial updates yet.
    // This is expensive (deserialize -> append -> serialize), but correct for the model.

    let current_bytes = if let Some(thing) = kernel.graph.get(LOG_STREAM_ID) {
        thing.body.bytes.clone()
    } else {
        return;
    };

    if let Ok(mut body) = postcard::from_bytes::<LogStreamBody>(&current_bytes) {
        // Append
        for e in new_entries {
            body.entries.push(e);
            body.head_seq += 1;
        }

        // Trim
        while body.entries.len() > body.capacity as usize {
            body.entries.remove(0);
            body.dropped += 1;
        }

        // Write Back
        let new_body_bytes = postcard::to_allocvec(&body).unwrap();

        let new_tb = ThingBody::from(&TypedBytes {
            type_id: TypeId(THING_LOG_STREAM_KIND.0 as u128),
            codec_id: CodecId::POSTCARD,
            bytes: new_body_bytes,
        })
        .unwrap();

        let _ = kernel.graph.update_thing(LOG_STREAM_ID, new_tb);
    }
}
