use rayon::prelude::*;

use lexifier::lexifier_symbol::LexifierSymbol;

use crate::preparser_symbols::PreParserSymbols;

pub struct PreParser {
    data: Vec<LexifierSymbol>,
}

pub struct PreParserError {}

impl PreParser {
    pub fn from(data: Vec<LexifierSymbol>) -> Self {
        Self { data }
    }

    ///
    /// Re-allocates because planning for multithreading which would require owned data
    ///
    pub fn split_into(data: Vec<LexifierSymbol>) -> Vec<Vec<LexifierSymbol>> {
        let mut res = vec![];
        let mut current = vec![];
        for i in 0..data.len() {
            match &data[i] {
                LexifierSymbol::Semicolon => {
                    current.push(LexifierSymbol::Semicolon);
                    res.push(current);
                    current = vec![];
                },
                other => current.push(other.clone()),
            }
        }
        res.push(current);
        res
    }

    fn process_variable(name: &String, _type: &String, value: &LexifierSymbol, mutable: bool) -> PreParserSymbols {
        match value {
            LexifierSymbol::Integer(val) => {
                PreParserSymbols::LiteralIntegerVariableDefinition(name.clone(), _type.clone(), *val, mutable)
            }
            LexifierSymbol::Literal(val) => {
                PreParserSymbols::LiteralStringVariableDefinition(name.clone(), _type.clone(), val.clone(), mutable)
            }
            LexifierSymbol::FloatingPoint(val) => {
                PreParserSymbols::LiteralFloatingPointVariableDefinition(name.clone(), _type.clone(), *val, mutable)
            }
            LexifierSymbol::Token(val) => {
                PreParserSymbols::OtherVariableDefinition(name.clone(), _type.clone(), val.clone(), mutable)
            },
            _ => PreParserSymbols::Unknown,
        }
    }

    pub fn process(input: &Vec<LexifierSymbol>) -> PreParserSymbols {
        match &input[..] {
            // Mutable variable
            [LexifierSymbol::VariableDefinition, LexifierSymbol::Mutable, LexifierSymbol::Token(name), LexifierSymbol::Colon, LexifierSymbol::Token(_type), LexifierSymbol::Assignment, value, LexifierSymbol::Semicolon] => {
                Self::process_variable(name, _type, value, true)
            },
            // Immutable variable
            [LexifierSymbol::VariableDefinition, LexifierSymbol::Token(name), LexifierSymbol::Colon, LexifierSymbol::Token(_type), LexifierSymbol::Assignment, value, LexifierSymbol::Semicolon] => {
                Self::process_variable(name, _type, value, false)
            },
            [other @ ..] => {
                println!("Other: {:#?}", other);
                let mut comments = vec![];
                for each in other {
                    if let LexifierSymbol::Comment(val) = each {
                        comments.push(val.clone());
                    }
                }
                PreParserSymbols::Comments(comments)
            },
            _ => PreParserSymbols::Unknown,
        }
    }

    pub fn preparse(self) -> Vec<PreParserSymbols> {
        Self::split_into(self.data)
            .par_iter() // <-- parallel iteration
            .map(Self::process)
            .collect()
    }
}
