use abi::display::DisplayInfo;
use spin::Mutex;

static PRIMARY_DISPLAY: Mutex<Option<DisplayInfo>> = Mutex::new(None);

pub fn set_primary(info: DisplayInfo) {
    *PRIMARY_DISPLAY.lock() = Some(info);
}

pub fn get_primary() -> Option<DisplayInfo> {
    *PRIMARY_DISPLAY.lock()
}
