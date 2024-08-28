mod game;
mod actor;
mod math;
mod component;
mod asteroid;
mod circle_component;
mod texture;
mod random;
mod move_component;
mod sprite_component;
mod shader;

use crate::game::Game;

fn main() {
    let mut game = Game::new();
    game.run_loop();
}

