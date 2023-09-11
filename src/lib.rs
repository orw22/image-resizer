mod consts;

use crate::consts::Constants::MaxFileSize;
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
    needs_resize: bool,
}

impl Image {
    pub fn new(path: String) -> Result<Self> {
        let img = ImageReader::open(&path).unwrap().decode()?;
        let size = fs::metadata(&path)?.len();
        let needs_resize = size > MaxFileSize as u64;

        Ok(Self {
            path,
            img,
            size,
            needs_resize,
        })
    }

    pub fn resize(&self) -> Result<()> {
        let size_multiplier: f64 = MaxFileSize as u64 as f64 / self.size as f64;
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
        info!("Resized and saved {}!", self.path);

        Ok(())
    }

    pub fn needs_resize(&self) -> bool {
        self.needs_resize
    }
}
