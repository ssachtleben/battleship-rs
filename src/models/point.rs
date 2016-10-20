use utils::stringutil as Stringutil;

pub struct Point {
    x: usize,
    y: usize
}

impl Point {
    pub fn new(x: usize, y: usize) -> Point {
        Point {
            x: x,
            y: y
        }
    }

    pub fn get_x(&self) -> usize {
        self.x
    }

    pub fn get_y(&self) -> usize {
        self.y
    }

    pub fn from_string(string: &str) -> Option<Point> {
        if string.len() < 2 || string.len() > 3 {
            println!("Invalid input");
            return None;
        }
        let first_char = String::from(string).to_uppercase().chars().skip(0).take(1).next().unwrap();
        match Stringutil::get_position_in_alphabet(first_char) {
            Some(x) => {
                match string.split_at(1).1.parse::<usize>() {
                    Ok(y) => return Some(Point::new(x, y)),
                    Err(_) => return None
                }
            }
            None => return None
        }
    }
}

#[cfg(test)]
mod tests {
    use models::point::Point as Point;

    #[test]
    fn get_x() {
        assert_eq!(3, Point::new(3, 5).get_x());
    }

    #[test]
    fn get_y() {
        assert_eq!(5, Point::new(3, 5).get_y());
    }

    #[test]
    fn from_string() {
        assert_eq!(true, Point::from_string("C5").is_some());
        assert_eq!(true, Point::from_string("CC").is_none());
        assert_eq!(true, Point::from_string("55").is_none());
        let point: Point = Point::from_string("C5").unwrap();
        assert_eq!(3, point.get_x());
        assert_eq!(5, point.get_y());
    }
}
