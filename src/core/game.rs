use models::player::Player as Player;

use std::collections::LinkedList;

pub static BOARD_WIDTH: usize = 10;
pub static BOARD_HEIGHT: usize = 10;

pub struct Game {
    players: LinkedList<Player>
}

impl Game {
    pub fn new() -> Game {
        let mut players: LinkedList<Player> = LinkedList::new();
        players.push_back(Player::new("Player 1"));
        Game {
            players: players
        }
    }

    pub fn get_players(&mut self) -> &mut LinkedList<Player> {
        &mut self.players
    }

    pub fn run(&mut self) {
        loop {
            for player in self.get_players() {
                player.print();
                if player.is_finish() {
                    println!("You are done!");
                    break;
                }
                player.input();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use core::game::Game as Game;

    #[test]
    fn get_players() {
        assert_eq!(1, Game::new().get_players().len());
    }
}
