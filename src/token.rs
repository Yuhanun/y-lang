use parser::from_map::FromMap;
pub use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
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
    VariableDefinition,
    LogicEquals,
    Assignment,
    ExpressionEnd,
    TypeAnnotation,
    StringDelimiter,
    Token(String),
}

impl FromMap<Token> for Token {
    fn from(map: &HashMap<&str, Self>, data: String) -> Self {
        if let Some(data) = map.get(&data[..]) {
            data.clone()
        } else {
            Token::Token(data)
        }
    }

    fn skip(&self) -> bool {
        match self {
            Token::Token(val) => val.is_empty(),
            _ => false,
        }
    }
}
