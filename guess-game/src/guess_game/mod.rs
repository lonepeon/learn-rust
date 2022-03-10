pub mod game;
pub mod input;

#[derive(Debug, PartialEq, Eq)]
pub enum Hint {
    TooSmall,
    TooBig,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Status {
    Victory,
    Miss(Hint),
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Guess(u32);

impl Guess {
    pub fn against(&self, expected: &u32) -> Status {
        match self.0.cmp(expected) {
            std::cmp::Ordering::Less => Status::Miss(Hint::TooSmall),
            std::cmp::Ordering::Greater => Status::Miss(Hint::TooBig),
            std::cmp::Ordering::Equal => Status::Victory,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn too_small() {
        assert_eq!(Status::Miss(Hint::TooSmall), Guess(20).against(&50));
    }

    #[test]
    fn too_big() {
        assert_eq!(Status::Miss(Hint::TooBig), Guess(80).against(&50));
    }

    #[test]
    fn equal() {
        assert_eq!(Status::Victory, Guess(50).against(&50));
    }
}
