pub struct Word<'a>(&'a str);

impl<'a> Word<'a> {
    pub fn new(s: &'a str) -> Self {
        assert_eq!(s.len(), 5, "word must be 5 characters long");
        Word(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "word must be 5 characters long")]
    fn word_is_too_short() {
        Word::new("nope");
    }

    #[test]
    #[should_panic(expected = "word must be 5 characters long")]
    fn word_is_too_long() {
        Word::new("nopeagain");
    }
}
