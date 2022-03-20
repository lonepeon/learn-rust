#[derive(Debug, Eq, PartialEq)]
pub enum State {
    Lost,
    Missed,
    Win,
}

pub struct Game {
    word: crate::word::Word,
    pub history: Vec<crate::word::Guess>,
}

impl Game {
    pub fn new(word: crate::word::Word) -> Self {
        Game {
            word,
            history: Vec::new(),
        }
    }

    pub fn guess(&mut self, word: [char; 5]) -> State {
        let guess = self.word.assess(word);

        self.history.push(guess);

        if guess.is_guessed() {
            return State::Win;
        }

        if self.history.len() >= 6 {
            return State::Lost;
        }

        State::Missed
    }

    pub fn tries(&self) -> usize {
        self.history.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn guess_fill_history() {
        let mut g = Game::new(crate::word::Word::new(['r', 'i', 'g', 'h', 't']));

        assert_eq!(0, g.tries());

        g.guess(['w', 'r', 'o', 'n', 'g']);
        assert_eq!(1, g.tries());
        g.guess(['f', 'a', 'l', 's', 'e']);
        assert_eq!(2, g.tries());

        assert_eq!(['w', 'r', 'o', 'n', 'g'], g.history[0].word());
        assert_eq!(['f', 'a', 'l', 's', 'e'], g.history[1].word());
    }

    #[test]
    fn guess_win_on_match() {
        let mut g = Game::new(crate::word::Word::new(['r', 'i', 'g', 'h', 't']));

        assert_eq!(State::Win, g.guess(['r', 'i', 'g', 'h', 't']));
    }

    #[test]
    fn guess_lose_after_6_tries() {
        let mut g = Game::new(crate::word::Word::new(['r', 'i', 'g', 'h', 't']));

        assert_eq!(State::Missed, g.guess(['f', 'a', 'l', 's', 'e']));
        assert_eq!(State::Missed, g.guess(['f', 'a', 'l', 's', 'e']));
        assert_eq!(State::Missed, g.guess(['f', 'a', 'l', 's', 'e']));
        assert_eq!(State::Missed, g.guess(['f', 'a', 'l', 's', 'e']));
        assert_eq!(State::Missed, g.guess(['f', 'a', 'l', 's', 'e']));
        assert_eq!(State::Lost, g.guess(['f', 'a', 'l', 's', 'e']));
        assert_eq!(State::Lost, g.guess(['f', 'a', 'l', 's', 'e']));
    }
}
