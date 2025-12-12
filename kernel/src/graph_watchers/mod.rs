pub mod display;
pub mod io;

pub fn init() {
    display::init();
    io::init();
}
