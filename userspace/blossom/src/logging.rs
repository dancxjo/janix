// use stem::info;

#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {
        stem::info!("bloom: {}", format_args!($($arg)*))
    };
}

pub fn init() {
    log!("logging initialized");
}
