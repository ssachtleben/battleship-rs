use models::point::Point as Point;

#[derive(PartialEq)]
pub enum Direction {
    Horizontal, Vertical
}

pub struct Ship {
    position: Point,
    direction: Direction,
    length: usize
}

impl Ship {
    pub fn new(position_x: usize, position_y: usize, direction: Direction, length: usize) -> Ship {
        Ship {
            position: Point::new(position_x, position_y),
            direction: direction,
            length: length
        }
    }

    pub fn get_position(&self) -> &Point {
        &self.position
    }

    pub fn get_direction(&self) -> &Direction {
        &self.direction
    }

    pub fn get_length(&self) -> usize {
        self.length
    }

    pub fn is_direction(&self, direction: Direction) -> bool {
        self.direction == direction
    }

    pub fn is_here(&self, x: usize, y:usize) -> bool {
        if self.is_direction(Direction::Horizontal) {
            return self.get_position().get_x() <= x && self.get_position().get_x()
                + self.get_length() > x && self.get_position().get_y() == y;
        }
        return self.get_position().get_x() == x && self.get_position().get_y() <= y
            && self.get_position().get_y() + self.get_length() > y;
    }
}

#[cfg(test)]
mod tests {
    use models::ship::Direction as Direction;
    use models::ship::Ship as Ship;

    #[test]
    fn get_position() {
        let ship: Ship = Ship::new(3, 5, Direction::Horizontal, 5);
        assert_eq!(3, ship.get_position().get_x());
        assert_eq!(5, ship.get_position().get_y());
    }

    #[test]
    fn get_direction() {
        let ship: Ship = Ship::new(1, 1, Direction::Horizontal, 5);
        assert_eq!(true, ship.is_direction(Direction::Horizontal)); // TODO: use get_direction func
    }

    #[test]
    fn is_direction() {
        let ship: Ship = Ship::new(1, 1, Direction::Horizontal, 5);
        assert_eq!(true, ship.is_direction(Direction::Horizontal));
    }

    #[test]
    fn get_length() {
        let ship: Ship = Ship::new(1, 1, Direction::Horizontal, 5);
        assert_eq!(5, ship.get_length());
    }

    #[test]
    fn is_here() {
        let ship1: Ship = Ship::new(2, 5, Direction::Horizontal, 3);
        assert_eq!(true, ship1.is_here(2, 5));
        assert_eq!(true, ship1.is_here(4, 5));
        assert_eq!(false, ship1.is_here(1, 5));
        assert_eq!(false, ship1.is_here(5, 5));
        let ship2: Ship = Ship::new(2, 5, Direction::Vertical, 3);
        assert_eq!(true, ship2.is_here(2, 5));
        assert_eq!(true, ship2.is_here(2, 7));
        assert_eq!(false, ship2.is_here(2, 4));
        assert_eq!(false, ship2.is_here(2, 8));
    }
}
