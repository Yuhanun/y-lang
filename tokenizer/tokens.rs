pub struct Tokenizer<'a> {
    text: String,
    own_tokens: &'a [&'a str],
    delimiters: &'a [&'a str],
    except_between: &'a [(&'a str, &'a str)],
    result: Vec<(usize, usize)>,
    last_token_end: usize,
    current_index: usize,
}

impl<'a> Tokenizer<'a> {
    pub fn new(text: String) -> Self {
        Self::from(
            text,
            &["let", "=", ":", ";"],
            &[" ", "\n", "\t"],
            &[("\"", "\"")],
        )
    }

    pub fn from(
        text: String,
        own_tokens: &'a [&'a str],
        delimiters: &'a [&'a str],
        except_between: &'a [(&'a str, &'a str)],
    ) -> Self {
        Self {
            text,
            own_tokens,
            delimiters,
            except_between,
            result: vec![],
            last_token_end: 0,
            current_index: 0,
        }
    }

    pub fn get_current_char(&self) -> char {
        return self.text.chars().skip(self.current_index).next().unwrap();
    }

    pub fn push_token(&mut self, begin: usize, end: usize) {
        self.result.push((begin, end));
    }

    pub fn push_between(&mut self, begin: usize, end: usize, bgn_tok: &str, end_tok: &str) {
        self.push_token(begin, begin + bgn_tok.len());
        self.push_token(begin + 1, end);
        self.push_token(end + 1, end + 1 + end_tok.len());
        self.current_index = end;
    }

    pub fn handle_between(&mut self) {
        let min_index = self
            .except_between
            .iter()
            .map(|elem| {
                (self.text.find(elem.0), self.text[self.current_index + 1..].find(elem.1), elem.0, elem.1)
            })
            .min();

        if let Some(item) = min_index {
            let first_index = item.0;
            let second_index = item.1;
            if first_index.is_none() {
                return;
            }
            if second_index.is_none() {
                return;
            }
            self.push_between(first_index.unwrap(), second_index.unwrap(), item.2, item.3);
        }
    }

    pub fn handle_individual_tokens(&mut self) {}

    pub fn handle_delimiters(&mut self) {}

    pub fn get_tokens(&mut self) -> Vec<String> {
        while self.current_index < self.text.len() {
            self.handle_between();
            self.handle_individual_tokens();
            self.handle_delimiters();
            self.current_index += 1;
        }
        self.result
            .iter()
            .map(|elem| self.text[elem.0..elem.1].trim().to_owned())
            .collect()
    }
}
