use std::{fs::File, io::Write, path::PathBuf};

use anyhow::{bail, Result};
use base64::{engine::general_purpose, Engine};

pub fn string_to_base64(content: String) -> Result<PathBuf> {
    let data = base64_data(content)?.clone();

    let content = fix_base64_padding(&data.0.unwrap()).unwrap();
    let bytes = general_purpose::STANDARD.decode(content).unwrap();

    let path = PathBuf::from("generated").join(format!("category_image.{}", data.2.unwrap()));
    let mut output_file = File::create(&path).unwrap();

    output_file.write_all(&bytes).unwrap();
    Ok(path)
}

fn fix_base64_padding(base64_string: &str) -> Result<String> {
    let trimmed_string = base64_string.trim();
    let padding_chars = calculate_padding(trimmed_string.len());
    let padded_string = format!("{}{}", trimmed_string, "=".repeat(padding_chars));
    match general_purpose::STANDARD.decode(padded_string) {
        Ok(bytes) => Ok(general_purpose::STANDARD.encode(bytes)),
        Err(err) => bail!(format!("Error decoding base64: {}", err)),
    }
}

fn base64_data(data: String) -> Result<(Option<String>, Option<String>, Option<String>)> {
    let mut content = None;
    let mut file_type = None;
    let mut extension = None;

    let splited = data.split(",").collect::<Vec<&str>>();

    match splited.last() {
        Some(c) => {
            content = Some(c.to_string());
        }
        None => {}
    }
    // match splited.first() {
    //     Some(v) => {
    //         let types = v.split("/").collect::<Vec<&str>>();
    //         match types.first(){}
    //     }
    //     None => {}
    // }

    let delimiter_pos = data.find("; base64,").or_else(|| data.find(";base64,"));

    match delimiter_pos {
        Some(pos) => {
            let header_part = &data[..pos];
            let parts: Vec<&str> = header_part.splitn(2, '/').collect();
            if parts.len() == 2 {
                file_type = Some(parts[0].to_string());
                extension = Some(parts[1].to_string());
                return Ok((content, file_type, extension));
            }
        }
        None => {}
    }
    Ok((content, file_type, extension))
}

fn calculate_padding(length: usize) -> usize {
    let remainder = length % 4;
    match remainder {
        0 => 0,
        1 => 3,
        2 => 2,
        3 => 1,
        _ => unreachable!(), // This shouldn't happen with valid base64 strings
    }
}
