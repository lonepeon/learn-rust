use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..1024);
    let word = wordle::dictionary::pick_word(index);

    wordle::cli::play(word);
}
