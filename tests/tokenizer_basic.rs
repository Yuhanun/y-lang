#[cfg(test)]
mod tokenizer_tests {
    #[test]
    fn basic_string_test() {
        let mut tokenizer = tokenizer::tokens::Tokenizer::new(";;;\"hello world!\";;;".to_owned());
        let tokens = tokenizer.get_tokens();
        let tokens: Vec<&str> = tokens.iter().map(|elem| &elem[..]).collect();
        assert!(match tokens.as_slice() {
            [";", ";", ";", "\"hello world!\"", ";", ";", ";"] => Ok(()),
            _ => Err(())
        }.is_ok())
    }
}