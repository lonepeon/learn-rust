#[derive(Debug, PartialEq, Eq)]
pub enum Hint {
    TooSmall,
    TooBig,
}

pub struct Game {
    expected: u32,
}

impl Game {
    pub fn new(value: u32) -> Game {
        Game { expected: value }
    }

    pub fn guess(&self, value: u32) -> Result<(), Hint> {
        match value.cmp(&self.expected) {
            std::cmp::Ordering::Less => Err(Hint::TooSmall),
            std::cmp::Ordering::Greater => Err(Hint::TooBig),
            std::cmp::Ordering::Equal => Ok(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn too_small() {
        assert_eq!(Hint::TooSmall, Game::new(50).guess(20).unwrap_err());
    }

    #[test]
    fn too_big() {
        assert_eq!(Hint::TooBig, Game::new(50).guess(80).unwrap_err());
    }

    #[test]
    fn equal() {
        Game::new(50).guess(50).expect("guess should be correct");
    }
}
