use crate::{
    clcierror::ClciError,
    Interchange,
    Structure,
    Value
};

use std::{
    error::Error,
};

pub struct Ini {
    pub content: String,
}

impl Ini {
    pub fn new(content: String) -> Self {
        Self {
            content,
        }
    }
}

impl Interchange for Ini {
    type Output = Self;

    fn parse(&self) -> Result<Structure, Box<dyn Error>> {
        let lines = self.content.lines();
        let mut structure = Structure::new();
        let mut current_obj: Option<Structure> = None;
        let mut current_obj_name = String::new();

        for (i, line) in lines.enumerate() {
            if line.is_empty() | line.starts_with(";") { continue; }

            if line.starts_with("[") {
                if let Some(section) = current_obj.take() {
                    structure.add_item(current_obj_name.clone(), Value::Object(section));
                }

                current_obj = Some(Structure::new());
                current_obj_name = line.strip_prefix("[")
                    .and_then(|l| l.strip_suffix("]"))
                    .ok_or_else(|| ClciError::MalformedSectionHeader(i, line.to_string()))?
                    .to_string();
            } else {
                if let Some((key, value)) = line.split_once("=") {
                    let key = key.trim().to_string();
                    let value = value.trim().to_string();

                    if let Some(section) = &mut current_obj {
                        section.add_item(key, Value::String(value));
                    } else {
                        structure.add_item(key, Value::String(value));
                    }
                } else {
                    Err(ClciError::MissingAssignmentOperator(i, line.to_string()))?;
                }                
            }
        }

        if let Some(section) = current_obj {
            structure.add_item(current_obj_name, Value::Object(section));
        }

        dbg!(&structure);
        Ok(structure)
    }
}

impl<T> Interchange for Vec<T> {
    type Output = i32;
    fn parse(&self) -> Result<Structure, Box<dyn Error>> {
        Ok(Structure::new())
    }
}