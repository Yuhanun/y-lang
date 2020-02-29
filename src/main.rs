pub use parser::parser::Parser;

pub use std::collections::HashMap;

mod token;
pub use token::Token;

fn main() {
    let mut tokens = HashMap::new();
    tokens.insert(".", Token::MemberAccess);
    tokens.insert("||", Token::LogicOr);
    tokens.insert("&&", Token::LogicAnd);
    tokens.insert("+", Token::Add);
    tokens.insert("-", Token::Subtract);
    tokens.insert("/", Token::FloatDivision);
    tokens.insert("//", Token::FloorDivision);
    tokens.insert("::", Token::NamespaceOrStaticAccess);
    tokens.insert("*", Token::Multiply);
    tokens.insert("->", Token::Dereference);
    // modulo
    // +=
    // /= 
    // ...
    // 

    Parser::<Token>::new(String::from("test.hello||world+test&&hello"), tokens);
}
