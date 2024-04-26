use std::path::PathBuf;

use image::{Pixel, Rgba, RgbaImage};
use rusttype::{Font, Scale};

fn main() {
    // Load your image
    let src = PathBuf::from("assets/pxfuel.jpg");
    let font_src = PathBuf::from("assets/Freedom-10eM.ttf");
    let target = PathBuf::from("generated");

    let mut img: RgbaImage = image::open(src).unwrap().to_rgba8();

    // Define the text to write
    let text = "Hello, Rust!";

    // Load a font
    let font_data = std::fs::read(&font_src).expect("Unable to read font file");
    let font = Font::try_from_bytes(font_data.as_slice())
        .unwrap_or_else(|| panic!("error constructing a Font from the given slice"));

    // Set the font size
    let font_size = 80.0;
    let scale = Scale::uniform(font_size);

    // Set the position where you want to draw the text
    let x = 350;
    let y = 750;

    // Set the color of the text (in this case, white)
    let text_color = Rgba([255, 255, 255, 255]);

    // Render the text onto the image
    draw_text(&mut img, x, y, text_color, scale, &font, text);

    // Save the modified image
    img.save(target.join("output_image.png")).unwrap();
}

fn draw_text(
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
