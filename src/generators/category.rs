use crate::{
    constants::variables::CATEGORY_IMAGE_SIZE,
    file::{base_64::string_to_base64, read::read_base64_file},
};
use anyhow::Result;
use image::RgbaImage;

pub fn category_image() -> Result<RgbaImage> {
    // Load Category Iage from base64
    let content = read_base64_file()?;
    let category_img_path = string_to_base64(content)?;
    let category_img = image::open(category_img_path)
        .unwrap()
        .thumbnail(CATEGORY_IMAGE_SIZE, CATEGORY_IMAGE_SIZE)
        .to_rgba8();

    Ok(category_img)
}
