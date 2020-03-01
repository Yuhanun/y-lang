pub use parser::parser::Parser;

pub use std::collections::HashMap;

mod token;
pub use token::Token;

fn main() {
    // let mut data: HashMap<String, String> = HashMap::new();

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
                let variable_name = tokens.next();
                let assignment_or_end = tokens.next();
                
                if let None = variable_name {
                    println!("Expected variable name, found EOF");
                    return;
                }
                
                if let None = assignment_or_end {
                    println!("Expected either of `=`, `;`, found EOF");
                }

                let name_token = variable_name.unwrap();

                let mut variable_name = String::new();
                if let Token::Token(name) = name_token {
                    variable_name = name.clone();
                } else {
                    println!("Expected variable name, found {:?}", name_token);
                }

                let assignment_or_end = assignment_or_end.unwrap();

                match assignment_or_end {
                    Token::Assignment => {
                        let initial_value = tokens.next();
                        if let None = initial_value {
                            println!("Expected value, found None");
                            return;
                        }

                        let initial_value = initial_value.unwrap();
                        if let Token::Token(val) = initial_value {
                            println!("Variable named: {:?} = {:?}", variable_name, val);
                        } else {
                            println!("Expected value, found {:?}", initial_value);
                        }

                        let end_token = tokens.next();
                        
                        if let None = end_token {
                            println!("Expected ';', found EOF");
                            return;
                        }

                        let end_token = end_token.unwrap();

                        if let Token::ExpressionEnd = end_token {

                        } else {
                            println!("Expected ';', found {:?}", end_token);
                        }

                        
                    },
                    Token::ExpressionEnd => {
                        println!("Variable named: {:?}, no type known", variable_name);
                    },
                    val => {
                        println!("Expected either of `=`, `;`, found {:?}", val);
                        return;
                    }
                }
            }
            _ => {}
        }
        current_token = tokens.next();
    }
}
