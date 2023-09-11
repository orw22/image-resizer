mod consts;
mod input;
mod logs;

use crate::{input::Args, logs::setup_logger};
use clap::Parser;
use image_resizer::{Image, Result};
use std::fs;

/**
* image-resizer
*
*   - given a directory path, this program will resize all images in the directory
*     that are larger than 2MB to be ~2MB by scaling down their dimensions proportionally
*     to the diff between their size and the max file size (2MB).
*/

fn main() -> Result<()> {
    setup_logger();
    let args: Args = Args::parse();
    for entry in fs::read_dir(args.path)? {
        let file_path = entry?.path().to_string_lossy().into_owned();
        let image = Image::new(file_path)?;
        if image.needs_resize() {
            image.resize()?;
        }
    }
    return Ok(());
}
