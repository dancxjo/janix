use abi::ids::ThingId;
use core::sync::atomic::{AtomicU64, Ordering};

static NEXT_THING_ID: AtomicU64 = AtomicU64::new(0x1_0000_0000);

pub fn next_thing_id() -> ThingId {
    let id = NEXT_THING_ID.fetch_add(1, Ordering::Relaxed);
    ThingId(id as u128)
}
