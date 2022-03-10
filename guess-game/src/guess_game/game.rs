use std::io;

use crate::guess_game::{input, Hint, Status};

pub fn run<T: io::BufRead, U: io::Write>(console: &mut input::Console<T, U>, expected: u32) {
    console.println("enter a valid integer:");

    loop {
        let guess = match console.read_guess() {
            Ok(guess) => guess,
            Err(_) => {
                println!("can't convert input to integer.");
                continue;
            }
        };

        match guess.against(&expected) {
            Status::Miss(Hint::TooSmall) => console.println("too small! try again..."),
            Status::Miss(Hint::TooBig) => console.println("too big! try again..."),
            Status::Victory => {
                console.println("🏆 you win! 🏆");
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_valid() {
        let mut output = Vec::new();
        let mut console = input::Console::new("12\n50\n42".as_bytes(), &mut output);
        run(&mut console, 42);

        assert_eq!(
            "enter a valid integer:\ntoo small! try again...\ntoo big! try again...\n🏆 you win! 🏆\n",
            String::from_utf8(output).expect("cannot extract string from output")
        )
    }
}
