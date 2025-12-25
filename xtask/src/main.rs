use clap::Parser;
use anyhow::Result;

mod fetch;
mod iso;
mod run;

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
    },
}

fn main() -> Result<()> {
    let args = Cli::parse();

    match args.command {
        Commands::Fetch(args) => fetch::run(args),
        Commands::Build => {
            println!("xtask build: TODO");
            Ok(())
        },
        Commands::Iso { env } => iso::run(env), 
        Commands::Run { env, gdb } => run::run(run::RunArgs { env, gdb }),
    }
}
