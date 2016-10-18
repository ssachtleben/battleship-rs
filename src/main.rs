extern crate ansi_term;
extern crate rand;
extern crate terminal_size;

use terminal_size::{Width, Height, terminal_size};

use core::game::Game as Game;

pub mod core;
pub mod models;
pub mod utils;

fn main() {
    test_terminal_size();
    Game::new().run();
}

fn test_terminal_size() {
    let size = terminal_size();
    if let Some((Width(w), Height(h))) = size {
        println!("Your terminal is {} cols wide and {} lines tall", w, h);
    } else {
        println!("Unable to get terminal size");
    }
}
