//! Render profiling for Bloom compositor.
//!
//! Provides `RenderWindowProfile` to collect timing and counters during window rendering,
//! enabling identification of slow render paths.

use abi::ids::ThingId;
use alloc::format;
use thing_std::time::monotonic_now;
use thing_std::log_info;

/// Runtime toggle for profiling (always enabled for now)
pub const PROF_RENDER_WINDOW: bool = true;

/// Threshold in milliseconds - log profiles above this
const LOG_THRESHOLD_MS: u64 = 50;
/// Threshold for "top contributors" breakdown
const TOP_BREAKDOWN_MS: u64 = 1000;

/// Timing bucket identifiers for span_end
#[derive(Clone, Copy)]
pub enum TimingBucket {
    Chrome,
    Text,
    Drawlist,
    Widgets,
    BytespaceMap,
}

/// Profile data collected during a single `render_window` call.
#[derive(Default)]
pub struct RenderWindowProfile {
    // Metadata
    window_id_low: u64,
    window_w: u32,
    window_h: u32,
    
    // Timing (nanoseconds)
    start_ns: u64,
    span_start_ns: u64,
    
    /// Total render time
    pub t_total: u64,
    /// Chrome (title bar, close button, borders)
    pub t_chrome: u64,
    /// Text rendering (title + widget labels)
    pub t_text: u64,
    /// DrawList iteration in paint_drawlist
    pub t_drawlist: u64,
    /// Widget painting (labels, buttons, canvas)
    pub t_widgets: u64,
    /// Bytespace mapping time (inside drawlist)
    pub t_bytespace_map: u64,
    
    // Counters
    /// Total draw commands recorded
    pub draw_cmds: u32,
    /// fill_rect calls
    pub rect_fills: u32,
    /// draw_text calls
    pub text_calls: u32,
    /// Characters drawn across all text
    pub chars_drawn: u32,
    /// DrawList widget count
    pub drawlist_count: u32,
    /// Commands decoded from drawlists
    pub drawlist_cmds: u32,
    /// Bytespace maps performed
    pub bytespace_maps: u32,
    /// Total bytes mapped from bytespaces
    pub bytespace_bytes: u64,
}

impl RenderWindowProfile {
    /// Create a new profile and start the timer.
    #[inline]
    pub fn new(window_id: ThingId, w: u32, h: u32) -> Self {
        Self {
            window_id_low: window_id.low(),
            window_w: w,
            window_h: h,
            start_ns: monotonic_now(),
            span_start_ns: 0,
            ..Default::default()
        }
    }
    
    /// Start a sub-span timer.
    #[inline]
    pub fn span_start(&mut self) {
        self.span_start_ns = monotonic_now();
    }
    
    /// End a sub-span and accumulate into the specified bucket.
    #[inline]
    pub fn span_end(&mut self, bucket: TimingBucket) {
        let end = monotonic_now();
        let elapsed = end.saturating_sub(self.span_start_ns);
        match bucket {
            TimingBucket::Chrome => self.t_chrome += elapsed,
            TimingBucket::Text => self.t_text += elapsed,
            TimingBucket::Drawlist => self.t_drawlist += elapsed,
            TimingBucket::Widgets => self.t_widgets += elapsed,
            TimingBucket::BytespaceMap => self.t_bytespace_map += elapsed,
        }
    }
    
    /// Finish profiling and compute total time.
    #[inline]
    pub fn finish(&mut self) {
        self.t_total = monotonic_now().saturating_sub(self.start_ns);
    }
    
    /// Log profile if total time exceeds threshold.
    pub fn log_if_slow(&self, threshold_ms: u64) {
        if !PROF_RENDER_WINDOW {
            return;
        }
        
        let total_ms = self.t_total / 1_000_000;
        if total_ms < threshold_ms {
            return;
        }
        
        // Format: BLOOM PROF win=123 456ms cmds=42 fills=8 text=3(45ch) dl=1(12cmds) map=2(64KB) t:chrome=50 text=800 dl=300 widgets=84
        let map_kb = self.bytespace_bytes / 1024;
        
        log_info(&format!(
            "BLOOM PROF win={} {}ms {}x{} cmds={} fills={} text={}({}ch) dl={}({}cmds) map={}({}KB) t:chrome={} text={} dl={} widgets={}",
            self.window_id_low,
            total_ms,
            self.window_w,
            self.window_h,
            self.draw_cmds,
            self.rect_fills,
            self.text_calls,
            self.chars_drawn,
            self.drawlist_count,
            self.drawlist_cmds,
            self.bytespace_maps,
            map_kb,
            self.t_chrome / 1_000_000,
            self.t_text / 1_000_000,
            self.t_drawlist / 1_000_000,
            self.t_widgets / 1_000_000,
        ));
        
        // For extreme stalls, show top contributors
        if total_ms >= TOP_BREAKDOWN_MS {
            self.log_top_contributors();
        }
    }
    
    fn log_top_contributors(&self) {
        // Collect timing buckets with names
        let mut buckets = [
            ("chrome", self.t_chrome / 1_000_000),
            ("text", self.t_text / 1_000_000),
            ("drawlist", self.t_drawlist / 1_000_000),
            ("widgets", self.t_widgets / 1_000_000),
            ("bytemap", self.t_bytespace_map / 1_000_000),
        ];
        
        // Sort descending by time
        buckets.sort_by(|a, b| b.1.cmp(&a.1));
        
        let total_ms = self.t_total / 1_000_000;
        
        log_info(&format!(
            "BLOOM PROF TOP win={} {}ms top=[{}={} {}={} {}={}]",
            self.window_id_low,
            total_ms,
            buckets[0].0, buckets[0].1,
            buckets[1].0, buckets[1].1,
            buckets[2].0, buckets[2].1,
        ));
    }
}

/// Convert nanoseconds to milliseconds.
#[inline]
pub fn ns_to_ms(ns: u64) -> u64 {
    ns / 1_000_000
}
