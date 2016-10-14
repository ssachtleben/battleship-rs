use board::Board as Board;

pub static CHARACTERS: &'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

pub fn get_character(number: usize) -> char {
    return CHARACTERS.chars().skip(number - 1).take(1).next().unwrap();
}

pub fn get_coords(board: &Board, string: &str) -> Option<(usize, usize)> {
    if string.len() < 2 || string.len() > 3 {
        println!("Invalid input");
        return None;
    }
    let mut x: usize = 1;
    let first_char = String::from(string).to_uppercase().chars().skip(0).take(1).next().unwrap();
    for current_char in CHARACTERS.chars().skip(0).take(board.width) {
        if current_char == first_char {
            let y: usize = string.split_at(1).1.parse::<usize>().unwrap();
            return Some((x, y));
        }
        x = x + 1;
    }
    return None;
}
