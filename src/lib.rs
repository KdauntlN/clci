pub mod parsing;
pub mod cli;
pub mod clcierror;

use parsing::Convert;
use parsing::Ini;
use clcierror::ClciError;
use std::error::Error;
use std::ffi::OsStr;
use std::path::Path;
use std::fs::File;
use std::io::Read;

pub fn open_file(filepath: &str) -> Result<impl Convert, Box<dyn Error>> {
    let extension = Path::new(filepath)
        .extension()
        .and_then(OsStr::to_str)
        .unwrap_or("");
    
    let mut file_handle = File::open(filepath)?;

    let mut file_content = String::new();

    file_handle.read_to_string(&mut file_content)?;

    match extension {
        "ini" => {
            Ok(Ini::new(file_content))
        },
        _ => Err(ClciError::InvalidFileTypeError(extension.to_string()))?
    }
}