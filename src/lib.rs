pub mod parsing;
pub mod cli;
pub mod clcierror;

use parsing::{
    Interchange,
    Ini
};

use clcierror::ClciError;

use std::{
    error::Error,
    fs,
};

pub fn open_file(filepath: String) -> Result<impl Interchange, Box<dyn Error>> {
    let split_filepath: Vec<&str> = filepath.split(".").collect();

    let (extension, _rest) = split_filepath
        .split_last()
        .ok_or_else(|| ClciError::InvalidFileTypeError("".to_string()))?;

    let file_content = fs::read_to_string(&filepath)?;

    match extension {
        &"ini" => {
            Ok(Ini::new(file_content))
        },
        _ => Err(ClciError::InvalidFileTypeError(extension.to_string()))?
    }
}