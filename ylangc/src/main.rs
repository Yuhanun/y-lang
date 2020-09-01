use lexifier;
use preparser;
use tokenizer;

fn main() {
    let tokenizer = tokenizer::tokenizer::Tokenizer::from_file("tests/variable.yl").unwrap();
    let res = tokenizer.tokenize().unwrap();

    println!("{:#?}", res);

    let lexifier = lexifier::lexifier::Lexifier::from(res);
    let res = lexifier.lexify();

    println!("{:#?}", res);

    let preparser = preparser::preparser::PreParser::from(res);
    let res = preparser.preparse();

    println!("{:#?}", res);

    let parser = parser::parser::Parser::from(res);
    let res = parser.parse();

    println!("{:#?}", res);
}
