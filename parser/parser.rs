use std::fs::File;
use std::io::prelude::*;

pub struct Parser {
    content: String,    
}

impl Parser {
    pub fn new(data: &String) -> Self {
        Self {
            content: data.clone()
        }
    }

    pub fn from(file_name: &'str) -> std::io::Result<Self> {
        let mut file = File::open(file_name)?;
        let mut content = String::new();
        file.read_to_string

    }
}