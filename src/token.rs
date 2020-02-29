pub use std::collections::HashMap;
use parser::from_map::FromMap;

#[derive(Clone, Debug)]
pub enum Token {
    MemberAccess,
    LogicOr,
    LogicAnd,
    Add,
    Subtract,
    FloatDivision,
    FloorDivision,
    NamespaceOrStaticAccess,
    Multiply,
    Dereference,
    Token(String)
}

impl FromMap<Token> for Token {
    fn from(map: &HashMap<&str, Self>, data: String) -> Self {
        println!("{:?} {}", map, data);
        if let Some(data) = map.get(&data[..]) {
            data.clone()
        } else {
            Token::Token(data)
        }
    }
}