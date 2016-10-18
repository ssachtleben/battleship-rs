static CHARACTERS: &'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

pub fn get_alphabet() -> String {
    return String::from(CHARACTERS);
}

pub fn get_character_in_alphabet(number: usize) -> Option<char> {
    return get_alphabet().chars().skip(number - 1).take(1).next();
}

pub fn get_position_in_alphabet(val: char) -> Option<usize> {
    match get_alphabet().chars().enumerate().filter(| &(_, item) | item == val).next() {
        Some((x, _)) => return Some(x + 1),
        None => return None
    }
}

#[test]
fn get_character_in_alphabet_test() {
    assert_eq!(Some('A'), get_character_in_alphabet(1));
    assert_eq!(None, get_character_in_alphabet(27));
}

#[test]
fn get_position_in_alphabet_test() {
    assert_eq!(Some(5), get_position_in_alphabet('E'));
    assert_eq!(None, get_position_in_alphabet('!'));
}
