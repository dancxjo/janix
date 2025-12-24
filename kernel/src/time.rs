extern crate alloc;

use alloc::boxed::Box;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicBool, AtomicI64, AtomicU64, Ordering};

use crate::{graph, graph_kinds};
use abi::ThingId;
use thing_models::PropValue;
use thing_models::{AlarmEvent, AlarmRequest, TimeSource};

/// Trait for hardware timer abstraction
pub trait HardwareTimer: Sync + Send {
    /// Initialize the hardware timer for this CPU / system.
    /// Called once at boot per arch.
    fn init(&self);

    /// Returns a monotonically increasing time in nanoseconds since boot.
    /// (Monotonic, not wall-clock.)
    fn now_ns(&self) -> u64;

    /// Schedule a timer interrupt at or after `deadline_ns`.
    /// For now this can be a no-op or “next tick” on architectures where this is hard.
    fn set_deadline_ns(&self, deadline_ns: u64);
}

static mut TIMER: Option<&'static dyn HardwareTimer> = None;

/// Register the global timer instance.
/// This should be called by the architecture initialization code.
pub fn register_timer(timer: &'static dyn HardwareTimer) {
    unsafe {
        TIMER = Some(timer);
    }
}

/// Get the global timer instance.
/// Logs if no timer has been registered.
pub fn timer() -> &'static dyn HardwareTimer {
    unsafe {
        match TIMER {
            Some(t) => t,
            None => {
                struct DummyTimer;
                impl HardwareTimer for DummyTimer {
                    fn init(&self) {}
                    fn now_ns(&self) -> u64 {
                        0
                    }
                    fn set_deadline_ns(&self, _deadline_ns: u64) {}
                }
                static DUMMY: DummyTimer = DummyTimer;
                static WARNED: AtomicBool = AtomicBool::new(false);

                if !WARNED.swap(true, Ordering::Relaxed) {
                    crate::log("No hardware timer registered; using dummy timer.");
                }
                &DUMMY
            }
        }
    }
}

/// Returns a monotonically increasing time in nanoseconds since boot.
pub fn monotonic_now_ns() -> u64 {
    timer().now_ns()
}

/// Convert ns to a simple `Duration` type (your own, not std).
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Duration {
    pub secs: u64,
    pub nanos: u32,
}

pub fn ns_to_duration(ns: u64) -> Duration {
    let secs = ns / 1_000_000_000;
    let rem = ns % 1_000_000_000;
    Duration {
        secs,
        nanos: rem as u32,
    }
}

/// Real-time clock: wall-clock time, ideally UTC since Unix epoch.
pub trait RealTimeClock: Sync + Send {
    /// Initialize the RTC hardware / integration.
    fn init(&self);

    /// Returns (seconds, nanoseconds) since Unix epoch (1970-01-01T00:00:00Z),
    /// or a best-effort approximation if true UTC time is unavailable.
    fn now_utc(&self) -> (u64, u32);
}

static mut RTC: Option<&'static dyn RealTimeClock> = None;

/// Access the global RTC instance.
pub fn rtc() -> Option<&'static dyn RealTimeClock> {
    unsafe { RTC }
}

/// Register the concrete RTC instance (called from arch init).
pub fn register_rtc(rtc_impl: &'static dyn RealTimeClock) {
    unsafe {
        RTC = Some(rtc_impl);
    }
}

/// Helper: gets current UTC time in a single u64 ns value.
pub fn system_time_ns() -> Option<u64> {
    rtc().map(|r| {
        let (secs, nanos) = r.now_utc();
        secs.saturating_mul(1_000_000_000) + nanos as u64
    })
}

const TICK_HZ: u32 = 1000;
const NANOS_PER_TICK: u64 = 1_000_000;
const TIME_SOURCE_UPDATE_INTERVAL_TICKS: u64 = 100;
const ALARM_ACTUALIZER_INTERVAL_TICKS: u64 = 10;
const SANITY_LOG_TICKS: u64 = 5 * TICK_HZ as u64;

static TICKS_SINCE_BOOT: AtomicU64 = AtomicU64::new(0);
static RTC_EPOCH_SECONDS: AtomicI64 = AtomicI64::new(0);
static LAST_MONOTONIC_NS: AtomicU64 = AtomicU64::new(0);
static TIME_SOURCE_ID: AtomicU64 = AtomicU64::new(u64::MAX);
static LAST_TIME_SOURCE_UPDATE: AtomicU64 = AtomicU64::new(0);
static LAST_ALARM_POLL_TICKS: AtomicU64 = AtomicU64::new(0);
static SANITY_LOGGED: AtomicBool = AtomicBool::new(false);

