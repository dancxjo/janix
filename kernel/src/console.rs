pub trait ConsoleSink: Sync + Send {
    fn write_str(&self, s: &str);
}

struct NullSink;
impl ConsoleSink for NullSink {
    fn write_str(&self, _s: &str) {}
}

const MAX_SINKS: usize = 4;
static mut SINKS: [&'static dyn ConsoleSink; MAX_SINKS] = [&NullSink; MAX_SINKS];
static mut SINK_COUNT: usize = 0;

pub fn register_sink(sink: &'static dyn ConsoleSink) {
    unsafe {
        if SINK_COUNT < MAX_SINKS {
            SINKS[SINK_COUNT] = sink;
            SINK_COUNT += 1;
        }
    }
}

pub fn print(s: &str) {
    unsafe {
        for i in 0..SINK_COUNT {
            SINKS[i].write_str(s);
        }
    }
}

use core::fmt;

pub struct ConsoleWriter;

impl fmt::Write for ConsoleWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        print(s);
        Ok(())
    }
}

pub fn print_fmt(args: fmt::Arguments) {
    use core::fmt::Write;
    let mut writer = ConsoleWriter;
    let _ = writer.write_fmt(args);
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::console::print_fmt(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}
