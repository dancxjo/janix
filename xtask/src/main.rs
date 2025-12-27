use anyhow::Result;
use clap::Parser;

mod fetch;
mod iso;
mod run;
mod build;
mod kill; // Add module

#[derive(Parser, Debug)]
#[command(name = "xtask", about = "Build and management tasks for ThingOS")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(clap::Subcommand, Debug)]
enum Commands {
    /// Fetch vendor assets (Limine, OVMF, Fonts)
    Fetch(fetch::FetchArgs),
    /// Build the kernel and bridges
    Build,
    /// Create bootable ISO
    Iso {
        #[arg(long, default_value = "x86_64")]
        env: String,
        #[arg(long)]
        cmdline: Option<String>,
    },
    /// Run the OS (hosted or qemu)
    Run {
        #[arg(long, default_value = "hosted")]
        env: String,
        /// Enable GDB stub (-s -S)
        #[arg(long)]
        gdb: bool,
        /// Custom GDB port (default: 1234)
        #[arg(long)]
        gdb_port: Option<u16>,
        /// Run for fixed seconds then kill (for testing)
        #[arg(long)]
        timeout_secs: Option<u64>,
        /// Run in interactive mode (show QEMU window)
        #[arg(long)]
        interactive: bool,
        /// Kernel command line arguments
        #[arg(long)]
        cmdline: Option<String>,
    },
    /// Kill running QEMU instances
    Kill,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    match args.command {
        Commands::Fetch(args) => fetch::run(args),
        Commands::Build => build::run(),
        Commands::Iso { env, cmdline } => iso::run(env, cmdline),
        Commands::Run {
            env,
            gdb,
            gdb_port,
            timeout_secs,
            interactive,
            cmdline,
        } => run::run(run::RunArgs {
            env,
            gdb,
            gdb_port,
            timeout_secs,
            interactive,
            cmdline,
        }),
        Commands::Kill => kill::run(),
    }
}
