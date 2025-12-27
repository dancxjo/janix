use core::sync::atomic::{AtomicU64, Ordering};
use crate::graph::GraphStore;
use thing_models::builtins::ids::THING_TIME_INSTANCE;
use thing_models::builtins::ids::THING_TIME_NOW_KIND;
// Use the model struct for the property body, assuming it exists in thing_models
// or we can use a local struct if it matches serialization.
// However, the seed uses `thing_models::core::time::TimeNow`.
// We'll trust that matching struct for updates.

// Monotonic time in nanoseconds since boot
static MONOTONIC_NS: AtomicU64 = AtomicU64::new(0);

// We might want valid system time offset later, for now system = monotonic
static SYSTEM_OFFSET_NS: AtomicU64 = AtomicU64::new(0);

pub fn monotonic_ns() -> u64 {
    MONOTONIC_NS.load(Ordering::Relaxed)
}

pub fn system_ns() -> u64 {
    monotonic_ns() + SYSTEM_OFFSET_NS.load(Ordering::Relaxed)
}

pub fn tick(graph: &mut GraphStore, delta_ns: u64) {
    let now = MONOTONIC_NS.fetch_add(delta_ns, Ordering::Relaxed) + delta_ns;
    
    // Update the :Time thing in the graph.
    // We construct the body and update.
    // note: using postcard serialization.
    
    // We need the struct definition. To avoid circular deps or complex imports, 
    // we can define a local compatible struct or import from thing_models if available.
    // Let's try to import.
    // If thing_models is not available in kernel_core (it is, see Cargo.toml), use it.
    
    use thing_models::core::time::TimeNow; 
    use thing_models::value::ThingBody;
    use abi::wire::typed::{TypedBytes, TypeId, CodecId};

    let body = TimeNow {
        monotonic_ns: now,
        system_ns: now, // For now equal
    };

    // Serialize
    if let Ok(bytes) = postcard::to_allocvec(&body) {
         let typed = TypedBytes {
             type_id: TypeId(THING_TIME_NOW_KIND.0 as u128),
             codec_id: CodecId::POSTCARD,
             bytes,
         };
         
         if let Ok(tb) = ThingBody::from(&typed) {
             // Update the thing
             let _ = graph.update_thing(THING_TIME_INSTANCE, tb);
         }
    }
}
