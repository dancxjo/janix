//! xtask - Build automation for Thing-OS
//!
//! This crate provides Rust-based build automation, replacing shell scripts
//! in the justfile with proper type-safe implementations.

mod bdd;
mod build;
mod clean;
mod common;
mod image;
mod limine;
mod ovmf;
mod run;

use clap::{Parser, Subcommand};
use xshell::Shell;

use crate::bdd::bdd;
use crate::build::build;
use crate::clean::{clean, distclean};
use crate::common::project_root;
use crate::image::{build_hdd, build_iso};
use crate::limine::limine;
use crate::ovmf::ovmf;
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
    },
    /// Create an HDD image
    Hdd {
        /// Target architecture
        #[arg(long, default_value = "x86_64")]
        env: String,
        /// Rust profile
        #[arg(long, default_value = "dev")]
        profile: String,
    },
    /// Run in QEMU (UEFI mode)
    Run {
        /// Target architecture
        #[arg(long, default_value = "x86_64")]
        env: String,
        /// Rust profile
        #[arg(long, default_value = "dev")]
        profile: String,
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
        /// Additional QEMU flags
        #[arg(long, default_value = "-m 2G", allow_hyphen_values = true)]
        qemu_flags: String,
    },
    /// Clone and build Limine bootloader
    Limine,
    /// Download OVMF firmware
    Ovmf {
        /// Target architecture
        #[arg(long, default_value = "x86_64")]
        env: String,
    },
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
        /// Target architecture(s) - can specify multiple (default: x86_64)
        #[arg(long, short = 'a', num_args = 1.., default_value = "x86_64")]
        arch: Vec<String>,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let sh = Shell::new()?;

    // Change to project root (parent of xtask directory)
    let project_root = project_root();
    sh.change_dir(&project_root);

    match cli.command {
        Commands::Build { env, profile } => build(&sh, &env, &profile)?,
        Commands::Iso { env, profile } => {
            limine(&sh)?;
            build(&sh, &env, &profile)?;
            build_iso(&sh, &env)?;
        }
        Commands::Hdd { env, profile } => {
            limine(&sh)?;
            build(&sh, &env, &profile)?;
            build_hdd(&sh, &env)?;
        }
        Commands::Run { env, profile, qemu_flags } => {
            ovmf(&sh, &env)?;
            limine(&sh)?;
            build(&sh, &env, &profile)?;
            build_iso(&sh, &env)?;
            run(&sh, &env, &qemu_flags)?;
        }
        Commands::RunBios { qemu_flags } => {
            limine(&sh)?;
            build(&sh, "x86_64", "dev")?;
            build_iso(&sh, "x86_64")?;
            run_bios(&sh, &qemu_flags)?;
        }
        Commands::RunHdd { env, profile, qemu_flags } => {
            ovmf(&sh, &env)?;
            limine(&sh)?;
            build(&sh, &env, &profile)?;
            build_hdd(&sh, &env)?;
            run_hdd(&sh, &env, &qemu_flags)?;
        }
        Commands::Limine => limine(&sh)?,
        Commands::Ovmf { env } => ovmf(&sh, &env)?,
        Commands::Clean => clean(&sh)?,
        Commands::Distclean => distclean(&sh)?,
        Commands::Bdd { feature, tags, arch } => bdd(&sh, feature, tags, arch)?,
    }

    Ok(())
}
