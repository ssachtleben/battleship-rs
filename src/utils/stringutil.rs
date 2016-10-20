static CHARACTERS: &'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

pub fn get_alphabet() -> &'static str {
    return CHARACTERS;
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

#[cfg(test)]
mod tests {
    use utils::stringutil as Stringutil;

    #[test]
    fn get_character_in_alphabet() {
        assert_eq!(Some('A'), Stringutil::get_character_in_alphabet(1));
        assert_eq!(None, Stringutil::get_character_in_alphabet(27));
    }

    #[test]
    fn get_position_in_alphabet() {
        assert_eq!(Some(5), Stringutil::get_position_in_alphabet('E'));
        assert_eq!(None, Stringutil::get_position_in_alphabet('!'));
    }
}
