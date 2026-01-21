use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicU64, Ordering};
use spin::Mutex;

pub struct PerfState {
    pub current_frame: PerfFrame,
    pub history: Vec<PerfFrame>,
}

#[derive(Clone)]
pub struct PerfReport {
    pub spans: BTreeMap<&'static str, u64>, // Max in ns
    pub avg_spans: BTreeMap<&'static str, f64>, // Avg in ms
    pub avg_counters: BTreeMap<&'static str, f64>,
}

static LAST_REPORT: Mutex<Option<PerfReport>> = Mutex::new(None);
static FRAME_COUNTER: AtomicU64 = AtomicU64::new(0);

/// How often to print a compact perf line (every N frames)
pub const PERF_PRINT_CADENCE: u64 = 60;

pub struct PerfFrame {
    pub spans: BTreeMap<&'static str, u64>,
    pub counters: BTreeMap<&'static str, u64>,
    pub events: BTreeMap<&'static str, &'static str>,
}

impl PerfFrame {
    pub fn new() -> Self {
        Self {
            spans: BTreeMap::new(),
            counters: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }
}

static PERF: Mutex<Option<PerfState>> = Mutex::new(None);

pub fn init() {
    *PERF.lock() = Some(PerfState {
        current_frame: PerfFrame::new(),
        history: Vec::with_capacity(128),
    });
    stem::perf::register_hooks(span_enter, span_exit, add_counter, log_event);
}

pub fn span_enter(_name: &'static str) {}

pub fn span_exit(name: &'static str, duration: u64) {
    if let Some(state) = PERF.lock().as_mut() {
        *state.current_frame.spans.entry(name).or_insert(0) += duration;
    }
}

pub fn log_event(name: &'static str, data: &'static str) {
    if let Some(state) = PERF.lock().as_mut() {
        state.current_frame.events.insert(name, data);
    }
}

pub fn add_counter(name: &'static str, value: u64) {
    if let Some(state) = PERF.lock().as_mut() {
        *state.current_frame.counters.entry(name).or_insert(0) += value;
    }
}

pub fn end_frame() -> Option<PerfFrame> {
    let frame_no = FRAME_COUNTER.fetch_add(1, Ordering::Relaxed);
    
    let mut lock = PERF.lock();
    let state = lock.as_mut()?;
    let finished_frame = core::mem::replace(&mut state.current_frame, PerfFrame::new());
    
    // Print compact one-liner every N frames
    if frame_no % PERF_PRINT_CADENCE == 0 && frame_no > 0 {
        print_compact_stats(&finished_frame, frame_no);
    }
    
    state.history.push(finished_frame);
    
    if state.history.len() >= 120 {
        let mut agg_spans: BTreeMap<&'static str, Vec<u64>> = BTreeMap::new();
        let mut agg_counters: BTreeMap<&'static str, Vec<u64>> = BTreeMap::new();
        
        let frames = core::mem::replace(&mut state.history, Vec::with_capacity(128));
        let count = frames.len() as f64;
        
        for f in &frames {
            for (name, &dur) in &f.spans {
                agg_spans.entry(name).or_default().push(dur);
            }
            for (name, &val) in &f.counters {
                agg_counters.entry(name).or_default().push(val);
            }
        }
        
        let mut report = PerfReport {
            spans: BTreeMap::new(),
            avg_spans: BTreeMap::new(),
            avg_counters: BTreeMap::new(),
        };

        crate::log!("--- PERF REPORT ({} frames) ---", frames.len());
        for (name, values) in agg_spans {
            let sum: u64 = values.iter().sum();
            let avg = sum as f64 / count / 1_000_000.0;
            let mut sorted = values.clone();
            sorted.sort();
            let p50 = sorted[values.len() / 2] as f64 / 1_000_000.0;
            let p95 = sorted[(values.len() * 95) / 100] as f64 / 1_000_000.0;
            let max = sorted.last().unwrap_or(&0) / 1_000_000;
            
            report.avg_spans.insert(name, avg);
            report.spans.insert(name, (max * 1_000_000) as u64);

            crate::log!("  {:<25} avg={:>6.2}ms p50={:>6.2}ms p95={:>6.2}ms max={:>4}ms", 
                name, avg, p50, p95, max);
        }
        for (name, values) in agg_counters {
            let sum: u64 = values.iter().sum();
            let avg = sum as f64 / count;
            report.avg_counters.insert(name, avg);
            crate::log!("  {:<25} avg={:>6.1}", name, avg);
        }
        
        *LAST_REPORT.lock() = Some(report);
    }
    
    None
}

/// Print a compact one-liner for quick monitoring
fn print_compact_stats(frame: &PerfFrame, frame_no: u64) {
    // Extract key metrics
    let snap_ms = frame.spans.get("ui.snap").map(|&ns| ns as f64 / 1_000_000.0).unwrap_or(0.0);
    let prop_ms = frame.spans.get("ui.snap.prop_get").map(|&ns| ns as f64 / 1_000_000.0).unwrap_or(0.0);
    let layout_ms = frame.spans.get("ui.layout").map(|&ns| ns as f64 / 1_000_000.0).unwrap_or(0.0);
    let present_ms = frame.spans.get("present").map(|&ns| ns as f64 / 1_000_000.0).unwrap_or(0.0);
    let raster_ms = frame.spans.get("raster").map(|&ns| ns as f64 / 1_000_000.0).unwrap_or(0.0);
    
    // Syscall counts
    let syscalls_prop = frame.counters.get("snap.syscalls.prop_get").copied().unwrap_or(0);
    let syscalls_kind = frame.counters.get("snap.syscalls.get_kind").copied().unwrap_or(0);
    let syscalls_edges = frame.counters.get("snap.syscalls.get_edges").copied().unwrap_or(0);
    let syscalls_str = frame.counters.get("snap.syscalls.read_string").copied().unwrap_or(0);
    let total_syscalls = syscalls_prop + syscalls_kind + syscalls_edges + syscalls_str;
    
    let nodes = frame.counters.get("ui.snap.nodes_total").copied().unwrap_or(0);
    
    crate::log!("[PERF] f={} snap={:.1}ms prop={:.1}ms layout={:.1}ms nodes={} syscalls={} (prop={} kind={} edges={} str={})",
        frame_no, snap_ms, prop_ms, layout_ms, nodes, total_syscalls,
        syscalls_prop, syscalls_kind, syscalls_edges, syscalls_str);
}

pub fn get_last_report() -> Option<PerfReport> {
    LAST_REPORT.lock().clone()
}

#[macro_export]
macro_rules! trace_span {
    ($name:expr) => {
        let _span = $crate::perf::PerfSpan::new($name);
    };
}

#[macro_export]
macro_rules! trace_counter {
    ($name:expr, $val:expr) => {
        $crate::perf::add_counter($name, $val as u64);
    };
}

#[macro_export]
macro_rules! trace_event {
    ($name:expr, $data:expr) => {
        $crate::perf::log_event($name, $data);
    };
}

// Re-export for macro use
pub use stem::perf::PerfSpan;
pub use stem::perf::counter;
