use super::DevTreeCtx;
use stem::info;

pub fn enumerate(_ctx: &DevTreeCtx) -> Result<(), ()> {
    info!("SPROUT: Enumerating RISC-V 64 platform (stub)...");
    Ok(())
}
