use anyhow::Result;
use clap::Parser;

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
    },
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
        /// Kernel command line arguments
        #[arg(long)]
        cmdline: Option<String>,
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
    }
}
