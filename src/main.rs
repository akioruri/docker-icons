use std::path::{Path, PathBuf};

use anyhow::Result;
use clap::Parser;
use image::{open, RgbaImage};

/// Apply the alpha channel of one image onto another.
#[derive(Parser)]
#[command(name = "docker-icons", version, about)]
struct Cli {
    /// Mask image (source of the alpha channel).
    mask: PathBuf,

    /// Fill image (source of the RGB channels).
    fill: PathBuf,

    /// Output PNG path. Defaults to "<fill stem>_with_alpha.png".
    #[arg(short, long)]
    output: Option<PathBuf>,
}

const OUTPUT_SUFFIX: &str = "_with_alpha";

fn apply_alpha(mask: &RgbaImage, fill: &mut RgbaImage) {
    debug_assert_eq!(mask.dimensions(), fill.dimensions());
    for (m, f) in mask.pixels().zip(fill.pixels_mut()) {
        f[3] = m[3];
    }
}

fn default_output(fill: &Path) -> PathBuf {
    let stem = fill.file_stem().and_then(|s| s.to_str()).unwrap_or("out");
    fill.with_file_name(format!("{stem}{OUTPUT_SUFFIX}.png"))
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let mask = open(&cli.mask)?.to_rgba8();
    let mut fill = open(&cli.fill)?.to_rgba8();

    apply_alpha(&mask, &mut fill);

    let out = cli.output.unwrap_or_else(|| default_output(&cli.fill));
    fill.save(&out)?;

    println!("Saved: {}", out.display());
    Ok(())
}
