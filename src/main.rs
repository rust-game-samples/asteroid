mod game;
mod actor;
mod math;
mod component;

use crate::game::Game;

fn main() {
    let mut game = Game::new();
    game.run_loop();
}

