use models::fleet::Fleet as Fleet;
use models::point::Point as Point;

pub struct Board {
    width: usize,
    height: usize,
    fleet: Fleet
}

impl Board {
    pub fn new(width: usize, height: usize) -> Board {
        let fleet: Fleet = Fleet::new();
        Board {
            width: width,
            height: height,
            fleet: fleet
        }
    }

    pub fn get_width(&self) -> usize {
        return self.width;
    }

    pub fn get_height(&self) -> usize {
        return self.height;
    }

    pub fn get_fleet(&self) -> &Fleet {
        return &self.fleet;
    }

    pub fn is_valid(&self, x: usize, y: usize) -> bool {
        return x > 0 && y > 0 && x <= self.get_width() && y <= self.get_height();
    }

    pub fn is_valid_point(&self, point: Point) -> bool {
        return self.is_valid(point.get_x(), point.get_y());
    }
}

#[test]
fn get_width_test() {
    assert_eq!(3, Board::new(3, 5).get_width());
}

#[test]
fn get_height_test() {
    assert_eq!(5, Board::new(3, 5).get_height());
}

#[test]
fn get_fleet_test() {
    assert_eq!(0, Board::new(3, 5).get_fleet().get_ships().len());
}

#[test]
fn is_valid_test() {
    let board: Board = Board::new(10, 10);
    assert_eq!(true, board.is_valid(1, 1));
    assert_eq!(true, board.is_valid(10, 10));
    assert_eq!(false, board.is_valid(0, 0));
    assert_eq!(false, board.is_valid(1, 0));
    assert_eq!(false, board.is_valid(0, 1));
    assert_eq!(false, board.is_valid(11, 11));
    assert_eq!(false, board.is_valid(11, 10));
    assert_eq!(false, board.is_valid(10, 11));
}
