use core::input as input;
use models::board::Board as Board;
use models::fleet::Fleet as Fleet;

use std::collections::LinkedList;

pub static BOARD_WIDTH: usize = 10;
pub static BOARD_HEIGHT: usize = 10;

pub struct Player {
    board: Board,
    fleet: Fleet,
    hits: LinkedList<(usize, usize)>
}

impl Player {
    pub fn new() -> Player {
        Player {
            board: Board::new(BOARD_WIDTH, BOARD_HEIGHT),
            fleet: Fleet::new(),
            hits: LinkedList::new()
        }
    }

    pub fn input() {
        input::handle2();
    }

    pub fn get_board(&self) -> &Board {
        &self.board
    }

    pub fn get_fleet(&self) -> &Fleet {
        &self.fleet
    }

    pub fn get_hits(&self) -> &LinkedList<(usize, usize)> {
        &self.hits
    }
}

#[test]
fn get_board_test() {
    let player: Player = Player::new();
    assert_eq!(BOARD_WIDTH, player.get_board().get_width());
    assert_eq!(BOARD_HEIGHT, player.get_board().get_height());
}

#[test]
fn get_fleet_test() {
    assert_eq!(0, Player::new().get_fleet().get_ships().len());
}

#[test]
fn get_hits_test() {
    assert_eq!(0, Player::new().get_hits().len());
}
