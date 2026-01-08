//! Limine bootloader setup.

use xshell::{Shell, cmd};
use crate::common::Result;

/// Clone and build Limine bootloader if not present.
pub fn limine(sh: &Shell) -> Result<()> {
    if sh.path_exists("limine") {
        println!("Limine already present, skipping clone.");
        return Ok(());
    }

    println!("Cloning Limine v10.x...");
    cmd!(sh, "git clone https://github.com/limine-bootloader/limine.git --branch=v10.x-binary --depth=1").run()?;
    cmd!(sh, "make -C limine").run()?;

    Ok(())
}
