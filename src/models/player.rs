use models::board::Board as Board;
use models::fleet::Fleet as Fleet;
use models::ship::Direction as Direction;
use utils::stringutil as Stringutil;

use ansi_term::Colour;
use std::collections::LinkedList;
use std::io::stdin;

pub static BOARD_WIDTH: usize = 10;
pub static BOARD_HEIGHT: usize = 10;

pub struct Player {
    name: String,
    board: Board,
    fleet: Fleet,
    hits: LinkedList<(usize, usize)>
}

impl Player {
    pub fn new(name: &str) -> Player {
        let board: Board = Board::new(BOARD_WIDTH, BOARD_HEIGHT);
        let mut fleet: Fleet = Fleet::new();
        fleet.create(&board);
        Player {
            name: String::from(name),
            board: board,
            fleet: fleet,
            hits: LinkedList::new()
        }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_board(&self) -> &Board {
        &self.board
    }

    pub fn get_fleet(&mut self) -> &mut Fleet {
        &mut self.fleet
    }

    pub fn get_hits(&mut self) -> &mut LinkedList<(usize, usize)> {
        &mut self.hits
    }

    pub fn input(&mut self) {
        println!("Enter a attack point (for example A1):");
        let mut line = String::new();
        let input = stdin().read_line(&mut line);
        let guess: Option<&str> = input.ok().map_or(None, |_| Some(line.trim()));
        match guess {
            Some(s) => { self.attack(s); }
            None => {} // TODO: can this be dropped somehow?
        }
    }

    pub fn attack(&mut self, string: &str) -> bool {
        match convert_to_point(self.get_board(), &string) {
            None => return false,
            Some(c) => {
                let coords: (usize, usize) = (c.0, c.1);
                if !&self.get_hits().contains(&coords) {
                    &self.get_hits().push_back(coords);
                    println!("");
                    println!("{}. attack on {} {:?}", self.get_hits().len(), string, coords);
                    return true;
                }
                return false;
            }
        }
    }

    pub fn is_finish(&mut self) -> bool {
        let hits: LinkedList<(usize, usize)> = self.get_hits().clone(); // TODO: this sucks
        for ship in self.get_fleet().get_ships().iter() {
            for i in 0..ship.get_length() {
                let position_x = if ship.is_direction(Direction::Horizontal) { ship.get_position().get_x() + i } else { ship.get_position().get_x() };
                let position_y = if ship.is_direction(Direction::Horizontal) { ship.get_position().get_y() } else { ship.get_position().get_y() + i };
                if !hits.contains(&(position_x, position_y)) {
                    return false;
                }
            }
        }
        return true;
    }

    pub fn print_board_line(prefix : &str, ) {

    }

    pub fn repeat_line(content : &str, amount: usize) -> String {
        let s: String = String::from(content);
        // (0..amount).fold("", |x, i| (s.clone() + x).as_str())
        // (0..amount).fold(String::new(), |x, i| format!("{}{}", x, s.clone())).as_str()
        let mut ret_str = String::new();
        for i in 0..amount {
            ret_str.push_str(content);
        }
        return ret_str;
    }

    pub fn print(&mut self) {
        //print!("{}[2J", 27 as char);
        println!("Battleship - Board: {}x{}", self.get_board().get_width(), self.get_board().get_height());
        println!("");
        println!("Name: {}", self.get_name());
        println!("");

        let prefix = "     |";
        let prefix_full = "|----|";

        print!("{}", Colour::Fixed(244).paint(prefix));
        for i in 0..self.get_board().get_width() {
            print!("{}", Colour::Fixed(244).paint("---|"));
        }
        println!("");

        print!("{}", Colour::Fixed(244).paint(prefix));
        for i in Stringutil::get_alphabet().chars().skip(0).take(self.get_board().get_width()) {
            print!(" {} ", Colour::Fixed(244).bold().paint(i.to_string()));
            print!("{}", Colour::Fixed(244).paint("|"));
        }
        println!("");

        print!("{}", Colour::Fixed(244).paint(prefix_full));
        for i in 0..self.get_board().get_width() {
            print!("{}", Colour::Fixed(244).paint("---|"));
        }
        println!("");

        for i in 0..self.get_board().get_height() {
            print!("{}", Colour::Fixed(244).paint("|"));
            if i < 9 {
                print!(" ");
            }
            print!(" {} ", Colour::Fixed(244).bold().paint((i + 1).to_string()));
            print!("{}", Colour::Fixed(244).paint("|"));
            for j in 0..self.get_board().get_width() {
                if self.get_hits().contains(&(j + 1, i + 1)) {
                    print!(" {} |", Colour::Red.paint("x"));
                } else {
                    let mut found: bool = false;
                    for ship in self.get_fleet().get_ships().iter() {
                        if ship.is_here(j, i) {
                            print!(" {} ", Colour::Blue.paint("A"));
                            print!("{}", Colour::Fixed(244).paint("|"));
                            found = true;
                            break;
                        }
                    }
                    if !found {
                        print!("{}", Colour::Fixed(244).paint(" - |"));
                    }
                }
            }
            println!("");

            print!("{}", Colour::Fixed(244).paint(prefix_full));
            for i in 0..self.get_board().get_width() {
                print!("{}", Colour::Fixed(244).paint("---|"));
            }
            println!("");
        }
        println!("");
    }
}

// TODO: remove this
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

#[cfg(test)]
mod tests {
    use models::player as player;
    use models::player::Player as Player;

    #[test]
    fn get_name() {
        assert_eq!(&String::from("John Doe"), Player::new("John Doe").get_name());
    }

    #[test]
    fn get_board() {
        let player: Player = Player::new("John Doe");
        assert_eq!(player::BOARD_WIDTH, player.get_board().get_width());
        assert_eq!(player::BOARD_HEIGHT, player.get_board().get_height());
    }

    #[test]
    fn get_fleet() {
        assert_eq!(5, Player::new("John Doe").get_fleet().get_ships().len());
    }

    #[test]
    fn get_hits() {
        assert_eq!(0, Player::new("John Doe").get_hits().len());
    }
}
