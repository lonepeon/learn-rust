use std::io;

use crate::guess_game::input;

#[derive(Debug, PartialEq, Eq)]
pub enum Hint {
    TooSmall,
    TooBig,
}

pub struct Game<T: io::BufRead, U: io::Write> {
    console: input::Console<T, U>,
    expected: u32,
}

impl<T: io::BufRead, U: io::Write> Game<T, U> {
    pub fn new(console: input::Console<T, U>, expected: u32) -> Game<T, U> {
        Game { console, expected }
    }

    pub fn run(&mut self) {
        loop {
            let trial = match self.console.read_guess() {
                Ok(trial) => trial,
                Err(_) => {
                    println!("can't convert input to integer. enter a valid integer.");
                    continue;
                }
            };

            match self.guess(&trial) {
                Err(Hint::TooSmall) => self.console.println("too small! try again..."),
                Err(Hint::TooBig) => self.console.println("too big! try again..."),
                Ok(()) => {
                    self.console.println("🏆 you win! 🏆");
                    break;
                }
            }
        }
    }

    pub fn guess(&self, value: &u32) -> Result<(), Hint> {
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
    fn run() {
        let mut output = Vec::with_capacity(10);
        let console = input::Console::new("12\n50\n42".as_bytes(), &mut output);
        Game::new(console, 42).run();

        assert_eq!(
            "too small! try again...\ntoo big! try again...\n🏆 you win! 🏆\n",
            String::from_utf8(output).expect("cannot extract string from output")
        )
    }

    #[test]
    fn too_small() {
        let mut output = Vec::with_capacity(10);
        let console = input::Console::new("12\n".as_bytes(), &mut output);
        assert_eq!(
            Hint::TooSmall,
            Game::new(console, 50).guess(&20).unwrap_err()
        );
    }

    #[test]
    fn too_big() {
        let mut output = Vec::with_capacity(10);
        let console = input::Console::new("12\n".as_bytes(), &mut output);
        assert_eq!(Hint::TooBig, Game::new(console, 50).guess(&80).unwrap_err());
    }

    #[test]
    fn equal() {
        let mut output = Vec::with_capacity(10);
        let console = input::Console::new("12\n".as_bytes(), &mut output);
        Game::new(console, 50)
            .guess(&50)
            .expect("guess should be correct");
    }
}
