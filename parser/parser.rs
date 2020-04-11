use crate::from_map::FromMap;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

#[derive(Clone)]
pub struct Parser<T>
where
    T: FromMap<T> + std::fmt::Debug + std::clone::Clone + std::cmp::PartialEq,
{
    content: String,
    index: usize,
    split_tokens: HashMap<&'static str, T>,
    tokens: Vec<T>,
    iterator_current: usize,
}

impl<T> Iterator for Parser<T>
where
    T: FromMap<T> + std::fmt::Debug + std::clone::Clone + std::cmp::PartialEq,
{
    type Item = T;

    fn next(&mut self) -> std::option::Option<T> {
        if self.iterator_current < self.tokens.len() {
            let mut result = self.tokens[self.iterator_current].clone();
            self.iterator_current += 1;
            while result.skip() && self.iterator_current < self.tokens.len() {
                result = self.tokens[self.iterator_current].clone();
                self.iterator_current += 1;
            }
            Some(result)
        } else {
            None
        }
    }
}

impl<T> Parser<T>
where
    T: FromMap<T> + std::fmt::Debug + std::clone::Clone + std::cmp::PartialEq,
{
    pub fn new(data: String, split_tokens: HashMap<&'static str, T>) -> Self {
        let mut data = Self {
            content: data.clone(),
            index: 0,
            split_tokens: split_tokens,
            tokens: vec![],
            iterator_current: 0,
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

    fn next_is(&mut self, val: T) -> bool {
        if self.iterator_current >= self.tokens.len() {
            return false;
        }
        let value = self.tokens[self.iterator_current].clone();
        value == val
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

            let min = Parser::<T>::get_smallest_index(&indices).unwrap();
            println!("{:?}[{}] -> {}", tokens, min, tokens[min]);
            let str_tok = T::from(
                &self.split_tokens,
                self.content[index..indices[min]].trim().to_owned(),
            );
            let delim = T::from(&self.split_tokens, tokens[min].trim().to_owned());
            println!("string token = {:?}, delim = {:?}", str_tok, delim);

            if !str_tok.skip() {
                self.tokens.push(str_tok);
            }
            if !delim.skip() {
                self.tokens.push(delim);
            }
            index = indices[min] + tokens[min].len();
            println!("{}", index);
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

    pub fn skipt_to_token(&mut self, tok_type: T) -> Option<Vec<T>> {
        let mut data = vec![];
        for i in self.iterator_current..self.tokens.len() {
            self.iterator_current += 1;
            data.push(self.tokens[i].clone());
            if self.tokens[i] == tok_type {
                return Some(data);
            }
        }
        None
    }
}
