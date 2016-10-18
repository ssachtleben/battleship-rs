extern crate rand;

use models::board::Board as Board;
use models::ship::Direction as Direction;
use models::ship::Ship as Ship;
use utils::stringutil as Stringutil;

use std::collections::LinkedList;
use rand::Rng;

static FLEET_SIZE: [usize; 5] = [5, 4, 3, 3, 2];

pub struct Fleet {
    size: [usize; 5],
    pub ships: LinkedList<Ship>
}

impl Fleet {
    pub fn new() -> Fleet {
        Fleet {
            size: FLEET_SIZE,
            ships: LinkedList::new()
        }
    }

    pub fn get_size(&self) -> [usize; 5] {
        return self.size;
    }

    pub fn get_ships(&self) -> &LinkedList<Ship> {
        return &self.ships;
    }

    pub fn create(&mut self, board: &Board) {
        for ship_size in self.size.clone().iter() {
            // Todo: add logic to place ships smarter
            loop {
                let is_horizontal: bool = rand::thread_rng().gen_range(0, 2) == 0;
                let direction: Direction = if is_horizontal { Direction::Horizontal } else { Direction::Vertical };
                let position_x = if is_horizontal { rand::thread_rng().gen_range(1, board.get_width() - ship_size + 1) } else { rand::thread_rng().gen_range(1, board.get_width() + 1) };
                let position_y = if is_horizontal { rand::thread_rng().gen_range(1, board.get_height() + 1) } else { rand::thread_rng().gen_range(1, board.get_height() - ship_size + 1) };
                if self.add_ship(&board, Ship::new(position_x, position_y, direction, ship_size + 0)) {
                    println!("Place ship: position={}{}, horizontal={}, size={}", Stringutil::get_character_in_alphabet(position_x).unwrap(), position_y, is_horizontal, ship_size);
                    break;
                } else {
                    println!("Failed to place ship: position={}{}, horizontal={}, size={}", Stringutil::get_character_in_alphabet(position_x).unwrap(), position_y, is_horizontal, ship_size);
                }
            }
        }
    }

    pub fn add_ship(&mut self, board: &Board, ship: Ship) -> bool {
        if self.ships.len() > 0 {
            let length_x = if ship.is_direction(Direction::Horizontal) { ship.get_position().get_x() + ship.get_length() } else { ship.get_position().get_x() + 1 };
            let length_y = if ship.is_direction(Direction::Horizontal) { ship.get_position().get_y() + 1 } else { ship.get_position().get_y() + ship.get_length() };
            for i in ship.get_position().get_x() - 1..length_x + 1 {
                for j in ship.get_position().get_y() - 1..length_y + 1 {
                    if board.is_valid(i, j) && self.is_ship_here(i, j) {
                        return false;
                    }
                }
            }
        }
        self.ships.push_back(ship);
        return true;
    }

    fn is_ship_here(&self, x: usize, y: usize) -> bool {
        for ship in self.ships.iter() {
            for i in 0..ship.get_length() {
                let position_x = if ship.is_direction(Direction::Horizontal) { ship.get_position().get_x() + i } else { ship.get_position().get_x() };
                let position_y = if ship.is_direction(Direction::Horizontal) { ship.get_position().get_y() } else { ship.get_position().get_y() + i };
                if position_x == x && position_y == y {
                    return true;
                }
            }
        }
        return false;
    }
}

#[test]
fn get_size_test() {
    assert_eq!(FLEET_SIZE, Fleet::new().get_size());
}

#[test]
fn get_ships_test() {
    assert_eq!(true, Fleet::new().get_ships().len() == 0);
}

#[test]
fn create_test() {
    let board: Board = Board::new(10, 10);
    let mut fleet: Fleet = Fleet::new();
    fleet.create(&board);
    assert_eq!(FLEET_SIZE.len(), fleet.get_size().len());
}

#[test]
fn is_ship_here_test() {
    let board: Board = Board::new(10, 10);
    let ship: Ship = Ship::new(1, 1, Direction::Horizontal, 5);
    let mut fleet: Fleet = Fleet::new();
    fleet.add_ship(&board, ship);
    assert_eq!(true, fleet.is_ship_here(1, 1));
    assert_eq!(true, fleet.is_ship_here(5, 1));
    assert_eq!(false, fleet.is_ship_here(1, 2));
    assert_eq!(false, fleet.is_ship_here(6, 1));
}
