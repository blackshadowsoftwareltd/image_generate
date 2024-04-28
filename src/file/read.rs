use anyhow::Result;
use std::{
    fs::{self},
    path::PathBuf,
};

pub fn read_base64_file() -> Result<String> {
    let file_path = PathBuf::from("assets/image-base64.md");
    let contents = fs::read_to_string(file_path)?;
    Ok(contents)
}
