use tokenizer;

fn main() {
    let tokenizer =
        tokenizer::tokenizer::Tokenizer::from_file("tests/variable.yl").unwrap();
    let res = tokenizer.tokenize().unwrap();

    println!("{:#?}", res);
}
