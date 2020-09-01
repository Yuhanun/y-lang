use crate::tokenizer_error::TokenizerError;
use crate::tokens;

#[derive(Debug)]
pub struct Tokenizer {
    data: String,
}

impl Into<Result<Vec<String>, TokenizerError>> for Tokenizer {
    fn into(self) -> Result<Vec<String>, TokenizerError> {
        self.tokenize()
    }
}

impl Tokenizer {
    pub fn from_file(filename: &str) -> std::io::Result<Self> {
        let contents = std::fs::read_to_string(filename)?;
        Ok(Self {
            data: contents
        })
    }

    pub fn from(data: String) -> Self {
        Self { data }
    }

    pub fn into_string(data: &[u8]) -> String {
        String::from_utf8(data.to_owned()).unwrap()
    }

    pub fn skip_to(c: u8, curr_slice: &[u8]) -> Result<(&[u8], String), TokenizerError> {
        let pos = curr_slice[1..].iter().position(|e| *e == c);
        if pos.is_none() {
            return Err(TokenizerError::unexpected_eof());
        }
        let idx = pos.unwrap();
        Ok((
            &curr_slice[idx + 1..],
            Self::into_string(
                &curr_slice[..if tokens::ADD_TOKENIZATION.contains(&c) {
                    idx + 2
                } else {
                    idx + 1
                }],
            ),
        ))
    }

    pub fn tokenize(self) -> Result<Vec<String>, TokenizerError> {
        let char_arr: Vec<u8> = self.data.into_bytes().into_iter().collect();

        let mut res = vec![];

        let mut curr_slice = char_arr.as_slice();

        while !curr_slice.is_empty().clone() {
            if curr_slice[0] == tokens::QUOTATION {
                // Find until next quotation and push
                let (slice, data) = Self::skip_to(tokens::QUOTATION, &mut curr_slice)?;
                res.push(data);
                curr_slice = &slice[1..];
                continue;
            }
            if curr_slice[0] == tokens::SLASH && curr_slice[1] == tokens::SLASH {
                // Find until newline and push
                let (slice, data) = Self::skip_to(tokens::NEWLINE, &mut curr_slice)?;
                res.push(data);
                curr_slice = slice;
                continue;
            }
            let indexes: Vec<usize> = tokens::SPLIT_ON
                .iter()
                .map(|e| curr_slice.iter().position(|s_e| *s_e == *e))
                .filter(|e| e.is_some())
                .map(|e| e.unwrap())
                .collect();

            if indexes.is_empty() {
                res.push(Self::into_string(curr_slice));
                break;
            }

            let min_idx = *indexes.iter().min().unwrap();

            res.push(Self::into_string(&curr_slice[..min_idx]));
            if tokens::KEEP_TOKENS.contains(&curr_slice[min_idx]) {
                res.push(String::from(curr_slice[min_idx] as char));
            }
            curr_slice = &curr_slice[min_idx + 1..];
        }

        // remove empty strings
        Ok(res.into_iter().filter(|e| !e.trim().is_empty()).collect())
    }
}
