use core::input as input;
use models::player::Player as Player;
use models::board::Board as Board;
use models::fleet::Fleet as Fleet;
use models::ship::Direction as Direction;
use utils::stringutil as Stringutil;

use std::collections::LinkedList;
use ansi_term::Colour;

pub static BOARD_WIDTH: usize = 10;
pub static BOARD_HEIGHT: usize = 10;

pub struct Game {
    players: LinkedList<Player>,
    pub board: Board, // TODO: remove
    pub fleet: Fleet, // TODO: remove
    hits: LinkedList<(usize, usize)> // TODO: remove
}

impl Game {
    pub fn new() -> Game {
        let board: Board = Board::new(BOARD_WIDTH, BOARD_HEIGHT);
        let fleet: Fleet = Fleet::new();
        let mut game: Game = Game {
            players: LinkedList::new(),
            board: board,
            fleet: fleet,
            hits: LinkedList::new()
        };
        game.fleet.create(&game.board);
        return game;
    }

    pub fn get_players(&self) -> &LinkedList<Player> {
        &self.players
    }

    pub fn run(&mut self) {
        loop {
            // TODO: update player here
            self.print();
            if self.is_finish() {
                println!("You are done!");
                break;
            }
            input::handle(self);
        }
    }

    pub fn attack(&mut self, string: &str) -> bool {
        match convert_to_point(&self.board, &string) {
            None => return false,
            Some(c) => {
                let coords: (usize, usize) = (c.0, c.1);
                if !self.hits.contains(&coords) {
                    self.hits.push_back(coords);
                    println!("");
                    println!("{}. attack on {} {:?}", self.hits.len(), string, coords);
                    return true;
                }
                return false;
            }
        }
    }

    pub fn is_finish(&self) -> bool {
        for ship in self.fleet.ships.iter() {
            for i in 0..ship.get_length() {
                let position_x = if ship.is_direction(Direction::Horizontal) { ship.get_position().get_x() + i } else { ship.get_position().get_x() };
                let position_y = if ship.is_direction(Direction::Horizontal) { ship.get_position().get_y() } else { ship.get_position().get_y() + i };
                if !self.hits.contains(&(position_x, position_y)) {
                    return false;
                }
            }
        }
        return true;
    }

    pub fn print(&self) {
        print!("{}[2J", 27 as char);
        println!("Battleship - Board: {}x{}", self.board.get_width(), self.board.get_height());
        println!("");
        print!("   ");
        for i in Stringutil::get_alphabet().chars().skip(0).take(self.board.get_width()) {
            print!("{} ", i);
        }
        println!("");
        for i in 0..self.board.get_height() {
            if i < 9 {
                print!(" ");
            }
            print!("{} ", i + 1);
            for j in 0..self.board.get_width() {
                if self.hits.contains(&(j + 1, i + 1)) {
                    print!("{} ", Colour::Red.paint("x"));
                } else {
                    let mut found: bool = false;
                    for ship in self.fleet.ships.iter() {
                        if ship.is_here(j, i) {
                            print!("{} ", Colour::Blue.paint("A"));
                            found = true;
                            break;
                        }
                    }
                    if !found {
                        print!("{} ", "-");
                    }
                }
            }
            println!("");
        }
        println!("");
    }
}

pub fn convert_to_point(board: &Board, string: &str) -> Option<(usize, usize)> {
    if string.len() < 2 || string.len() > 3 {
        println!("Invalid input");
        return None;
    }
    let mut x: usize = 1;
    let first_char = String::from(string).to_uppercase().chars().skip(0).take(1).next().unwrap();
    for current_char in Stringutil::get_alphabet().chars().skip(0).take(board.get_width()) {
        if current_char == first_char {
            let y: usize = string.split_at(1).1.parse::<usize>().unwrap();
            return Some((x, y));
        }
        x = x + 1;
    }
    return None;
}

#[test]
fn get_players_test() {
    assert_eq!(0, Game::new().get_players().len());
}
