#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum State {
    Misplaced,
    Exact,
    Absent,
}

struct FrequencyDistribution {
    pub value: char,
    pub counter: i32,
}

struct WordDistribution(Vec<FrequencyDistribution>);

impl WordDistribution {
    pub fn new() -> Self {
        WordDistribution(Vec::new())
    }

    pub fn incr(&mut self, c: char) {
        for mut frequency in self.0.iter_mut() {
            if frequency.value == c {
                frequency.counter += 1;
                return;
            }
        }

        self.0.push(FrequencyDistribution {
            value: c,
            counter: 1,
        });
    }

    pub fn decr(&mut self, c: char) -> bool {
        for mut counter in self.0.iter_mut() {
            if counter.value != c {
                continue;
            }

            if counter.counter - 1 < 0 {
                return false;
            }
            counter.counter -= 1;
            return true;
        }

        false
    }
}

#[derive(Clone, Copy)]
pub struct GuessLetter {
    pub state: State,
    pub letter: char,
}

#[derive(Clone, Copy)]
pub struct Guess([GuessLetter; 5]);

impl Guess {
    pub fn word(&self) -> [char; 5] {
        self.0.map(|g| g.letter)
    }

    pub fn is_guessed(&self) -> bool {
        for guess_letter in self.0 {
            if guess_letter.state != State::Exact {
                return false;
            }
        }

        true
    }
}

pub struct Word([char; 5]);

impl Word {
    pub fn new(s: [char; 5]) -> Self {
        Word(s)
    }

    pub fn assess(&self, guess: &[char; 5]) -> Guess {
        let mut distribution = WordDistribution::new();
        self.0.iter().for_each(|c| distribution.incr(c.to_owned()));

        let mut mask = [State::Absent; 5];

        for i in 0..self.0.len() {
            if guess[i] == self.0[i] {
                mask[i] = State::Exact;
                distribution.decr(guess[i].to_owned());
            }
        }

        for (guess_index, guess_char) in guess.iter().enumerate() {
            if mask[guess_index] != State::Absent {
                continue;
            }

            self.0.iter().any(|expected_char| {
                if expected_char == guess_char && distribution.decr(guess_char.to_owned()) {
                    mask[guess_index] = State::Misplaced;
                    return true;
                }

                false
            });
        }

        let mut i = 0;
        let letters = guess.map(|letter| {
            let mask_value = mask[i];
            i += 1;
            GuessLetter {
                letter: letter.to_owned(),
                state: mask_value,
            }
        });

        Guess(letters)
    }
}

#[cfg(test)]
mod tests {
    mod word {
        use super::super::{
            State::{Absent, Exact, Misplaced},
            *,
        };

        #[test]
        fn guess_match() {
            let guess = Word::new(['a', 'b', 'c', 'd', 'e']).assess(&['a', 'b', 'c', 'd', 'e']);

            assert_guess((['a', 'b', 'c', 'd', 'e'], [Exact; 5]), guess);
        }

        #[test]
        fn guess_no_match() {
            let guess = Word::new(['a', 'b', 'c', 'd', 'e']).assess(&['f', 'g', 'h', 'i', 'j']);
            assert_guess((['f', 'g', 'h', 'i', 'j'], [Absent; 5]), guess);
        }

        #[test]
        fn guess_misplaced() {
            let guess = Word::new(['a', 'b', 'c', 'd', 'e']).assess(&['b', 'c', 'd', 'e', 'a']);
            assert_guess((['b', 'c', 'd', 'e', 'a'], [Misplaced; 5]), guess);
        }

        #[test]
        fn guess_a_bit_of_everything() {
            let guess = Word::new(['a', 'b', 'c', 'd', 'e']).assess(&['a', 'b', 'd', 'c', 'f']);
            assert_guess(
                (
                    ['a', 'b', 'd', 'c', 'f'],
                    [Exact, Exact, Misplaced, Misplaced, Absent],
                ),
                guess,
            );
        }

        #[test]
        fn guess_same_letters_multiple_times() {
            let guess = Word::new(['a', 'b', 'a', 'd', 'e']).assess(&['a', 'a', 'b', 'c', 'a']);
            assert_guess(
                (
                    ['a', 'a', 'b', 'c', 'a'],
                    [Exact, Misplaced, Misplaced, Absent, Absent],
                ),
                guess,
            );
        }

        fn assert_guess(want: ([char; 5], [State; 5]), got: Guess) {
            let mut i = 0;
            let wanted_guess_letters = want.0.map(|letter| {
                let mask_value = want.1[i];
                i += 1;
                GuessLetter {
                    letter: letter.to_owned(),
                    state: mask_value,
                }
            });

            for (i, expected) in wanted_guess_letters.iter().enumerate() {
                assert_eq!(
                    expected.state, got.0[i].state,
                    "invalid letter value at index {} for letter {}",
                    i, got.0[i].letter
                );
            }
        }
    }

    mod guess {
        use super::super::*;

        #[test]
        fn word() {
            let guess = Guess([
                GuessLetter {
                    state: State::Misplaced,
                    letter: 'r',
                },
                GuessLetter {
                    state: State::Exact,
                    letter: 'i',
                },
                GuessLetter {
                    state: State::Absent,
                    letter: 'g',
                },
                GuessLetter {
                    state: State::Exact,
                    letter: 'h',
                },
                GuessLetter {
                    state: State::Misplaced,
                    letter: 't',
                },
            ]);

            assert_eq!(['r', 'i', 'g', 'h', 't'], guess.word())
        }

        #[test]
        fn state_is_guessed_true() {
            let guess = Guess([
                GuessLetter {
                    state: State::Exact,
                    letter: 'r',
                },
                GuessLetter {
                    state: State::Exact,
                    letter: 'i',
                },
                GuessLetter {
                    state: State::Exact,
                    letter: 'g',
                },
                GuessLetter {
                    state: State::Exact,
                    letter: 'h',
                },
                GuessLetter {
                    state: State::Exact,
                    letter: 't',
                },
            ]);

            assert!(guess.is_guessed(), "word should be guessed")
        }

        #[test]
        fn state_is_guessed_false() {
            let guess = Guess([
                GuessLetter {
                    state: State::Exact,
                    letter: 'r',
                },
                GuessLetter {
                    state: State::Misplaced,
                    letter: 'i',
                },
                GuessLetter {
                    state: State::Exact,
                    letter: 'g',
                },
                GuessLetter {
                    state: State::Exact,
                    letter: 'h',
                },
                GuessLetter {
                    state: State::Exact,
                    letter: 't',
                },
            ]);

            assert!(!guess.is_guessed(), "word should not be guessed")
        }
    }
}
