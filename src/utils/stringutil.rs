static CHARACTERS: &'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

pub fn get_alphabet() -> String {
    return String::from(CHARACTERS);
}

pub fn get_character(number: usize) -> Option<char> {
    return get_alphabet().chars().skip(number - 1).take(1).next();
}

pub fn get_position_in_alphabet(val: char) -> Option<usize> {
    match get_alphabet().chars().enumerate().filter(| &(_, item) | item == val).next() {
        Some((x, _)) => return Some(x + 1),
        None => return None
    }
}

#[test]
fn test_get_position_in_alphabet_match_result() {
    assert_eq!(Some(5), get_position_in_alphabet('E'));
}

#[test]
fn test_get_position_in_alphabet_wrong_char() {
    assert_eq!(None, get_position_in_alphabet('!'));
}
