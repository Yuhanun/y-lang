use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use crate::from_map::FromMap;

pub struct Parser<T> 
where
    T: std::fmt::Debug
{
    content: String,
    index: usize,
    split_tokens: HashMap<&'static str, T>,
    tokens: Vec<T>,
}

impl<T> Parser<T> 
where
    T: FromMap<T>,
    T: std::fmt::Debug
{
    pub fn new(data: String, split_tokens: HashMap<&'static str, T>) -> Self {
        let mut data = Self {
            content: data.clone(),
            index: 0,
            split_tokens: split_tokens,
            tokens: vec![],
        };
        data.tokenize();
        println!("{:?}", data.tokens);
        data
    }

    pub fn from(file_name: &str, split_tokens: HashMap<&'static str, T>) -> std::io::Result<Self> {
        let mut file = File::open(file_name)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        Ok(Self::new(content, split_tokens))
    }

    fn get_smallest_index<S>(data: &Vec<S>) -> Option<usize>
    where
        S: std::cmp::PartialOrd,
    {
        let mut smallest_index = 0;
        if data.len() == 0 {
            None
        } else {
            for i in 0..data.len() {
                if data[i] < data[smallest_index] {
                    smallest_index = i;
                }
            }
            Some(smallest_index)
        }
    }

    fn tokenize(&mut self) {
        let mut index = 0;
        while index < self.content.len() {
            println!("Index: {}", index);
            let mut indices: Vec<usize> = vec![];
            let mut tokens: Vec<String> = vec![];
            for token in &self.split_tokens {
                indices.push(
                    *self.content[index..]
                        .find(token.0)
                        .map(|i| (index + i))
                        .get_or_insert(self.content.len()),
                );
                tokens.push(token.0.to_string());
            }
            println!("Indices: {:#?}", indices);

            let min = Parser::<T>::get_smallest_index(&indices).unwrap();
            println!("Min: {}", min);
            self.tokens.push(T::from(&self.split_tokens, tokens[min].clone()));
            self.tokens
                .push(T::from(&self.split_tokens, self.content[index..indices[min]].to_string()));
            index = indices[min] + tokens[min].len();
            println!("Index: {}", index);
        }
    }

    pub fn skip(&mut self, count: usize) {
        self.index += count;
    }

    pub fn read(&mut self, count: usize) -> Option<String> {
        let data = String::from(self.content.get(self.index..count)?);
        self.skip(count);
        Some(data)
    }
}
