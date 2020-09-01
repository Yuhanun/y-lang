use crate::ast;

use crate::preparser::preparser_symbols::PreParserSymbols;

pub struct Parser {
    data: Vec<PreParserSymbols>,
}

impl Parser {
    pub fn from(data: Vec<PreParserSymbols>) -> Self {
        Parser{ data }
    }

    pub fn parse(self) -> ast::AST {
        ast::AST{}
    }
}