pub fn init_timekeeping(rtc_epoch_seconds: i64) {
    RTC_EPOCH_SECONDS.store(rtc_epoch_seconds, Ordering::Relaxed);
    LAST_MONOTONIC_NS.store(monotonic_now_ns(), Ordering::Relaxed);
    SANITY_LOGGED.store(false, Ordering::Relaxed);
}

pub fn rtc_epoch_seconds() -> i64 {
    RTC_EPOCH_SECONDS.load(Ordering::Relaxed)
}

pub fn tick_hz() -> u32 {
    TICK_HZ
}

pub fn ticks_since_boot() -> u64 {
    TICKS_SINCE_BOOT.load(Ordering::Relaxed)
}

pub fn bind_time_source(id: ThingId) {
    TIME_SOURCE_ID.store(id.0, Ordering::Relaxed);
    refresh_time_source();
}

pub fn refresh_time_source() {
    let raw_id = TIME_SOURCE_ID.load(Ordering::Relaxed);
    if raw_id == u64::MAX {
        return;
    }
    let ticks = ticks_since_boot();
    refresh_time_source_with(raw_id, ticks);
}

fn refresh_time_source_with(raw_id: u64, ticks: u64) {
    let (unix_seconds, unix_nanos) = now_unix_from_rtc_ticks(ticks);
    let props_str = TimeSource::update_from_kernel(ticks, unix_seconds, unix_nanos);
    let props = props_str
        .into_iter()
        .map(|(k, v)| (crate::symbols::intern(&k), v))
        .collect();
    let _ = graph::update_thing(ThingId(raw_id), props);
}

pub fn now_unix_from_rtc() -> (i64, u32) {
    now_unix_from_rtc_ticks(ticks_since_boot())
}

fn now_unix_from_rtc_ticks(ticks: u64) -> (i64, u32) {
    let base = rtc_epoch_seconds();
    let seconds_from_ticks = (ticks / TICK_HZ as u64) as i64;
    let nanos = ((ticks % TICK_HZ as u64) * NANOS_PER_TICK) as u32;
    (base.saturating_add(seconds_from_ticks), nanos)
}

pub fn unix_to_ticks(target_unix_seconds: i64, target_unix_nanos: u32) -> u64 {
    let base = rtc_epoch_seconds();
    if target_unix_seconds <= base {
        return 0;
    }
    let delta_secs = target_unix_seconds.saturating_sub(base) as u64;
    let secs_ticks = delta_secs.saturating_mul(TICK_HZ as u64);
    let nanos_ticks = (target_unix_nanos as u64) / NANOS_PER_TICK;
    secs_ticks.saturating_add(nanos_ticks)
}

pub fn poll_time() {
    let now_ns = monotonic_now_ns();
    let mut last_ns = LAST_MONOTONIC_NS.load(Ordering::Relaxed);
    if last_ns == 0 {
        LAST_MONOTONIC_NS.store(now_ns, Ordering::Relaxed);
        return;
    }
    if now_ns <= last_ns {
        return;
    }

    loop {
        let delta_ns = now_ns.saturating_sub(last_ns);
        if delta_ns < NANOS_PER_TICK {
            break;
        }
        let ticks = delta_ns / NANOS_PER_TICK;
        let advance_ns = ticks * NANOS_PER_TICK;
        last_ns = last_ns.saturating_add(advance_ns);
        LAST_MONOTONIC_NS.store(last_ns, Ordering::Relaxed);
        advance_ticks(ticks);
    }
}

pub fn advance_ticks(delta: u64) {
    if delta == 0 {
        return;
    }
    let current = TICKS_SINCE_BOOT.fetch_add(delta, Ordering::Relaxed) + delta;
    maybe_refresh_time_source(current);
    maybe_run_alarm_actualizer(current);
    maybe_log_time_sanity(current);
}

fn maybe_refresh_time_source(current_ticks: u64) {
    let last = LAST_TIME_SOURCE_UPDATE.load(Ordering::Relaxed);
    if current_ticks.saturating_sub(last) < TIME_SOURCE_UPDATE_INTERVAL_TICKS {
        return;
    }
    LAST_TIME_SOURCE_UPDATE.store(current_ticks, Ordering::Relaxed);
    rotate_time_source(current_ticks);
}

fn rotate_time_source(current_ticks: u64) {
    let raw_id = TIME_SOURCE_ID.load(Ordering::Relaxed);
    if raw_id == u64::MAX {
        return;
    }
    refresh_time_source_with(raw_id, current_ticks);
}

fn maybe_log_time_sanity(current_ticks: u64) {
    if SANITY_LOGGED.load(Ordering::Relaxed) {
        return;
    }
    if current_ticks < SANITY_LOG_TICKS {
        return;
    }
    let (unix_seconds, unix_nanos) = now_unix_from_rtc();
    log_message(format!(
        "Time sanity: ticks={} unix_seconds={} unix_nanos={}",
        current_ticks, unix_seconds, unix_nanos
    ));
    SANITY_LOGGED.store(true, Ordering::Relaxed);
}

