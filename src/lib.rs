pub mod parsing;
pub mod cli;
pub mod clcierror;

use parsing::{
    Ini
};

use clcierror::ClciError;

use std::{
    error::Error,
    fs,
};

use indexmap::IndexMap;

#[derive(Debug, Clone)]
pub enum Value {
    Comment(String),
    Integer(i64),
    Float(f64),
    String(String),
    Bool(bool),
    Array(Vec<Value>),
    Object(Structure),
}

#[derive(Debug, Clone)]
pub struct Structure {
    content: IndexMap<String, Value>
}

impl Structure {
    fn new() -> Self {
        Self {
            content: IndexMap::new(),
        }
    }

    fn add_item(&mut self, key: String, value: Value) {
        self.content.insert(key, value);
    }
}

pub trait Interchange {
    type Output;

    fn parse(&self) -> Result<Structure, Box<dyn Error>>;
    // fn reconstruct(ir: Structure) -> io::Result<Self::Output>;
}

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