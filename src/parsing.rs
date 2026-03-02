use std::{
    io,
    collections::HashMap,
};

pub trait Convert: Sized {
    type Output;

    fn to_ir(self) -> io::Result<Structure>;
    // fn from_ir(ir: Structure) -> io::Result<Self::Output>;
}

pub struct Structure {
    content: Vec<Value>
}

pub enum Value {
    Integer(i64),
    Float(f64),
    String(String),
    Bool(bool),
    Array(Vec<Value>),
    Object(Vec<HashMap<String, String>>),
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
    fn to_ir(self) -> io::Result<Structure> {
        Ok(Structure { content: vec![] })
    }
}