use anyhow::Result;
use image::{Rgba, RgbaImage};

pub fn blank_image(width: u32, height: u32) -> Result<RgbaImage> {
    let mut img = RgbaImage::new(width, height);
    for pixel in img.pixels_mut() {
        *pixel = Rgba([255, 255, 255, 255]);
    }
    Ok(img)
}
