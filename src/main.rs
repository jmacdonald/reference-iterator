mod tokenize {
    pub fn tokenize(data: &str) -> Iter {
        Iter {
            data: Some(data),
        }
    }

    pub struct Iter<'a> {
        data: Option<&'a str>,
    }

    impl<'a> Iterator for Iter<'a> {
        type Item = &'a str;

        fn next(&mut self) -> Option<Self::Item> {
            if self.data.is_none() { return None; }
            let d = self.data.take().unwrap();

            // Try to find a non-whitespace character that
            // will denote the beginning of another token.
            let next_char_index = d
                .char_indices()
                .find(|&(_, c)| !c.is_whitespace())
                .map(|(i, _)| i);

            if let Some(start_index) = next_char_index {
                // We've found the start of another token.
                let (_, d) = d.split_at(start_index);

                // Find the end of the token, whether that
                // is whitespace or the end of the string.
                let end_index = d
                    .char_indices()
                    .find(|&(_, c)| c.is_whitespace())
                    .map(|(i, _)| i)
                    .unwrap_or(d.len());

                let (ret, d) = d.split_at(end_index);
                self.data = Some(d);
                Some(ret)
            } else {
                None
            }
        }
    }

    pub fn tokenize_mut(data: &mut str) -> IterMut {
        IterMut {
            data: Some(data),
        }
    }

    pub struct IterMut<'a> {
        data: Option<&'a mut str>,
    }

    impl<'a> Iterator for IterMut<'a> {
        type Item = &'a mut str;

        fn next(&mut self) -> Option<Self::Item> {
            if self.data.is_none() { return None; }
            let d = self.data.take().unwrap();

            // Try to find a non-whitespace character that
            // will denote the beginning of another token.
            let next_char_index = d
                .char_indices()
                .find(|&(_, c)| !c.is_whitespace())
                .map(|(i, _)| i);

            if let Some(start_index) = next_char_index {
                // We've found the start of another token.
                let (_, d) = d.split_at_mut(start_index);

                // Find the end of the token, whether that
                // is whitespace or the end of the string.
                let end_index = d
                    .char_indices()
                    .find(|&(_, c)| c.is_whitespace())
                    .map(|(i, _)| i)
                    .unwrap_or(d.len());

                let (ret, d) = d.split_at_mut(end_index);
                self.data = Some(d);
                Some(ret)
            } else {
                None
            }
        }
    }
}

fn main() {
    let tokens: Vec<&str> = tokenize::tokenize("iterator data").collect();
    for token in tokens {
        println!("{}", token);
    }

    let mut string = "mutable iterator data".to_string();
    let tokens: Vec<&mut str> = tokenize::tokenize_mut(&mut string).collect();
    for token in tokens {
        use std::ascii::AsciiExt;
        token.make_ascii_uppercase();
        println!("{}", token);
    }
}
