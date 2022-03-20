use std::io;

pub fn play<R: io::BufRead, W: io::Write>(reader: &mut R, writer: &mut W) {
    let mut game = crate::game::Game::new(crate::word::Word::new(['r', 'i', 'g', 'h', 't']));
    let mut input = String::new();

    loop {
        input.clear();
        if let Err(e) = reader.read_line(&mut input) {
            writeln!(writer, "invalid input line: {}", e).expect("to output error");
            continue;
        }

        let guess = input.trim();
        if guess.len() != 5 {
            writeln!(writer, "guess must be 5 characters long")
                .expect("to output validation error");
            continue;
        }

        let input_guess = [
            guess
                .chars()
                .nth(0)
                .expect("cannot get character at position 0"),
            guess
                .chars()
                .nth(1)
                .expect("cannot get character at position 1"),
            guess
                .chars()
                .nth(2)
                .expect("cannot get character at position 2"),
            guess
                .chars()
                .nth(3)
                .expect("cannot get character at position 3"),
            guess
                .chars()
                .nth(4)
                .expect("cannot get character at position 4"),
        ];

        match game.guess(input_guess) {
            crate::game::State::Lost => {
                writeln!(writer, "you lost the game").expect("can't display lost message");
                return;
            }
            crate::game::State::Win => {
                writeln!(writer, "you won the game in {} tries", game.tries())
                    .expect("can't display victory message");
                return;
            }
            crate::game::State::Missed => {
                for entry in game.history.iter() {
                    for guessed_letter in entry.guessed_letters() {
                        let color = match guessed_letter.state {
                            crate::word::State::Misplaced => "33",
                            crate::word::State::Exact => "32",
                            crate::word::State::Absent => "39",
                        };

                        write!(
                            writer,
                            "\x1b[1;{}m{}\u{20DE}\x1b[0;39m ",
                            color,
                            guessed_letter.letter.to_uppercase()
                        )
                        .expect("display the guessed letter");
                    }
                    writeln!(writer, "").expect("newline at end of line");
                }
            }
        };
    }
}
