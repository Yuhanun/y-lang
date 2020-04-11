pub use parser::parser::Parser;

pub use std::collections::HashMap;

mod token;
mod variables;
pub use token::Token;

fn main() {
    let mut variables: HashMap<String, String> = HashMap::new();

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
    tokens.insert("let", Token::VariableDefinition);
    tokens.insert("==", Token::LogicEquals);
    tokens.insert("=", Token::Assignment);
    tokens.insert(";", Token::ExpressionEnd);
    tokens.insert(":", Token::TypeAnnotation);
    // modulo
    // +=
    // /=
    // ...
    //

    let mut tokens = Parser::<Token>::from("test.y", tokens).unwrap();
    let mut current_token = tokens.next();
    while let Some(tok) = current_token {
        println!("{:?}", tok);
        match tok {
            Token::VariableDefinition => {
                let toks = tokens.skipt_to_token(Token::ExpressionEnd);
                println!("skipt_to_token: {:?}", toks);

                let result = variables::variable_decl(toks, &mut variables);
                println!("{:?}", result);
            }
            _ => {}
        }
        current_token = tokens.next();
    }
}
