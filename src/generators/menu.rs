use std::path::PathBuf;

use anyhow::Result;
use image::Rgba;
use rusttype::{Font, Scale};

use crate::{create_blank_image::blank_image, data::menus::menu_category};

use super::draw::{draw_left_align, draw_right_align};

pub fn create_menu(target: &PathBuf) -> Result<()> {
    let menu = menu_category();
    let mut img = blank_image(menu.width.unwrap() as u32, menu.height.unwrap() as u32).unwrap();

    // Load a font
    let menu_font_src = PathBuf::from("assets/Exo-Bold.otf");
    let item_font_src = PathBuf::from("assets/Exo-Regular.otf");
    let menu_font_data = std::fs::read(&menu_font_src)?;
    let item_font_data = std::fs::read(&item_font_src)?;
    let menu_font = Font::try_from_bytes(menu_font_data.as_slice())
        .unwrap_or_else(|| panic!("error constructing a Font from the given slice"));
    let item_font = Font::try_from_bytes(item_font_data.as_slice())
        .unwrap_or_else(|| panic!("error constructing a Font from the given slice"));

    // Set the font size and color
    let menu_name_size = 30.0;
    let menu_name_scale = Scale::uniform(menu_name_size);
    let item_name_size = 25.0;
    let item_name_scale = Scale::uniform(item_name_size);

    let color = Rgba([0, 0, 0, 255]);

    // Set initial position for drawing menu items
    let x = 25;
    let gap = x;
    let mut y = x;
    let currency = menu.currency.unwrap_or_else(|| "$".to_string());
    // Render each menu and its items onto the image
    for category in menu.categories.unwrap() {
        // Render menu name
        draw_left_align(
            &mut img,
            x,
            y,
            color,
            menu_name_scale,
            &menu_font,
            category
                .name
                .unwrap_or_else(|| "Name not found".to_string())
                .as_str(),
        );
        y += 30; // Increase y position for next menu items

        // Render each item in the menu
        for product in category.products.unwrap_or_default() {
            let product_name = format!(
                "{}",
                product.name.unwrap_or_else(|| "Name not found".to_string())
            );

            draw_left_align(
                &mut img,
                x,
                y,
                color,
                item_name_scale,
                &item_font,
                &product_name,
            );
            let product_price = format!("{}{}", &currency, product.price.unwrap_or_else(|| 0.0),);
            draw_right_align(
                menu.width.unwrap() as u32,
                &mut img,
                x,
                y,
                color,
                item_name_scale,
                &item_font,
                &product_price,
            );
            y += gap; // Increase y position for next menu item
        }

        y += gap; // Increase y position for next menu
    }
    img.save(target)?;
    Ok(())
}