fn maybe_run_alarm_actualizer(current_ticks: u64) {
    let last = LAST_ALARM_POLL_TICKS.load(Ordering::Relaxed);
    if current_ticks.saturating_sub(last) < ALARM_ACTUALIZER_INTERVAL_TICKS {
        return;
    }
    LAST_ALARM_POLL_TICKS.store(current_ticks, Ordering::Relaxed);
    run_alarm_actualizer(current_ticks);
}

enum AlarmAction {
    Arm { id: ThingId, target_ticks: u64 },
    Fire { id: ThingId },
}

fn run_alarm_actualizer(current_ticks: u64) {
    let mut actions: Vec<AlarmAction> = Vec::new();

    graph::iter_things(|thing| {
        if thing.kind != crate::symbols::intern(graph_kinds::KIND_ALARM_REQUEST) {
            return;
        }
        let id = thing.id;
        let mut armed = false;
        let mut fired = false;
        let mut target_secs = None::<i64>;
        let mut target_nanos = None::<u32>;
        let mut target_ticks = None::<u64>;

        for (key, val) in thing.props.iter() {
            if *key == crate::symbols::intern("armed") {
                if let PropValue::Bool(v) = val {
                    armed = *v;
                }
            } else if *key == crate::symbols::intern("fired") {
                if let PropValue::Bool(v) = val {
                    fired = *v;
                }
            } else if *key == crate::symbols::intern("target_unix_seconds") {
                if let PropValue::I64(v) = val {
                    target_secs = Some(*v);
                }
            } else if *key == crate::symbols::intern("target_unix_nanos") {
                if let PropValue::U64(v) = val {
                    target_nanos = Some(*v as u32);
                }
            } else if *key == crate::symbols::intern("target_ticks") {
                if let PropValue::U64(v) = val {
                    target_ticks = Some(*v);
                }
            }
        }

        if !armed && !fired {
            // Pending: try to arm it
            if let (Some(secs), Some(nanos)) = (target_secs, target_nanos) {
                let ticks = unix_to_ticks(secs, nanos);
                actions.push(AlarmAction::Arm {
                    id,
                    target_ticks: ticks,
                });
            }
        } else if armed {
            // Armed: check if it should fire
            if let Some(target) = target_ticks {
                if current_ticks >= target {
                    actions.push(AlarmAction::Fire { id });
                }
            }
        }
    });

    for action in actions {
        match action {
            AlarmAction::Arm { id, target_ticks } => {
                let props_str = AlarmRequest::arm_props(target_ticks);
                let props = props_str
                    .into_iter()
                    .map(|(k, v)| (crate::symbols::intern(&k), v))
                    .collect();
                let _ = graph::update_thing(id, props);
                log_message(format!(
                    "AlarmRequest id={} armed target_ticks={}",
                    id.0, target_ticks
                ));
            }
            AlarmAction::Fire { id } => {
                let props_str = AlarmRequest::fired_props();
                let props = props_str
                    .into_iter()
                    .map(|(k, v)| (crate::symbols::intern(&k), v))
                    .collect();
                let _ = graph::update_thing(id, props);
                let (secs, nanos) = now_unix_from_rtc();
                let event_props_str = AlarmEvent::from_fire(id, secs, nanos);
                let event_props = event_props_str
                    .into_iter()
                    .map(|(k, v)| (crate::symbols::intern(&k), v))
                    .collect();
                let _ = graph::create_thing(
                    crate::symbols::intern(graph_kinds::KIND_ALARM_EVENT),
                    event_props,
                );
                log_message(format!(
                    "AlarmRequest id={} fired at {}.{}",
                    id.0, secs, nanos
                ));
            }
        }
    }
}

fn log_message(msg: String) {
    let leaked: &'static str = Box::leak(msg.into_boxed_str());
    crate::log(leaked);
}

/// Start a named span for boot instrumentation.
/// Returns the start time in nanoseconds.
pub fn boot_span_start(name: &str) -> u64 {
    let now = monotonic_now_ns();
    crate::log(alloc::format!("[SPAN] start {}", name).as_str());
    now
}

/// End a named span for boot instrumentation.
/// Logs the duration in microseconds.
pub fn boot_span_end(name: &str, start_ns: u64) {
    let now = monotonic_now_ns();
    let dt_ns = now.saturating_sub(start_ns);
    let dt_us = dt_ns / 1_000;
    crate::log(alloc::format!("[SPAN] end {} taken={}us", name, dt_us).as_str());
}
