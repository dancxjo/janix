use super::DevTreeCtx;
use stem::kprintln;

pub fn enumerate(_ctx: &DevTreeCtx) -> Result<(), ()> {
    kprintln!("SPROUT: Enumerating RISC-V 64 platform (stub)...");
    Ok(())
}
