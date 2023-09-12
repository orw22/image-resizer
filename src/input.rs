use clap::Parser;
use log::error;
use std::{
    io::{self, Write},
    path::PathBuf,
};

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Args {
    #[arg(short, long)]
    pub path: Option<PathBuf>,
}

pub fn process_args(path: Option<PathBuf>) -> Option<PathBuf> {
    match path {
        Some(_path) => return Some(_path),
        None => {
            print!("!!! This will resize all the images in this directory to under 2MB. Are you sure you wish to continue? (y/n) ");
            io::stdout().flush().unwrap();

            let mut res = String::new();
            io::stdin()
                .read_line(&mut res)
                .expect("Failed to read user input");

            match res.trim().to_lowercase().as_str() {
                "y" => {
                    return Some(PathBuf::from("."));
                }
                "n" => {
                    return None;
                }
                _ => {
                    error!("Invalid input");
                    return None;
                }
            }
        }
    }
}
