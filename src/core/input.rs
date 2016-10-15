use std::io::stdin;

use core::game::Game as Game;

pub fn handle(game: &mut Game) {
    println!("Enter a attack point (for example A1):");

    let mut line = String::new();
    let input = stdin().read_line(&mut line);

    let guess: Option<&str> = input.ok().map_or(None, |_| Some(line.trim()));
    match guess {
        None => println!("Invalid input"),
        Some(s) => if game.attack(s) {
            // do something
        }
    }
}
