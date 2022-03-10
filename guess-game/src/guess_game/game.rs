use std::io;

use crate::guess_game::input;

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

pub struct Game<T: io::BufRead, U: io::Write> {
    console: input::Console<T, U>,
    expected: u32,
}

impl<T: io::BufRead, U: io::Write> Game<T, U> {
    pub fn new(console: input::Console<T, U>, expected: u32) -> Game<T, U> {
        Game { console, expected }
    }

    pub fn run(&mut self) {
        self.console.println("enter a valid integer:");

        loop {
            let trial = match self.console.read_guess() {
                Ok(trial) => trial,
                Err(_) => {
                    println!("can't convert input to integer.");
                    continue;
                }
            };

            match self.guess(&trial) {
                Status::Miss(Hint::TooSmall) => self.console.println("too small! try again..."),
                Status::Miss(Hint::TooBig) => self.console.println("too big! try again..."),
                Status::Victory => {
                    self.console.println("🏆 you win! 🏆");
                    break;
                }
            }
        }
    }

    pub fn guess(&self, value: &u32) -> Status {
        match value.cmp(&self.expected) {
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
    fn run() {
        let mut output = Vec::new();
        let console = input::Console::new("12\n50\n42".as_bytes(), &mut output);
        Game::new(console, 42).run();

        assert_eq!(
            "enter a valid integer:\ntoo small! try again...\ntoo big! try again...\n🏆 you win! 🏆\n",
            String::from_utf8(output).expect("cannot extract string from output")
        )
    }

    #[test]
    fn too_small() {
        let mut output = Vec::new();
        let console = input::Console::new("12\n".as_bytes(), &mut output);

        assert_eq!(
            Status::Miss(Hint::TooSmall),
            Game::new(console, 50).guess(&20)
        );
    }

    #[test]
    fn too_big() {
        let mut output = Vec::new();
        let console = input::Console::new("12\n".as_bytes(), &mut output);

        assert_eq!(
            Status::Miss(Hint::TooBig),
            Game::new(console, 50).guess(&80)
        );
    }

    #[test]
    fn equal() {
        let mut output = Vec::new();
        let console = input::Console::new("12\n".as_bytes(), &mut output);

        assert_eq!(Status::Victory, Game::new(console, 50).guess(&50));
    }
}
