use crate::tokenizer_errorvariant::TokenizerErrorVariant;

#[derive(Debug)]
pub struct TokenizerError {
    err: TokenizerErrorVariant,
}

impl TokenizerError {
    pub fn unexpected_eof() -> Self {
        Self {
            err: TokenizerErrorVariant::UnexpectedEOF,
        }
    }
}
