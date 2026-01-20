//! xtask - Build automation for Thing-OS
//!
//! This crate provides Rust-based build automation, replacing shell scripts
//! in the justfile with proper type-safe implementations.

mod bdd;
mod build;
mod clean;
mod common;
mod fetch;
mod image;
mod kill;
mod limine;
mod run;

use clap::{Parser, Subcommand};
use xshell::Shell;

use crate::bdd::bdd;
use crate::build::build;
use crate::clean::{clean, distclean};
use crate::common::project_root;
use crate::fetch::fetch;
use crate::image::{build_hdd, build_iso, build_iso_with_config, default_programs, IsoConfig, ProgramConfig};
use crate::limine::limine;
use crate::run::{run, run_bios, run_hdd};

/// Thing-OS build automation tool
#[derive(Parser)]
#[command(name = "xtask")]
#[command(about = "Build automation for Thing-OS", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Build the kernel for a target architecture
    Build {
        /// Target architecture (x86_64, aarch64, riscv64, loongarch64)
        #[arg(long, default_value = "x86_64")]
        env: String,
        /// Rust profile (dev, release)
        #[arg(long, default_value = "dev")]
        profile: String,
    },
    /// Create an ISO image
    Iso {
        /// Target architecture
        #[arg(long, default_value = "x86_64")]
        env: String,
        /// Rust profile
        #[arg(long, default_value = "dev")]
        profile: String,
        /// Initial program to launch
        #[arg(long)]
        init: Option<String>,
        /// Display resolution (e.g., 1280x720, 1920x1080)
        #[arg(long)]
        resolution: Option<String>,
        /// Output ISO file path
        #[arg(long)]
        output: Option<String>,
    },
    /// Create an HDD image
    Hdd {
        /// Target architecture
        #[arg(long, default_value = "x86_64")]
        env: String,
        /// Rust profile
        #[arg(long, default_value = "dev")]
        profile: String,
        /// Initial program to launch
        #[arg(long)]
        init: Option<String>,
    },
    /// Run in QEMU (UEFI mode)
    Run {
        /// Target architecture
        #[arg(long, default_value = "x86_64")]
        env: String,
        /// Rust profile
        #[arg(long, default_value = "dev")]
        profile: String,
        /// Initial program to launch
        #[arg(long)]
        init: Option<String>,
        /// Additional QEMU flags
        #[arg(long, default_value = "-m 2G", allow_hyphen_values = true)]
        qemu_flags: String,
    },
    /// Run in QEMU (BIOS mode, x86_64 only)
    RunBios {
        /// Additional QEMU flags
        #[arg(long, default_value = "-m 2G", allow_hyphen_values = true)]
        qemu_flags: String,
    },
    /// Run HDD image in QEMU (UEFI mode)
    RunHdd {
        /// Target architecture
        #[arg(long, default_value = "x86_64")]
        env: String,
        /// Rust profile
        #[arg(long, default_value = "dev")]
        profile: String,
        /// Initial program to launch
        #[arg(long)]
        init: Option<String>,
        /// Additional QEMU flags
        #[arg(long, default_value = "-m 2G", allow_hyphen_values = true)]
        qemu_flags: String,
    },
    /// Clone and build Limine bootloader
    Limine,
    /// Download OVMF firmware for specific architecture
    Ovmf {
        /// Target architecture
        #[arg(long, default_value = "x86_64")]
        env: String,
    },
    /// Download OVMF firmware for all architectures
    OvmfAll,
    /// Clean build artifacts
    Clean,
    /// Clean everything including downloaded dependencies
    Distclean,
    /// Run BDD tests
    Bdd {
        /// Run a specific feature file (without .feature extension)
        #[arg(long)]
        feature: Option<String>,
        /// Cucumber tag expression (e.g., @smoke)
        #[arg(long, short = 't')]
        tags: Option<String>,
        /// Target architecture(s) - can specify multiple (default: all architectures)
        #[arg(long, short = 'a', num_args = 1.., default_values_t = ["x86_64".to_string(), "aarch64".to_string(), "riscv64".to_string(), "loongarch64".to_string()])]
        arch: Vec<String>,
    },
    /// Kill running QEMU instances
    Kill,
    /// Fetch vendor assets (Limine, OVMF, Fonts, Icons, Cursors)
    Fetch,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let sh = Shell::new()?;

    // Change to project root (parent of xtask directory)
    let project_root = project_root();
    sh.change_dir(&project_root);

    match cli.command {
        Commands::Build { env, profile } => build(&sh, &env, &profile)?,
        Commands::Iso { env, profile, init, resolution, output } => {
            limine(&sh)?;
            build(&sh, &env, &profile)?;
            let mut programs = default_programs();
            apply_init(&mut programs, init);
            
            // Use config if resolution or output specified
            let path = if resolution.is_some() || output.is_some() {
                let output_path = output.as_ref().map(|s| std::path::Path::new(s.as_str()));
                let config = IsoConfig {
                    resolution: resolution.as_deref(),
                    iso_path: output_path,
                };
                build_iso_with_config(&sh, &env, &programs, &config)?
            } else {
                build_iso(&sh, &env, &programs)?
            };
            println!("ISO generated at: {}", path.display());
        }
        Commands::Hdd { env, profile, init } => {
            limine(&sh)?;
            build(&sh, &env, &profile)?;
            let mut programs = default_programs();
            apply_init(&mut programs, init);
            let path = build_hdd(&sh, &env, &programs)?;
            println!("HDD generated at: {}", path.display());
        }
        Commands::Run {
            env,
            profile,
            init,
            qemu_flags,
        } => {
            fetch()?;
            limine(&sh)?;
            build(&sh, &env, &profile)?;
            let mut programs = default_programs();
            apply_init(&mut programs, init);
            let iso_path = build_iso(&sh, &env, &programs)?;
            run(&sh, &env, &qemu_flags, &iso_path)?;
        }
        Commands::RunBios { qemu_flags } => {
            limine(&sh)?;
            build(&sh, "x86_64", "dev")?;
            let programs = default_programs();
            let iso = build_iso(&sh, "x86_64", &programs)?;
            run_bios(&sh, &qemu_flags, &iso)?;
        }
        Commands::RunHdd {
            env,
            profile,
            init,
            qemu_flags,
        } => {
            fetch()?;
            limine(&sh)?;
            build(&sh, &env, &profile)?;
            let mut programs = default_programs();
            apply_init(&mut programs, init);
            let hdd_path = build_hdd(&sh, &env, &programs)?;
            run_hdd(&sh, &env, &qemu_flags, &hdd_path)?;
        }
        Commands::Limine => limine(&sh)?,
        Commands::Ovmf { env: _ } => fetch()?,  // OVMF handled by unified fetch
        Commands::OvmfAll => fetch()?,  // OVMF handled by unified fetch
        Commands::Clean => clean(&sh)?,
        Commands::Distclean => distclean(&sh)?,
        Commands::Bdd {
            feature,
            tags,
            arch,
        } => bdd(&sh, feature, tags, arch)?,
        Commands::Kill => kill::run()?,
        Commands::Fetch => fetch()?,
    }

    Ok(())
}

fn apply_init(programs: &mut [ProgramConfig], init: Option<String>) {
    if let Some(init_name) = init {
        for prog in programs {
            if prog.name == init_name {
                prog.is_init = true;
            }
        }
    }
}
