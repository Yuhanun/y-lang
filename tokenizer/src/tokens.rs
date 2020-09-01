pub const QUOTATION: u8 = b'"';
pub const SPACE: u8 = b' ';
pub const NEWLINE: u8 = b'\n';
pub const SLASH: u8 = b'/';
pub const SEMICOLON: u8 = b';';

pub const KEEP_TOKENS: &[u8] = &[SEMICOLON];
pub const SPLIT_ON: &[u8] = &[SPACE, NEWLINE, SEMICOLON];
pub const ADD_TOKENIZATION: &[u8] = &[QUOTATION];
