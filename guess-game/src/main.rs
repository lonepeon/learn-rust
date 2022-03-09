mod guess_game;

use guess_game::game;

fn main() {
    let g = game::Game::new(42);
    let tries = vec![12, 45, 42, 10];

    for trial in &tries {
        match g.guess(&trial) {
            Err(game::Hint::TooSmall) => println!("too small! try again..."),
            Err(game::Hint::TooBig) => println!("too big! try again..."),
            Ok(()) => {
                println!("you won");
                break;
            }
        }
    }
}
