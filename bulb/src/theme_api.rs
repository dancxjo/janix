//! Boot Theme Contract API
//!
//! Defines the interface between the boot loader and theme modules.
//! Themes receive parsed log events and boot phase, then decide how to render.

/// Parsed log event passed to theme
#[derive(Debug, Clone, Copy)]
pub struct LogEvent<'a> {
    pub timestamp: Option<u64>,
    pub level: LogLevel,
    pub source: Option<&'a str>,
    pub cpu: Option<u32>,
    pub message: &'a str,
}

/// Log severity level
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
    Unknown,
}

impl LogLevel {
    pub fn from_str(s: &str) -> Self {
        if s.contains("ERROR") {
            Self::Error
        } else if s.contains("WARN") {
            Self::Warn
        } else if s.contains("INFO") {
            Self::Info
        } else if s.contains("DEBUG") {
            Self::Debug
        } else {
            Self::Unknown
        }
    }
}

/// Boot phase progression
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum BootPhase {
    /// Initial spark - bootloader handoff
    Spark = 0,
    /// Memory subsystem initializing
    Memory = 1,
    /// CPUs coming online
    Cpu = 2,
    /// Services starting
    Services = 3,
    /// Fully awake
    Awake = 4,
}

impl BootPhase {
    pub fn name(self) -> &'static str {
        match self {
            Self::Spark => "Spark",
            Self::Memory => "Memory",
            Self::Cpu => "CPU",
            Self::Services => "Services",
            Self::Awake => "Awake",
        }
    }

    /// Detect phase from log content
    pub fn detect_from_log(line: &str, current: Self) -> Self {
        let new = if line.contains("Compositor") || line.contains("bloom") {
            Self::Awake
        } else if line.contains("svc.") || line.contains("Scheduler") || line.contains("sprout") {
            Self::Services
        } else if line.contains("CPU") || line.contains("SMP") || line.contains("heap") {
            Self::Cpu
        } else if line.contains("Memory") || line.contains("HHDM") || line.contains("paging") {
            Self::Memory
        } else {
            current
        };

        // Only progress forward
        if new > current {
            new
        } else {
            current
        }
    }
}

/// Display information for theme initialization
#[derive(Debug, Clone, Copy)]
pub struct DisplayInfo {
    pub width: u32,
    pub height: u32,
    pub stride: u32,
}

/// Boot theme contract - theme implements full display control
pub trait BootTheme {
    /// Initialize theme with display dimensions
    fn init(&mut self, info: DisplayInfo);

    /// Called when a log line is received
    fn on_log(&mut self, event: LogEvent<'_>);

    /// Called when boot phase changes
    fn on_phase_change(&mut self, phase: BootPhase);

    /// Render the current state to framebuffer
    /// Returns true if display was updated
    fn render(&mut self, fb: &mut [u8]) -> bool;
}
