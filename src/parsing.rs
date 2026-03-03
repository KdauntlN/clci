use std::{
    io,
    collections::HashMap,
};

pub trait Convert: Sized {
    type Output;

    fn parse(self) -> io::Result<Structure>;
    // fn reconstruct(ir: Structure) -> io::Result<Self::Output>;
}

#[derive(Debug, Clone)]
pub struct Structure {
    content: HashMap<String, Value>
}

impl Structure {
    fn new() -> Self {
        Self {
            content: HashMap::new(),
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

impl Convert for Ini {
    type Output = Self;

    fn parse(self) -> io::Result<Structure> {
        let lines = self.content.lines();
        let mut structure = Structure::new();
        let mut current_obj: Option<Structure> = None;
        let mut current_obj_name = String::new();

        for (i, line) in lines.enumerate() {
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

            }
        }

        if let Some(section) = current_obj {
            structure.add_item(current_obj_name, Value::Object(section));
        }

        dbg!(&structure);
        Ok(structure)
    }
}