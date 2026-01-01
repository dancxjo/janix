use anyhow::Result;
use clap::Parser;
use std::process::Command;

mod build;
mod clean;
mod fetch;
mod inspect;
mod iso;
mod run;
mod test;

#[derive(Parser, Debug)]
#[command(name = "xtask", about = "Build and management tasks for ThingOS")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}


#[derive(clap::Subcommand, Debug)]
enum Commands {
    /// Fetch vendor assets (Limine, OVMF, Fonts)
    Fetch,
    /// Build the kernel and bridges
    Build {
        #[arg(long, default_value = "x86_64")]
        env: String,
    },
    /// Remove fetched assets and build artifacts
    Clean,
    /// Run BDD verification suite
    Test {
        #[arg(long, default_value = "all")]
        arch: String,
    },
    /// Inspect running kernel with GDB
    Inspect {
        #[arg(long, default_value = "x86_64")]
        env: String,
        #[arg(long)]
        port: Option<u16>,
    },
    /// Create bootable ISO
    Iso {
        #[arg(long, default_value = "x86_64")]
        env: String,
        #[arg(long)]
        cmdline: Option<String>,
        #[arg(long)]
        init_module: Option<String>,
    },
    /// Update documentation from BDD artifacts
    UpdateDocs,
    /// Run the OS (hosted or qemu)
    Run {
        #[arg(long, default_value = "x86_64")]
        env: String,
        /// Enable GDB stub (-s -S)
        #[arg(long)]
        gdb: bool,
        /// Custom GDB port
        #[arg(long)]
        gdb_port: Option<u16>,
        /// Run for fixed seconds then kill (for testing)
        #[arg(long)]
        timeout_secs: Option<u64>,
        /// Run in interactive mode (show QEMU window)
        #[arg(long)]
        interactive: bool,
        /// Start in frozen state (wait for GDB)
        #[arg(long)]
        frozen: bool,
        /// Kernel command line arguments
        #[arg(long)]
        cmdline: Option<String>,
        /// Enable debug mode (socket serial, frozen, clean panic)
        #[arg(long)]
        debug: bool,
    },
}

fn main() -> Result<()> {
    let args = Cli::parse();

    match args.command {
        Commands::Fetch => fetch::fetch(),
        Commands::Build { env } => build::run(&env),
        Commands::Clean => clean::run(),
        Commands::Test { arch } => test::run(Some(arch)),
        Commands::Inspect { env, port } => inspect::run(env, port),
        Commands::Iso { env, cmdline, init_module } => iso::run(env, cmdline, init_module),

        Commands::UpdateDocs => {
            Command::new("cargo")
                .args(["run", "-p", "docgen"])
                .status()?;
            Ok(())
        }
        Commands::Run {
            env,
            gdb,
            gdb_port,
            timeout_secs,
            interactive,
            frozen,
            cmdline,
            debug,
        } => {
            let res = run::run(run::RunArgs {
                env,
                gdb,
                gdb_port,
                timeout_secs,
                interactive,
                frozen,
                cmdline,
                debug,
            });
            fix_terminal();
            res
        }
    }
}

fn fix_terminal() {
    // Restore terminal cursor and cooked mode
    // let _ = Command::new("reset").status();
}
