use clap::Parser;
use std::fs;
use std::path::PathBuf;
use walkdir::WalkDir;
use tiny_skia::{Pixmap, Transform};
use resvg::usvg::{Options, Tree};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input directory containing SVGs
    #[arg(short, long)]
    input: PathBuf,

    /// Output directory for BMPs
    #[arg(short, long)]
    output: PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    fs::create_dir_all(&args.output)?;

    for entry in WalkDir::new(&args.input) {
        let entry = entry?;
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) == Some("svg") {
            let file_name = path.file_stem().unwrap().to_str().unwrap();
            println!("Processing: {}", file_name);

            let svg_data = fs::read(path)?;
            let opt = Options::default();
            let mut tree = Tree::from_data(&svg_data, &opt)?;

            // Force 32x32 size for icons, or respect original size?
            // Tango icons are often 32x32 or 48x48. Let's aim for 32x32 for now as default OS icons.
            // Or maybe keep original size.
            // Let's force 32x32 for uniformity in Limine/OS for now.
            // Actually, let's make it configurable or just default to 32x32.
            let width = 32;
            let height = 32;

            let mut pixmap = Pixmap::new(width, height).unwrap();
            
            let size = tree.size().to_int_size();
            let sx = width as f32 / size.width() as f32;
            let sy = height as f32 / size.height() as f32;
            let scale = sx.min(sy);

            let transform = Transform::from_scale(scale, scale);
            resvg::render(&tree, transform, &mut pixmap.as_mut());

            // Convert to DynamicImage to save as BMP
            let img = image::ImageBuffer::<image::Rgba<u8>, _>::from_raw(width, height, pixmap.data().to_vec()).unwrap();
            let dynamic_image = image::DynamicImage::ImageRgba8(img);
            
            let output_path = args.output.join(format!("{}.bmp", file_name));
            dynamic_image.save_with_format(output_path, image::ImageFormat::Bmp)?;
        }
    }

    Ok(())
}
