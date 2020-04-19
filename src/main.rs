pub use tokenizer::tokens::Tokenizer;

fn main() {
    let mut tokenizer = Tokenizer::new("let data: int = 5;".to_owned());
    let tokens = tokenizer.into_tokens();
    println!("{:#?}", tokens);
    println!("-------");

    let mut tokenizer = Tokenizer::new("\"hello world!\"".to_owned());
    let tokens = tokenizer.into_tokens();
    println!("{:#?}", tokens);
    println!("-------");

    let mut tokenizer = Tokenizer::new(";;;;\"hello world!\"".to_owned());
    let tokens = tokenizer.into_tokens();
    println!("{:#?}", tokens);
    println!("-------");
}
