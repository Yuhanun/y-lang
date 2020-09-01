use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum LexifierSymbol {
    Colon,
    Mutable,
    Addition,
    Semicolon,
    Assignment,
    Subtraction,
    VariableDefinition,
    Token(String),
    Integer(usize),
    Comment(String),
    Literal(String),
    FloatingPoint(f64),
    Unknown,
}

lazy_static! {
    static ref SYMBOL_MAPPER: HashMap<&'static str, LexifierSymbol> = {
        let mut data = HashMap::new();
        data.insert(";", LexifierSymbol::Semicolon);
        data.insert("+", LexifierSymbol::Addition);
        data.insert("-", LexifierSymbol::Subtraction);
        data.insert("let", LexifierSymbol::VariableDefinition);
        data.insert(":", LexifierSymbol::Colon);
        data.insert("mut", LexifierSymbol::Mutable);
        data.insert("=", LexifierSymbol::Assignment);
        data
    };
}

fn into_string(data: &[u8]) -> String {
    String::from_utf8(data.to_owned()).unwrap()
}

fn with_none_variant(data: String) -> LexifierSymbol {
    let num = data.parse::<usize>();
    let float = data.parse::<f64>();
    let data = data.as_bytes().to_owned();
    // Comment
    if data.len() > 1 && data[0] == tokenizer::tokens::SLASH && data[1] == tokenizer::tokens::SLASH
    {
        LexifierSymbol::Comment(into_string(&data[2..]))
    // string literal
    } else if data[0] == tokenizer::tokens::QUOTATION
        && data[data.len() - 1] == tokenizer::tokens::QUOTATION
    {
        LexifierSymbol::Literal(into_string(&data[1..data.len() - 1]))
    // integer
    } else if let Ok(val) = num {
        LexifierSymbol::Integer(val)
    // floating point
    } else if let Ok(val) = float {
        LexifierSymbol::FloatingPoint(val)
    } else {
        LexifierSymbol::Token(into_string(&data[..]))
    }
}

pub fn get_symbol(data: String) -> LexifierSymbol {
    match SYMBOL_MAPPER.get(data.as_str()) {
        Some(symbol) => symbol.clone(),
        None => with_none_variant(data),
    }
}
