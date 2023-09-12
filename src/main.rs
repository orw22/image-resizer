mod consts;
mod input;
mod logs;

use crate::{consts::IMAGE_EXTENSIONS, input::Args, logs::setup_logger};
use clap::Parser;
use image_resizer::{Image, Result};
use log::{error, info};
use std::{
    fs,
    io::{self, Write},
    path::PathBuf,
};

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
    let dir_path: PathBuf;

    match args.path {
        Some(path) => dir_path = path,
        None => {
            dir_path = PathBuf::from(".");
            print!("!!! This will resize all the images in this directory to under 2MB. Are you sure you wish to continue? (y/n) ");
            io::stdout().flush().unwrap();

            let mut res = String::new();
            io::stdin()
                .read_line(&mut res)
                .expect("Failed to read user input");

            match res.trim().to_lowercase().as_str() {
                "y" => {}
                "n" => {
                    info!("Exiting ...");
                    return Ok(());
                }
                _ => {
                    error!("Invalid input. Please enter 'y' for yes or 'n' for no.");
                }
            }
        }
    }

    for entry in fs::read_dir(dir_path)? {
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
    return Ok(());
}
