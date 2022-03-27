use rand::Rng;

fn main() {
    let words = [['R', 'I', 'G', 'H', 'T']];
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..words.len());

    wordle::cli::play(words[index]);
}
