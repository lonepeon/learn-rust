mod guess_game;

use guess_game::{game, input};
use std::io;

fn main() {
    let console = input::Console::new(io::BufReader::new(io::stdin()), io::stdout());
    game::Game::new(console, 42).run()
}
