fn main() {
    wordle::cli::play(
        &mut std::io::BufReader::new(std::io::stdin()),
        &mut std::io::stdout(),
    );
}
