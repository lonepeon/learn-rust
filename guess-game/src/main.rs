mod guess_game;

use guess_game::{game, input};
use std::io;

use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let number = rng.gen_range(1..100);
    let mut console = input::Console::new(io::BufReader::new(io::stdin()), io::stdout());
    game::run(&mut console, number)
}
