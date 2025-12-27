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
    },
    /// Run the OS (hosted or qemu)
    Run {
        #[arg(long, default_value = "hosted")]
        env: String,
        /// Enable GDB stub (-s -S)
        #[arg(long)]
        gdb: bool,
        /// Run for fixed seconds then kill (for testing)
        #[arg(long)]
        timeout_secs: Option<u64>,
        /// Run in interactive mode (show QEMU window)
        #[arg(long)]
        interactive: bool,
    },
    /// Kill running QEMU instances
    Kill,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    match args.command {
        Commands::Fetch(args) => fetch::run(args),
        Commands::Build => build::run(),
        Commands::Iso { env } => iso::run(env),
        Commands::Run {
            env,
            gdb,
            timeout_secs,
            interactive,
        } => run::run(run::RunArgs {
            env,
            gdb,
            timeout_secs,
            interactive,
        }),
        Commands::Kill => kill::run(),
    }
}
