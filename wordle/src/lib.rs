#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum LetterLocation {
    Misplaced,
    Exact,
    Absent,
}

impl Default for LetterLocation {
    fn default() -> Self {
        LetterLocation::Absent
    }
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

pub struct Word([char; 5]);

impl Word {
    pub fn new(s: [char; 5]) -> Self {
        Word(s)
    }

    pub fn assess(&self, guess: &[char; 5]) -> [LetterLocation; 5] {
        let mut distribution = WordDistribution::new();
        self.0.iter().for_each(|c| distribution.incr(c.to_owned()));

        let mut rst = [LetterLocation::Absent; 5];

        for i in 0..self.0.len() {
            if guess[i] == self.0[i] {
                rst[i] = LetterLocation::Exact;
                distribution.decr(guess[i].to_owned());
            }
        }

        for (guess_index, guess_char) in guess.iter().enumerate() {
            if rst[guess_index] != LetterLocation::Absent {
                continue;
            }

            self.0.iter().any(|expected_char| {
                if expected_char == guess_char && distribution.decr(guess_char.to_owned()) {
                    rst[guess_index] = LetterLocation::Misplaced;
                    return true;
                }

                false
            });
        }

        rst
    }
}

#[cfg(test)]
mod tests {
    use super::{
        LetterLocation::{Absent, Exact, Misplaced},
        *,
    };

    #[test]
    fn guess_match() {
        let rst = Word::new(['a', 'b', 'c', 'd', 'e']).assess(&['a', 'b', 'c', 'd', 'e']);
        assert_guess([Exact; 5], rst);
    }

    #[test]
    fn guess_no_match() {
        let rst = Word::new(['a', 'b', 'c', 'd', 'e']).assess(&['f', 'g', 'h', 'i', 'j']);
        assert_guess([Absent; 5], rst);
    }

    #[test]
    fn guess_misplaced() {
        let rst = Word::new(['a', 'b', 'c', 'd', 'e']).assess(&['b', 'c', 'd', 'e', 'a']);
        assert_guess([Misplaced; 5], rst);
    }

    #[test]
    fn guess_a_bit_of_everything() {
        let rst = Word::new(['a', 'b', 'c', 'd', 'e']).assess(&['a', 'b', 'd', 'c', 'f']);
        assert_guess([Exact, Exact, Misplaced, Misplaced, Absent], rst);
    }

    #[test]
    fn guess_same_letters_multiple_times() {
        let rst = Word::new(['a', 'b', 'a', 'd', 'e']).assess(&['a', 'a', 'b', 'c', 'a']);
        assert_guess([Exact, Misplaced, Misplaced, Absent, Absent], rst);
    }

    fn assert_guess(want: [LetterLocation; 5], got: [LetterLocation; 5]) {
        for (i, expected) in want.iter().enumerate() {
            assert_eq!(expected, &got[i], "invalid letter value at index {}", i);
        }
    }
}
