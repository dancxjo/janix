pub mod actualizer;
pub mod xhci;

pub fn init() {
    xhci::init_xhci();
    actualizer::init();
}
