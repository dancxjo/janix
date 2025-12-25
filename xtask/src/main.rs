use anyhow::{bail, Result};

fn main() -> Result<()> {
    let mut args = std::env::args().skip(1);
    let cmd = args.next().unwrap_or_else(|| "help".to_string());

    match cmd.as_str() {
        "fetch" => {
            // TODO: implement pinned downloads + limine clone
            println!("xtask fetch: TODO");
            Ok(())
        }
        "build" => {
            // TODO: build kernels
            println!("xtask build: TODO");
            Ok(())
        }
        "iso" => {
            // TODO: create ISO for env
            println!("xtask iso: TODO");
            Ok(())
        }
        "run" => {
            // TODO: run hosted or qemu boot
            println!("xtask run: TODO");
            Ok(())
        }
        "help" | "-h" | "--help" => {
            eprintln!("xtask commands: fetch | build | iso | run");
            Ok(())
        }
        other => bail!("unknown xtask command: {other}"),
    }
}
