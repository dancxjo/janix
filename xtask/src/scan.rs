use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use image::GenericImageView;
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Parser)]
pub struct ScanArgs {
    #[command(subcommand)]
    pub cmd: ScanCommand,
}

#[derive(Subcommand)]
pub enum ScanCommand {
    /// List top colors in the image or region
    TopColors {
        image: PathBuf,
        #[arg(long, default_value_t = 20)]
        count: usize,
        #[arg(long)]
        x: Option<u32>,
        #[arg(long)]
        y: Option<u32>,
        #[arg(long)]
        w: Option<u32>,
        #[arg(long)]
        h: Option<u32>,
    },
    /// Count pixels matching specific criteria (e.g. red)
    CountRed {
        image: PathBuf,
        #[arg(long)]
        exclude_top_right: bool,
    },
    /// Scan a specific hardcoded region (legacy support for scan_region.py)
    LegacyRegion { image: PathBuf },
}

pub fn run(args: ScanArgs) -> Result<()> {
    match args.cmd {
        ScanCommand::TopColors {
            image,
            count,
            x,
            y,
            w,
            h,
        } => top_colors(&image, count, x, y, w, h),
        ScanCommand::CountRed {
            image,
            exclude_top_right,
        } => count_red(&image, exclude_top_right),
        ScanCommand::LegacyRegion { image } => {
            // Replicates scan_region.py: x=800, y=600, w=480, h=120
            top_colors(&image, 100, Some(800), Some(600), Some(480), Some(120))
        }
    }
}

fn top_colors(
    path: &PathBuf,
    limit: usize,
    x: Option<u32>,
    y: Option<u32>,
    w: Option<u32>,
    h: Option<u32>,
) -> Result<()> {
    let img = image::open(path).context("Failed to open image")?;
    let (width, height) = img.dimensions();

    let x = x.unwrap_or(0);
    let y = y.unwrap_or(0);
    let w = w.unwrap_or(width - x);
    let h = h.unwrap_or(height - y);

    println!("Scanning region: x={}, y={}, w={}, h={}", x, y, w, h);

    let mut colors = HashMap::new();

    for iy in y..std::cmp::min(y + h, height) {
        for ix in x..std::cmp::min(x + w, width) {
            let pixel = img.get_pixel(ix, iy);
            *colors.entry(pixel).or_insert(0) += 1;
        }
    }

    let mut sorted_colors: Vec<_> = colors.into_iter().collect();
    sorted_colors.sort_by(|a, b| b.1.cmp(&a.1));

    println!("Top {} colors:", limit);
    for (pixel, count) in sorted_colors.iter().take(limit) {
        println!("  {:?}: {}", pixel, count);
    }
    Ok(())
}

fn count_red(path: &PathBuf, exclude_top_right: bool) -> Result<()> {
    let img = image::open(path).context("Failed to open image")?;
    let (width, height) = img.dimensions();

    let mut reds = 0;

    for y in 0..height {
        for x in 0..width {
            if exclude_top_right && x > width.saturating_sub(50) && y < 50 {
                continue;
            }

            let pixel = img.get_pixel(x, y);
            let r = pixel[0];
            let g = pixel[1];
            let b = pixel[2];

            // Heuristic for red used in python script
            if r > 100 && g < 100 && b < 100 {
                reds += 1;
            }
        }
    }

    println!("{}: {} red pixels", path.display(), reds);
    Ok(())
}
