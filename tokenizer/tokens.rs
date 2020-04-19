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
        Self {
            text,
            own_tokens: &["let", "=", ":", ";"],
            delimiters: &[" ", "\n", "\t"],
            except_between: &[("\"", "\"")],
            result: vec![],
            last_token_end: 0,
            current_index: 0,
        }
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

    pub fn push_result(&self, begin: usize, end: usize) -> Option<(usize, usize)> {
        println!(
            "Pushing: begin={}, end={}, value={}, length={}",
            begin,
            end,
            &self.text[begin..end],
            end - begin,
        );
        if self.text[begin..end].trim().is_empty() {
            return None;
        }
        Some((begin, end))
    }

    pub fn handle_between(&mut self) {
        if self.current_index >= self.text.len() {
            return;
        }
        for each in self.except_between {
            if self.text[self.current_index..].starts_with(each.0) {
                // find each.1
                // skip until each.1, while pushing that token onto self.result
                let found = self.text[self.current_index + 1..].find(each.1);
                if found.is_none() {
                    panic!("Expected {}, found EOF", each.1);
                }

                let pushable_tuple = self.push_result(
                    self.current_index,
                    self.current_index + found.unwrap() + each.1.len(),
                );

                if pushable_tuple.is_some() {
                    let pushable_tuple = pushable_tuple.unwrap();
                    self.result.push(pushable_tuple);
                    self.current_index += pushable_tuple.1;
                    self.last_token_end = self.current_index;
                }
            }
        }
    }

    pub fn handle_individual_tokens(&mut self) {
        if self.current_index >= self.text.len() {
            return;
        }
        for each in self.own_tokens {
            if self.text[self.current_index..].starts_with(each) {
                let pushable_tuple =
                    self.push_result(self.current_index, self.current_index + each.len());
                if pushable_tuple.is_some() {
                    let pushable_tuple = pushable_tuple.unwrap();
                    self.result.push(pushable_tuple);
                    self.current_index = pushable_tuple.1;
                    self.last_token_end = self.current_index;
                }
            }
        }
    }

    pub fn handle_delimiters(&mut self) {
        if self.current_index >= self.text.len() {
            return;
        }
        for each in self.delimiters {
            if self.text[self.current_index..].starts_with(each) {
                let pushable_tuple =
                    self.push_result(self.current_index, self.current_index + each.len());
                if pushable_tuple.is_some() {
                    let pushable_tuple = pushable_tuple.unwrap();
                    self.result.push(pushable_tuple);
                    self.current_index = pushable_tuple.1;
                    self.last_token_end = self.current_index;
                }
                self.current_index += 1;
            }
        }
        self.current_index += 1;
    }

    pub fn into_tokens(&mut self) -> Vec<String> {
        while self.current_index < self.text.len() {
            self.handle_between();
            self.handle_individual_tokens();
            self.handle_delimiters();
        }
        self.result
            .iter()
            .map(|elem| self.text[elem.0..elem.1].trim().to_owned())
            .collect()
    }
}
