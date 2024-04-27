use image::{Pixel, Rgba, RgbaImage};
use rusttype::{Font, Point, Scale};

pub fn draw_left_align(
    img: &mut RgbaImage,
    x: i32,
    y: i32,
    color: Rgba<u8>,
    scale: Scale,
    font: &Font,
    text: &str,
) {
    let v_metrics = font.v_metrics(scale);
    let offset = rusttype::point(x as f32, y as f32 + v_metrics.ascent);
    // println!("l: {:?}", offset);
    // Layout the text
    let glyphs: Vec<_> = font.layout(text, scale, offset).collect();

    // Get the dimensions of the image
    let (width, height) = img.dimensions();
    // println!("l w: {}, h: {}", width, height);

    // Draw each glyph onto the image
    for g in glyphs {
        if let Some(bb) = g.pixel_bounding_box() {
            g.draw(|x, y, v| {
                let px = x as i32 + bb.min.x;
                let py = y as i32 + bb.min.y;
                // Adjust intensity value to range from 0 to 255 (8-bit alpha)
                let alpha = (v * 255.0) as u8;
                // Check if pixel position is within image bounds
                if px >= 0 && px < width as i32 && py >= 0 && py < height as i32 {
                    // Get mutable reference to the pixel at (px, py)
                    let pixel = img.get_pixel_mut(px as u32, py as u32);
                    // Blend color with specified alpha value onto the pixel
                    pixel.blend(&image::Rgba([color.0[0], color.0[1], color.0[2], alpha]));
                }
            });
        }
    }
}

pub fn draw_right_align(
    img_width: u32,
    img: &mut RgbaImage,
    x: i32,
    y: i32,
    color: Rgba<u8>,
    scale: Scale,
    font: &Font,
    text: &str,
) {
    // Layout the text
    let text_glyph: Vec<_> = font.layout(text, scale, Point { x: 0.0, y: 0.0 }).collect();
    let text_width = text_glyph
        .iter()
        .map(|x| x.position().x)
        .collect::<Vec<f32>>();
    println!("->>: {:?}", text_width);

    let text_width = text_width.last().unwrap();
    println!("<>: {:?}", text_width);

    let v_metrics = font.v_metrics(scale);
    let offset = rusttype::point(
        -x as f32 + (img_width) as f32 - text_width - 10.0,
        y as f32 + v_metrics.ascent,
    );
    println!("r: {:?}", scale);

    // Layout the text
    let glyphs: Vec<_> = font.layout(text, scale, offset).collect();

    // Get the dimensions of the image
    let (width, height) = img.dimensions();

    // Draw each glyph onto the image
    for g in glyphs {
        if let Some(bb) = g.pixel_bounding_box() {
            g.draw(|x, y, v| {
                let px = x as i32 + bb.min.x;
                let py = y as i32 + bb.min.y;
                // Adjust intensity value to range from 0 to 255 (8-bit alpha)
                let alpha = (v * 255.0) as u8;
                // Check if pixel position is within image bounds
                if px >= 0 && px < width as i32 && py >= 0 && py < height as i32 {
                    // Get mutable reference to the pixel at (px, py)
                    let pixel = img.get_pixel_mut(px as u32, py as u32);
                    // Blend color with specified alpha value onto the pixel
                    pixel.blend(&image::Rgba([color.0[0], color.0[1], color.0[2], alpha]));
                }
            });
        }
    }
}
