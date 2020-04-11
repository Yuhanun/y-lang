pub use crate::token::Token;
pub use std::collections::HashMap;

pub fn insert_variable(
    name: String,
    _type: String,
    vars: &mut HashMap<String, String>,
) -> Result<String, String> {
    if vars.contains_key(&name) {
        return Err(format!(
            "A variable with name {:?} already exists, has type {:?}",
            name, _type
        ));
    }
    vars.insert(name.clone(), _type);
    Ok(name)
}

///
/// Parses the following syntax:
/// let {name}: {T} = 5;
/// let {name}: T;
/// let {name} : T;
///
pub fn variable_decl(
    tokens: Option<Vec<Token>>,
    variables: &mut HashMap<String, String>,
) -> Result<String, String> {
    let final_name: String;
    let final_type: String;
    let final_value: Option<String>;

    if let None = tokens {
        return Err(format!("No matching `;` found for variable declaration"));
    }
    let mut tokens = tokens.unwrap().into_iter();

    let name = tokens.next();
    let _type_colon = tokens.next();
    let _type = tokens.next();
    let assignment_or_end = tokens.next();
    let value_or_end = tokens.next();
    let end = tokens.next();
    // assert_eq!(_let_keyword, Token::VariableDefinition);

    if let None = name {}
    let name = name.unwrap();
    if let Token::Token(nm) = name.clone() {
        final_name = nm;
    } else {
        return Err(format!("Expected variable name, found {:?}", name));
    }

    if let None = _type_colon {}
    let _type_colon = _type_colon.unwrap();
    if let Token::TypeAnnotation = _type_colon {
    } else {
        return Err(format!("Expected `:`, found {:?}", final_name.clone()));
    }

    if let None = _type {}
    let _type = _type.unwrap();
    if let Token::Token(val) = _type {
        final_type = val;
    } else {
        return Err(format!("Expected type after `:`, found {:?}", name));
    }

    if let None = assignment_or_end {}
    let assignment_or_end = assignment_or_end.unwrap();

    if let None = value_or_end {}
    let value_or_end = value_or_end.unwrap();

    match value_or_end {
        Token::Token(val) => {
            final_value = Some(val);
            println!("value = {:?}", final_value);
            if let None = end {
                return Err(format!("Expected `;`, found end of file"));
            }
            let end = end.unwrap();

            if let Token::ExpressionEnd = end {
                insert_variable(final_name, final_type, variables)
            } else {
                return Err(format!("Expected ;, found {:?}", end));
            }
        }
        Token::ExpressionEnd => {
            insert_variable(final_name, final_type, variables)
        }
        val => {
            return Err(format!(
                "Expected some value or `ExpressionEnd`, found {:?}",
                val
            ));
        }
    }
}
