mod consts;

use crate::consts::MAX_FILE_SIZE;
use image::io::Reader as ImageReader;
use image::{DynamicImage, GenericImageView};
use log::info;
use std::error::Error;
use std::fs;

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub struct Image {
    path: String,
    img: DynamicImage,
    size: u64,
}

impl Image {
    pub fn new(path: String) -> Result<Self> {
        let img = ImageReader::open(&path).unwrap().decode()?;
        let size = fs::metadata(&path)?.len();

        Ok(Self { path, img, size })
    }

    pub fn resize(&self) -> Result<()> {
        let size_multiplier = MAX_FILE_SIZE as u64 as f64 / self.size as f64;
        let dimensions = self.img.dimensions();
        let ndimensions = (
            (size_multiplier.sqrt() * (dimensions.0 as f64)) as u32,
            (size_multiplier.sqrt() * (dimensions.1 as f64)) as u32,
        );

        info!("Resizing {} ...", self.path);
        let buffer = image::imageops::resize(
            &self.img,
            ndimensions.0,
            ndimensions.1,
            image::imageops::FilterType::Lanczos3,
        );
        buffer.save(&self.path.replace("\\", "/"))?;
        info!("Resized and saved {}", self.path);

        Ok(())
    }

    pub fn needs_resize(&self) -> bool {
        self.size > MAX_FILE_SIZE as u64
    }
}
