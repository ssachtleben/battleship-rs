use board::Board as Board;
use fleet::Fleet as Fleet;
use utils;

use std::collections::LinkedList;
use ansi_term::Colour;

static BOARD_WIDTH: usize = 10;
static BOARD_HEIGHT: usize = 10;

pub struct Game {
    pub board: Board,
    pub fleet: Fleet,
    hits: LinkedList<(usize, usize)>
}

impl Game {
    pub fn new() -> Game {
        let mut game: Game = Game {
            board: Board::new(BOARD_WIDTH, BOARD_HEIGHT),
            fleet: Fleet::new(),
            hits: LinkedList::new()
        };
        game.fleet.create(&game.board);
        return game;
    }

    pub fn update(&mut self) {
        self.print();
    }

    pub fn attack(&mut self, string: &str) -> bool {
        match utils::get_coords(&self.board, &string) {
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
            for i in 0..ship.length {
                let position_x = if ship.horizontal { ship.position.0 + i } else { ship.position.0 };
                let position_y = if ship.horizontal { ship.position.1 } else { ship.position.1 + i };
                if !self.hits.contains(&(position_x, position_y)) {
                    return false;
                }
            }
        }
        return true;
    }

    pub fn print(&self) {
        print!("{}[2J", 27 as char);
        println!("Battleship - Board: {}x{}", self.board.width, self.board.height);
        println!("");
        print!("   ");
        for i in utils::CHARACTERS.chars().skip(0).take(self.board.width) {
            print!("{} ", i);
        }
        println!("");
        for i in 0..self.board.height {
            if i < 9 {
                print!(" ");
            }
            print!("{} ", i + 1);
            for j in 0..self.board.width {
                if self.hits.contains(&(j + 1, i + 1)) {
                    print!("{} ", Colour::Red.paint("x"));
                } else {
                    let mut found: bool = false;
                    for ship in self.fleet.ships.iter() {
                        if ship.horizontal {
                            if ship.position.0 <= j + 1 && ship.position.0 + ship.length > j + 1 && ship.position.1 == i + 1 {
                                print!("{} ", Colour::Blue.paint("A"));
                                found = true;
                                break;
                            }
                        } else {
                            if ship.position.0 == j + 1 && ship.position.1 <= i + 1 && ship.position.1 + ship.length > i + 1 {
                                print!("{} ", Colour::Blue.paint("A"));
                                found = true;
                                break;
                            }
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
