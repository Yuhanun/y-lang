use crate::lexifier_symbol;

use rayon::prelude::*;

pub struct Lexifier {
    data: Vec<String>,
}

impl Lexifier {
    pub fn from(data: Vec<String>) -> Self {
        Self { data }
    }

    pub fn lexify(self) -> Vec<lexifier_symbol::LexifierSymbol> {
        self.data
            .par_iter()
            .map(|e| lexifier_symbol::get_symbol(e.to_string()))
            .collect()
    }
}
