pub mod display;
#[cfg(not(test))]
pub mod io;

pub fn init() {
    display::init();
    #[cfg(not(test))]
    io::init();
}
