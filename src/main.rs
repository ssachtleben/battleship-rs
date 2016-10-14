extern crate ansi_term;
extern crate rand;

use game::Game as Game;

pub mod board;
pub mod fleet;
pub mod game;
pub mod input;
pub mod ship;
pub mod utils;

fn main() {
    let mut game: Game = Game::new();
    loop {
        game.update();
        if game.is_finish() {
            println!("You are done!");
            break;
        }
        input::handle(&mut game);
    }
}
