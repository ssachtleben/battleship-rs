extern crate rand;

use board::Board as Board;
use ship::Ship as Ship;
use utils;

use std::collections::LinkedList;
use rand::Rng;

pub struct Fleet {
    size: [usize; 5],
    pub ships: LinkedList<Ship>
}

impl Fleet {
    pub fn new() -> Fleet {
        Fleet {
            size: [5, 4, 3, 3, 2],
            ships: LinkedList::new()
        }
    }

    pub fn create(&mut self, board: &Board) {
        for ship_size in self.size.clone().iter() {
            // Todo: add logic to place ships smarter
            loop {
                let is_horizontal: bool = rand::thread_rng().gen_range(0, 2) == 0;
                let position_x = if is_horizontal { rand::thread_rng().gen_range(1, board.width - ship_size + 1) } else { rand::thread_rng().gen_range(1, board.width + 1) };
                let position_y = if is_horizontal { rand::thread_rng().gen_range(1, board.height + 1) } else { rand::thread_rng().gen_range(1, board.height - ship_size + 1) };
                if self.add_ship(&board, Ship::new(position_x, position_y, is_horizontal, ship_size + 0)) {
                    println!("Place ship: position={}{}, horizontal={}, size={}", utils::get_character(position_x), position_y, is_horizontal, ship_size);
                    break;
                } else {
                    println!("Failed to place ship: position={}{}, horizontal={}, size={}", utils::get_character(position_x), position_y, is_horizontal, ship_size);
                }
            }
        }
    }

    pub fn add_ship(&mut self, board: &Board, ship: Ship) -> bool {
        if self.ships.len() > 0 {
            let length_x = if ship.horizontal { ship.position.0 + ship.length } else { ship.position.0 + 1 };
            let length_y = if ship.horizontal { ship.position.1 + 1 } else { ship.position.1 + ship.length };
            for i in ship.position.0 - 1..length_x + 1 {
                for j in ship.position.1 - 1..length_y + 1 {
                    if self.is_valid_coords(&board, i, j) && self.is_ship_here(i, j) {
                        return false;
                    }
                }
            }
        }
        self.ships.push_back(ship);
        return true;
    }

    fn is_valid_coords(&self, board: &Board, x: usize, y: usize) -> bool {
        return x > 0 && y > 0 && x <= board.width && y <= board.height;
    }

    fn is_ship_here(&self, x: usize, y: usize) -> bool {
        for ship in self.ships.iter() {
            for i in 0..ship.length {
                let position_x = if ship.horizontal { ship.position.0 + i } else { ship.position.0 };
                let position_y = if ship.horizontal { ship.position.1 } else { ship.position.1 + i };
                if position_x == x && position_y == y {
                    return true;
                }
            }
        }
        return false;
    }
}
