pub mod constants;
pub mod create_blank_image;
pub mod data;
pub mod file;
pub mod generators;
pub mod structs;

use crate::generators::menu::create_menu;
use std::path::PathBuf;

fn main() {
    let target = PathBuf::from("generated").join("output_image.png");
    create_menu(&target).unwrap();
}
