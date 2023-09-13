use clap::Parser;
use dialoguer::Confirm;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Args {
    #[arg(short, long)]
    pub path: Option<PathBuf>,
}

pub fn process_args(path: Option<PathBuf>) -> Option<PathBuf> {
    match path {
        Some(pb) => return Some(pb),
        None => {
            if Confirm::new()
                .with_prompt("! This will resize all the images in this directory to under 2MB. Are you sure you wish to continue?")
                .wait_for_newline(true)
                .interact()
                .unwrap()
            {
                Some(PathBuf::from("."))
            } else {
                return None;
            }
        }
    }
}
