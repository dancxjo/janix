pub mod xhci;
pub mod actualizer;

pub fn init() {
    xhci::init_xhci();
    actualizer::init();
}

