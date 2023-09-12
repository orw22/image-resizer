mod consts;
mod input;
mod logs;

use crate::{
    consts::IMAGE_EXTENSIONS,
    input::{process_args, Args},
    logs::setup_logger,
};
use clap::Parser;
use image_resizer::{Image, Result};
use log::info;
use std::{fs, path::PathBuf};

/**
* image-resizer
*
*   - given a directory path, this program will resize all images in the directory
*     that are larger than 2MB to be ~2MB by scaling down their dimensions proportionally
*     to the diff between their size and the max file size (2MB).
*   - run .exe: ./image-resizer --path <path_to_directory>
*/

fn main() -> Result<()> {
    setup_logger();
    let args: Args = Args::parse();
    let dir_path: Option<PathBuf> = process_args(args.path);

    match dir_path {
        Some(path) => {
            for entry in fs::read_dir(path)? {
                let entry = entry?;
                if !entry.file_type()?.is_file()
                    || !IMAGE_EXTENSIONS.contains(
                        &entry
                            .path()
                            .extension()
                            .unwrap_or_default()
                            .to_str()
                            .unwrap_or_default()
                            .to_lowercase()
                            .as_str(),
                    )
                {
                    info!("Skipping {:?} (non-image file or directory)", entry.path());
                    continue;
                }
                let file_path = entry.path().to_string_lossy().into_owned();
                let image = Image::new(file_path)?;
                if image.needs_resize() {
                    image.resize()?;
                }
            }
        }
        None => {
            info!("Exiting...");
        }
    }

    return Ok(());
}
