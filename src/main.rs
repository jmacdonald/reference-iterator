struct TokenIteratorData {
    data: String
}

impl<'a> TokenIteratorData {
    pub fn new(data: String) -> TokenIteratorData {
        TokenIteratorData{ data: data }
    }

    pub fn iter(&'a self) -> TokenIterator<'a> {
        TokenIterator::new(&self.data)
    }
}

struct TokenIterator<'a> {
    data: &'a str,
    token_start: usize,
    token_end: usize,
}

impl<'a> TokenIterator<'a> {
    pub fn new(data: &'a str) -> TokenIterator<'a> {
        TokenIterator{
            data: data,
            token_start: 0,
            token_end: 0
        }
    }
}

impl<'a> Iterator for TokenIterator<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        // Move beyond the previous token, if any.
        self.token_start = self.token_end;

        // Try to find a non-whitespace character that
        // will denote the beginning of another token.
        let next_char_index = self.data[self.token_start..]
            .char_indices()
            .find(|&(_, c)| !c.is_whitespace())
            .map(|(i, _)| i + self.token_start);

        if let Some(start_index) = next_char_index {
            // We've found the start of another token.
            self.token_start = start_index;

            // Find the end of the token, whether that
            // is whitespace or the end of the string.
            self.token_end = self.data[self.token_start..]
                .char_indices()
                .find(|&(_, c)| c.is_whitespace())
                .map(|(i, _)| i + self.token_start)
                .unwrap_or(self.data.len());

            Some(&self.data[self.token_start..self.token_end])
        } else {
            None
        }
    }
}

fn main() {
    let iterator = TokenIteratorData::new("iterator data".to_string());

    for token in iterator.iter() {
        println!("{}", token);
    }
}
