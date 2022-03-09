use std::io;

pub struct Console<T: io::BufRead, U: io::Write> {
    input: T,
    output: U,
    buffer: String,
}

impl<T: io::BufRead, U: io::Write> Console<T, U> {
    pub fn new(input: T, output: U) -> Console<T, U> {
        Console {
            input,
            output,
            buffer: String::new(),
        }
    }

    pub fn read_guess(&mut self) -> Result<u32, std::num::ParseIntError> {
        self.buffer.clear();

        self.input
            .read_line(&mut self.buffer)
            .expect("can't read input from STDIN");

        self.buffer.trim().parse::<u32>()
    }

    pub fn println(&mut self, sentence: &str) {
        writeln!(&mut self.output, "{}", sentence).expect("cannot write into STDOUT");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_valid_int() {
        let mut output = Vec::with_capacity(10);
        let value = Console::new("12\n".as_bytes(), &mut output)
            .read_guess()
            .expect("can't get integer value");

        assert_eq!(12, value);
    }

    #[test]
    #[should_panic(expected = "kind: InvalidDigit")]
    fn read_invalid_int() {
        let mut output = Vec::with_capacity(10);
        Console::new("nope\n".as_bytes(), &mut output)
            .read_guess()
            .unwrap();
    }

    #[test]
    fn println() {
        let mut output = Vec::with_capacity(10);
        Console::new("".as_bytes(), &mut output).println("hello");

        assert_eq!(
            "hello\n",
            String::from_utf8(output).expect("cannot extract string from output")
        );
    }
}
