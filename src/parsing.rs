use std::{
    io,
};

use indexmap::IndexMap;

pub trait Interchange {
    type Output;

    fn parse(&self) -> io::Result<Structure>;
    // fn reconstruct(ir: Structure) -> io::Result<Self::Output>;
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

    fn add_item(&mut self, name: String, value: Value) {
        self.content.insert(name, value);
    }
}

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

    fn parse(&self) -> io::Result<Structure> {
        let lines = self.content.lines();
        let mut structure = Structure::new();
        let mut current_obj: Option<Structure> = None;
        let mut current_obj_name = String::new();

        for (_i, line) in lines.enumerate() {
            if line.is_empty() { continue; }
            if line.starts_with("[") {
                if let Some(section) = &mut current_obj {
                    structure.add_item(current_obj_name.clone(), Value::Object(section.clone()));
                    current_obj = None;
                }

                current_obj = Some(Structure::new());
                current_obj_name = line.strip_prefix("[")
                    .unwrap()
                    .strip_suffix("]")
                    .map(|line| line.to_string())
                    .unwrap();
            } else {
                let parts: Vec<&str> = line.split('=').collect();
                let name = parts.get(0).unwrap().trim().to_string();
                let value = parts.get(1).unwrap().trim().to_string();


                if let Some(section) = &mut current_obj {
                    section.add_item(name, Value::String(value));
                } else {
                    structure.add_item(name, Value::String(value));
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
    fn parse(&self) -> io::Result<Structure> {
        Ok(Structure::new())
    }
}