use crate::machine::machine;
use models::*;
use abi::ids::{SymbolId, ThingId};
use graph::store;
use graph::symbols::sym;
use spin::Mutex;

static TIME_STATE: Mutex<TimeState> = Mutex::new(TimeState::new());

struct TimeState {
    monotonic_thing: Option<ThingId>,
    system_thing: Option<ThingId>,
    offset_thing: Option<ThingId>,

    system_offset_ns: i64,
    system_set: bool,
}

impl TimeState {
    const fn new() -> Self {
        Self {
            monotonic_thing: None,
            system_thing: None,
            offset_thing: None,
            system_offset_ns: 0,
            system_set: false,
        }
    }
}

pub fn init() {
    let mut state = TIME_STATE.lock();

    store::with_store(|s| {
        // 1. Create/Find Places
        let place_time = s.find_by_name(sym::PLACE_TIME).expect("place.time missing");

        let place_mono = s
            .create_thing(sym::KIND_PLACE)
            .expect("create place.time.monotonic");
        s.register_name(place_mono, sym::PLACE_TIME_MONOTONIC);
        s.create_relationship(sym::PRED_CONTAINS, place_time, place_mono)
            .expect("link place.time.monotonic");

        let place_system = s
            .create_thing(sym::KIND_PLACE)
            .expect("create place.time.system");
        s.register_name(place_system, sym::PLACE_TIME_SYSTEM);
        s.create_relationship(sym::PRED_CONTAINS, place_time, place_system)
            .expect("link place.time.system");

        // 2. Create MonotonicClock Thing
        let source = match () {
            #[cfg(target_arch = "x86_64")]
            _ => sym::TIME_SOURCE_APIC_TSC,
            #[cfg(target_arch = "aarch64")]
            _ => sym::TIME_SOURCE_ARM_CNTVCT,
            #[cfg(target_arch = "riscv64")]
            _ => sym::TIME_SOURCE_RISCV_RDTIME,
            #[cfg(target_arch = "loongarch64")]
            _ => sym::TIME_SOURCE_LOONGARCH_RDTIME,
        };

        let mono_body = MonotonicClock {
            now_ns: machine().monotonic_now(),
            resolution_ns: 1, // nanoseconds
            source,
            last_update_ns: machine().monotonic_now(),
        };

        let mono_thing = s
            .create_thing(sym::KIND_MONOTONIC_CLOCK)
            .expect("create MonotonicClock");
        mono_body.write(s, mono_thing).expect("write MonotonicClock");
        s.create_relationship(sym::PRED_CONTAINS, place_mono, mono_thing)
            .expect("link MonotonicClock");
        state.monotonic_thing = Some(mono_thing);

        // 3. Create SystemClock Thing (Unset)
        let sys_body = SystemClock {
            unix_epoch_ns: 0,
            status: 0, // Unset
            _pad: 0,
            last_set_mono_ns: 0,
            accuracy_ns: 0,
            source: sym::TIME_SOURCE_MANUAL,
        };

        let sys_thing = s
            .create_thing(sym::KIND_SYSTEM_CLOCK)
            .expect("create SystemClock");
        sys_body.write(s, sys_thing).expect("write SystemClock");
        s.create_relationship(sym::PRED_CONTAINS, place_system, sys_thing)
            .expect("link SystemClock");
        state.system_thing = Some(sys_thing);
    });
}

pub fn monotonic_now() -> u64 {
    machine().monotonic_now()
}

pub fn system_now() -> Result<i64, ()> {
    let state = TIME_STATE.lock();
    if !state.system_set {
        return Err(());
    }

    let mono = machine().monotonic_now();
    Ok(mono as i64 + state.system_offset_ns)
}

pub fn set_system_time(unix_epoch_ns: i64) {
    let mut state = TIME_STATE.lock();
    let mono = machine().monotonic_now();
    state.system_offset_ns = unix_epoch_ns - mono as i64;
    state.system_set = true;

    // Update graph
    store::with_store(|s| {
        if let Some(sys_thing) = state.system_thing {
            let sys_body = SystemClock {
                unix_epoch_ns,
                status: 1, // Set
                _pad: 0,
                last_set_mono_ns: mono,
                accuracy_ns: 0,
                source: sym::TIME_SOURCE_MANUAL,
            };
            sys_body.write(s, sys_thing).expect("write SystemClock");
        }

        // Create or update TimeOffset Thing
        let offset_body = TimeOffset {
            offset_ns: state.system_offset_ns,
            rate_ppb: 0,
            updated_mono_ns: mono,
        };

        if let Some(offset_thing) = state.offset_thing {
            offset_body.write(s, offset_thing).expect("write TimeOffset");
        } else {
            let place_system = s
                .find_by_name(sym::PLACE_TIME_SYSTEM)
                .expect("place.time.system missing");
            let offset_thing = s
                .create_thing(sym::KIND_TIME_OFFSET)
                .expect("create TimeOffset");
            offset_body.write(s, offset_thing).expect("write TimeOffset");
            s.create_relationship(sym::PRED_CONTAINS, place_system, offset_thing)
                .expect("link TimeOffset");
            state.offset_thing = Some(offset_thing);
        }
    });
}

pub fn update_monotonic_thing() {
    let state = TIME_STATE.lock();
    if let Some(mono_thing) = state.monotonic_thing {
        let now = machine().monotonic_now();
        store::with_store(|s| {
            // We only update now_ns and last_update_ns
            // For efficiency, we might want a way to partially update bodies,
            // but for now we just rewrite the whole thing.
            let source = match () {
                #[cfg(target_arch = "x86_64")]
                _ => sym::TIME_SOURCE_APIC_TSC,
                #[cfg(target_arch = "aarch64")]
                _ => sym::TIME_SOURCE_ARM_CNTVCT,
                #[cfg(target_arch = "riscv64")]
                _ => sym::TIME_SOURCE_RISCV_RDTIME,
                #[cfg(target_arch = "loongarch64")]
                _ => sym::TIME_SOURCE_LOONGARCH_RDTIME,
            };

            let mono_body = MonotonicClock {
                now_ns: now,
                resolution_ns: 1,
                source,
                last_update_ns: now,
            };
            mono_body.write(s, mono_thing).expect("write MonotonicClock");
        });
    }
}
