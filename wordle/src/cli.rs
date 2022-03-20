use std::io;

pub struct Display<W: io::Write> {
    out: W,
}

impl<W: io::Write> Display<W> {
    pub fn new(out: W) -> Self {
        Display { out }
    }

    pub fn print_hint(&mut self, guess: &crate::word::Guess) {
        for h in guess.guessed_letters() {
            self.print_top_cell(h);
        }
        writeln!(self.out).unwrap();

        for h in guess.guessed_letters() {
            self.print_content_cell(h);
        }
        writeln!(self.out).unwrap();

        for h in guess.guessed_letters() {
            self.print_bottom_cell(h);
        }
        writeln!(self.out).unwrap();
    }

    pub fn print_msg(&mut self, msg: String) {
        writeln!(self.out, "{}", msg).unwrap();
    }

    pub fn print_error(&mut self, msg: String) {
        self.print(31, msg);
        writeln!(self.out).unwrap();
    }

    fn print_top_cell(&mut self, hint: &crate::word::Hint) {
        let (color, _) = Self::extract_color_and_letter(hint);
        self.print(color, " ╔═══╗ ".to_string())
    }

    fn print_content_cell(&mut self, hint: &crate::word::Hint) {
        let (color, letter) = Self::extract_color_and_letter(hint);
        self.print(color, format!(" ║ {} ║ ", letter.to_uppercase()))
    }

    fn print_bottom_cell(&mut self, hint: &crate::word::Hint) {
        let (color, _) = Self::extract_color_and_letter(hint);
        self.print(color, " ╚═══╝ ".to_string())
    }

    fn print(&mut self, color: u32, content: String) {
        write!(self.out, "\x1b[{}m{}\x1b[39m", color, content).unwrap();
    }

    fn extract_color_and_letter(hint: &crate::word::Hint) -> (u32, &char) {
        match hint {
            crate::word::Hint::Misplaced(c) => (33, c),
            crate::word::Hint::Exact(c) => (32, c),
            crate::word::Hint::Absent(c) => (39, c),
        }
    }
}

pub fn play<R: io::BufRead, W: io::Write>(reader: &mut R, writer: &mut W) {
    let mut game = crate::game::Game::new(crate::word::Word::new(['R', 'I', 'G', 'H', 'T']));
    let mut input = String::new();
    let mut display = Display::new(writer);

    loop {
        input.clear();
        if let Err(e) = reader.read_line(&mut input) {
            display.print_error(format!("invalid input line: {}", e));
            continue;
        }

        let guess = input.trim().to_uppercase();
        if guess.len() != 5 {
            display.print_error("guess must be 5 characters long".to_string());
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

        let rst = game.guess(input_guess);
        for entry in game.history.iter() {
            display.print_hint(entry);
        }

        match rst {
            crate::game::State::Lost => {
                display.print_msg("you lost the game".to_string());
                return;
            }
            crate::game::State::Win => {
                display.print_msg(format!("you won the game in {} tries", game.tries()));
                return;
            }
            crate::game::State::Missed => {}
        };
    }
}
